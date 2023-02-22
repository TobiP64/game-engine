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

use {super::*, std::alloc::Allocator};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Transform {
	pub translation: Vec3<f32>,
	pub rotation:    Quat32,
	pub scaling:     f32
}

impl Transform {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn translation(mut self, translation: Vec3<f32>) -> Self {
		self.translation = translation;
		self
	}
	
	pub fn rotation(mut self, rotation: Quat32) -> Self {
		self.rotation = rotation;
		self
	}
	
	pub fn scaling(mut self, scaling: f32) -> Self {
		self.scaling = scaling;
		self
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LocalTransform(pub Mat4<f32>);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct GlobalTransform(pub Mat4<f32>);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Velocity(pub Transform);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Acceleration(pub Transform);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct MainCamera;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Camera {
	Pers(PerspectiveCamera),
	Ortho(OrthographicCamera),
	OrthoSym(OrthographicSymmetricCamera)
}

impl Default for Camera {
	fn default() -> Self {
		Self::Pers(PerspectiveCamera::default())
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PerspectiveCamera {
	pub far:    f32,
	pub near:   f32,
	pub aspect: f32,
	pub fov:    f32
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct OrthographicCamera {
	pub far:    f32,
	pub near:   f32,
	pub planes: Vec4<f32>
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct OrthographicSymmetricCamera {
	pub far:    f32,
	pub near:   f32,
	pub planes: Vec2<f32>
}

pub fn update_velocity(velocity: &mut Velocity, acceleration: &Acceleration) {
	velocity.0.translation += acceleration.0.translation;
	velocity.0.scaling += acceleration.0.scaling;
	velocity.0.rotation.rotate(acceleration.0.rotation);
}

pub fn update_transform(transform: &mut Transform, velocity: &Velocity) {
	transform.translation += velocity.0.translation;
	transform.scaling += velocity.0.scaling;
	transform.rotation.rotate(velocity.0.rotation);
}

pub fn update_local_transform(transform: Mutated<Transform>, local_transform: &mut LocalTransform) {
	local_transform.0 = Mat4::from_transform(
		transform.translation,
		transform.rotation,
		Vec3::from(transform.scaling)
	);
}

pub struct OrderedEntity(Entity);

pub fn update_global_transform<'a, A: Allocator + Clone>(
	_entity:      OrderedEntity,
	parent:       Option<&Parent>,
	local:        &LocalTransform,
	global:       &mut GlobalTransform,
	parent_query: &'a mut CachedQuery<&'a GlobalTransform, A>
) {
	global.0 = match parent {
		Some(parent) => parent_query.get(parent.0).unwrap().0 * local.0,
		None => local.0
	}
}

pub fn propagate_parent_removes(entity: Removed<Entity>, children: &ChildrenAccelStorage, world: &World) {
	children.for_each_child(entity.0, &mut |child, _| {
		world.remove_component::<Parent>(child);
		false
	})
}