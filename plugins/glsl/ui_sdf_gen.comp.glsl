// MIT License
//
// Copyright (c) 2019-2023 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#version 460

const float eps       = 1E-6;
const float frac_1_3  = 0.3333333333;
const uint  move      = 0;
const uint  curve1    = 1;
const uint  curve2    = 2;
const uint  curve3    = 3;
const uint  steps     = 0x2000;
const float step_size = 1.0 / float(steps);
const float pi        = 3.141592653589793;

struct Vertex {
	uint type;
	uint _pad0;
	vec2 p0;
	vec2 p1;
	vec2 p2;
};

layout (push_constant) uniform pushConstants {
	ivec2 offset;
	ivec2 extent;
    float scale;
    int   vertexCount;
};

layout (binding = 0) readonly buffer SSBOShape {
    Vertex vertices[ ];
};

layout (binding = 1, r8_snorm) uniform writeonly image2D sdf;

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uint square_roots(float c0, float c1, float c2, out float results[2])
{
	if (abs(c0) < eps) {
		results[0] = -c2 /c1;
		return 1;
	}
	
	float desc = c1 * c1 - (4.0 * c0 * c2);
	
	if (desc > eps) {
		results[0] = (-c1 + sqrt(desc)) / (2.0 * c0);
		results[1] = (-c1 - sqrt(desc)) / (2.0 * c0);
		return 2;
	} else if (desc < eps) {
		return 0;
	} else {
		return 0; // TODO
	}
}

uint cubic_roots(float c0, float c1, float c2, float c3, out float results[3])
{
	// check if it's a quadratic equation to avoid div by 0
	if (abs(c0) < eps) {
		float[2] res;
		uint count = square_roots(c1, c2, c3, res);
		results[0] = res[0];
		results[1] = res[1];
		return count;
	}
	
	c1 /= c0;
	c2 /= c0;
	c3 /= c0;
	
	float d0 = (3.0 * c2 - c1 * c1) / 9.0;
	float d1 = (9.0 * c1 * c2 - 27.0 * c3 - 2.0 * c1 * c1 * c1) / 54.0;
	float d = d0 * d0 * d0 + d1 * d1;
	
	if (d > eps) {
		float p = d1 + sqrt(d);
		float m = d1 - sqrt(d);
		float s = sign(p) * pow(abs(p), frac_1_3);
		float t = sign(m) * pow(abs(m), frac_1_3);
		
		if (abs(s - t) < eps && (s + t) > eps) {
			results[0] = -c1 * frac_1_3 + s + t;
			results[1] = -c1 * frac_1_3 - (s + t) * 0.5;
			return 2;
		} else {
			results[0] = -c1 * frac_1_3 + s + t;
			return 1;
		}
	} else {
		float theta = acos(d1 / sqrt(-d0 * d0 * d0));
		float d0 = 2.0 * sqrt(-d0);
		results[0] = d0 * cos(theta              * frac_1_3) - c1 * frac_1_3;
		results[1] = d0 * cos((theta + 2.0 * pi) * frac_1_3) - c1 * frac_1_3;
		results[2] = d0 * cos((theta + 4.0 * pi) * frac_1_3) - c1 * frac_1_3;
		return 2;
	}
}



float distanceCurve1(vec2 p, vec2 p0, vec2 p1) {
	vec2 d = p1 - p0;
	return distance(p, p0 + d * clamp((dot(d, p - p0) / dot(d, d)), 0.0, 1.0));
}

uint intersectionsCurve2(vec2 l0, vec2 l1, vec2 p0, vec2 p1, vec2 p2)
{
	// Convert line to normal form: ax + by + c = 0
	vec2 n = vec2(l0.y - l1.y, l1.x - l0.x); // Find normal to line: negative inverse of original line's slope
	float cl = l0.x * l1.y - l1.x * l0.y;    // Determine new c coefficient
	
	// solve quadratic equation and calculate point on bezier
	float rts[2];
	uint rtc = square_roots(
		dot(n, p0 + p1 * -2 + p2),
		dot(n, p0 * -2 + p1 * 2),
		dot(n, p0) + cl,
		rts
	);
	
	vec2 t = vec2(rts[0], rts[1]);
	vec2 b_ = mix(mix(p0.xx, p1.xx, t), mix(p1.xx, p2.xx, t), t);
	float min = min(l0.x, l1.x);
	float max = max(l0.x, l1.x);
	
	return int(rtc > 0 && t.x >= 0.0 && t.x <= 1.0 && b_.x >= min && b_.x <= max)
			+ int(rtc > 1 && t.y >= 0.0 && t.y <= 1.0 && b_.y >= min && b_.y <= max);
}

// https://stackoverflow.com/questions/563198
uint intersectionsCurve1(vec2 l0, vec2 l1, vec2 p0, vec2 p1) {
	return intersectionsCurve2(l0, l1, p0, p0 + (p1 - p0) * 0.5, p1);
}

float distanceCurve2(vec2 p, vec2 p0, vec2 p1, vec2 p2)
{
	vec2 a = p1 - p0;
	vec2 b = p2 - p1 - a;
	vec2 c = p0 - p;
	
	float rts[3];
	uint rtc = cubic_roots(
		dot(b, b),
		3.0 * dot(a, b),
		2.0 * dot(a, a) + dot(c, b),
		dot(c, a),
		rts
	);
	
	float dist = min(distance(p, p0), distance(p, p2));
	
	for (uint i = 0; i < rtc; i++)
	{
		float t = rts[i];
		if (t < 0.0 || t > 1.0) continue;
		vec2 b = mix(mix(p0, p1, t), mix(p1, p2, t), t);
		dist = min(dist, distance(p, b));
	}
	
	return dist;
}

uint intersectionsCurve3(vec2 l0, vec2 l1, vec2 p0, vec2 p1, vec2 p2, vec2 p3)
{
	// Convert line to normal form: ax + by + c = 0
	vec2 n = vec2(l0.y - l1.y, l1.x - l0.x); // Find normal to line: negative inverse of original line's slope
	float cl = l0.x * l1.y - l1.x * l0.y;    // Determine new c coefficient

	// ?Rotate each cubic coefficient using line for new coordinate system?
	// Find roots of rotated cubic
	float rts[3];
	uint rtc = cubic_roots(
		dot(n, p0 * -1.0 + p1 * 3.0 + p2 * -3.0 + p3),
		dot(n, p0 * 3.0 + p1 * -6.0 + p2 * 3.0),
		dot(n, p0 * -3.0 + p1 * 3.0),
		dot(n, p0) + cl,
		rts
	);
	
	vec3 t = vec3(rts[0], rts[1], rts[2]);
	vec3 p01 = mix(p0.xxx, p1.xxx, t);
	vec3 p12 = mix(p1.xxx, p2.xxx, t);
	vec3 p23 = mix(p2.xxx, p3.xxx, t);
	vec3 b_ = mix(mix(p01, p12, t), mix(p12, p23, t), t);
	float min = min(l0.x, l1.x);
	float max = max(l0.x, l1.x);
	
	return int(rtc > 0 && t.x >= 0.0 && t.x <= 1.0 && b_.x >= min && b_.x <= max)
			+ int(rtc > 1 && t.y >= 0.0 && t.y <= 1.0 && b_.y >= min && b_.y <= max)
			+ int(rtc > 2 && t.z >= 0.0 && t.z <= 1.0 && b_.z >= min && b_.z <= max);
}

float distanceCurve3(vec2 p, vec2 p0, vec2 p1, vec2 p2, vec2 p3)
{
	float dist = distance(p, p3);
	
	for (uint i = 0; i < steps; i++)
	{
		float t = step_size * i;
		vec2 p01 = mix(p0, p1, t);
		vec2 p12 = mix(p1, p2, t);
		vec2 p23 = mix(p2, p3, t);
		vec2 b = mix(mix(p01, p12, t), mix(p12, p23, t), t);
		dist = min(dist, distance(p, b));
	}
	
	return dist;
}

float curve2_alpha(vec2 p) {
	vec2 f = vec2(2 * p.x * p.x - p.y, 2 * p.x * p.x - p.y); // Chain rule
	float sd = (p.x * p.x - p.y) / length(f); // Signed distance
	return clamp(0.5 - sd, 0.0, 1.0); // Linear alpha + clamp
}

void main()
{
    vec2 p = vec2(gl_GlobalInvocationID.xy) / vec2(extent);	// the start point of the intersection line
    vec2 r = vec2(-1.0, p.y);								// the end point of the intersection line
    uint intersections = 0;									// the number of curves the line p, r intersects
    float distance = 1.0;							        // the distance to the closest point on a curve
	
    for (uint i = 1; i < vertexCount; i++)
		if (vertices[i].type == curve1) {
			intersections += intersectionsCurve1(p, r, vertices[i - 1].p0, vertices[i].p0);
			distance = min(distance, distanceCurve1(p, vertices[i - 1].p0, vertices[i].p0));
		} else if (vertices[i].type == curve2) {
			intersections += intersectionsCurve2(p, r, vertices[i - 1].p0, vertices[i].p1, vertices[i].p0);
			distance = min(distance, distanceCurve2(p, vertices[i - 1].p0, vertices[i].p1, vertices[i].p0));
		} else if (vertices[i].type == curve3) {
			intersections += intersectionsCurve3(p, r, vertices[i - 1].p0, vertices[i].p1, vertices[i].p2, vertices[i].p0);
			distance = min(distance, distanceCurve3(p, vertices[i - 1].p0, vertices[i].p1, vertices[i].p2, vertices[i].p0));
		}

    imageStore(
		sdf,
		ivec2(gl_GlobalInvocationID.xy) + offset,
		vec4((intersections % 2 == 0 ? distance : -distance) * scale, 0.0.rrr)
	);
}
