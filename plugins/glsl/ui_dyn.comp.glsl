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

struct Instance {
	mat4 model;
	vec4 color;
};

struct InstanceDyn {
	uint mode_props_frame_packed;
	float weight;
	float start;
};

layout (set = 0, binding = 0) uniform ubo_time           { float global_time; };
layout (set = 0, binding = 1) buffer  ssbo_instances     { Instance instances[ ]; };
layout (set = 0, binding = 2) buffer  ssbo_instances_dyn { InstanceDyn instances_dyn[ ]; };

layout (set = 4, binding = 0) readonly buffer ssbo_slot0_meta     { DynMeta slot0_meta[ ]; };
layout (set = 4, binding = 1) readonly buffer ssbo_slot0_inst_pos { vec3 slot0_inst_pos[ ]; };
layout (set = 4, binding = 2) readonly buffer ssbo_slot0_inst_rot { vec4 slot0_inst_rot[ ]; };
layout (set = 4, binding = 3) readonly buffer ssbo_slot0_inst_sca { vec3 slot0_inst_sca[ ]; };
layout (set = 4, binding = 4) readonly buffer ssbo_slot0_inst_col { vec4 slot0_inst_col[ ]; };

layout (set = 5, binding = 0) readonly buffer ssbo_slot1_meta     { DynMeta slot1_meta[ ]; };
layout (set = 5, binding = 1) readonly buffer ssbo_slot1_inst_pos { vec3 slot1_inst_pos[ ]; };
layout (set = 5, binding = 2) readonly buffer ssbo_slot1_inst_rot { vec4 slot1_inst_rot[ ]; };
layout (set = 5, binding = 3) readonly buffer ssbo_slot1_inst_sca { vec3 slot1_inst_sca[ ]; };
layout (set = 5, binding = 4) readonly buffer ssbo_slot1_inst_col { vec4 slot1_inst_col[ ]; };

layout (set = 6, binding = 0) readonly buffer ssbo_slot2_meta     { DynMeta slot2_meta[ ]; };
layout (set = 6, binding = 1) readonly buffer ssbo_slot2_inst_pos { vec3 slot2_inst_pos[ ]; };
layout (set = 6, binding = 2) readonly buffer ssbo_slot2_inst_rot { vec4 slot2_inst_rot[ ]; };
layout (set = 6, binding = 3) readonly buffer ssbo_slot2_inst_sca { vec3 slot2_inst_sca[ ]; };
layout (set = 6, binding = 4) readonly buffer ssbo_slot2_inst_col { vec4 slot2_inst_col[ ]; };

layout (set = 7, binding = 0) readonly buffer ssbo_slot3_meta     { DynMeta slot3_meta[ ]; };
layout (set = 7, binding = 1) readonly buffer ssbo_slot3_inst_pos { vec3 slot3_inst_pos[ ]; };
layout (set = 7, binding = 2) readonly buffer ssbo_slot3_inst_rot { vec4 slot3_inst_rot[ ]; };
layout (set = 7, binding = 3) readonly buffer ssbo_slot3_inst_sca { vec3 slot3_inst_sca[ ]; };
layout (set = 7, binding = 4) readonly buffer ssbo_slot3_inst_col { vec4 slot3_inst_col[ ]; };

void main() {
	
}
