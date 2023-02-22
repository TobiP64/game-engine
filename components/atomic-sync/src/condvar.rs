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
	std::{
		sync::{Arc, Weak},
		task::{Waker, Poll, Context},
		future::Future,
		pin::Pin
	},
	crate::block_on,
	concurrent_queue::ConcurrentQueue
};

#[derive(Debug, Clone)]
pub struct AsyncCondvar(Arc<ConcurrentQueue<Waker>>);

impl AsyncCondvar {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn wait(&self) {
		block_on(self.wait_async());
	}
	
	pub fn wait_async(&self) -> AsyncCondvarFuture {
		AsyncCondvarFuture(Arc::downgrade(&self.0))
	}
	
	pub fn notify_one(&self) {
		if let Ok(waker) = self.0.pop() {
			waker.wake()
		}
	}
	
	pub fn notify_all(&self) {
		while let Ok(waker) = self.0.pop() {
			waker.wake();
		}
	}
}

impl Default for AsyncCondvar {
	fn default() -> Self {
		Self(Arc::new(ConcurrentQueue::unbounded()))
	}
}

#[derive(Default)]
pub struct AsyncCondvarFuture(Weak<ConcurrentQueue<Waker>>);

impl Future for AsyncCondvarFuture {
	type Output = ();
	
	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		match std::mem::replace(&mut self.0, Weak::new()).upgrade() {
			None => Poll::Ready(()),
			Some(queue) => {
				std::mem::drop(queue.push(cx.waker().clone()));
				Poll::Pending
			}
		}
	}
}