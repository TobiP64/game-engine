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

use {
	super::{Component, EntityComponent},
	core::{
		any::TypeId,
		ptr::NonNull,
		alloc::{Allocator, Layout},
		sync::atomic::{AtomicUsize, Ordering},
		cell::UnsafeCell,
		fmt,
		cmp,
		mem,
		ptr,
		marker::PhantomData
	}
};

pub type EntityIdx = u32;

const CACHE_LINE:              usize     = 64;
const DEFAULT_CHUNKS_CAPACITY: usize     = 64;
const UNIQUE_BIT:              usize     = !(!0 >> 1);
const CHUNK_IDX_SHIFT:         EntityIdx = 16;
const ENTITY_IDX_MASK:         EntityIdx = 0xFFFF;
const END_OF_CHAIN:            EntityIdx = !0 >> 1;
pub const OCCUPIED_BIT:        EntityIdx = !(!0 >> 1);

#[derive(Debug)]
pub struct Archetype<A: Allocator> {
	types_hash:      u64,
	descriptors:     Box<[ComponentDescriptor]>,
	// next_free and chunks are protected by write locking all components
	next_used:       UnsafeCell<EntityIdx>,
	next_free:       UnsafeCell<EntityIdx>,
	chunks:          UnsafeCell<Vec<*mut u8>>,
	chunk_capacity:  usize,
	chunk_layout:    Layout,
	chunk_alloc:     A
}

impl<A: Allocator> Archetype<A> {
	pub fn new(types: impl IntoIterator<Item: AsRef<TypeInfo>>, types_hash: u64, alloc: A, chunk_layout: Layout) -> Self {
		let mut descriptors = types.into_iter()
			.map(|v| *v.as_ref())
			.map(ComponentDescriptor::from)
			.collect::<Vec<_>>()
			.into_boxed_slice();
		
		let mut chunk_capacity = (chunk_layout.size() / (descriptors.iter()
			.map(|desc| desc.info.layout.pad_to_align().size())
			.sum::<usize>() + 4))+ 1;
		
		loop {
			let chunk_size = descriptors.iter_mut()
				.fold(chunk_capacity * 4, |offset, desc| {
					let layout = desc.info.layout
						.repeat(chunk_capacity).unwrap().0
						.align_to(CACHE_LINE).unwrap();
					desc.offset = crate::align(offset, layout.align());
					desc.offset + layout.size()
				});
			
			if chunk_size <= chunk_layout.size() {
				break;
			}
			
			chunk_capacity -= 1;
		}
		
		#[cfg(debug_assertions)]
		descriptors[1..].windows(2).for_each(|v| match v[0].info.cmp(&v[1].info) {
			cmp::Ordering::Less    => (),
			cmp::Ordering::Equal   => panic!("failed to create archetype: duplicate type info for type `{}`", v[0].info.name),
			cmp::Ordering::Greater => panic!("failed to create archetype: type infos are unsorted"),
		});
		
		Self {
			types_hash,
			next_used:      UnsafeCell::new(0),
			next_free:      UnsafeCell::new(0),
			chunks:         UnsafeCell::new(Vec::with_capacity(DEFAULT_CHUNKS_CAPACITY)),
			chunk_capacity,
			chunk_layout:   Layout::from_size_align(chunk_layout.size(), chunk_layout.align()
				.max(descriptors.iter()
				.map(|ty| ty.info.layout.align())
				.max()
				.unwrap_or(0)
				.max(CACHE_LINE)))
				.unwrap(),
			chunk_alloc:    alloc,
			descriptors
		}
	}
	
	/// Determines if this archetypes has the required types.
	/// Types must be sorted.
	pub fn filter(
		&self,
		include: impl IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator>,
		exclude: impl IntoIterator<Item = TypeId>
	) -> bool {
		let mut include = include.into_iter().peekable();
		let mut exclude = exclude.into_iter().peekable();
		
		if include.len() > self.descriptors.len() {
			return false;
		}
		
		for desc in &*self.descriptors {
			match (desc.info.id, include.peek(), exclude.peek()) {
				(ty, Some(&inc), _) if ty == inc => { include.next(); },
				(ty, _, Some(&exc)) if ty == exc => return false,
				(ty, Some(&inc), _) if ty > inc  => return false,
				(ty, _, Some(&exc)) if ty > exc  => loop {
					match exclude.peek() {
						Some(&exc) if ty > exc => { exclude.next(); },
						_ => break
					}
				}
				_  => continue,
			}
		}
		
		include.next().is_none()
	}
	
	pub fn has_type(&self, ty: TypeId) -> bool {
		self.descriptor(ty).is_some()
	}
	
	pub fn has_types(&self, types: impl IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator>) -> bool {
		let mut types = types.into_iter();
		let mut descs = self.descriptors.iter().peekable();
		
		types.len() <= self.descriptors.len() && types.all(|ty| loop {
			match descs.next() {
				Some(desc) if desc.info.id == ty => return true,
				Some(_)                          => continue,
				None                             => return false
			}
		})
	}
	
	pub fn has_exact_types(&self, types: impl IntoIterator<Item: AsRef<TypeId>, IntoIter: ExactSizeIterator>) -> bool {
		let types = types.into_iter();
		
		types.len() == self.descriptors.len() && self.descriptors.iter()
			.zip(types)
			.all(|(desc, ty)| desc.info.id == *ty.as_ref())
	}
	
	pub fn get_hash(&self) -> u64 {
		self.types_hash
	}
	
	pub fn get_types<'a>(&'a self) -> impl ExactSizeIterator<Item = TypeInfo> + Clone + 'a {
		self.descriptors.iter().map(|desc| desc.into())
	}
	
	/// Repacks component storage by moving data from the end to empty slots. Empty chunks
	/// will be deallocated. `limit` specifies the maximum number of entities that will be moved.
	/// Returns the number of moved entities and the previous and current number of chunks.
	///
	/// # Safety
	///
	/// - The archetype must be in a valid state.
	#[allow(unused_variables, unused_mut)]
	pub unsafe fn repack(&self, limit: usize) -> (usize, usize, usize) {
		self.lock_chunks();
		let mut remaining = limit;
		let mut next      = *self.next_free.get();
		let chunk_len     = (&*self.chunks.get()).len();
		
		for (i, chunk) in (&*self.chunks.get()).iter().enumerate().rev() {
			for desc in self.descriptors.iter() {
				for id in 0..self.chunk_capacity as u32 {
					if next == END_OF_CHAIN {
						break;
					}
					
					if next >= ((i as u32) << CHUNK_IDX_SHIFT) | id {
						continue;
					}
					
					todo!()
				}
			}
		}
		
		let len = (&*self.chunks.get()).len();
		self.unlock_chunks();
		(limit - remaining, chunk_len, len)
	}
	
	/// # Safety
	///
	/// - The archetype must be in a valid state.
	/// - The memory referenced by the yielded pointers must be initialized to a valid state before the iterator is dropped.
	/// - The memory referenced by the yielded pointers must not be accessed after the iterator is dropped.
	/// - The iterator and all iterators it yields must be fully consumed before being dropped.
	pub unsafe fn add_packed<'a>(&'a self, len: usize)
		-> (impl ExactSizeIterator<Item = EntityIdx> + 'a, impl ExactSizeIterator<Item = (&'a TypeInfo, impl ExactSizeIterator<Item = *mut [u8]> + 'a)> + 'a)
	{
		self.lock_chunks();
		self.alloc_chunks(len);
		let offset = (&*self.chunks.get()).len();
		
		(
			(offset * self.chunk_capacity) as u32..(offset * self.chunk_capacity + len) as u32,
			PackedTypeIter { archetype: self, descs: self.descriptors.iter(), offset, len }
		)
	}
	
	/// # Safety
	///
	/// - The archetype must be in a valid state.
	/// - The memory referenced by the yielded pointers must be initialized to a valid state before the iterator is dropped.
	/// - The memory referenced by the yielded pointers must not be accessed after the iterator is dropped.
	/// - The iterator and all iterators it yields must be fully consumed before being dropped.
	pub unsafe fn add_soa<'a>(&'a self, len: usize) -> (impl ExactSizeIterator<Item = EntityIdx> + 'a, AddTypeIter<'a, A>) {
		debug_assert_ne!(len, 0, "len must be greater 0");
		self.lock_chunks();
		let mut idx = *self.next_free.get();
		
		((0..len).map(move |_| {
			let chunks = &mut *self.chunks.get();
			let next = chunks
				.get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize)
				.cast::<u32>()
				.add((idx & ENTITY_IDX_MASK) as usize);
			
			if *next == END_OF_CHAIN {
				let chunk = self.chunk_alloc.allocate(self.chunk_layout)
					.expect("failed to allocate new chunk")
					.as_mut_ptr();
				let idx = (chunks.len() as u32) << CHUNK_IDX_SHIFT;
				let mut ptr = chunk as *mut u32;
				
				for i in idx..idx + self.chunk_capacity as u32 - 1 {
					*ptr = i + 1;
					ptr = ptr.add(1);
				}
				
				*ptr = END_OF_CHAIN;
				*next = idx;
				chunks.push(chunk);
			}
			
			mem::replace(&mut idx, *next)
		}), AddTypeIter { archetype: self, descs: self.descriptors.iter() })
	}
	
	/// # Safety
	///
	/// - The archetype must be in a valid state.
	/// - The memory referenced by the yielded pointers must be initialized to a valid state before the iterator is dropped.
	/// - The memory referenced by the yielded pointers must not be accessed after the iterator is dropped.
	/// - The iterator and all iterators it yields must be fully consumed before being dropped.
	pub unsafe fn add_aos<'a>(&'a self) -> AddEntityIter<'a, A> {
		self.lock_chunks();
		AddEntityIter { archetype: self }
	}
	
	/// # Safety
	///
	/// - The archetype must be in a valid state.
	/// - The memory referenced by the yielded pointers must be copied or dropped before the iterator is dropped.
	/// - The memory referenced by the yielded pointers must not be accessed after the iterator is dropped.
	/// - The iterator and all iterators it yields must be fully consumed before being dropped.
	pub unsafe fn move_soa<'a>(&'a self, indices: &'a (impl AsRef<[EntityIdx]> + 'a)) -> MoveTypeIter<'a, A> {
		let indices = indices.as_ref();
		debug_assert_ne!(indices.len(), 0, "len must be greater than 0");
		self.lock_chunks();
		MoveTypeIter { archetype: self, descs: self.descriptors.iter(), indices }
	}
	
	/// # Safety
	///
	/// - The archetype must be in a valid state.
	/// - The memory referenced by the yielded pointers must be copied or dropped before the iterator is dropped.
	/// - The memory referenced by the yielded pointers must not be accessed after the iterator is dropped.
	/// - The iterator and all iterators it yields must be fully consumed before being dropped.
	pub unsafe fn move_aos<'a, I: IntoIterator<Item = EntityIdx> + 'a>(&'a self, indices: I) -> MoveEntityIter<'a, A, I::IntoIter> {
		self.lock_chunks();
		MoveEntityIter { archetype: self, indices: indices.into_iter() }
	}
	
	/// # Safety
	///
	/// The archetype must be in a valid state.
	pub unsafe fn remove_soa(&self, indices: &impl AsRef<[EntityIdx]>) {
		self.move_soa(indices).for_each(|(ty, iter)| match ty.drop_fn {
			Some(f) => iter.for_each(|ptr| (f)(ptr as _)),
			None    => mem::forget(iter) // forget the iter to not trigger the unconsumed elements panic
		});
	}
	
	/// # Safety
	///
	/// The archetype must be in a valid state.
	pub unsafe fn remove_soa_deferred<'a>(&'a self, indices: &'a (impl AsRef<[EntityIdx]> + 'a)) -> RemoveSoaDeferred<'a, A> {
		RemoveSoaDeferred(self.move_soa(indices))
	}
	
	/// # Safety
	///
	/// The archetype must be in a valid state.
	pub unsafe fn remove_aos(&self, indices: impl IntoIterator<Item = EntityIdx>) {
		for entity in self.move_aos(indices) {
			for (ty, ptr) in entity {
				if let Some(f) = ty.drop_fn {
					(f)(ptr as _);
				}
			}
		}
	}
	
	/// # Safety
	///
	/// The archetype must be in a valid state.
	pub unsafe fn remove_aos_deferred<'a, I: IntoIterator<Item = EntityIdx> + 'a>(&'a self, indices: I) -> RemoveAosDeferred<'a, I, A> {
		RemoveAosDeferred(self.move_aos(indices))
	}
	
	/// Drops all components without deallocating memory.
	pub fn clear(&self) {
		self.lock_chunks();
		unsafe { self.clear_unsafe(); }
		self.unlock_chunks();
	}
	
	/// # Safety
	///
	/// All components must be write locked prior to calling this method.
	unsafe fn clear_unsafe(&self) {
		for desc in self.descriptors.iter() {
			for chunk in (&*self.chunks.get()).iter() {
				if let Some(drop) = desc.info.drop_fn {
					let mut idx = *chunk as *const u32;
					let mut ptr = chunk.add(desc.offset);
					let end     = ptr.add(desc.stride * self.chunk_capacity);
					
					while end.offset_from(ptr) > 0 {
						if *idx != END_OF_CHAIN {
							(drop)(ptr);
						}
						
						idx = idx.add(1);
						ptr = ptr.add(desc.stride);
					}
				}
			}
		}
	}
	
	pub fn accessor<T: Component>(&self) -> Option<ComponentAccessor<T, A>> {
		ComponentAccessor::new(self)
	}
	
	/// # Safety
	///
	/// All components that will be accessed must be locked before reading/writing.
	pub unsafe fn chunks(&self) -> (&[*mut u8], usize, Layout) {
		(&*self.chunks.get(), self.chunk_capacity, self.chunk_layout)
	}
	
	pub fn chunk_capacity(&self) -> usize {
		self.chunk_capacity
	}
	
	fn descriptor(&self, ty: TypeId) -> Option<&ComponentDescriptor> {
		if ty == TypeId::of::<EntityComponent>() {
			return Some(&self.descriptors[0])
		}
		
		self.descriptors.binary_search_by_key(&ty, |d| d.info.id)
			.map(|i| &self.descriptors[i])
			.ok()
	}
	
	fn lock_chunks(&self) {
		if cfg!(feature = "parallel") {
			self.descriptors.iter()
				.for_each(|desc| desc.borrow_mut_spin());
		}
	}
	
	fn unlock_chunks(&self) {
		if cfg!(feature = "parallel") {
			self.descriptors.iter()
				.for_each(ComponentDescriptor::release_mut);
		}
	}
	
	pub fn lock_components(&self, read: impl IntoIterator<Item = TypeId>, write: impl IntoIterator<Item = TypeId>) {
		if !cfg!(feature = "parallel") {
			return;
		}
		
		let mut read  = read.into_iter().peekable();
		let mut write = write.into_iter().peekable();
		
		for desc in &*self.descriptors {
			match (desc.info.id, read.peek(), write.peek()) {
				(ty, Some(&v), _)  if ty == v => {
					desc.borrow_spin();
					read.next();
				}
				(ty, _, Some(&v)) if ty == v => {
					desc.borrow_mut_spin();
					write.next();
				}
				_ => ()
			}
		}
		
		debug_assert_eq!(read.next(), None);
		debug_assert_eq!(write.next(), None);
	}
	
	pub fn unlock_components(&self, read: impl IntoIterator<Item = TypeId>, write: impl IntoIterator<Item = TypeId>) {
		if !cfg!(feature = "parallel") {
			return;
		}
		
		let mut read  = read.into_iter().peekable();
		let mut write = write.into_iter().peekable();
		
		for desc in &*self.descriptors {
			match (desc.info.id, read.peek(), write.peek()) {
				(ty, Some(&v), _)  if ty == v  => {
					desc.release();
					read.next();
				}
				(ty, _, Some(&v)) if ty == v => {
					desc.release_mut();
					write.next();
				}
				_ => ()
			}
		}
		
		debug_assert_eq!(read.next(), None);
		debug_assert_eq!(write.next(), None);
	}
	
	/// # Safety
	///
	/// All components must be write locked prior to calling this method.
	unsafe fn alloc_chunks(&self, additional: usize) {
		for _ in 0..(additional - 1) / self.chunk_capacity + 1 {
			let chunk = self.chunk_alloc.allocate(self.chunk_layout)
				.expect("failed to allocate new chunk").as_mut_ptr();
			(*self.chunks.get()).push(chunk);
		}
	}
}

unsafe impl<A: Allocator> Send for Archetype<A> {}
unsafe impl<A: Allocator> Sync for Archetype<A> {}

impl<A: Allocator> fmt::Display for Archetype<A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "#{:016X}[", self.get_hash())?;
		crate::utils::print_types(f, self.get_types())?;
		unsafe {
			write!(f, "] chunks: {}, chunk_capacity: {}", (&*self.chunks.get()).len(), self.chunk_capacity)?;
		}
		
		Ok(())
	}
}

impl<A: Allocator> Drop for Archetype<A> {
	fn drop(&mut self) {
		unsafe {
			self.clear_unsafe();
			for chunk in (&mut *self.chunks.get()).drain(..) {
				self.chunk_alloc.deallocate(
					NonNull::new_unchecked(chunk), self.chunk_layout);
			}
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct TypeInfo {
	pub id:      TypeId,
	pub name:    &'static str,
	pub layout:  Layout,
	pub drop_fn: Option<unsafe fn(*mut u8)>,
	pub cmp_fn:  Option<fn(*mut u8) -> cmp::Ordering>
}

impl TypeInfo {
	pub const fn of<T: 'static>() -> Self {
		unsafe fn drop_fn<T>(ptr: *mut u8) {
			ptr.cast::<T>().drop_in_place()
		}
		
		Self {
			id:      TypeId::of::<T>(),
			name:    core::any::type_name::<T>(),
			layout:  Layout::new::<T>(),
			drop_fn: if mem::needs_drop::<T>() {
				Some(drop_fn::<T> as _)
			} else {
				None
			},
			cmp_fn:  None
		}
	}
}

impl AsRef<TypeId> for TypeInfo {
	fn as_ref(&self) -> &TypeId {
		&self.id
	}
}

impl AsRef<TypeInfo> for TypeInfo {
	fn as_ref(&self) -> &TypeInfo {
		self
	}
}

impl PartialEq for TypeInfo {
	fn eq(&self, other: &Self) -> bool {
		self.id.eq(&other.id)
	}
}

impl Eq for TypeInfo {}

impl PartialOrd for TypeInfo {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		self.id.partial_cmp(&other.id)
	}
}

impl Ord for TypeInfo {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.id.cmp(&other.id)
	}
}

#[derive(Debug)]
struct ComponentDescriptor {
	info:    TypeInfo,
	offset:  usize,
	stride:  usize,
	borrow:  AtomicUsize
}

impl ComponentDescriptor {
	fn borrow(&self) -> bool {
		let value = self.borrow.fetch_add(1, Ordering::Acquire).wrapping_add(1);
		if value == 0 {
			panic!("borrow counter wrapped")
		} else if value & UNIQUE_BIT != 0 {
			self.borrow.fetch_sub(1, Ordering::Release);
			false
		} else {
			true
		}
	}
	
	fn borrow_mut(&self) -> bool {
		self.borrow
			.compare_exchange(0, UNIQUE_BIT, Ordering::Acquire, Ordering::Relaxed)
			.is_ok()
	}

	fn borrow_spin(&self) {
		while !self.borrow() {
			#[cfg(feature = "std")]
			std::thread::yield_now();
		}
	}

	fn borrow_mut_spin(&self) {
		while !self.borrow_mut() {
			#[cfg(feature = "std")]
			std::thread::yield_now();
		}
	}

	fn release(&self) {
		let value = self.borrow.fetch_sub(1, Ordering::Release);
		debug_assert_ne!(value, 0, "unbalanced release");
		debug_assert_eq!(value & UNIQUE_BIT, 0, "shared release of unique borrow");
	}
	
	fn release_mut(&self) {
		let value = self.borrow.fetch_and(!UNIQUE_BIT, Ordering::Release);
		debug_assert_ne!(value & UNIQUE_BIT, 0, "unique release of shared borrow");
	}
}

impl From<TypeInfo> for ComponentDescriptor {
	fn from(info: TypeInfo) -> Self {
		Self {
			info,
			offset:  0,
			stride:  info.layout.pad_to_align().size(),
			borrow:  AtomicUsize::new(0)
		}
	}
}

#[allow(clippy::from_over_into)]
impl<'a> Into<TypeInfo> for &'a ComponentDescriptor {
	fn into(self) -> TypeInfo {
		self.info
	}
}

impl PartialEq for ComponentDescriptor {
	fn eq(&self, other: &Self) -> bool {
		self.info.eq(&other.info)
	}
}

impl Eq for ComponentDescriptor {}

impl PartialOrd for ComponentDescriptor {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		self.info.partial_cmp(&other.info)
	}
}

impl Ord for ComponentDescriptor {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.info.cmp(&other.info)
	}
}

struct PackedTypeIter<'a, A: Allocator> {
	archetype: &'a Archetype<A>,
	descs:     <&'a [ComponentDescriptor] as IntoIterator>::IntoIter,
	offset:    usize,
	len:       usize
}

impl<'a, A: Allocator> Iterator for PackedTypeIter<'a, A> {
	type Item = (&'a TypeInfo, ChunkIter<'a>);
	
	fn next(&mut self) -> Option<Self::Item> {
		self.descs.next().map(|desc| (&desc.info, ChunkIter {
			desc,
			chunks:         unsafe { mem::transmute((&*self.archetype.chunks.get())[self.offset..].iter()) },
			chunk_capacity: self.archetype.chunk_capacity
		}))
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.descs.size_hint()
	}
}

impl<'a, A: Allocator> ExactSizeIterator for PackedTypeIter<'a, A> {}

impl<'a, A: Allocator> Drop for PackedTypeIter<'a, A> {
	fn drop(&mut self) {
		if self.descs.len() < self.archetype.descriptors.len() {
			check_consumed(self);
			
			unsafe {
				let offset = self.len % self.archetype.chunk_capacity;
				let mut ptr = (&*self.archetype.chunks.get())
					.get_unchecked((&*self.archetype.chunks.get()).len() - 1)
					.cast::<u32>()
					.add(offset);
				
				for i in self.offset * self.archetype.chunk_capacity + offset..(&*self.archetype.chunks.get()).len() * self.archetype.chunk_capacity {
					*ptr = *self.archetype.next_free.get();
					*self.archetype.next_free.get() = i as _;
					ptr = ptr.add(1);
				}
			}
		}
		
		self.archetype.unlock_chunks();
	}
}

pub struct AddTypeIter<'a, A: Allocator> {
	archetype: &'a Archetype<A>,
	descs:  <&'a [ComponentDescriptor] as IntoIterator>::IntoIter
}

impl<'a, A: Allocator> Iterator for AddTypeIter<'a, A> {
	type Item = (&'a TypeInfo, AddTypeComponentIter<'a, A>);
	
	fn next(&mut self) -> Option<Self::Item> {
		self.descs.next().map(|desc| (&desc.info, AddTypeComponentIter {
			archetype: self.archetype,
			desc,
			next:      unsafe { *self.archetype.next_free.get() }
		}))
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.descs.size_hint()
	}
}

impl<'a, A: Allocator> ExactSizeIterator for AddTypeIter<'a, A> {}

impl<'a, A: Allocator> Drop for AddTypeIter<'a, A> {
	fn drop(&mut self) {
		if self.descs.len() < self.archetype.descriptors.len() {
			check_consumed(self);
		}
		
		self.archetype.unlock_chunks();
	}
}

pub struct AddTypeComponentIter<'a, A: Allocator> {
	archetype: &'a Archetype<A>,
	desc:      &'a ComponentDescriptor,
	next:      u32
}

impl<'a, A: Allocator> Iterator for AddTypeComponentIter<'a, A> {
	type Item = *mut u8;
	
	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			let idx = self.next;
			self.next = *(&*self.archetype.chunks.get())
				.get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize)
				.cast::<u32>()
				.add((idx & ENTITY_IDX_MASK) as usize);
			
			Some((&*self.archetype.chunks.get())
				.get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize)
				.add(self.desc.offset + (idx & ENTITY_IDX_MASK) as usize * self.desc.stride)
				as _)
		}
	}
}

impl<'a, A: Allocator> ExactSizeIterator for AddTypeComponentIter<'a, A> {}

impl<'a, A: Allocator> Drop for AddTypeComponentIter<'a, A> {
	fn drop(&mut self) {
		check_consumed(self);
	}
}

pub struct AddEntityIter<'a, A: Allocator> {
	archetype: &'a Archetype<A>
}

impl<'a, A: Allocator> Iterator for AddEntityIter<'a, A> {
	type Item = (u32, EntityComponentIter<'a>);
	
	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			let mut idx = *self.archetype.next_free.get();
			
			let ptr = if idx == END_OF_CHAIN {
				let chunk = self.archetype.chunk_alloc.allocate(self.archetype.chunk_layout)
					.expect("failed to allocate new chunk")
					.as_mut_ptr();
				let base = ((&*self.archetype.chunks.get()).len() as u32) << CHUNK_IDX_SHIFT;
				let mut ptr = chunk as *mut u32;
				
				for i in base + 1..base + self.archetype.chunk_capacity as u32 {
					*ptr = i;
					ptr = ptr.add(1);
				}
				
				*ptr = END_OF_CHAIN;
				idx = base;
				(&mut *self.archetype.chunks.get()).push(chunk);
				chunk as *mut u32
			} else {
				(&*self.archetype.chunks.get())
					.get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize)
					.cast::<u32>()
					.add((idx & ENTITY_IDX_MASK) as usize)
			};
			
			*self.archetype.next_free.get() = mem::replace(&mut *ptr, OCCUPIED_BIT);
			
			Some((idx as _, EntityComponentIter {
				descs: self.archetype.descriptors.iter(),
				chunk: *(&*self.archetype.chunks.get()).get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize),
				idx:   idx & ENTITY_IDX_MASK
			}))
		}
	}
}

impl<'a, A: Allocator> Drop for AddEntityIter<'a, A> {
	fn drop(&mut self) {
		self.archetype.unlock_chunks();
	}
}

pub struct MoveTypeIter<'a, A: Allocator> {
	archetype: &'a Archetype<A>,
	descs:     <&'a [ComponentDescriptor] as IntoIterator>::IntoIter,
	indices:   &'a [u32]
}

impl<'a, A: Allocator> Iterator for MoveTypeIter<'a, A> {
	type Item = (&'a TypeInfo, TypeComponentIter<'a, A>);
	
	fn next(&mut self) -> Option<Self::Item> {
		self.descs.next().map(|desc| (&desc.info, TypeComponentIter {
			archetype: self.archetype,
			desc,
			indices:   self.indices.iter()
		}))
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.descs.size_hint()
	}
}

impl<'a, A: Allocator> ExactSizeIterator for MoveTypeIter<'a, A> {}

impl<'a, A: Allocator> Drop for MoveTypeIter<'a, A> {
	fn drop(&mut self) {
		if self.descs.len() < self.archetype.descriptors.len() {
			check_consumed(self);
			
			for idx in self.indices {
				unsafe {
					(&*self.archetype.chunks.get())
						.get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize)
						.cast::<u32>()
						.add((idx & ENTITY_IDX_MASK) as usize)
						.write(*self.archetype.next_free.get());
					*self.archetype.next_free.get() = *idx;
				}
			}
		}
		
		self.archetype.unlock_chunks()
	}
}

pub struct MoveEntityIter<'a, A: Allocator, I: Iterator<Item = u32> + 'a> {
	archetype: &'a Archetype<A>,
	indices:   I
}

impl<'a, A: Allocator, I: Iterator<Item = u32> + 'a> Iterator for MoveEntityIter<'a, A, I> {
	type Item = EntityComponentIter<'a>;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.indices.next().map(|idx| unsafe {
			debug_assert!(((idx >> CHUNK_IDX_SHIFT) as usize) < (&*self.archetype.chunks.get()).len()
				&& ((idx & ENTITY_IDX_MASK) as usize) < self.archetype.chunk_capacity,
				"index out of bounds: {}", idx);
			
			(&*self.archetype.chunks.get())
				.get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize)
				.cast::<u32>()
				.add((idx & ENTITY_IDX_MASK) as usize)
				.write(*self.archetype.next_free.get());
			*self.archetype.next_free.get() = idx;
			
			EntityComponentIter {
				descs: self.archetype.descriptors.iter(),
				chunk: *(&*self.archetype.chunks.get()).get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize),
				idx:   idx & ENTITY_IDX_MASK
			}
		})
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.indices.size_hint()
	}
}

impl<'a, A: Allocator, I: ExactSizeIterator<Item = u32> + 'a> ExactSizeIterator for MoveEntityIter<'a, A, I> {}

impl<'a, A: Allocator, I: Iterator<Item = u32> + 'a> Drop for MoveEntityIter<'a, A, I> {
	fn drop(&mut self) {
		self.archetype.unlock_chunks();
	}
}

pub struct TypeComponentIter<'a, A: Allocator> {
	archetype: &'a Archetype<A>,
	desc:      &'a ComponentDescriptor,
	indices:   <&'a [u32] as IntoIterator>::IntoIter
}

impl<'a, A: Allocator> Iterator for TypeComponentIter<'a, A> {
	type Item = *mut u8;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.indices.next().map(|idx| unsafe { (&*self.archetype.chunks.get())
			.get_unchecked((*idx >> CHUNK_IDX_SHIFT) as usize)
			.add(self.desc.offset + (*idx & ENTITY_IDX_MASK) as usize * self.desc.stride)
			as _ })
	}
}

impl<'a, A: Allocator> ExactSizeIterator for TypeComponentIter<'a, A> {}

impl<'a, A: Allocator> Drop for TypeComponentIter<'a, A> {
	fn drop(&mut self) {
		check_consumed(self);
	}
}

pub struct EntityComponentIter<'a> {
	descs: <&'a [ComponentDescriptor] as IntoIterator>::IntoIter,
	chunk: *mut u8,
	idx:   u32
}

impl<'a> Iterator for EntityComponentIter<'a> {
	type Item = (&'a TypeInfo, *mut u8);
	
	fn next(&mut self) -> Option<Self::Item> {
		self.descs.next().map(|desc| (
			&desc.info,
			unsafe { self.chunk.add(desc.offset + self.idx as usize * desc.stride) }
		))
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.descs.size_hint()
	}
}

impl<'a> ExactSizeIterator for EntityComponentIter<'a> {}

impl<'a> Drop for EntityComponentIter<'a> {
	fn drop(&mut self) {
		check_consumed(self);
	}
}

pub struct ChunkIter<'a> {
	desc:           &'a ComponentDescriptor,
	chunks:         <&'a [*mut u8] as IntoIterator>::IntoIter,
	chunk_capacity: usize
}

impl<'a> Iterator for ChunkIter<'a> {
	type Item = *mut [u8];
	
	fn next(&mut self) -> Option<Self::Item> {
		self.chunks.next().map(|chunk| ptr::slice_from_raw_parts_mut(
			unsafe { chunk.add(self.desc.offset) },
			self.desc.stride * self.chunk_capacity
		))
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.chunks.size_hint()
	}
}

impl<'a> ExactSizeIterator for ChunkIter<'a> {}

impl<'a> Drop for ChunkIter<'a> {
	fn drop(&mut self) {
		check_consumed(self);
	}
}

unsafe fn get_ptr<T: Component, A: Allocator>(archetype: &Archetype<A>, desc: &ComponentDescriptor, idx: usize) -> *mut T {
	let idx = idx as u32;
	(&*archetype.chunks.get())
		.get_unchecked((idx >> CHUNK_IDX_SHIFT) as usize)
		.add(desc.offset + (idx & ENTITY_IDX_MASK) as usize * desc.stride)
		.cast::<T>()
}

pub struct RemoveSoaDeferred<'a, A: Allocator>(MoveTypeIter<'a, A>);

impl<'a, A: Allocator> RemoveSoaDeferred<'a, A> {
	pub fn remove(&mut self) {
		self.0.by_ref().for_each(|(ty, iter)| match ty.drop_fn {
			Some(f) => iter.for_each(|ptr| unsafe { (f)(ptr as _) }),
			None    => mem::forget(iter) // forget the iter to not trigger the unconsumed elements panic
		});
	}
}

pub struct RemoveAosDeferred<'a, I: IntoIterator<Item = u32>, A: Allocator>(MoveEntityIter<'a, A, I::IntoIter>);

impl<'a, I: IntoIterator<Item = u32>, A: Allocator> RemoveAosDeferred<'a, I, A> {
	pub fn remove(&mut self) {
		for entity in self.0.by_ref() {
			for (ty, ptr) in entity {
				if let Some(f) = ty.drop_fn {
					unsafe { (f)(ptr as _); }
				}
			}
		}
	}
}

pub struct ComponentAccessor<'a, T: Component, A: Allocator> {
	archetype: &'a Archetype<A>,
	desc:      &'a ComponentDescriptor,
	_marker:   PhantomData<T>
}

impl<'a, T: Component, A: Allocator> ComponentAccessor<'a, T, A> {
	pub(super) fn new(archetype: &'a Archetype<A>) -> Option<Self> {
		archetype.descriptor(TypeId::of::<T>())
			.map(|desc| Self { archetype, desc, _marker: PhantomData })
	}
	
	pub fn borrow(&self) -> bool {
		self.desc.borrow()
	}
	
	pub fn borrow_mut(&self) -> bool {
		self.desc.borrow_mut()
	}
	
	pub fn borrow_spin(&self) {
		self.desc.borrow_spin()
	}
	
	pub fn borrow_mut_spin(&self) {
		self.desc.borrow_mut_spin()
	}
	
	pub fn release(&self) {
		self.desc.release();
	}
	
	pub fn release_mut(&self) {
		self.desc.release_mut();
	}
	
	/// # Safety
	///
	/// The component must be borrowed.
	pub unsafe fn get(&self, idx: usize) -> &'a T {
		&*get_ptr(self.archetype, self.desc, idx)
	}
	
	/// # Safety
	///
	/// The component must be mutably borrowed.
	pub unsafe fn get_mut(&self, idx: usize) -> &'a mut T {
		&mut*get_ptr(self.archetype, self.desc, idx)
	}
	
	pub fn offset(&self) -> usize {
		self.desc.offset
	}
}

fn check_consumed(iter: &mut impl Iterator) {
	#[cfg(all(feature = "std", debug_assertions))]
	if iter.next().is_some() && !std::thread::panicking() {
		panic!("iterator has not been consumed entirely");
	}
}