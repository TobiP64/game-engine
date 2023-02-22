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

#![feature(allocator_api)]
#![warn(clippy::all)]
#![allow(clippy::from_over_into)]

use {
	ecs::*,
	engine_core::Source,
	math::*,
	std::{alloc::Allocator, marker::PhantomData}
};

pub use self::{animation::*, audio::*, glyphs::*, handle::*, mesh::*, texture::*, transform::*, ui::*};

pub mod transform;
pub mod mesh;
pub mod path;
pub mod texture;
pub mod glyphs;
pub mod audio;
pub mod animation;
pub mod ui;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Hidden;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct HiddenPropagate;

#[derive(Clone, Debug, Default)]
pub struct Name(pub String);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Parent(pub Entity);

/// Used to accelerate tree traversal by mapping parents directly to their children.
/// This means duplicate keys are allowed, obviously.
#[derive(Clone, Debug, Default)]
pub struct ChildrenAccelStorage {
	//             parent  child
	children: Vec<(Entity, Entity)>
}

impl ChildrenAccelStorage {
	pub fn insert(&mut self, entity: Entity, parent: Entity) {
		for i in 0..self.children.len() {
			if self.children[i].0 >= parent {
				self.children.insert(i, (parent, entity));
				return;
			}
		}
		
		self.children.push((parent, entity));
	}
	
	pub fn remove(&mut self, entity: Entity) {
		for i in 0..self.children.len() {
			if self.children[i].1 == entity {
				self.children.remove(i);
				return;
			}
		}
	}
	
	pub fn for_each_child(&self, entity: Entity, f: &mut impl FnMut(Entity, Entity) -> bool) {
		self.children.iter()
			.skip_while(|(parent, _)| *parent != entity)
			.take_while(|(parent, _)| *parent == entity)
			.for_each(|(parent, child)| if f(*child, *parent) {
				self.for_each_child(*child, f);
			});
	}
	
	pub fn for_each_parent(&self, mut entity: Entity, f: &mut impl FnMut(Entity) -> bool) {
		loop {
			match self.children.iter().find(|(_, child)| *child == entity) {
				None => return,
				Some((parent, _)) => {
					if !f(*parent) {
						return;
					}
					
					entity = *parent;
				}
			}
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct Joints(pub Vec<(u32, Transform)>);

#[derive(Clone, Debug, Default)]
pub struct JointsGlobalTransform(pub Vec<Mat4<f32>>);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color(pub Vec4<f32>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Particles(pub Vec<Particle>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Particle {
	pub pos:    Vec3<f32>,
	pub normal: Vec3<f32>,
	pub size:   f32,
	pub color:  Vec4<f32>
}

pub mod handle {
	use super::*;
	
	pub struct Handle<T: ?Sized>(pub Entity, pub PhantomData<T>);
	
	impl<T: ?Sized> Handle<T> {
		pub fn new(entity: Entity) -> Self {
			Handle(entity, PhantomData)
		}
	}
	
	impl<T: ?Sized> std::ops::Deref for Handle<T> {
		type Target = Entity;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl<T: ?Sized> std::ops::DerefMut for Handle<T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl<T: ?Sized> Clone for Handle<T> {
		fn clone(&self) -> Self {
			Self(self.0, self.1)
		}
	}
	
	impl<T: ?Sized> Copy for Handle<T> {}
	
	impl<T: ?Sized> PartialEq for Handle<T> {
		fn eq(&self, other: &Self) -> bool {
			self.0 == other.0
		}
	}
	
	impl<T: ?Sized> Eq for Handle<T> {}
	
	impl<T: ?Sized> PartialOrd for Handle<T> {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			self.0.partial_cmp(&other.0)
		}
	}
	
	impl<T: ?Sized> Ord for Handle<T> {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			self.0.cmp(&other.0)
		}
	}
	
	impl<T: ?Sized> std::fmt::Debug for Handle<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_tuple("Handle<_>")
				.field(&self.0)
				.finish()
		}
	}
}

pub fn propagate_hidden_flag_added<'a, A: Allocator + Clone>(
	entity:   Entity,
	_:        Added<&Hidden>,
	children: &ChildrenAccelStorage,
	query:    &'a mut CachedQuery<&'a Hidden, A>,
	world:    &World
) {
	children.for_each_child(entity, &mut |entity, _| if query.has(entity) {
		world.add_component(entity, HiddenPropagate);
		true
	} else {
		false
	})
}

pub fn propagate_hidden_flag_removed<'a, A: Allocator + Clone>(
	entity:   Entity,
	_:        Removed<&Hidden>,
	children: &ChildrenAccelStorage,
	query:    &'a mut CachedQuery<&'a Hidden, A>,
	world:    &World
) {
	children.for_each_child(entity, &mut |entity, _| if query.has(entity) {
		world.remove_component::<HiddenPropagate>(entity);
		true
	} else {
		false
	})
}

#[cfg(test)]
mod tests {
}