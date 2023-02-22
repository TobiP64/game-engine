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

struct Shape {
	uint type;
	vec4 data;
};

// optimal workgroup size, according to AMD and ARM
layout (local_size_x = 4, local_size_y = 4, local_size_z = 4) in;

#ifndef MULTI_CHANNEL
layout (binding = 0, r16f) uniform writeonly image3D sdf;
#else
layout (binding = 0, rg16f) uniform writeonly image3D sdf;
#endif

layout (binding = 1) readonly buffer data {
#ifdef NURBS_VOLUME
	uvec2 extent;
	vec4 ctrl[ ];
#elif defined NURBS_SURFACE
	uvec3 extent;
	vec4 ctl[ ];
#elif defined MESH
	vec3 vertices[ ];
#elif defined SHAPES
	Shape shapes[ ];
#endif
};

void main() {
	
}

//
// SDF functions -------------------------------------------------------------------------------------------------------
//

float dot2(in vec3 v) {
	return dot(v, v);
}

float op_union(float d1, float d2) {
	return min(d1, d2);
}

float op_subtraction(float d1, float d2) {
	return max(-d1, d2);
}

float op_intersection(float d1, float d2) {
	return max(d1, d2);
}

float op_smooth_union(float d1, float d2, float k) {
	float h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
	return mix(d2, d1, h) - k * h * (1.0 - h);
}

float op_smooth_subtraction(float d1, float d2, float k) {
	float h = clamp(0.5 - 0.5 * (d2 + d1) / k, 0.0, 1.0);
	return mix(d2, -d1, h) + k * h * (1.0 - h);
}

float op_smooth_intersection(float d1, float d2, float k) {
	float h = clamp(0.5 - 0.5 * (d2 - d1) / k, 0.0, 1.0);
	return mix(d2, d1, h) + k * h * (1.0 - h);
}

float op_round(float distance, float radius) {
	return distance - radius;
}

float sdf_sphere(vec3 p, float radius) {
	return length(p) - radius;
}

float sdf_cuboid(vec3 p, vec3 b) {
	vec3 d = abs(p) - b;
	return length(max(d, 0.0)) + min(0.0, max(d.x, max(d.y, d.z)));
}

float sdf_cylinder(vec3 p, vec3 c) {
	return length(p.xz - c.xy) - c.z;
}

// n must be normalized
float sdf_plane(vec3 p, vec4 n) {
	return dot(p, n.xyz) + n.w;
}

float udf_triangle(vec3 p, vec3 a, vec3 b, vec3 c) {
	vec3 ba = b - a;
	vec3 pa = p - a;
	vec3 cb = c - b;
	vec3 pb = p - b;
	vec3 ac = a - c;
	vec3 pc = p - c;
	vec3 nor = cross(ba, ac);
	
	return sqrt((
			sign(dot(cross(ba, nor), pa)) +
			sign(dot(cross(cb, nor), pb)) +
			sign(dot(cross(ac, nor), pc)) < 2.0)
		? min(min(
			dot2(ba * clamp(dot(ba, pa) / dot2(ba), 0.0, 1.0) - pa),
			dot2(cb * clamp(dot(cb, pb) / dot2(cb), 0.0, 1.0) - pb)),
			dot2(ac * clamp(dot(ac, pc) / dot2(ac), 0.0, 1.0) - pc))
		: dot(nor, pa) * dot(nor, pa) / dot2(nor));
}

float udf_quad(vec3 p, vec3 a, vec3 b, vec3 c, vec3 d) {
	vec3 ba = b - a;
	vec3 pa = p - a;
	vec3 cb = c - b;
	vec3 pb = p - b;
	vec3 dc = d - c;
	vec3 pc = p - c;
	vec3 ad = a - d;
	vec3 pd = p - d;
	vec3 nor = cross(ba, ad);
	
	return sqrt((
			sign(dot(cross(ba, nor), pa)) +
			sign(dot(cross(cb, nor), pb)) +
			sign(dot(cross(dc, nor), pc)) +
			sign(dot(cross(ad, nor), pd)) < 3.0)
		? min(min(min(
			dot2(ba * clamp(dot(ba, pa) / dot2(ba), 0.0, 1.0) - pa),
			dot2(cb * clamp(dot(cb, pb) / dot2(cb), 0.0, 1.0) - pb)),
			dot2(dc * clamp(dot(dc, pc) / dot2(dc), 0.0, 1.0) - pc)),
			dot2(ad * clamp(dot(ad, pd) / dot2(ad), 0.0, 1.0) - pd))
		: dot(nor, pa) * dot(nor, pa) / dot2(nor));
}
