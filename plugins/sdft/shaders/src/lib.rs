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

#![feature(destructuring_assignment)]

#![cfg_attr(
target_arch = "spirv",
no_std,
feature(register_attr),
register_attr(spirv)
)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;

use {spirv_std::*, glam::*};

type ResourceId = u16;
type InstanceId = u16;

const INSTANCE_ID_INVALID:  InstanceId = !0;
const INSTANCE_ID_MULTIPLE: InstanceId = !0;
const INSTANCE_RES_UNBOUND: ResourceId = !0;

#[repr(C)]
struct Instance {
	transform: Mat4,
	sdf:       ResourceId,
	material:  ResourceId
}

#[repr(C)]
struct Camera {
	pos:               Vec3,
	lower_left_corner: Vec3,
	horizontal:        Vec3,
	vertical:          Vec3
}

#[repr(C)]
struct Material {
	albedo:            Vec3,
	metalness:         f32,
	emissive:          Vec3,
	roughness:         f32,
	normal_smoothness: f32
}

/// Returns the distance to the closest surface of the given instance at the given point in the scene.
fn sdf_lookup(
	instance: &Instance,
	pos:      Vec3,
	sampler:  Sampler,
	sdfs:     &[UniformConstant<Image3d>]
) -> Vec4 {
	sdfs[instance.sdf as usize].sample_by_lod(
		sampler,
		(instance.transform * vec4(pos, 0.0 /* TODO 0.0 or 1.0? */)).xyz()
			.clamp(0.0, 1.0),
		0.0
	)
}

/// Resturns the distance to the closest surface at the given point by sampling all instance sdfs.
fn sdf_lookup_all(
	pos:       Vec3,
	instances: &[Instance],
	sampler:   Sampler,
	sdfs:      &[UniformConstant<Image3d>]
) -> (InstanceId, Vec4) {
	let (mut min_id, mut min_dist) = (INSTANCE_ID_INVALID, vec4(f32::MAX));
	
	for i in 0..instances.len() {
		if instances[i].sdf == INSTANCE_RES_UNBOUND {
			continue;
		}
		
		let dist = sdf_lookup(&instances[i], pos, sampler, sdfs);
		
		if dist.x.abs() < min_dist.x.abs() {
			(min_id, min_dist) = (i as _, dist);
		}
	}
	
	(min_id, min_dist)
}

/// Calculates the normal at the given point in an sdf.
fn sdf_normal(
	instance:   &Instance,
	pos:        Vec3,
	smoothness: f32,
	sampler:    Sampler,
	sdfs:       &[UniformConstant<Image3d>]
) -> Vec3 {
	let eps = vec3(smoothness, 0.0, 0.0);
	vec3(
		sdf_lookup(instance, pos - eps.xyz(), sampler, sdfs).x - sdf_lookup(instance, pos + eps.xyz(), sampler, sdfs).x,
		sdf_lookup(instance, pos - eps.zxy(), sampler, sdfs).x - sdf_lookup(instance, pos + eps.zxy(), sampler, sdfs).x,
		sdf_lookup(instance, pos - eps.zyx(), sampler, sdfs).x - sdf_lookup(instance, pos + eps.zyx(), sampler, sdfs).x
	)
}

/// See https://www.iquilezles.org/www/articles/biplanar/biplanar.htm
fn biplanar_mapping(
	tex:       SampledImage<Image2d>,
	pos:       Vec3,
	normal:    Vec3,
	sharpness: f32
) -> Vec3 {
	let normal = normal.abs();
	
	let major_axis = match normal {
		n if n.x > n.y && n.x > n.z => uvec3(0, 1, 2),
		n if n.y > n.z              => uvec3(1, 2, 0),
		_                           => uvec3(2, 0, 1)
	};
	
	let minor_axis = match normal {
		n if n.x < n.y && n.x < n.z => uvec3(0, 1, 2),
		n if n.y < n.z              => uvec3(1, 2, 0),
		_                           => uvec3(2, 0, 1)
	};
	
	let median_axis = uvec3(3, 3, 3) - minor_axis - major_axis;
	let x = tex.sample(vec2(pos[major_axis.y as _], pos[major_axis.z as _]));
	let y = tex.sample(vec2(pos[median_axis.y as _], pos[median_axis.z as _]));
	let blending = ((vec2(normal[major_axis.x as _], normal[median_axis.x as _]) - 0.5773) / (1.0 - 0.5773))
		.clamp(0.0, 1.0)
		.powf(vec2(sharpness / 8.0));
	
	(x * blending.x + y * blending.y) / (blending.x + blending.y)
}

fn triplanar_mapping(
	tex:       SampledImage<Image2d>,
	pos:       Vec3,
	normal:    Vec3,
	sharpness: f32
) -> Vec3 {
	let x = tex.sample(pos.zy());
	let y = tex.sample(pos.xz());
	let z = tex.sample(pos.xy());
	let blending = normal.abs().normalize().powf(sharpness);
	
	(x * blending.x + y * blending.y + z * blending.z).xyz() / (blending.x + blending.y + blending.z)
}

fn trace(pos: Vec3, dir: Vec3, params: &TraceConstants) -> Option<(Vec3, u32)> {
	for step in 0..params.max_steps {
		let dist = textureLod(scene_sdf, clamp(pos, 0.0, 1.0), 0.0).r;
		
		if dist < params.min_dist_global {
			let id = textureLod(scene_ids, clamp(pos, 0.0, 1.0), 0.0).r;
			
			if id == INSTANCE_ID_MULTIPLE {
				dist = sdf_lookup(id, pos);
			} else {
				dist = sdf_lookup_all(pos, );
			}
			
			if dist <= params.min_dist && step >= params.min_steps {
				return Some((pos, id));
			}
		} else if dist > params.max_dist {
			return None;
		}
		
		pos += dir * max(dist, params.min_step_size);
	}
}

fn shade(
	id: u32,
	pos:       Vec3,
	normal:    Vec3,
	dir_in:    Vec3,
	dir_out:   Vec3
) -> Vec3 {
	let radiance = vec3(0.0);
	
	for probe in 0..4 {
		let (albedo, metalness, roughness) = ();
		
		for sample_x in 0..8 {
			for sample_y in 0..8 {
				let color = brdf(
					textureLod(scene_cache, , 0.0).r,
					dir_in,
					dir_out,
					normal,
					albedo,
					metalness,
					roughness
				);
				
				radiance += importance * color * (1 / 8 * 8);
			}
		}
	}
}

fn brdf(
	radiance:  Vec3,
	dir_in:    Vec3,
	dir_out:   Vec3,
	normal:    Vec3,
	albedo:    Vec4,
	metalness: f32,
	roughness: f32,
) -> Vec3 {
	unimplemented!()
}




struct SdfGenData {

}

#[spirv(compute(threads(4, 4, 4)))]
fn sdf3d_gen(
	#[spirv(global_invocation_id)]                              global_invocation_id: UVec4,
	#[spirv(descriptor_set = 0, binding = 0, uniform_constant)] sdf:                  StorageImage3d,
	#[spirv(descriptor_set = 0, binding = 1, storage_buffer)]   data:                 SdfGenData,
) {

}

#[repr(C)]
struct UpdateConstants {
	extent: UVec3
}

#[spirv(compute(threads(4, 4, 4)))]
fn sdf3d_upd(
	#[spirv(global_invocation_id)]                              global_invocation_id: UVec4,
	#[spirv(descriptor_set = 0, binding = 0, uniform_constant)] scene_sdf:            StorageImage3d,
	#[spirv(descriptor_set = 0, binding = 2, uniform_constant)] scene_ids:            StorageImage3d,
	#[spirv(descriptor_set = 0, binding = 3, storage_buffer)]   instances:            &[Instance],
	#[spirv(descriptor_set = 0, binding = 4, uniform_constant)] sdf_sampler:          Sampler,
	#[spirv(descriptor_set = 1, binding = 0, uniform_constant)] sdfs:                 &[Image3d],
	#[spriv(push_constant)]                                     constants:            &UpdateConstants,
) {
	let pos = vec3(global_invocation_id.xyz()) / vec3(constants.extent);
	let (mut min_id, mut min_dist) = (INSTANCE_ID_INVALID, f32::MAX);
	
	for (id, instance) in instances.iter().enumerate() {
		if instance.sdf == INSTANCE_RES_UNBOUND {
			continue;
		}
		
		let dist = sdf_lookup(instance, pos, sdf_sampler, sdfs).x;
		
		if dist.abs() < min_dist.abs() {
			(min_id, min_dist) = (id as _, dist);
		}
	}
	
	scene_sdf.write(global_invocation_id.xyz(), vec2(min_dist, 0.0));
	scene_sdf.write(global_invocation_id.xyz(), vec2(min_id as f32, 0.0));
}

#[repr(C)]
struct TraceConstants {
	extent:            Vec2,
	samples:           u32,
	bounces:           u32,
	min_step_size:     f32,
	min_dist:          f32,
	min_dist_global:   f32,
	max_dist:          f32,
	min_steps:         u32,
	max_steps:         u32,
	normal_polls:      u32,
	normal_smoothness: f32,
	features:          u32
}

#[spirv(compute(threads(8, 8)))]
fn sdf3d_trace(
	#[spirv(global_invocation_id)]                              global_invocation_id: UVec4,
	#[spirv(descriptor_set = 0, binding = 0, uniform_constant)] camera:               &Camera,
	#[spirv(descriptor_set = 0, binding = 1, uniform_constant)] scene_sdf:            StorageImage3d,
	#[spirv(descriptor_set = 0, binding = 2, uniform_constant)] scene_ids:            StorageImage3d,
	#[spirv(descriptor_set = 0, binding = 3, storage_buffer)]   instances:            &[Instance],
	#[spirv(descriptor_set = 0, binding = 4, storage_buffer)]   materials:            &[Material],
	#[spirv(descriptor_set = 0, binding = 6, uniform_constant)] sdf_sampler:          Sampler,
	#[spirv(descriptor_set = 1, binding = 0, uniform_constant)] sdfs:                 &[UniformConstant<Image3d>],
	#[spirv(descriptor_set = 2, binding = 0, uniform_constant)] material_spl:         &[UniformConstant<Image2d>],
	#[spirv(descriptor_set = 3, binding = 0, uniform_constant)] out_image:            UniformConstant<StorageImage2d>,
	#[spriv(push_constant)]                                     constants:            &TraceConstants,
) {
	let uv  = vec2(global_invocation_id.xy()) / vec2(constants.extent);
	let pos = camera.pos;
	let dir = camera.lower_left_corner
		+ camera.horizontal * uv.x
		+ camera.vertical * uv.y
		- camera.pos;
	
	
}