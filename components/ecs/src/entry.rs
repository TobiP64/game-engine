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

use {super::*, core::{cmp, fmt, alloc::Allocator}};

#[derive(Copy, Clone)]
pub struct Entry<'a, A: Allocator + Clone> {
	world:  &'a World<A>,
	entity: Entity
}

impl<'a, A: Allocator + Clone> Entry<'a, A> {
	/// Wraps the given entity without checking if it exists.
	pub fn new(world: &'a World<A>, entity: Entity) -> Self {
		Self { world, entity }
	}
	
	pub fn entity(&self) -> Entity {
		self.entity
	}
	
	pub fn has<T: Component>(&self) -> bool {
		self.world.has_bundle::<(T,), 1>(self.entity)
	}
	
	pub fn has_types<T: ComponentBundle<N>, const N: usize>(&mut self) -> bool {
		self.world.has_bundle::<T, N>(self.entity)
	}
	
	pub fn query<'b, T: QueryItem<'b>>(&'b self) -> Option<crate::query::DirectQueryGetGuard<'a, 'b, T, A>> {
		self.world.query().get(self.entity)
	}
	
	pub fn add<T: Component>(&mut self, component: T) {
		self.world.add_component(self.entity, component)
	}
	
	pub fn remove<T: Component>(&mut self) -> Option<T> {
		self.world.remove_component(self.entity)
	}
	
	pub fn add_bundle<T: ComponentBundle<N>, const N: usize>(&mut self, bundle: T) -> bool {
		self.world.add_bundle(self.entity, bundle)
	}
	
	pub fn remove_bundle<T: ComponentBundle<N>, const N: usize>(&mut self) -> Option<T> {
		self.world.remove_bundle(self.entity)
	}
	
	pub fn modify<ADD: ComponentBundle<AN>, REM: ComponentBundle<RN>, const AN: usize, const RN: usize>(&mut self, bundle: ADD) -> Result<REM, ADD> {
		self.world.modify_entity(self.entity, bundle)
	}
	
	pub fn delete(self) -> bool {
		self.world.remove_entity(self.entity)
	}
	
	pub fn with<T: Component>(mut self, v: T) -> Self {
		self.add(v);
		self
	}
	
	pub fn with_default<T: Component + Default>(self) -> Self {
		self.with(T::default())
	}
	
	pub fn finish(self) -> Entity {
		self.entity
	}
}

impl<'a, A: Allocator + Clone> cmp::PartialEq for Entry<'a, A> {
	fn eq(&self, other: &Self) -> bool {
		core::ptr::eq(self.world, other.world)
			&& self.entity == other.entity
	}
}

impl<'a, A: Allocator + Clone> cmp::Eq for Entry<'a, A> {}

impl<'a, A: Allocator + Clone> cmp::PartialOrd for Entry<'a, A> {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(cmp::Ord::cmp(self, other))
	}
}

impl<'a, A: Allocator + Clone> cmp::Ord for Entry<'a, A> {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		cmp::Ord::cmp(&self.entity, &other.entity)
	}
}

impl<'a, A: Allocator + Clone> fmt::Debug for Entry<'a, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct(core::any::type_name::<Self>())
			.field("entity", &self.entity)
			.finish()
	}
}

impl<'a, A: Allocator + Clone> fmt::Display for Entry<'a, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.entity, f)
	}
}