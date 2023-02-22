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
#extension GL_EXT_nonuniform_qualifier : enable

#define INVALID_ID 65536
#define INVALID_SDF_ID 1 << 32

#define SDF(v) (v & 0xFFFF)
#define MAT(v) (v << 16 & 0xFFFF)

struct Instance {
	mat4 transform;
	uint sdf_material_packed;
};

// optimal workgroup size, according to AMD and ARM
layout (local_size_x = 4, local_size_y = 4, local_size_z = 4) in;

layout (set = 0, binding = 0, r16f) uniform writeonly image3D scene_sdf;
layout (set = 0, binding = 1, r16 ) uniform writeonly image3D scene_ids;
layout (set = 0, binding = 2) readonly buffer Instances { uint instance_count; Instance instances[ ]; };
layout (set = 0, binding = 3) uniform sampler sdf_sampler;
layout (set = 1, binding = 0) uniform texture3D sdfs[ ];

layout (push_constant) uniform PushConsts {
	uvec3 extent;
};

/// Returns the distance to the closest surface of the given instance at the given point in the scene.
vec4 sdf_lookup(uint id, vec3 pos) {
	pos = (instances[id].transform * vec4(pos, 0.0)).xyz; // scene space to instance space to texture space
	return textureLod(sampler3D(sdfs[SDF(instances[id].sdf_material_packed)], sdf_sampler), clamp(pos, 0.0, 1.0), 0.0);
}

void main() {
	vec3 pos   = gl_GlobalInvocationID.xyz / vec3(extent);
	float dist = 1.0;
	uint id    = INVALID_ID;
	
	for (uint i = 0; i < instance_count; i++) {
		if (SDF(instances[0].sdf_material_packed) == INVALID_SDF_ID) continue;
		
		float d = sdf_lookup(i, pos).r;
		if (abs(d) < abs(dist)) {
			dist = d;
			id = i;
		}
	}
	
	imageStore(scene_sdf, ivec3(gl_GlobalInvocationID.xyz), vec4(dist, 0.0.rrr));
	imageStore(scene_ids, ivec3(gl_GlobalInvocationID.xyz), vec4(id, 0.0.rrr));
}

