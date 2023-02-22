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

#![feature(drain_filter, arbitrary_self_types, allocator_api)]
#![warn(clippy::all)]
#![allow(dead_code)]

use std::{
	sync::Arc,
	future::Future,
	task::{Context, Poll, Wake, Waker},
	pin::Pin,
	thread::Thread
};

pub use self::{condvar::*};

pub mod mpmc;
pub mod mpbc;
//mod mpbc_v2;
pub mod condvar;
pub mod concurrent_dequeue;
pub mod concurrent_linked_list;
//pub mod concurrent_tree;

pub fn block_on<F: Future<Output = T> + Unpin, T>(mut future: F) -> T {
	struct ThreadWaker(Thread);

	impl Wake for ThreadWaker {
		fn wake(self: Arc<Self>) {
			self.0.unpark()
		}

		fn wake_by_ref(self: &Arc<Self>) {
			self.0.unpark()
		}
	}

	let waker = Waker::from(Arc::new(ThreadWaker(std::thread::current())));
	let mut cx = Context::from_waker(&waker);

	loop {
		match Pin::new(&mut future).poll(&mut cx) {
			Poll::Ready(v) => return v,
			Poll::Pending => std::thread::park()
		}
	}
}