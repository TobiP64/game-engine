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

// TODO sdf_lookup: transform ray when entering sdf instead of on every lookup???
// TODO scene_sdf_lookup: edge case, where xyz size of the sdf are not equal
// TODO scene_sdf_lookup: scene to local coords conversion with transform matrix
// TODO main: generate samples
// TODO scene_sdf_lookup: scene space to texture space conversion
// TODO biplanar mapping
// TODO 3d material textures

#define INSTANCE_ID_MULTIPLE 65536
#define INSTANCE_RES_UNBOUND 65536
#define MATERIAL_NORMAL      0
#define MATERIAL_ALBEDO      1
#define MATERIAL_EMISSIVE    2
#define MATERIAL_MICRO_SURF  3
#define AO_MASK              0x3
#define AO_NONE              0
#define AO_5_TAP             1
#define AO_MULTI_RAY_5_TAP   2
#define AO_CONE_TRACED       3
#define MAX_SAMPLES          8

#define SDF(v) (v & 0xFFFF)
#define MAT(v) (v >> 16 & 0xFFFF)

struct Instance {
	mat4 transform;
	uint sdf_material_packed;
};

struct Material {
	vec3  albedo;
	float metallness;
	vec3  emissive;
	float roughness;
	float normal_smoothness;
#ifdef DISABLE_VARIABLE_DESC_COUNT
	vec3 offset;
	vec3 extent;
#endif
};

#ifdef DISABLE_VARIABLE_DESC_COUNT
struct Sdf {
	vec3 offset;
	vec3 extent;
};
#endif

struct Animation {
	float speed;
	float time;
};

const vec3[MAX_SAMPLES] static_samples = vec3[](
	vec3(1, 0, 0),
	vec3(0, 1, 0),
	vec3(0, 0, 1),
	vec3(-1, 0, 0),
	vec3(0, -1, 0),
	vec3(0, 0, -1),
	vec3(0),
	vec3(0)
);

// optimal workgroup size, according to AMD and ARM
layout (local_size_x = 8, local_size_y = 8) in;

layout (set = 0, binding = 0) uniform Camera {
	vec3 pos;
	vec3 lower_left_corner;
	vec3 horizontal;
	vec3 vertical;
} cam;
layout (set = 0, binding = 1) uniform sampler3D         scene_sdf;
layout (set = 0, binding = 2) uniform sampler3D         scene_ids;
layout (set = 0, binding = 3) readonly buffer           Instances { uint instance_count; Instance instances[ ]; };
layout (set = 0, binding = 4) readonly buffer           Materials { Material materials[ ]; };
layout (set = 0, binding = 5) readonly buffer           Lights    { uint lights[ ]; };
layout (set = 0, binding = 6) uniform sampler           sdf_sampler;
layout (set = 1, binding = 0) uniform texture3D         sdfs[ ];
layout (set = 2, binding = 0) uniform texture2D         materials_spl[ ];
layout (set = 3, binding = 0) uniform writeonly image2D out_image;

#ifdef DISABLE_VARIABLE_DESC_COUNT
layout (set = 0, binding = 6) readonly buffer           Sdfs { Sdf sdfs[ ]; };
layout (set = 0, binding = 7) uniform texture3D         sdfs_spl;
layout (set = 0, binding = 8) uniform texture2DArray    materials_spl;
#endif

layout (push_constant) uniform PushConsts {
	uvec2 extent;            // extent of the swapchain image
	uint  samples;           // number of samples taken per pixel
	uint  bounces;           // ignored, due to recursion beeing not supported in glsl. The actual number of bounces is hardcoded.
	float min_step_size;     // minimum step size
	float min_dist;          // distance threshold
	float min_dist_global;   // global distance threshold
	float max_dist;          // cutoff distance
	uint  min_steps;         // minimum performed steps
	uint  max_steps;         // maximum performed steps
	uint  normal_polls;      // number of polls for sdf normal calculation, currently ignored
	float normal_smoothness; // smoothness of sdf normals, muliplied with Material::normal_smoothness
	uint  features;          // bitmap of enabled features
};

/// Returns the distance to the closest surface of the given instance at the given point in the scene.
vec4 sdf_lookup(uint id, vec3 pos) {
	pos = (instances[id].transform * vec4(pos, 1.0)).xyz; // scene space to instance space to texture space
	return textureLod(sampler3D(sdfs[SDF(instances[id].sdf_material_packed)], sdf_sampler), clamp(pos, 0.0, 1.0), 0.0);
}

/// Resturns the distance to the closest surface at the given point by sampling all instance sdfs.
vec4 sdf_lookup_all(vec3 pos, out uint id) {
	vec4 dist = vec4(1.0);
	id = INSTANCE_ID_MULTIPLE;
	
	for (uint i = 0; i < instance_count; i++) {
		if (SDF(instances[i].sdf_material_packed) == INSTANCE_RES_UNBOUND) continue;
		
		vec4 d = sdf_lookup(i, pos);
		if (abs(d.r) < abs(dist.r)) {
			dist = d;
			id = i;
		}
	}
	
	return dist;
}

/// Calculates the normal at the given point in an sdf.
vec3 sdf_normal(uint id, vec3 pos, float smoothness) {
	vec3 eps = vec3(1, 0, 0) * smoothness;
	return normalize(vec3(
		sdf_lookup(id, pos - eps.xyz).r - sdf_lookup(id, pos + eps.xyz).r,
		sdf_lookup(id, pos - eps.zxy).r - sdf_lookup(id, pos + eps.zxy).r,
		sdf_lookup(id, pos - eps.zyx).r - sdf_lookup(id, pos + eps.zyx).r
	));
}

// Performs triplanar mapping, used for sdf texturing
vec3 triplanar_mapping(texture2D tex, vec3 normal, vec3 pos, float scale) {
	vec3 blending = normalize(abs(normal));
	blending /= vec3(blending.x + blending.y + blending.z);
	return (texture(sampler2D(tex, sdf_sampler), pos.zy * scale) * blending.x
		  + texture(sampler2D(tex, sdf_sampler), pos.xz * scale) * blending.y
		  + texture(sampler2D(tex, sdf_sampler), pos.xy * scale) * blending.z).xyz;
}

/// Samples the given texture of the given instance for the given position and normal.
/// All vectors are in scene space.
vec3 texture_lookup(uint id, uint tex, vec3 nor, vec3 pos, float scale) {
	return triplanar_mapping(materials_spl[MAT(instances[id].sdf_material_packed) * 4 + tex],
			(instances[id].transform * vec4(pos, 0.0)).xyz,
			(instances[id].transform * vec4(pos, 1.0)).xyz, scale);
}

/// Calculates the emitted color.
vec3 emissive(uint id, vec3 pos, vec3 normal, vec3 dir_out) {
	return materials[MAT(instances[id].sdf_material_packed)].emissive * texture_lookup(id, MATERIAL_EMISSIVE, normal, pos, 1.0);
}

/// Calculates the reflected color.
vec3 material(uint id, vec3 dir_in, vec3 dir_out) {
	return materials[MAT(instances[id].sdf_material_packed)].albedo/* * texture_lookup(id, MATERIAL_ALBEDO, normal, pos, 1.0)*/;
}

/// Finds the closest instance intersecting the given ray, returning intersection point, normal and instance id.
bool hit(in vec3 pos, in vec3 dir, out vec3 pos1, out vec3 normal, out uint id) {
	pos1 = pos;
	
	for (uint step = 0; step < max_steps; step++) {
		float dist = textureLod(scene_sdf, clamp(pos1, 0.0, 1.0), 0.0).r;
		
		// if distance is too small, lookup instance sdf
		if (dist <= min_dist_global) {
			id = uint(textureLod(scene_ids, clamp(pos1, 0.0, 1.0), 0.0).r);
			
			// if cell has multiple instances, lookup all instances
			if (id != INSTANCE_ID_MULTIPLE)
				dist = sdf_lookup(id, pos1).r;
			else
				dist = sdf_lookup_all(pos1, id).r;
			
			if (dist <= min_dist && step >= min_steps) {
				normal = sdf_normal(id, pos1, normal_smoothness * materials[id].normal_smoothness);
				return true;
			}
		} else if (dist > max_dist)
			return false;
		
		pos1 += dir * max(dist, min_step_size);
	}
	
	return false;
}

/// Calculates the color of a sample.
vec3 sample_color(
	uint id,
	vec3 normal,
	uint id1,
	vec3 pos1,
	vec3 normal1,
	vec3 dir_in,
	vec3 dir_out,
	vec3 col,
	uint sample_count
) {
	return material(id, dir_in, dir_out) * abs(dot(-dir_in, normal)) * (
		emissive(id1, pos1, normal1, -dir_in) + col / float(sample_count));
}

/// Calculates the color of the given sample, excluding the sample's samples.
vec3 sample_color_non_recursive(uint id, vec3 pos, vec3 normal, vec3 dir_in) {
	return emissive(id, pos, normal, dir_in);
}

struct SdfTraceFrame {
	vec3 spl;
	vec3 pos;
	vec3 nor;
	vec3 col;
	uint id;
	uint spl_count;
};

void main() {
	vec2 uv = gl_GlobalInvocationID.xy / vec2(extent);
	vec3 ori = cam.pos;
	vec3 dir = cam.lower_left_corner
		+ cam.horizontal * uv.x
		+ cam.vertical * uv.y
		- cam.pos;
	
	SdfTraceFrame frames[4];
	
	uint id0, id1, id2, id3, id4;
	vec3 spl0, spl1, spl2, spl3, spl4;
	vec3 pos0, pos1, pos2, pos3, pos4;
	vec3 nor0, nor1, nor2, nor3, nor4;
	vec3 col0, col1, col2, col3;
	uint spl_count0, spl_count1, spl_count2, spl_count3;
	
	vec3[MAX_SAMPLES] samples = static_samples;
	
	col0 = vec3(0); spl0 = dir; spl_count0 = MAX_SAMPLES;
	
	if (!hit(ori, spl0, pos0, nor0, id0)) {
		imageStore(out_image, ivec2(gl_GlobalInvocationID.xy), vec4(0));
		return;
	}
	
	for (uint i0 = 0; i0 < spl_count0; i0++) {
		col1 = vec3(0); spl1 = samples[i0]; spl_count1 = MAX_SAMPLES;
		if (!hit(pos0, spl1, pos1, nor1, id1)) continue;
		
		for (uint i1 = 0; i1 < spl_count1; i1++) {
			col2 = vec3(0); spl2 = samples[i1]; spl_count2 = MAX_SAMPLES;
			if (!hit(pos1, spl2, pos2, nor2, id2)) continue;
			
			for (uint i2 = 0; i2 < spl_count2; i2++) {
				col3 = vec3(0); spl3 = samples[i2]; spl_count3 = MAX_SAMPLES;
				if (!hit(pos2, spl3, pos3, nor3, id3)) continue;
				
				for (uint i3 = 0; i3 < spl_count3; i3++) {
					spl4 = samples[i3];
					if (!hit(pos3, spl4, pos4, nor4, id4)) continue;
					col3 += sample_color_non_recursive(id4, nor4, pos4, -spl3);
				}
				
				col2 += sample_color(id2, nor2, id3, pos3, nor3, -spl2, spl3, col3, spl_count3);
			}
			
			col1 += sample_color(id1, nor1, id2, pos2, nor2, -spl1, spl2, col2, spl_count2);
		}
		
		col0 += sample_color(id0, nor0, id1, pos1, nor1, -spl0, spl1, col1, spl_count1);
	}
	
	col0 = emissive(id0, pos0, nor0, -dir) + col0 / float(spl_count0);
	imageStore(out_image, ivec2(gl_GlobalInvocationID.xy), vec4(col0, 1.0));
}
