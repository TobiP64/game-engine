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

use core::{alloc::*, sync::atomic::{*, Ordering::*}, ptr::{self, NonNull}, fmt};

#[cfg(feature = "std")]
use std::alloc::Global;
#[cfg(not(feature = "std"))]
use alloc::alloc::Global;

static GLOBAL_CHUNK_ALLOC: ChunkAlloc = ChunkAlloc {
	block_layout: unsafe { Layout::from_size_align_unchecked(0x0800_0000, 0x4000) },
	chunk_layout: unsafe { Layout::from_size_align_unchecked(0x4000, 0x4000) },
	blocks:       AtomicUsize::new(0),
	ptr:          AtomicPtr::new(ptr::null_mut())
};

#[derive(Debug)]
pub struct ChunkAlloc {
	block_layout: Layout,
	chunk_layout: Layout,
	blocks:       AtomicUsize,
	ptr:          AtomicPtr<u8>
}

impl ChunkAlloc {
	unsafe fn alloc_block(&self) -> Result<*mut u8, AllocError> {
		let block   = Global.allocate(self.block_layout)?.as_mut_ptr();
		let blocks  = self.blocks.fetch_add(1, Relaxed) + 1;
		let end     = block.add(self.block_layout.size() - self.chunk_layout.size());
		let mut ptr = block;
		
		while (ptr as usize) < (end as usize) {
			let next = ptr.add(self.chunk_layout.size());
			*(ptr as *mut *mut u8) = next;
			ptr = next;
		}
		
		*(ptr as *mut *mut u8) = ptr::null_mut();
		log::debug!("[CHUNK-ALLOC] allocated new block (total: {}, {}MB)", blocks, (blocks * self.block_layout.size()) >> 20);
		Ok(block)
	}
}

unsafe impl Allocator for ChunkAlloc {
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		if layout.size() > self.chunk_layout.size() || layout.align() > self.chunk_layout.align() {
			return Global.allocate(layout);
		}
		
		let mut next = self.ptr.load(Acquire);
		unsafe {
			loop {
				if next.is_null() {
					let block = self.alloc_block()?;
					
					match self.ptr.compare_exchange(
						next, *(block as *mut *mut u8), Release, Relaxed)
					{
						Ok(_) => return Ok(NonNull::new_unchecked(
							core::slice::from_raw_parts_mut(block, self.chunk_layout.size()))),
						Err(ptr) => {
							Global.deallocate(NonNull::new_unchecked(next), self.block_layout);
							next = ptr;
						}
					}
				} else {
					match self.ptr.compare_exchange(
						next, *(next as *mut *mut u8), Release, Relaxed)
					{
						Ok(_) => return Ok(NonNull::new_unchecked(
							core::slice::from_raw_parts_mut(next, self.chunk_layout.size()))),
						Err(ptr) => next = ptr
					}
				}
			}
		}
	}
	
	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		if layout.size() > self.chunk_layout.size() || layout.align() > self.chunk_layout.align() {
			return Global.deallocate(ptr, layout);
		}
		
		let mut next = self.ptr.load(Acquire);
		loop {
			*(ptr.as_ptr() as *mut *mut u8) = next;
			
			match self.ptr.compare_exchange(next, ptr.as_ptr(), Release, Relaxed) {
				Ok(_) => return,
				Err(ptr) => next = ptr
			}
		}
	}
	
	unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		if old_layout.size() > self.chunk_layout.size() || old_layout.align() > self.chunk_layout.align() {
			Global.grow(ptr, old_layout, new_layout)
		} else if new_layout.size() > self.chunk_layout.size() || new_layout.align() > self.chunk_layout.align() {
			let new_region = Global.allocate(new_layout)?;
			ptr::copy(ptr.as_ptr(), new_region.as_mut_ptr(), old_layout.size());
			self.deallocate(ptr, old_layout);
			Ok(new_region)
		} else {
			Ok(NonNull::new_unchecked(core::slice::from_raw_parts_mut(ptr.as_ptr(), new_layout.size())))
		}
	}
	
	unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		if old_layout.size() <= self.chunk_layout.size() && old_layout.align() <= self.chunk_layout.align() {
			Ok(NonNull::new_unchecked(core::slice::from_raw_parts_mut(ptr.as_ptr(), new_layout.size())))
		} else if new_layout.size() <= self.chunk_layout.size() && new_layout.align() <= self.chunk_layout.align() {
			let new_region = self.allocate(new_layout)?;
			ptr::copy(ptr.as_ptr(), new_region.as_mut_ptr(), old_layout.size());
			Global.deallocate(ptr, old_layout);
			Ok(new_region)
		} else {
			Global.shrink(ptr, old_layout, new_layout)
		}
	}
}

#[derive(Copy, Clone, Debug, Default)]
pub struct GlobalChunkAlloc;

unsafe impl Allocator for GlobalChunkAlloc {
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		GLOBAL_CHUNK_ALLOC.allocate(layout)
	}
	
	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		GLOBAL_CHUNK_ALLOC.deallocate(ptr, layout)
	}
	
	unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		GLOBAL_CHUNK_ALLOC.shrink(ptr, old_layout, new_layout)
	}
	
	unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		GLOBAL_CHUNK_ALLOC.shrink(ptr, old_layout, new_layout)
	}
	
	unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		GLOBAL_CHUNK_ALLOC.shrink(ptr, old_layout, new_layout)
	}
}

const OCCUPIED: usize = !0;

pub struct PoolAlloc<T, A: Allocator> {
	chunks:   AtomicPtr<Node<T>>,
	free:     AtomicPtr<Node<T>>,
	capacity: usize,
	layout:   Layout,
	alloc:    A
}

impl<T, A: Allocator + Default> Default for PoolAlloc<T, A> {
	fn default() -> Self {
		Self::new_in(GLOBAL_CHUNK_ALLOC.chunk_layout, A::default())
	}
}

impl<T> PoolAlloc<T, Global> {
	pub fn new(layout: Layout) -> Self {
		Self::new_in(layout, Global)
	}
	
	pub fn with_capacity(capacity: usize, layout: Layout) -> Self {
		Self::with_capacity_in(capacity, layout, Global)
	}
}

impl<T, A: Allocator> PoolAlloc<T, A> {
	pub unsafe fn get_iteration(ptr: *mut T) -> u32 {
		(*Node::from_ptr(ptr)).iter.load(Ordering::SeqCst)
	}
	
	pub unsafe fn is_dirty(ptr: *mut T, iteration: u32) -> bool {
		(*Node::from_ptr(ptr)).iter.load(Ordering::SeqCst) != iteration
	}
	
	pub fn new_in(layout: Layout, alloc: A) -> Self {
		Self {
			chunks:   AtomicPtr::default(),
			free:     AtomicPtr::default(),
			capacity: layout.size() / Layout::new::<Node<T>>().pad_to_align().size(),
			layout,
			alloc
		}
	}
	
	pub fn with_capacity_in(capacity: usize, layout: Layout, alloc: A) -> Self {
		let self_ = Self::new_in(layout, alloc);
		unsafe { self_.reserve_chunks(capacity)
			.expect("failed to allocate new chunk"); }
		self_
	}
	
	pub unsafe fn reserve_chunks(&self, additional: usize) -> Result<(), AllocError> {
		if additional == 0 {
			return Ok(());
		}
		
		let current_next  = self.free.load(Acquire);
		let current_chunk = self.chunks.load(Acquire);
		let mut chunk_    = current_chunk;
		let mut next      = current_next;
		let mut first     = ptr::null_mut();
		
		for i in 0..additional {
			let chunk   = self.alloc.allocate(self.layout)?.as_ptr() as *mut Node<T>;
			let mut ptr = chunk;
			let end     = chunk.add(self.capacity - 1);
			
			if i == 0 {
				first = ptr.add(1);
			}
			
			while end.offset_from(ptr) > 0 {
				ptr = ptr.add(1);
				(*ptr).next = next;
				next = ptr;
			}
			
			(*chunk).next = chunk_;
			chunk_ = chunk;
		}
		
		loop {
			match self.free.compare_exchange(current_next, next, Release, Relaxed) {
				Ok(_)  => break,
				Err(v) => (*first).next = v
			}
		}
		
		loop {
			match self.chunks.compare_exchange(current_chunk, chunk_, Release, Relaxed) {
				Ok(_)  => return Ok(()),
				Err(v) => (*chunk_).next = v
			}
		}
	}
	
	pub unsafe fn acquire(&self) -> *mut T {
		let mut next = self.free.load(Acquire);
		
		loop {
			if !next.is_null() {
				match self.free.compare_exchange(next, (*next).next as _, Release, Relaxed) {
					Ok(_) => {
						(*next).next = OCCUPIED as _;
						return &mut *(*next).val
					},
					Err(v) => next = v
				}
			} else {
				let chunk = self.alloc.allocate(self.layout)
					.expect("failed to allocate new chunk")
					.as_mut_ptr()
					.cast::<Node<T>>();
				
				(*chunk).next              = self.chunks.load(Acquire);
				(*chunk.add(1)).next = OCCUPIED as _;
				let mut ptr                = chunk.add(2);
				let end                    = chunk.add(self.capacity - 1);
				
				while end.offset_from(ptr) > 0 {
					let p = ptr.add(1);
					(*ptr).next = p;
					ptr = p;
				}
				
				(*ptr).next = next;
				
				loop {
					match self.free.compare_exchange(next, chunk.add(2), Release, Relaxed) {
						Ok(_)  => break,
						Err(v) => (*ptr).next = v
					}
				}
				
				loop {
					match self.chunks.compare_exchange((*chunk).next as _, chunk, Release, Relaxed) {
						Ok(_)  => return &mut *(*chunk.add(1)).val,
						Err(v) => (*chunk).next = v
					}
				}
			}
		}
	}
	
	pub unsafe fn acquire_iter<'a>(&'a self, len: usize) -> impl ExactSizeIterator<Item = *mut T> + 'a {
		let mut next = self.free.load(Acquire);
		let mut first;
		
		loop {
			first = next;
			let mut ptr = next;
			
			if ptr.is_null() {
				let mut chunk = self.alloc.allocate(self.layout)
					.expect("failed to allocate new chunk")
					.as_mut_ptr() as *mut Node<T>;
				ptr = chunk;
				first = chunk;
				let end = chunk.add(self.capacity - 1);
				
				while end.offset_from(chunk) > 0 {
					let this = chunk;
					chunk = chunk.add(1);
					(*this).next = chunk;
				}
				
				(*chunk).next = ptr::null();
			}
			
			for _ in 0..len {
				if (*ptr).next.is_null() {
					let mut chunk = self.alloc.allocate(self.layout)
						.expect("failed to allocate new chunk")
						.as_mut_ptr() as *mut Node<T>;
					(*ptr).next = chunk;
					let end = chunk.add(self.capacity - 1);
					
					while end.offset_from(chunk) > 0 {
						let this = chunk;
						chunk = chunk.add(1);
						(*this).next = chunk;
					}
					
					(*chunk).next = ptr::null();
				} else {
					ptr = (*ptr).next as _;
				}
			}
			
			match self.free.compare_exchange(ptr, next, Release, Relaxed) {
				Ok(_)  => break,
				Err(v) => next = v
			}
		}
		
		struct Iter<T>(*mut Node<T>, usize);
		
		impl<T> Iterator for Iter<T> {
			type Item = *mut T;
			
			fn next(&mut self) -> Option<Self::Item> {
				if self.1 == 0 {
					return None;
				}
				
				unsafe {
					self.1 -= 1;
					let ptr = self.0;
					self.0 = (*self.0).next as _;
					(*self.0).next = OCCUPIED as _;
					Some(&mut*(*ptr).val as *mut T)
				}
			}
			
			fn size_hint(&self) -> (usize, Option<usize>) {
				(self.1, Some(self.1))
			}
		}
		
		impl<T> ExactSizeIterator for Iter<T> {}
		
		Iter(first, len)
	}
	
	pub unsafe fn release(&self, ptr: *const T) {
		if ptr.is_null() {
			return;
		}
		
		let ptr = &mut*Node::from_ptr(ptr as _);
		ptr.next = self.free.load(Acquire) as _;
		
		loop {
			match self.free.compare_exchange(ptr.next as _, ptr, Release, Relaxed) {
				Ok(_)  => break,
				Err(v) => ptr.next = v
			}
		}
	}
	
	pub unsafe fn release_iter(&self, iter: impl IntoIterator<Item = *const T>) {
		let mut next = self.free.load(Ordering::Acquire);
		let mut first = None;
		
		for ptr in iter {
			if ptr.is_null() {
				continue;
			}
			
			let ptr = &mut*Node::from_ptr(ptr as *mut T);
			
			if first.is_none() {
				first = Some(ptr as *mut Node<T>);
			}
			
			ptr.next = next;
			next = ptr;
		}
		
		if first.is_none() {
			return;
		}
		
		loop {
			match self.free.compare_exchange((*first.unwrap_unchecked()).next as _, next, Release, Relaxed) {
				Ok(_)  => break,
				Err(v) => (*first.unwrap_unchecked()).next = v
			}
		}
	}
	
	pub fn iter<'a>(&'a self) -> PoolAllocIter<'a, T, A> {
		let mut ptr = self.chunks.load(Relaxed) as *const Node<T>;
		let mut end = unsafe { ptr.add(self.capacity - 1) };
		PoolAllocIter { alloc: self, ptr, end }
	}
	
	pub unsafe fn is_alive(&self, ptr: *mut T) -> bool {
		!ptr.is_null() && (*Node::from_ptr(ptr)).next == OCCUPIED as _
	}
	
	pub unsafe fn clear(&self) {
		loop {
			self.free.store(ptr::null_mut(), SeqCst);
			let mut chunk = self.chunks.load(Relaxed);
			let mut next  = ptr::null_mut::<Node<T>>();
			
			while !chunk.is_null() {
				let mut ptr = chunk;
				let end     = chunk.add(self.capacity - 1);
				
				while end.offset_from(ptr) > 0 {
					ptr = ptr.add(1);
					(*ptr).next = next;
					next = ptr;
				}
				
				chunk = (*chunk).next as _;
			}
			
			match self.free.compare_exchange(ptr::null_mut(), next, SeqCst, Relaxed) {
				Ok(_)  => return,
				Err(_) => continue
			}
		}
	}
}

impl<T, A: Allocator + fmt::Debug> fmt::Debug for PoolAlloc<T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct(core::any::type_name::<Self>())
			.field("alloc", &self.alloc)
			.finish()
	}
}

impl<T, A: Allocator> Drop for PoolAlloc<T, A> {
	fn drop(&mut self) {
		let mut ptr = self.chunks.swap(ptr::null_mut(), Ordering::SeqCst) as *const Node<T>;
		
		while !ptr.is_null() {
			unsafe {
				let next = (*ptr).next;
				self.alloc.deallocate(ptr::NonNull::new_unchecked(ptr as _), self.layout);
				ptr = next;
			}
		}
	}
}

#[repr(C)]
struct Node<T> {
	next: *const Self,
	iter: AtomicU32,
	_pad: u32,
	val:  core::mem::ManuallyDrop<T>
}

impl<T> Node<T> {
	unsafe fn from_ptr(ptr: *mut T) -> *mut Self {
		ptr.cast::<u8>()
			.sub(core::mem::size_of::<*const Self>() + 8)
			.cast()
	}
}

pub struct PoolAllocIter<'a, T, A: Allocator> {
	alloc: &'a PoolAlloc<T, A>,
	ptr:   *const Node<T>,
	end:   *const Node<T>
}

impl<'a, T, A: Allocator> Iterator for PoolAllocIter<'a, T, A> {
	type Item = *const T;
	
	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			loop {
				if self.ptr.is_null() {
					return None;
				} else if self.end.offset_from(self.ptr) <= 0 {
					self.ptr = (*self.ptr.sub(self.alloc.capacity - 1)).next;
					self.end = self.ptr.add(self.alloc.capacity - 1);
				} else {
					self.ptr = self.ptr.add(1);
					
					if (*self.ptr).next as usize != OCCUPIED {
						continue;
					}
					
					return Some(&*(*self.ptr).val as *const _);
				}
			}
		}
	}
}