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

use {spirv_std::*, glam::*, core::cmp::Ordering};

const EPS:       f32  = 1e-6;
const FRAC_1_3:  f32  = 0.3333333333;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
enum EdgeType {
	Move   = 0,
	Curve1 = 1,
	Curve2 = 2,
	Curve3 = 3
}

struct Edge {
	ty:    EdgeType,
	_pad0: u32,
	p0:    Vec2,
	p1:    Vec2,
	p2:    Vec2,
}

struct Constants {
	offset:         UVec2,
	extent:         UVec2,
	distance_range: f32,
	vertex_count:   i32
}

fn signed_distance(prev: &Edge, edge: &Edge, p: Vec2) -> (f32, f32) {
	unimplemented!()
}

fn pseudo_signed_distance(prev: &Edge, edge: &Edge, p: Vec2) -> (f32, f32) {
	unimplemented!()
}

#[spirv(compute(threads(4, 4)))]
fn main_sdf_gen(
	#[spirv(global_invocation_id)]                              global_invocation_id: UVec4,
	#[spriv(push_constant)]                                     constants:            &Constants,
	#[spirv(descriptor_set = 0, binding = 0, storage_buffer)]   edges:               &[Edge],
	#[spirv(descriptor_set = 0, binding = 1, uniform_constant)] sdf:                 StorageImage2d
) {
	let p = vec2(global_invocation_id.xy()) / vec2(constants.extent);
	let (mut min_dist, mut min_ortho) = (f32::MAX, f32::MAX);
	
	for i in 1..edges.len() {
		let (dist, ortho) = signed_distance(&edges[i - 1], &edges[i], p);
		
		if dist + EPS < min_dist || ortho < min_ortho {
			(min_dist, min_ortho) = (dist, ortho);
		}
	}
	
	sdf.write(
		constants.offset + global_invocation_id.xy(),
		vec2(min_dist / constants.distance_range + 0.5, 0.0) // HACK: there are no vectors with 1 element
	);
}

fn cmp_dist(a: (f32, f32), b: (f32, f32)) -> Ordering {
	unimplemented!()
}

#[spirv(compute(threads(4, 4)))]
fn main_sdf_gen_multi_channel(
	#[spirv(global_invocation_id)]                              global_invocation_id: UVec4,
	#[spriv(push_constant)]                                     constants:            &Constants,
	#[spirv(descriptor_set = 0, binding = 0, storage_buffer)]   edges:                &[Edge],
	#[spirv(descriptor_set = 0, binding = 1, uniform_constant)] sdf:                  StorageImage2d
) {
	let p = vec2(global_invocation_id.xy()) / vec2(constants.extent);
	let (mut min_dist, mut min_ortho, mut min_edge) = (vec3(f32::MAX), vec3(f32::MAX), uvec3(0));
	let mut channels = 0b101;
	
	for i in 1..edges.len() {
		let (dist, ortho) = signed_distance(&edges[i - 1], &edges[i], p);
		
		if channels & 0b100 != 0 && cmp_dist((min_dist.x, min_ortho.x), (dist, ortho)) == Ordering::Less {
			(min_dist.x, min_ortho.x, min_edge.x) = (dist, ortho, i as _);
		}
		
		if channels & 0b010 != 0 && cmp_dist((min_dist.y, min_ortho.y), (dist, ortho)) == Ordering::Less {
			(min_dist.y, min_ortho.y, min_edge.y) = (dist, ortho, i as _);
		}
		
		if channels & 0b001 != 0 && cmp_dist((min_dist.z, min_ortho.z), (dist, ortho)) == Ordering::Less {
			(min_dist.z, min_ortho.z, min_edge.z) = (dist, ortho, i as _);
		}
		
		channels = if channels == 0b110 { 0b011 } else { 0b110 };
	}
	
	sdf.write(
		constants.offset + global_invocation_id.xy(),
		vec3(
			pseudo_signed_distance(&edges[min_edge.x as usize - 1], &edges[min_edge.x as usize], p).0,
			pseudo_signed_distance(&edges[min_edge.y as usize - 1], &edges[min_edge.y as usize], p).0,
			pseudo_signed_distance(&edges[min_edge.z as usize - 1], &edges[min_edge.z as usize], p).0
		)
	);
}