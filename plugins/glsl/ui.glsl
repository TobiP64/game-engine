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

#define DYN_GET_MODE  (x) ((x) & 0x0000000F)
#define DYN_GET_PROPS (x) ((x) & 0x000FFFF0 << 4)
#define DYN_GET_FRAME (x) ((x) & 0xFFF00000 << 20)

#define DYN_MODE_STEP   0
#define DYN_MODE_LINEAR 1
#define DYN_MODE_CUBIC  2

#define DYN_LOOP_MODE_ONCE (x) ((x) == 0)
#define DYN_LOOP_MODE_LOOP (x) ((x) > 0)
#define DYN_LOOP_MODE_CYCLE (x) ((x) < 0)

#define DYN_PROP_INST_POS 0x01
#define DYN_PROP_INST_ROT 0x02
#define DYN_PROP_INST_SCA 0x04
#define DYN_PROP_INST_COL 0x08
#define DYN_PROP_VERT_POS 0x10
#define DYN_PROP_VERT_COL 0x20
#define DYN_PROP_VERT_TEX 0x40

struct DynMeta {
	float time;
	float weight;
};

vec3 rotate(vec3 pos, vec4 quat) {
	return pos + 2.0 * cross(quat.xyz, cross(quat.xyz, pos) + quat.w * pos);
}

vec3 mix_vec3_rbc2(float t, vec3 p0, vec3 p1, float w0, float w1) {
	return mix(p0 * w0, p1 * w1, t) / mix(w0, w1, t);
}

vec3 mix_vec3_rbc3(float t, vec3 p0, vec3 p1, vec3 p2, float w0, float w1, float w2) {
	vec3 wp0 = p0 * w0;
	vec3 wp1 = p1 * w1;
	vec3 wp2 = p2 * w2;
	vec3 q0 = mix(wp0, wp1, t);
	vec3 q1 = mix(wp1, wp2, t);
	float w01 = mix(w0, w1, t);
	float w12 = mix(w1, w2, t);
	return mix(q0, q1, t) / mix(w01, w12, t);
}

vec3 mix_vec3_rbc4(float t, vec3 p0, vec3 p1, vec3 p2, vec3 p3, float w0, float w1, float w2, float w3) {
	vec3 wp0 = p0 * w0;
	vec3 wp1 = p1 * w1;
	vec3 wp2 = p2 * w2;
	vec3 wp3 = p3 * w3;
	vec3 q0 = mix(wp0, wp1, t);
	vec3 q1 = mix(wp1, wp2, t);
	vec3 q2 = mix(wp2, wp3, t);
	vec3 r0 = mix(q0, q1, t);
	vec3 r1 = mix(q1, q2, t);
	float w01 = mix(w0, w1, t);
	float w12 = mix(w1, w2, t);
	float w23 = mix(w2, w3, t);
	float w012 = mix(w01, w12, t);
	float w123 = mix(w12, w23, t);
	return mix(r0, r1, t) / mix(w012, w123, t);
}

vec3 inter_vec3(float t, uint mode, vec3 p[4], float w[4]) {
	switch (mode) {
		default:
		case DYN_MODE_STEP: return p[int(round(t))];
		case DYN_MODE_LINEAR: return mix_vec3_rbc2(t, p[0], p[1], w[0], w[1]);
		case DYN_MODE_CUBIC: return mix_vec3_rbc3(t, p[0], p[1], p[2], w[0], w[1], w[2]);
	}
}

const vec2[3] curve_points = vec2[](vec2(0.0, 0.0), vec2(0.5, 0.0), vec2(1.0, 1.0));

#ifdef VERT

layout (location = 0) in vec2 inPos;
layout (location = 8) in mat4 instModel;
layout (location = 12) in vec4 instColor;

out gl_PerVertex {
	vec4 gl_Position;
};

#ifdef GRADIENT // -----------------------------------------------------------------------------------------------------
layout (location = 1) in vec4 inColor;
layout (location = 0) out vec4 outColor;
#else
layout (location = 0) out flat vec4 outColor;
#endif

#if defined TEXTURED || defined SDF // ---------------------------------------------------------------------------------
layout (location = 2) in vec2 inUV;
layout (location = 1) out vec2 outUV;
#endif

#ifdef CURVE
layout (location = 1) out vec2 outUV;
#endif

#ifdef DYN // ----------------------------------------------------------------------------------------------------------

layout (location = 16) in uvec4 dyn_mode_props_frame_packed;
layout (location = 17) in vec4  dyn_weight;
layout (location = 18) in vec4  dyn_start;

#if defined TEXTURED || defined SDF
#define DEF_DYN_SLOT(SET, INDEX) \
layout (set = SET, binding = 0) readonly buffer ssbo_slot ##INDEX## _meta     { DynMeta slot ##INDEX## _meta[ ]; }; \
layout (set = SET, binding = 1) readonly buffer ssbo_slot ##INDEX## _inst_pos { vec3 slot ##INDEX## _inst_pos[ ]; }; \
layout (set = SET, binding = 2) readonly buffer ssbo_slot ##INDEX## _inst_rot { vec4 slot ##INDEX## _inst_rot[ ]; }; \
layout (set = SET, binding = 3) readonly buffer ssbo_slot ##INDEX## _inst_sca { vec3 slot ##INDEX## _inst_sca[ ]; }; \
layout (set = SET, binding = 4) readonly buffer ssbo_slot ##INDEX## _inst_col { vec4 slot ##INDEX## _inst_col[ ]; }; \
layout (set = SET, binding = 5) readonly buffer ssbo_slot ##INDEX## _vert_pos { vec2 slot ##INDEX## _vert_pos[ ]; }; \
layout (set = SET, binding = 6) readonly buffer ssbo_slot ##INDEX## _vert_col { vec4 slot ##INDEX## _vert_col[ ]; }; \
layout (set = SET, binding = 7) readonly buffer ssbo_slot ##INDEX## _vert_tex { vec2 slot ##INDEX## _vert_tex[ ]; };
#else
#define DEF_DYN_SLOT(SET, INDEX) \
layout (set = SET, binding = 0) readonly buffer ssbo_slot ##INDEX## _meta     { DynMeta slot ##INDEX## _meta[ ]; }; \
layout (set = SET, binding = 1) readonly buffer ssbo_slot ##INDEX## _inst_pos { vec3 slot ##INDEX## _inst_pos[ ]; }; \
layout (set = SET, binding = 2) readonly buffer ssbo_slot ##INDEX## _inst_rot { vec4 slot ##INDEX## _inst_rot[ ]; }; \
layout (set = SET, binding = 3) readonly buffer ssbo_slot ##INDEX## _inst_sca { vec3 slot ##INDEX## _inst_sca[ ]; }; \
layout (set = SET, binding = 4) readonly buffer ssbo_slot ##INDEX## _inst_col { vec4 slot ##INDEX## _inst_col[ ]; }; \
layout (set = SET, binding = 5) readonly buffer ssbo_slot ##INDEX## _vert_pos { vec2 slot ##INDEX## _vert_pos[ ]; }; \
layout (set = SET, binding = 6) readonly buffer ssbo_slot ##INDEX## _vert_col { vec4 slot ##INDEX## _vert_col[ ]; };
#endif

layout (set = 4, binding = 0) uniform ubo_time {
	float global_time;
};

DEF_DYN_SLOT(5, 0)
DEF_DYN_SLOT(6, 1)
DEF_DYN_SLOT(7, 2)
DEF_DYN_SLOT(8, 3)

#endif

// MAIN ----------------------------------------------------------------------------------------------------------------

void main()
{
#ifndef DYN
	gl_Position	= instModel * vec4(inPos, 0.0, 1.0);
#ifndef GRADIENT
	outColor = instColor;
#else
	outColor = instColor * inColor;
#endif

#ifdef TEXTURED
	outUV = inUV;
#elif defined CURVE
	outUV = curve_points[gl_VertexIndex];
#endif

#else
	/*if ((instDynSlotFlags & 0x1) != 0) {
		while (slot0.start + slot0_meta[slot0.frame] < global_time) slot0.frame++;
		float time = global_time - slot0.start - slot0_meta[slot0_frame - 1];
		uint vertex_index = slot0.frame * gl_VertexCount + gl_VertexIndex;
		switch (slot0.inter) {
			case DYN_MODE_STEP:
			gl_Position *= vec4(float(slot0.props & DYN_PROP_INST_SCA) * slot0.weight * slot0_inst_sca[slot0.frame], 0.0);
			gl_Position += vec4(0.0);
			gl_Position += vec4(float(slot0.props & DYN_PROP_INST_POS) * slot0.weight * slot0_inst_pos[slot0.frame], 0.0);
			gl_Position += vec4(float(slot0.props & DYN_PROP_VERT_POS) * slot0.weight * slot0_vert_pos[vertex_index], 0.0);
			break;
		}
	}*/
	
	gl_Position	= instModel * gl_Position;
#endif
}

#elif defined FRAG // --------------------------------------------------------------------------------------------------

layout (location = 0) in vec4 inColor;
layout (location = 0) out vec4 outColor;

#if defined TEXTURED || defined SDF
layout (location = 1) in vec2 inUV;
layout (binding = 0) uniform sampler2D splTexture;
#endif

#ifdef CURVE
layout (location = 1) in vec2 inUV;
#endif

float curve2_alpha(vec2 p) {
	// Gradients
	vec2 px = dFdx(p);
	vec2 py = dFdy(p);
	vec2 f = vec2(2 * p.x * px.x - px.y, 2 * p.x * py.x - py.y); // Chain rule
	float sd = (p.x * p.x - p.y) / length(f); // Signed distance
	return clamp(0.5 - sd, 0.0, 1.0); // Linear alpha + clamp
}

void main()
{
#ifdef TEXTURED
	outColor = texture(splTexture, inUV) * inColor;
#elif defined SDF
	float alpha = (1.0 - texture(splTexture, inUV).r);
	alpha = clamp((alpha - 0.9875) * 80.0, 0.0, 1.0);
	//float smoothWidth = fwidth(alpha);
	//alpha = smoothstep(0.5 - smoothWidth, 0.5 + smoothWidth, alpha);
	outColor = vec4(inColor.rgb, inColor.a * alpha);
#elif defined CURVE
	outColor = vec4(inColor.rgb, inColor.a * curve2_alpha(inUV));
#else
	outColor = inColor;
#endif
}

#endif