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
	super::*,
	core::{
		ops,
		mem,
		ptr,
		any::TypeId,
		marker::PhantomData,
		alloc::Allocator,
		stream::Stream
	}
};

#[cfg(feature = "std")]
use std::{alloc::Global, sync::Arc};
#[cfg(not(feature = "std"))]
use ::alloc::{alloc::Global, sync::Arc};

pub use self::{
	query_info::*,
	direct_query::*,
	cached_query::*,
	query_item::*,
	fetch_ref::*,
	fetch_mut::*,
	entity_id::*,
	with::*,
	without::*,
	option::*,
	insert::*,
	remove::*,
	added::*,
	mutated::*,
	removed::*,
	events::*
};

const DEFAULT_CACHE_SIZE: usize = 64;

pub trait QueryTrait<A: 'static + Allocator + Clone = Global> {
	type Item;
	type Iter:            Iterator<Item = Self::Item>          = core::iter::Empty<Self::Item>;
	type BatchedIter:     Iterator<Item = Self::BatchIter>     = core::iter::Empty<Self::BatchIter>;
	type BatchIter:       Iterator<Item = Self::Item>          = core::iter::Empty<Self::Item>;
	type Stream:          Stream<Item = Self::Item>            = crate::utils::EmptyStream<Self::Item>;
	type BatchedStream:   Stream<Item = Self::BatchStreamIter> = crate::utils::EmptyStream<Self::BatchStreamIter>;
	type BatchStreamIter: Iterator<Item = Self::Item>          = core::iter::Empty<Self::Item>;
	
	fn new(world: Arc<World<A>>) -> Self;
	
	fn update(&mut self);
	
	fn lock(&self);
	
	fn unlock(&self);
	
	fn iter(&self) -> Option<Self::Iter> {
		None
	}
	
	fn batch_iter(&self, _size: usize) -> Option<Self::BatchedIter> {
		None
	}
	
	fn stream(&self) -> Option<Self::Stream> {
		None
	}
	
	fn batch_stream(&self) -> Option<Self::BatchedStream> {
		None
	}
	
	fn has(&mut self, entity: Entity) -> bool;
	
	fn get(&mut self, entity: Entity) -> Option<Self::Item>;
}

mod query_info {
	use super::*;
	
	const BUF_LEN: usize = 64;
	
	#[derive(Debug)]
	pub struct QueryInfo {
		pub include: (usize, [TypeId; BUF_LEN]),
		pub exclude: (usize, [TypeId; BUF_LEN]),
		pub read:    (usize, [TypeId; BUF_LEN]),
		pub write:   (usize, [TypeId; BUF_LEN])
	}
	
	impl Clone for QueryInfo {
		fn clone(&self) -> Self {
			unsafe { ptr::read(self) }
		}
	}
	
	impl Copy for QueryInfo {}
	
	macro_rules! prepare_types {
	    ($fn_name:ident) => {
			{
				let mut types     = [TypeId::of::<()>(); BUF_LEN];
				let mut types_ref = &mut types[..];
				T::$fn_name(&mut types_ref);
				let mut len       = BUF_LEN - types_ref.len();
				
				if len == 0 {
					(0, types)
				} else {
					types[..len].sort_unstable();
					
					for i in 0..len {
						if types[i] == TypeId::of::<EntityComponent>() {
							if types[0] == TypeId::of::<EntityComponent>() {
								types.copy_within(i + 1..len, i);
								len -= 1;
							} else {
								types.copy_within(..i, 1);
								types[0] = TypeId::of::<EntityComponent>();
							}
						} else if i < len - 1 && types[i] == types[i + 1] {
							types.copy_within(i + 1..len, i);
							len -= 1;
						}
					}
					
					(len, types)
				}
			}
		};
	}
	
	impl QueryInfo {
		#[allow(clippy::new_without_default)]
		pub fn new<'a, T: QueryItem<'a>>() -> Self {
			Self {
				include: prepare_types!(get_include),
				exclude: prepare_types!(get_exclude),
				read:    prepare_types!(get_read),
				write:   prepare_types!(get_write)
			}
		}
		
		pub fn includes(&self) -> ConstSliceIter {
			ConstSliceIter { idx: 0, len: self.include.0, buf: self.include.1 }
		}
		
		pub fn excludes(&self) -> ConstSliceIter {
			ConstSliceIter { idx: 0, len: self.exclude.0, buf: self.exclude.1 }
		}
		
		pub fn reads(&self) -> ConstSliceIter {
			ConstSliceIter { idx: 0, len: self.read.0, buf: self.read.1 }
		}
		
		pub fn writes(&self) -> ConstSliceIter {
			ConstSliceIter { idx: 0, len: self.write.0, buf: self.write.1 }
		}
	}
	
	#[derive(Copy, Clone)]
	pub struct ConstSliceIter {
		idx: usize,
		len: usize,
		buf: [TypeId; BUF_LEN]
	}
	
	impl Iterator for ConstSliceIter {
		type Item = TypeId;
		
		fn next(&mut self) -> Option<Self::Item> {
			if self.idx >= self.len {
				None
			} else {
				self.idx += 1;
				Some(self.buf[self.idx - 1])
			}
		}
		
		fn size_hint(&self) -> (usize, Option<usize>) {
			let len = self.len - self.idx;
			(len, Some(len))
		}
	}
	
	impl ExactSizeIterator for ConstSliceIter {}
}

mod direct_query {
	use super::*;
	
	#[derive(Copy, Clone, Debug)]
	pub struct DirectQuery<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone = Global> {
		world:   &'a World<A>,
		info:    QueryInfo,
		_marker: PhantomData<T>
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> DirectQuery<'a, T, A> {
		pub fn new(world: &'a World<A>) -> Self {
			Self { world, info: QueryInfo::new(), _marker: PhantomData,  }
		}
		
		#[allow(clippy::uninit_assumed_init)]
		pub fn iter(&mut self) -> DirectQueryIter<'a, T, A> {
			DirectQueryIter {
				info:           self.info,
				world:          self.world,
				types:          self.world.archetypes_filtered(self.info.includes(), self.info.excludes()),
				r#type:         ptr::null(),
				type_chunk_cap: 0,
				type_chunks:    [].iter(),
				type_fetch:     mem::MaybeUninit::uninit(),
				chunk_iter:     mem::MaybeUninit::uninit(),
				chunk_rem:      0,
				chunk_ptr:      ptr::null()
			}
		}
		
		pub fn iter_batched<'b>(&'b mut self, size: usize) -> DirectQueryBatchesIter<'a, T, A> {
			DirectQueryBatchesIter {
				batch_size:     size,
				types:          Arc::from({
					let mut cache = Vec::with_capacity(DEFAULT_CACHE_SIZE);
					cache.extend(self.world
						.archetypes_filtered(self.info.includes(), self.info.excludes())
						.filter(|archetype| T::Fetch::filter(archetype))
						.map(|archetype| unsafe {
							let fetch = T::Fetch::new(self.world, archetype);
							archetype.lock_components(self.info.reads(), self.info.writes());
							fetch.lock();
							(archetype, fetch)
						}));
					cache
				}),
				types_idx:      0,
				type_chunk_cap: 0,
				type_chunks:    [].iter(),
				type_fetch:     unsafe { &*ptr::null() },
				chunk_ptr:      ptr::null_mut(),
				chunk_idx:      0
			}
		}
		
		pub fn has(&mut self, entity: Entity) -> bool {
			let (archetype, _) = unsafe { entity.location::<A>() };
			archetype.filter(self.info.includes(), self.info.excludes())
				&& T::Fetch::filter(archetype)
		}
		
		pub fn get(&mut self, entity: Entity) -> Option<DirectQueryGetGuard<'_, 'a, T, A>> {
			unsafe {
				let (archetype, idx) = entity.location::<A>();
				if !archetype.filter(self.info.includes(), self.info.excludes())
					|| !T::Fetch::filter(archetype)
				{
					return None;
				}
				
				archetype.lock_components(self.info.reads(), self.info.writes());
				let fetch = T::Fetch::new(self.world, archetype);
				fetch.lock();
				
				let r = Some(DirectQueryGetGuard {
					query:  self,
					archetype,
					fetch:  ptr::read(&fetch),
					item:   fetch.get(entity, archetype.chunks().0[idx as usize >> 16], idx as usize & 0xFFFF)?
				});
				mem::forget(fetch);
				r
			}
		}
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> IntoIterator for &'a mut DirectQuery<'a, T, A> {
		type Item     = T;
		type IntoIter = DirectQueryIter<'a, T, A>;
		
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	/// Items yielded by this iterator are only valid until the next call to `next`.
	pub struct DirectQueryIter<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> {
		info:           QueryInfo,
		world:          &'a World<A>,
		types:          FilteredArchetypesIter<'a, A, ConstSliceIter, ConstSliceIter>,
		r#type:         *const Archetype<A>,
		type_chunk_cap: usize,
		type_chunks:    <&'a [*mut u8] as IntoIterator>::IntoIter,
		type_fetch:     mem::MaybeUninit<T::Fetch<A>>,
		chunk_iter:     mem::MaybeUninit<<T::Fetch<A> as Fetch<'a, A>>::Iter>,
		chunk_rem:      isize,
		chunk_ptr:      *const EntityIdx
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Iterator for DirectQueryIter<'a, T, A> {
		type Item = T;
		
		fn next(&mut self) -> Option<Self::Item> {
			loop {
				unsafe {
					if self.chunk_rem > 0 {
						let step = (*self.chunk_ptr & !crate::archetype::OCCUPIED_BIT) as usize;
						self.chunk_rem -= step as isize;
						self.chunk_ptr = self.chunk_ptr.add(step);
						
						match self.chunk_iter.assume_init_mut().next() {
							Some(v) => return Some(v),
							None    => continue
						}
					} else if let Some(chunk) = self.type_chunks.next() {
						self.chunk_iter.assume_init_drop();
						self.chunk_iter = mem::MaybeUninit::new(self.type_fetch.assume_init_ref().iter(*chunk));
						self.chunk_rem += self.type_chunk_cap as isize;
						self.chunk_ptr  = *chunk as _;
					} else if let Some(archetype) = self.types.next() {
						if !T::Fetch::filter(archetype) {
							continue;
						} else if self.type_chunk_cap > 0 {
							self.type_fetch.assume_init_ref().unlock();
							self.type_fetch.assume_init_drop();
							(&*self.r#type).unlock_components(self.info.reads(), self.info.writes());
						}
						
						let (chunks, chunk_cap, _) = archetype.chunks();
						self.r#type                = archetype;
						self.type_chunk_cap        = chunk_cap;
						self.type_chunks           = chunks.iter();
						self.type_fetch            = mem::MaybeUninit::new(T::Fetch::new(self.world, &*archetype));
						(&*self.r#type).lock_components(self.info.reads(), self.info.writes());
						self.type_fetch.assume_init_ref().lock();
					} else {
						return None;
					}
				}
			}
		}
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Drop for DirectQueryIter<'a, T, A> {
		fn drop(&mut self) {
			if self.type_chunk_cap > 0 {
				unsafe {
					self.chunk_iter.assume_init_drop();
					self.type_fetch.assume_init_ref().unlock();
					self.type_fetch.assume_init_drop();
					(&*self.r#type).unlock_components(self.info.reads(), self.info.writes());
				}
			}
		}
	}
	
	pub struct DirectQueryBatchesIter<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> {
		batch_size:     usize,
		types:          Arc<[(&'a Archetype<A>, T::Fetch<A>)]>,
		types_idx:      usize,
		type_chunk_cap: usize,
		type_chunks:    <&'a [*mut u8] as IntoIterator>::IntoIter,
		type_fetch:     *const T::Fetch<A>,
		chunk_ptr:      *mut u8,
		chunk_idx:      usize
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Iterator for DirectQueryBatchesIter<'a, T, A> {
		type Item = DirectQueryBatchIter<'a, T, A>;
		
		fn next(&mut self) -> Option<Self::Item> {
			loop {
				unsafe {
					if self.chunk_idx < self.type_chunk_cap {
						let len      = self.batch_size.min(self.type_chunk_cap - self.chunk_idx);
						let ptr      = (self.chunk_ptr as *const u32).add(self.chunk_idx);
						let mut iter = (&*self.type_fetch).iter(self.chunk_ptr);
						iter.seek(self.chunk_idx as _);
						self.chunk_idx += len;
						
						return Some(DirectQueryBatchIter {
							types:      self.types.clone(),
							chunk_iter: iter,
							chunk_rem:  len as _,
							chunk_ptr:  ptr
						});
					} else if let Some(&chunk) = self.type_chunks.next() {
						self.chunk_ptr = chunk;
						self.chunk_idx = 0;
					} else if let Some((archetype, fetch)) = self.types.get(self.types_idx) {
						let (chunks, chunk_cap, _) = archetype.chunks();
						self.type_chunk_cap        = chunk_cap;
						self.type_chunks           = chunks.iter();
						self.type_fetch            = fetch;
						self.types_idx            += 1;
						self.chunk_idx             = chunk_cap;
					} else {
						return None;
					}
				}
			}
		}
	}
	
	unsafe impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Send for DirectQueryBatchesIter<'a, T, A> {}
	
	pub struct DirectQueryBatchIter<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> {
		types:      Arc<[(&'a Archetype<A>, T::Fetch<A>)]>,
		chunk_iter: <T::Fetch<A> as Fetch<'a, A>>::Iter,
		chunk_rem:  isize,
		chunk_ptr:  *const EntityIdx
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Iterator for DirectQueryBatchIter<'a, T, A> {
		type Item = T;
		
		fn next(&mut self) -> Option<Self::Item> {
			loop {
				unsafe {
					if self.chunk_rem > 0 {
						let step = (*self.chunk_ptr & !crate::archetype::OCCUPIED_BIT) as usize;
						self.chunk_rem -= step as isize;
						self.chunk_ptr = self.chunk_ptr.add(step);
						
						match self.chunk_iter.next() {
							Some(v) => return Some(v),
							None    => continue
						}
					} else {
						return None;
					}
				}
			}
		}
	}
	
	unsafe impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Send for DirectQueryBatchIter<'a, T, A> {}
	
	pub struct DirectQueryGetGuard<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> {
		query:     &'a DirectQuery<'b, T, A>,
		archetype: &'a Archetype<A>,
		fetch:     T::Fetch<A>,
		item:      T
	}
	
	impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> ops::Deref for DirectQueryGetGuard<'a, 'b, T, A> {
		type Target = T;
		
		fn deref(&self) -> &Self::Target {
			&self.item
		}
	}
	
	impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> ops::DerefMut for DirectQueryGetGuard<'a, 'b, T, A> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.item
		}
	}
	
	impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> ops::Drop for DirectQueryGetGuard<'a, 'b, T, A> {
		fn drop(&mut self) {
			unsafe {
				self.fetch.unlock();
				self.archetype.unlock_components(self.query.info.reads(), self.query.info.writes());
			}
		}
	}
}

mod cached_query {
	use super::*;
	
	#[derive(Clone, Debug)]
	pub struct CachedQuery<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone = Global> {
		world:   &'a World<A>,
		info:    QueryInfo,
		cache:   QueryCacheEntry<A>,
		_marker: PhantomData<T>,
		locked:  bool,
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> CachedQuery<'a, T, A> {
		pub fn new(world: &'a World<A>) -> Self {
			let info = QueryInfo::new();
			Self {
				world,
				info,
				cache:   unsafe { world.query_cache(&info) },
				_marker: PhantomData,
				locked:  false
			}
		}
		
		pub fn update(&mut self) {
			let iteration = self.world.get_iteration();
			
			if self.cache.0 == iteration {
				return;
			}
			
			self.cache = unsafe { self.world.query_cache(&self.info) };
		}
		
		pub fn lock(&mut self) {
			self.locked = true;
			self.cache.1.iter().for_each(|(archetype, fetch)| unsafe {
				archetype.lock_components(self.info.reads(), self.info.writes());
				fetch.lock();
			});
		}
		
		pub fn unlock(&mut self) {
			self.locked = false;
			self.cache.1.iter().for_each(|(archetype, fetch)| unsafe {
				fetch.unlock();
				archetype.unlock_components(self.info.reads(), self.info.writes());
			});
		}
		
		pub fn iter<'b>(&'b mut self) -> CachedQueryIter<'b, 'a, T, A> {
			CachedQueryIter {
				types:          self.cache.1.iter(),
				type_chunk_cap: 0,
				type_chunks:    [].iter(),
				type_fetch:     unsafe { &*ptr::null() },
				chunk_iter:     mem::MaybeUninit::uninit(),
				chunk_rem:      0,
				chunk_ptr:      ptr::null()
			}
		}
		
		pub fn iter_batched<'b>(&'b mut self, size: usize) -> CachedQueryBatchesIter<'b, 'a, T, A> {
			CachedQueryBatchesIter {
				batch_size:     size,
				types:          self.cache.1.iter(),
				type_chunk_cap: 0,
				type_chunks:    [].iter(),
				type_fetch:     unsafe { &*ptr::null() },
				chunk_ptr:      ptr::null_mut(),
				chunk_idx:      0
			}
		}
		
		pub fn has(&mut self, entity: Entity) -> bool {
			let (archetype_, _) = unsafe { entity.location::<A>() };
			self.cache.1.iter()
				.any(|(archetype, _)| ptr::eq(*archetype, archetype_))
		}
		
		pub fn get(&mut self, entity: Entity) -> Option<T> {
			let (archetype_, idx) = unsafe { entity.location::<A>() };
			self.cache.1.iter_mut()
				.find(|(archetype, _)| ptr::eq(*archetype, archetype_))
				.and_then(|(archetype, fetch)| unsafe {
					let (chunks, cap, _) = (**archetype).chunks();
					fetch.get(entity, chunks[idx as usize / cap], idx as usize % cap)
				})
		}
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Drop for CachedQuery<'a, T, A> {
		fn drop(&mut self) {
			if self.locked {
				self.unlock();
			}
		}
	}
	
	impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> IntoIterator for &'a mut CachedQuery<'b, T, A> {
		type Item     = T;
		type IntoIter = CachedQueryIter<'a, 'b, T, A>;
		
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	/// Items yielded by this iterator are only valid until the next call to `next`.
	pub struct CachedQueryIter<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> {
		types:          <&'a [(&'a Archetype<A>, T::Fetch<A>)] as IntoIterator>::IntoIter,
		type_chunk_cap: usize,
		type_chunks:    <&'a [*mut u8] as IntoIterator>::IntoIter,
		type_fetch:     &'a T::Fetch<A>,
		chunk_iter:     mem::MaybeUninit<<T::Fetch<A> as Fetch<'b, A>>::Iter>,
		chunk_rem:      isize,
		chunk_ptr:      *const EntityIdx
	}
	
	impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> Iterator for CachedQueryIter<'a, 'b, T, A> {
		type Item = T;
		
		fn next(&mut self) -> Option<Self::Item> {
			loop {
				unsafe {
					if self.chunk_rem > 0 {
						let step = (*self.chunk_ptr & !crate::archetype::OCCUPIED_BIT) as usize;
						self.chunk_rem -= step as isize;
						self.chunk_ptr = self.chunk_ptr.add(step);
						
						match self.chunk_iter.assume_init_mut().next() {
							Some(v) => return Some(v),
							None    => continue
						}
					} else if let Some(chunk) = self.type_chunks.next() {
						self.chunk_iter.assume_init_drop();
						self.chunk_iter = mem::MaybeUninit::new(self.type_fetch.iter(*chunk));
						self.chunk_rem += self.type_chunk_cap as isize;
						self.chunk_ptr  = *chunk as _;
					} else if let Some((archetype, fetch)) = self.types.next() {
						if !T::Fetch::filter(archetype) {
							continue;
						}
						
						let (chunks, chunk_cap, _) = archetype.chunks();
						self.type_chunk_cap        = chunk_cap;
						self.type_chunks           = chunks.iter();
						self.type_fetch            = fetch;
					} else {
						return None;
					}
				}
			}
		}
	}
	
	impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> Drop for CachedQueryIter<'a, 'b, T, A> {
		fn drop(&mut self) {
			if self.type_chunk_cap > 0 {
				unsafe {
					self.chunk_iter.assume_init_drop();
				}
			}
		}
	}
	
	/// Items yielded by this iterator are only valid until the next call to `next`.
	pub struct CachedQueryBatchesIter<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> {
		batch_size:     usize,
		types:          <&'a [(&'a Archetype<A>, T::Fetch<A>)] as IntoIterator>::IntoIter,
		type_chunk_cap: usize,
		type_chunks:    <&'a [*mut u8] as IntoIterator>::IntoIter,
		type_fetch:     &'a T::Fetch<A>,
		chunk_ptr:      *mut u8,
		chunk_idx:      usize
	}
	
	impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> Iterator for CachedQueryBatchesIter<'a, 'b, T, A> {
		type Item = CachedQueryBatchIter<'b, T, A>;
		
		fn next(&mut self) -> Option<Self::Item> {
			loop {
				unsafe {
					if self.chunk_idx < self.type_chunk_cap {
						let len      = self.batch_size.min(self.type_chunk_cap - self.chunk_idx);
						let ptr      = (self.chunk_ptr as *const u32).add(self.chunk_idx);
						let mut iter = self.type_fetch.iter(self.chunk_ptr);
						iter.seek(self.chunk_idx as _);
						self.chunk_idx += len;
						
						return Some(CachedQueryBatchIter {
							chunk_iter: iter,
							chunk_rem:  len as _,
							chunk_ptr:  ptr
						});
					} else if let Some(&chunk) = self.type_chunks.next() {
						self.chunk_ptr = chunk;
						self.chunk_idx = 0;
					} else if let Some((archetype, fetch)) = self.types.next() {
						let (chunks, chunk_cap, _) = archetype.chunks();
						self.type_chunk_cap        = chunk_cap;
						self.type_chunks           = chunks.iter();
						self.type_fetch            = fetch;
						self.chunk_idx             = chunk_cap;
					} else {
						return None;
					}
				}
			}
		}
	}
	
	unsafe impl<'a, 'b, T: QueryItem<'b>, A: 'static + Allocator + Clone> Send for CachedQueryBatchesIter<'a, 'b, T, A> {}
	
	pub struct CachedQueryBatchIter<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> {
		chunk_iter: <T::Fetch<A> as Fetch<'a, A>>::Iter,
		chunk_rem:  isize,
		chunk_ptr:  *const EntityIdx
	}
	
	impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Iterator for CachedQueryBatchIter<'a, T, A> {
		type Item = T;
		
		fn next(&mut self) -> Option<Self::Item> {
			loop {
				unsafe {
					if self.chunk_rem > 0 {
						let step = (*self.chunk_ptr & !crate::archetype::OCCUPIED_BIT) as usize;
						self.chunk_rem -= step as isize;
						self.chunk_ptr = self.chunk_ptr.add(step);
						
						match self.chunk_iter.next() {
							Some(v) => return Some(v),
							None    => continue
						}
					} else {
						return None;
					}
				}
			}
		}
	}
	
	unsafe impl<'a, T: QueryItem<'a>, A: 'static + Allocator + Clone> Send for CachedQueryBatchIter<'a, T, A> {}
}

mod query_item {
	use {super::*, core::{pin::Pin, task::{Context, Poll}}};
	
	pub trait QueryItem<'a>: 'a + Sized {
		type Fetch<A: 'static + Allocator + Clone>: Fetch<'a, A, Item = Self>;
		type Events<A: 'static + Allocator + Clone>: Events<A, Item = Self> = EmptyEvents<Self>;
		
		const INCLUDE:   &'static [TypeId] = &[];
		const EXCLUDE:   &'static [TypeId] = &[];
		const READ:      &'static [TypeId] = &[];
		const WRITE:     &'static [TypeId] = &[];
		const EXCLUSIVE: bool              = false;
		const EVENTS:    bool              = false;
		const HASH:      u64               = 0;
		
		// these getters are necessary as long as the compiler is obsessed with const generic constraints
		
		fn get_read(types: &mut &mut [TypeId]) {
			types[..Self::READ.len()].copy_from_slice(Self::READ);
			unsafe { *types = mem::transmute(&mut (*types)[Self::READ.len()..]); }
		}
		
		fn get_write(types: &mut &mut [TypeId]) {
			types[..Self::WRITE.len()].copy_from_slice(Self::WRITE);
			unsafe { *types = mem::transmute(&mut (*types)[Self::WRITE.len()..]); }
		}
		
		fn get_include(types: &mut &mut [TypeId]) {
			types[..Self::INCLUDE.len()].copy_from_slice(Self::INCLUDE);
			unsafe { *types = mem::transmute(&mut (*types)[Self::INCLUDE.len()..]); }
		}
		
		fn get_exclude(types: &mut &mut [TypeId]) {
			types[..Self::EXCLUDE.len()].copy_from_slice(Self::EXCLUDE);
			unsafe { *types = mem::transmute(&mut (*types)[Self::EXCLUDE.len()..]); }
		}
	}
	
	pub trait Fetch<'a, A: 'static + Allocator + Clone>: Sized {
		type Item: QueryItem<'a>;
		type Iter: ChunkIter<Item = Self::Item>;
		
		fn filter(_archetype: &Archetype<A>) -> bool {
			true
		}
		
		/// # Safety
		///
		/// This method may do unspeakable things.
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self;
		
		/// # Safety
		///
		/// The Fetch must not be locked when calling this method.
		unsafe fn lock(&self) {}
		
		/// # Safety
		///
		/// The Fetch must be locked when calling this method.
		unsafe fn unlock(&self) {}
		
		/// # Safety
		///
		/// `lock` must be called before calling this method, otherwise doing so is UB.
		unsafe fn get(&self, _entity: Entity, chunk: *mut u8, idx: usize) -> Option<Self::Item> {
			let mut iter = self.iter(chunk);
			iter.seek(idx as _);
			iter.next()
		}
		
		/// # Safety
		///
		/// `lock` must be called before calling this method, otherwise doing so is UB.
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter;
	}
	
	pub trait ReadAccess {}
	pub trait WriteAccess {}
	
	pub trait ChunkIter {
		type Item;
		
		/// # Safety
		///
		/// There must be at least one unconsumed component left in the chunk,
		/// otherwise calling this function is UB.
		unsafe fn next(&mut self) -> Option<Self::Item>;
		
		/// # Safety
		///
		/// `n` must be valid
		unsafe fn seek(&mut self, n: isize);
	}
	
	pub trait Events<A: 'static + Allocator + Clone>: Stream {
		type Item = <Self as Stream>::Item;
		
		fn new(world: &World<A>) -> Self;
	}
	
	pub struct EmptyEvents<T>(PhantomData<T>);
	
	impl<T> Stream for EmptyEvents<T> {
		type Item = T;
		
		fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
			Poll::Ready(None)
		}
	}
	
	impl<T, A: 'static + Allocator + Clone> Events<A> for EmptyEvents<T> {
		fn new(_world: &World<A>) -> Self {
			Self(PhantomData)
		}
	}
}

mod fetch_ref {
	use super::*;
	
	impl<'a, T: Component> QueryItem<'a> for &'a T {
		type Fetch<A: 'static + Allocator + Clone> = FetchRef<'a, T, A>;
		
		const READ:    &'static [TypeId] = &[TypeId::of::<T>()];
		const INCLUDE: &'static [TypeId] = &[TypeId::of::<T>()];
	}
	
	pub struct FetchRef<'a, T: Component, A: 'static + Allocator + Clone>(ComponentAccessor<'a, T, A>);
	
	impl<'a, T: Component, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchRef<'a, T, A> {
		type Item = &'a T;
		type Iter = FetchRefIter<'a, T>;
		
		unsafe fn new(_world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(archetype.accessor::<T>().unwrap())
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			FetchRefIter(chunk.add(self.0.offset()) as _, PhantomData)
		}
	}
	
	pub struct FetchRefIter<'a, T: Component>(*mut T, PhantomData<&'a [T]>);
	
	impl<'a, T: Component> ChunkIter for FetchRefIter<'a, T> {
		type Item = &'a T;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			let val = &*self.0;
			self.0 = self.0.add(1);
			Some(val)
		}
		
		unsafe fn seek(&mut self, n: isize) {
			self.0 = self.0.offset(n)
		}
	}
	
	unsafe impl<'a, T: Component> Send for FetchRefIter<'a, T> {}
}

mod fetch_mut {
	use super::*;
	
	impl<'a, T: Component> QueryItem<'a> for &'a mut T {
		type Fetch<A: 'static + Allocator + Clone> = FetchMut<'a, T, A>;
		
		const WRITE:   &'static [TypeId] = &[TypeId::of::<T>()];
		const INCLUDE: &'static [TypeId] = &[TypeId::of::<T>()];
	}
	
	pub struct FetchMut<'a, T: Component, A: 'static + Allocator + Clone>(ComponentAccessor<'a, T, A>);
	
	impl<'a, T: Component, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchMut<'a, T, A> {
		type Item = &'a mut T;
		type Iter = FetchMutIter<'a, T>;
		
		unsafe fn new(_world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(archetype.accessor::<T>().unwrap())
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			FetchMutIter(chunk.add(self.0.offset()) as _, PhantomData)
		}
	}
	
	pub struct FetchMutIter<'a, T: Component>(*mut T, PhantomData<&'a mut [T]>);
	
	impl<'a, T: Component> ChunkIter for FetchMutIter<'a, T> {
		type Item = &'a mut T;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			let val = &mut*self.0;
			self.0 = self.0.add(1);
			Some(val)
		}
		
		unsafe fn seek(&mut self, n: isize) {
			self.0 = self.0.offset(n)
		}
	}
	
	unsafe impl<'a, T: Component> Send for FetchMutIter<'a, T> {}
}

mod entity_id {
	use super::*;
	
	impl<'a> QueryItem<'a> for Entity {
		type Fetch<A: 'static + Allocator + Clone> = FetchEntityId<'a, A>;
		
		const READ:    &'static [TypeId] = &[TypeId::of::<EntityComponent>()];
		const INCLUDE: &'static [TypeId] = &[TypeId::of::<EntityComponent>()];
	}
	
	pub struct FetchEntityId<'a, A: 'static + Allocator + Clone>(FetchRef<'a, EntityComponent, A>);
	
	impl<'a, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchEntityId<'a, A> {
		type Item = Entity;
		type Iter = FetchEntityIdIter<'a, A>;
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(FetchRef::new(world, archetype))
		}
		
		unsafe fn lock(&self) {
			self.0.lock();
		}
		
		unsafe fn unlock(&self) {
			self.0.unlock();
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			FetchEntityIdIter(self.0.iter(chunk))
		}
	}
	
	pub struct FetchEntityIdIter<'a, A: 'static + Allocator + Clone>(<FetchRef<'a, EntityComponent, A> as Fetch<'a, A>>::Iter);
	
	impl<'a, A: 'static + Allocator + Clone> ChunkIter for FetchEntityIdIter<'a, A> {
		type Item = Entity;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			self.0.next().map(EntityComponent::entity)
		}
		
		unsafe fn seek(&mut self, n: isize) {
			self.0.seek(n);
		}
	}
}

mod with {
	use super::*;
	
	pub struct With<T: Component>(PhantomData<T>);
	
	impl<'a, T: 'a + Component> QueryItem<'a> for With<T> {
		type Fetch<A: 'static + Allocator + Clone> = FetchWith<T, A>;
		
		const INCLUDE: &'static [TypeId] = &[TypeId::of::<T>()];
	}
	
	pub struct FetchWith<T: Component, A: 'static + Allocator + Clone>(PhantomData<(T, A)>);
	
	impl<'a, T: Component, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchWith<T, A> {
		type Item = With<T>;
		type Iter = WithIter<T>;
		
		unsafe fn new(_world: &'a World<A>, _archetype: &'a Archetype<A>) -> Self {
			Self(PhantomData)
		}
		
		unsafe fn iter(&self, _chunk: *mut u8) -> Self::Iter {
			WithIter(PhantomData)
		}
	}
	
	pub struct WithIter<T>(PhantomData<T>);
	
	impl<T: Component> ChunkIter for WithIter<T> {
		type Item = With<T>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			Some(With(PhantomData))
		}
		
		unsafe fn seek(&mut self, _n: isize) {}
	}
}

mod without {
	use super::*;
	
	pub struct Without<T: Component>(PhantomData<T>);
	
	impl<'a, T: 'a + Component> QueryItem<'a> for Without<T> {
		type Fetch<A: 'static + Allocator + Clone> = FetchWithout<T, A>;
		
		const EXCLUDE: &'static [TypeId] = &[TypeId::of::<T>()];
	}
	
	pub struct FetchWithout<T: Component, A: Allocator + Clone>(PhantomData<(T, A)>);
	
	impl<'a, T: Component, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchWithout<T, A> {
		type Item = Without<T>;
		type Iter = WithoutIter<T>;
		
		unsafe fn new(_world: &'a World<A>, _archetype: &'a Archetype<A>) -> Self {
			Self(PhantomData)
		}
		
		unsafe fn iter(&self, _chunk: *mut u8) -> Self::Iter {
			WithoutIter(PhantomData)
		}
	}
	
	pub struct WithoutIter<T>(PhantomData<T>);
	
	impl<T: Component> ChunkIter for WithoutIter<T> {
		type Item = Without<T>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			Some(Without(PhantomData))
		}
		
		unsafe fn seek(&mut self, _n: isize) {}
	}
}

mod option {
	use super::*;
	
	impl<'a, T: QueryItem<'a>> QueryItem<'a> for Option<T> {
		type Fetch<A: 'static + Allocator + Clone> = FetchOption<T::Fetch<A>>;
		
		fn get_read(types: &mut &mut [TypeId]) {
			T::get_read(types)
		}
		
		fn get_write(types: &mut &mut [TypeId]) {
			T::get_write(types)
		}
		
		fn get_include(types: &mut &mut [TypeId]) {
			T::get_include(types)
		}
		
		fn get_exclude(types: &mut &mut [TypeId]) {
			T::get_exclude(types)
		}
	}
	
	pub struct FetchOption<T>(Option<T>);
	
	impl<'a, T: Fetch<'a, A>, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchOption<T> {
		type Item = Option<T::Item>;
		type Iter = FetchOptionIter<T::Iter>;
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(T::filter(archetype).then(|| T::new(world, archetype)))
		}
		
		unsafe fn lock(&self) {
			if let Some(fetch) = &self.0 {
				fetch.lock();
			}
		}
		
		unsafe fn unlock(&self) {
			if let Some(fetch) = &self.0 {
				fetch.unlock();
			}
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			FetchOptionIter(self.0.as_ref().map(|fetch| fetch.iter(chunk)))
		}
	}
	
	pub struct FetchOptionIter<T: ChunkIter>(Option<T>);
	
	impl<T: ChunkIter> ChunkIter for FetchOptionIter<T> {
		type Item = Option<T::Item>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			Some(self.0.as_mut().and_then(|iter| iter.next()))
		}
		
		unsafe fn seek(&mut self, n: isize) {
			if let Some(iter) = self.0.as_mut() { iter.seek(n) }
		}
	}
}

// TODO query insert
mod insert {
	use super::*;
	
	pub struct Insert<'a, T: Component>(*mut T, PhantomData<&'a mut T>);
	
	impl<'a, T: Component> Insert<'a, T> {
		pub fn insert(self, v: T) -> &'a mut T {
			unsafe {
				*self.0 = v;
				let ptr = self.0;
				mem::forget(self);
				&mut *ptr
			}
		}
	}
	
	impl<'a, T: Component> Drop for Insert<'a, T> {
		fn drop(&mut self) {
			panic!("Memory has not been initialized!");
		}
	}
	
	impl<'a, T: Component> QueryItem<'a> for Insert<'a, T> {
		type Fetch<A: 'static + Allocator + Clone + Clone> = FetchInsert<'a, T, A>;
		
		const EXCLUSIVE: bool = true;
	}
	
	pub struct FetchInsert<'a, T: Component, A: 'static + Allocator + Clone> {
		accessor: ComponentAccessor<'a, T, A>,
		src:      &'a Archetype<A>,
		dst:      &'a Archetype<A>
	}
	
	impl<'a, T: Component, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchInsert<'a, T, A> {
		type Item = Insert<'a, T>;
		type Iter = FetchInsertIter<'a, T>;
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			let dst = world.archetype(archetype.get_types()
				.chain(Some(TypeInfo::of::<T>())));
			
			Self {
				accessor: (&*(&*dst as *const Archetype<A>)).accessor::<T>().unwrap(),
				src:      archetype,
				dst
			}
		}
		
		unsafe fn lock(&self) {
			unimplemented!()
		}
		
		unsafe fn unlock(&self) {
			unimplemented!()
		}
		
		unsafe fn iter(&self, _chunk: *mut u8) -> Self::Iter {
			unimplemented!()
		}
	}
	
	pub struct FetchInsertIter<'a, T: Component>(*mut u8, PhantomData<&'a mut T>);
	
	impl<'a, T: Component> ChunkIter for FetchInsertIter<'a, T> {
		type Item = Insert<'a, T>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			unimplemented!()
		}
		
		unsafe fn seek(&mut self, _n: isize) {
			unimplemented!()
		}
	}
}

// TODO query remove
mod remove {
	use super::*;
	
	pub struct Remove<'a, T: Component>(*mut T, PhantomData<&'a mut T>);
	
	impl<'a, T: Component> Remove<'a, T> {
		pub fn get(self) -> T {
			unsafe { ptr::read(self.0) }
		}
	}
	
	impl<'a, T: Component> ops::Deref for Remove<'a, T> {
		type Target = T;
		
		fn deref(&self) -> &Self::Target {
			unsafe { &*self.0 }
		}
	}
	
	impl<'a, T: Component> ops::DerefMut for Remove<'a, T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			unsafe { &mut*self.0 }
		}
	}
	
	impl<'a, T: Component> QueryItem<'a> for Remove<'a, T> {
		type Fetch<A: 'static + Allocator + Clone> = FetchRemove<'a, T, A>;
		
		const EXCLUSIVE: bool = true;
	}
	
	pub struct FetchRemove<'a, T: Component, A: 'static + Allocator + Clone> {
		accessor: ComponentAccessor<'a, T, A>,
		src:      &'a Archetype<A>,
		dst:      &'a Archetype<A>
	}
	
	impl<'a, T: Component, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchRemove<'a, T, A> {
		type Item = Remove<'a, T>;
		type Iter = FetchRemoveIter<'a, T>;
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			let dst = world.archetype(archetype.get_types()
				.filter(|ty| ty == &TypeInfo::of::<T>()));
			
			Self {
				accessor: (&*(&*dst as *const Archetype<A>)).accessor::<T>().unwrap(),
				src:      archetype,
				dst
			}
		}
		
		unsafe fn lock(&self) {
			unimplemented!()
		}
		
		unsafe fn unlock(&self) {
			unimplemented!()
		}
		
		unsafe fn iter(&self, _chunk: *mut u8) -> Self::Iter {
			unimplemented!()
		}
	}
	
	pub struct FetchRemoveIter<'a, T: Component>(PhantomData<&'a mut T>);
	
	impl<'a, T: Component> ChunkIter for FetchRemoveIter<'a, T> {
		type Item = Remove<'a, T>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			unimplemented!()
		}
		
		unsafe fn seek(&mut self, _n: isize) {
			unimplemented!()
		}
	}
}

mod added {
	use super::*;
	
	pub struct Added<T>(pub T);
	
	impl<T> ops::Deref for Added<T> {
		type Target = T;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl<T> ops::DerefMut for Added<T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl<'a, T: QueryItem<'a>> QueryItem<'a> for Added<T> {
		type Fetch<A: 'static + Allocator + Clone> = FetchAdded<T::Fetch<A>>;
	}
	
	pub struct FetchAdded<T>(T);
	
	impl<'a, T: Fetch<'a, A>, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchAdded<T> {
		type Item = Added<T::Item>;
		type Iter = FetchAddedIter<T::Iter>;
		
		fn filter(archetype: &Archetype<A>) -> bool {
			T::filter(archetype)
		}
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(T::new(world, archetype))
		}
		
		unsafe fn lock(&self) {
			self.0.lock()
		}
		
		unsafe fn unlock(&self) {
			self.0.unlock()
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			FetchAddedIter(self.0.iter(chunk))
		}
	}
	
	pub struct FetchAddedIter<T>(T);
	
	impl<T: ChunkIter> ChunkIter for FetchAddedIter<T> {
		type Item = Added<T::Item>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			None
		}
		
		unsafe fn seek(&mut self, n: isize) {
			self.0.seek(n);
		}
	}
}

mod mutated {
	use super::*;
	
	pub struct Mutated<T>(pub T);
	
	impl<T> ops::Deref for Mutated<T> {
		type Target = T;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl<T> ops::DerefMut for Mutated<T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl<'a, T: QueryItem<'a>> QueryItem<'a> for Mutated<T> {
		type Fetch<A: 'static + Allocator + Clone> = FetchMutated<T::Fetch<A>>;
	}
	
	pub struct FetchMutated<T>(T);
	
	impl<'a, T: Fetch<'a, A>, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchMutated<T> {
		type Item = Mutated<T::Item>;
		type Iter = FetchMutatedIter<T::Iter>;
		
		fn filter(archetype: &Archetype<A>) -> bool {
			T::filter(archetype)
		}
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(T::new(world, archetype))
		}
		
		unsafe fn lock(&self) {
			self.0.lock()
		}
		
		unsafe fn unlock(&self) {
			self.0.unlock()
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			FetchMutatedIter(self.0.iter(chunk))
		}
	}
	
	pub struct FetchMutatedIter<T>(T);
	
	impl<T: ChunkIter> ChunkIter for FetchMutatedIter<T> {
		type Item = Mutated<T::Item>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			None
		}
		
		unsafe fn seek(&mut self, n: isize) {
			self.0.seek(n);
		}
	}
}

mod removed {
	use super::*;
	
	pub struct Removed<T>(pub T);
	
	impl<T> ops::Deref for Removed<T> {
		type Target = T;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl<T> ops::DerefMut for Removed<T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl<'a, T: QueryItem<'a>> QueryItem<'a> for Removed<T> {
		type Fetch<A: 'static + Allocator + Clone> = FetchRemoved<T::Fetch<A>>;
	}
	
	pub struct FetchRemoved<T>(T);
	
	impl<'a, T: Fetch<'a, A>, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchRemoved<T> {
		type Item = Removed<T::Item>;
		type Iter = FetchRemovedIter<T::Iter>;
		
		fn filter(archetype: &Archetype<A>) -> bool {
			T::filter(archetype)
		}
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(T::new(world, archetype))
		}
		
		unsafe fn lock(&self) {
			self.0.lock()
		}
		
		unsafe fn unlock(&self) {
			self.0.unlock()
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			FetchRemovedIter(self.0.iter(chunk))
		}
	}
	
	pub struct FetchRemovedIter<T>(T);
	
	impl<T: ChunkIter> ChunkIter for FetchRemovedIter<T> {
		type Item = Removed<T::Item>;
		
		unsafe fn next(&mut self) -> Option<Self::Item> {
			None
		}
		
		unsafe fn seek(&mut self, n: isize) {
			self.0.seek(n);
		}
	}
}

/*mod any {
	use super::*;
	
	pub trait QueryItemAny<'a>: QueryItem<'a> {
		type Any;
	}
	
	pub struct Any<'a, T: QueryItemAny<'a>>(T::Any);
	
	macro_rules! query_item_any_impls {
		($ident:ident, ) => {};
		($head:ident $(, $tail:ident )*, ) => {
			query_item_impls!($( $tail, )* );
			
			
		};
	}
}*/

pub mod events {
	use super::*;
	
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum Event {
		Insert(Entity),
		Mutate(Entity),
		Remove(Entity)
	}
	
	impl Event {
		pub fn entity(self) -> Entity {
			match self {
				Self::Insert(e) => e,
				Self::Mutate(e) => e,
				Self::Remove(e) => e
			}
		}
	}
	
	impl ops::Deref for Event {
		type Target = Entity;
		
		fn deref(&self) -> &Self::Target {
			match self {
				Self::Insert(e) => e,
				Self::Mutate(e) => e,
				Self::Remove(e) => e
			}
		}
	}
	
	pub struct Events<T>(pub T);
	
	impl<T> ops::Deref for Events<T> {
		type Target = T;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl<T> ops::DerefMut for Events<T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
}

/*mod filter {
	use super::*;
	
	pub trait Filter {
		type Item<'a>: QueryItem<'a>;
		
		fn filter_archetype<A: Allocator>(archetype: &Archetype<A>) -> bool {
			true
		}
		
		fn filter_item<'a>(item: Self::Item<'a>) -> bool {
			true
		}
	}
	
	pub struct FetchFilter<'a, T: Filter, A: 'static + Allocator + Clone>(<T::Item<'a> as QueryItem<'a>>::Fetch<A>);
	
	impl<'a, T: Filter, A: 'static + Allocator + Clone> Fetch<'a, A> for FetchFilter<'a, T, A> {
		type Item = ();
		type Iter = ();
		
		const READ:   &'static [TypeId] = <T::Item<'a> as QueryItem<'a>>::Fetch::<A>::READ;
		const WRITE:  &'static [TypeId] = <T::Item<'a> as QueryItem<'a>>::Fetch::<A>::WRITE;
		
		fn filter(archetype: &Archetype<A>) -> bool {
			<T::Item<'a> as QueryItem<'a>>::Fetch::<A>::filter(archetype) && T::filter_archetype(archetype)
		}
		
		unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
			Self(<T::Item<'a> as QueryItem<'a>>::Fetch::<A>::new(world, archetype))
		}
		
		unsafe fn lock(&self) {
			self.0.lock()
		}
		
		unsafe fn unlock(&self) {
			self.0.unlock()
		}
		
		unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
			unimplemented!()
		}
	}
}*/

mod query_item_impl {
	#![allow(non_snake_case)]
	
	use super::*;
	
	pub struct TupleIter<T>(T);
	
	macro_rules! query_item_impls {
		($ident:ident, ) => {};
		($head:ident $(, $tail:ident )*, ) => {
			query_item_impls!($( $tail, )* );
			
			impl<'a, $head: QueryItem<'a> $(, $tail: QueryItem<'a>)* > QueryItem<'a> for ( $head $(, $tail)* ) {
				type Fetch<A: 'static + Allocator + Clone> = ( $head::Fetch<A> $(, $tail::Fetch<A> )* );
				
				fn get_read(types: &mut &mut [TypeId]) {
					$head::get_read(types);
					$(
						$tail::get_read(types);
					)*
				}
				
				fn get_write(types: &mut &mut [TypeId]) {
					$head::get_write(types);
					$(
						$tail::get_write(types);
					)*
				}
				
				fn get_include(types: &mut &mut [TypeId]) {
					$head::get_include(types);
					$(
						$tail::get_include(types);
					)*
				}
				
				fn get_exclude(types: &mut &mut [TypeId]) {
					$head::get_exclude(types);
					$(
						$tail::get_exclude(types);
					)*
				}
			}
			
			impl<'a, A: 'static + Allocator + Clone, $head: Fetch<'a, A> $(, $tail: Fetch<'a, A>)* > Fetch<'a, A> for ( $head $(, $tail)* ) {
				type Item = ( $head::Item $(, $tail::Item )* );
				type Iter = TupleIter<( $head::Iter $(, $tail::Iter)* )>;
				
				fn filter(archetype: &Archetype<A>) -> bool {
					$head::filter(archetype) $( && $tail::filter(archetype) )*
				}
				
				unsafe fn new(world: &'a World<A>, archetype: &'a Archetype<A>) -> Self {
					( $head::new(world, archetype) $(, $tail::new(world, archetype) )* )
				}
				
				unsafe fn lock(&self) {
					let ( $head $(, $tail)* ) = self;
				
					$head.lock();
					$(
						$tail.lock();
					)*
				}
				
				unsafe fn unlock(&self) {
					let ( $head $(, $tail)* ) = self;
				
					$head.unlock();
					$(
						$tail.unlock();
					)*
				}
				
				unsafe fn get(&self, entity: Entity, chunk: *mut u8, idx: usize) -> Option<Self::Item> {
					let ( $head $(, $tail)* ) = self;
					Some(( $head.get(entity, chunk, idx)? $(, $tail.get(entity, chunk, idx)? )* ))
				}
				
				unsafe fn iter(&self, chunk: *mut u8) -> Self::Iter {
					let ( $head $(, $tail)* ) = self;
					TupleIter(( $head.iter(chunk) $(, $tail.iter(chunk) )* ))
				}
			}
			
			impl<$head: ChunkIter $(, $tail: ChunkIter)* > ChunkIter for TupleIter< ( $head $(, $tail )* ) > {
				type Item = ( $head::Item $(, $tail::Item )* );
				
				unsafe fn next(&mut self) -> Option<Self::Item> {
					let Self((  $head $(, $tail )* )) = self;
					Some(( $head.next()? $(, $tail.next()? )* ))
				}
				
				unsafe fn seek(&mut self, n: isize) {
					let Self((  $head $(, $tail )* )) = self;
					$head.seek(n); $( $tail.seek(n); )*
				}
			}
		};
	}
	
	query_item_impls!(Q0, Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10, Q11, Q12, Q13, Q14, Q15, );
}