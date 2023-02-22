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

use std::{alloc::Allocator, sync::atomic::*, ptr};

pub struct ConcurrentLinkedList<T, A: Allocator = std::alloc::Global> {
	head:  AtomicPtr<Node<T>>,
	tail:  AtomicPtr<Node<T>>,
	cache: AtomicPtr<Node<T>>,
	len:   AtomicUsize,
	alloc: A
}

pub struct Node<T> {
	prev: AtomicPtr<Node<T>>,
	next: AtomicPtr<Node<T>>,
	val:  AtomicPtr<T>
}

impl<T> ConcurrentLinkedList<T> {
	pub fn new() -> Self {
		Self::new_in(std::alloc::Global)
	}
}

impl<T, A: Allocator> ConcurrentLinkedList<T, A> {
	pub fn new_in(alloc: A) -> Self {
		Self {
			head:  AtomicPtr::default(),
			tail:  AtomicPtr::default(),
			cache: AtomicPtr::default(),
			len:   AtomicUsize::new(0),
			alloc
		}
	}
	
	pub fn len(&self) -> usize {
		self.len.load(Ordering::Relaxed)
	}
	
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
	
	pub fn contains(&self, e: &T) -> bool {
		unimplemented!()
	}
	
	pub fn front(&self) -> T where T: Copy {
		unimplemented!()
	}
	
	pub fn back(&self) -> T where T: Copy {
		unimplemented!()
	}
	
	pub fn push_front(&self, e: T) {
		unimplemented!()
	}
	
	pub fn push_back(&self, e: T) {
		unimplemented!()
	}
	
	pub fn pop_front(&self) -> Option<T> {
		unimplemented!()
	}
	
	pub fn pop_back(&self) -> Option<T> {
		unimplemented!()
	}
	
	pub fn clear(&self) {
		let head = self.head.swap(ptr::null_mut(), Ordering::Relaxed);
		self.tail.store(ptr::null_mut(), Ordering::Relaxed);
		unimplemented!()
	}
}
impl<T, A: Allocator + Default> Default for ConcurrentLinkedList<T, A> {
	fn default() -> Self {
		Self::new_in(A::default())
	}
}

impl<T: std::fmt::Debug, A: Allocator + std::fmt::Debug> std::fmt::Debug for ConcurrentLinkedList<T, A> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct(std::any::type_name::<Self>())
			.field("len", &self.len.load(Ordering::Relaxed))
			.field("alloc", &self.alloc)
			.finish()
	}
}