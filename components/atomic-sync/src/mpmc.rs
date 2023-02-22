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

use std::{collections::VecDeque, sync::{*, mpsc::TryRecvError, atomic::{AtomicIsize, Ordering}}};

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
	let state = Arc::new(State {
		senders:   AtomicIsize::new(1),
		receivers: AtomicIsize::new(1),
		buf:       Mutex::new(VecDeque::new()),
		wait_recv: Condvar::new()
	});
	
	(Sender(state.clone()), Receiver(state))
}

#[derive(Debug)]
struct State<T> {
	senders:   AtomicIsize,
	receivers: AtomicIsize,
	buf:       Mutex<VecDeque<T>>,
	wait_recv: Condvar
}

impl<T> State<T> {
	fn close(&self) {
		self.senders.store(0, Ordering::SeqCst);
		self.receivers.store(0, Ordering::SeqCst);
		self.wait_recv.notify_all();
	}
}

#[derive(Debug)]
pub struct Sender<T>(Arc<State<T>>);

impl<T> Sender<T> {
	pub fn send(&self, v: T) -> bool {
		if self.0.receivers.load(Ordering::SeqCst) <= 0 {
			return false;
		}
		
		self.0.buf.lock().unwrap().push_back(v);
		self.0.wait_recv.notify_one();
		true
	}
	
	pub fn send_iter(&self, iter: impl IntoIterator<Item = T>) -> bool {
		if self.0.receivers.load(Ordering::SeqCst) <= 0 {
			return false;
		}
		
		self.0.buf.lock().unwrap().extend(iter);
		self.0.wait_recv.notify_all();
		true
	}
	
	pub fn receiver(&self) -> Receiver<T> {
		self.0.receivers.fetch_add(1, Ordering::SeqCst);
		Receiver(self.0.clone())
	}
	
	pub fn close(&self) {
		self.0.close();
	}
}

impl<T> Clone for Sender<T> {
	fn clone(&self) -> Self {
		self.0.senders.fetch_add(1, Ordering::SeqCst);
		Self(self.0.clone())
	}
}

impl<T> Drop for Sender<T> {
	fn drop(&mut self) {
		if self.0.senders.fetch_sub(1, Ordering::SeqCst) == 1 {
			self.0.wait_recv.notify_all();
		}
	}
}

#[derive(Debug)]
pub struct Receiver<T>(Arc<State<T>>);

impl<T> Receiver<T> {
	pub fn recv(&self) -> Option<T> {
		self.0.wait_recv.wait_while(
			self.0.buf.lock().unwrap(),
			|buf| buf.is_empty() && self.0.senders.load(Ordering::SeqCst) > 0
		).unwrap().pop_front()
	}
	
	pub fn try_recv(&self) -> Result<T, TryRecvError> {
		if self.0.senders.load(Ordering::SeqCst) <= 0 {
			return Err(TryRecvError::Disconnected);
		}
		
		self.0
			.buf
			.lock()
			.unwrap()
			.pop_front()
			.ok_or(TryRecvError::Empty)
	}
	
	pub fn iter(&self) -> RecvIter<T> {
		RecvIter(self)
	}
	
	pub fn try_iter(&self) -> TryRecvIter<T> {
		TryRecvIter(self)
	}
	
	pub fn sender(&self) -> Sender<T> {
		self.0.senders.fetch_add(1, Ordering::SeqCst);
		Sender(self.0.clone())
	}
	
	pub fn close(&self) {
		self.0.close();
	}
}

impl<T> Clone for Receiver<T> {
	fn clone(&self) -> Self {
		self.0.receivers.fetch_add(1, Ordering::SeqCst);
		Self(self.0.clone())
	}
}

impl<T> Drop for Receiver<T> {
	fn drop(&mut self) {
		self.0.receivers.fetch_sub(1, Ordering::SeqCst);
	}
}

pub struct RecvIter<'a, T>(&'a Receiver<T>);

impl<'a, T> Iterator for RecvIter<'a, T> {
	type Item = T;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.recv()
	}
}

pub struct TryRecvIter<'a, T>(&'a Receiver<T>);

impl<'a, T> Iterator for TryRecvIter<'a, T> {
	type Item = T;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.try_recv().ok()
	}
}
