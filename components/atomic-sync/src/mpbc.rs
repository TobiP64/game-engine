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
		sync::{*, atomic::*, mpsc::TryRecvError},
		collections::VecDeque,
		task::{Poll, Context, Waker},
		pin::Pin,
		future::Future
	},
	crate::block_on
};

pub fn channel<T>(len: usize) -> (Sender<T>, Receiver<T>) {
	let state = Arc::new(RwLock::new(State {
		buf:       VecDeque::with_capacity(len),
		idx:       0,
		len,
		receivers: Vec::new(),
		recv:      VecDeque::new()
	}));
	
	let recv = Receiver {
		state: Arc::downgrade(&state),
		idx:   Arc::new(AtomicUsize::new(0))
	};
	
	state.write().unwrap().receivers.push(Arc::downgrade(&recv.idx));
	
	(Sender { state }, recv)
}

#[derive(Debug)]
struct State<T> {
	buf:       VecDeque<T>,
	idx:       usize,
	len:       usize,
	// TODO instead of this, ref count could also be stored with each element
	receivers: Vec<Weak<AtomicUsize>>,
	//send:      VecDeque<Waker>,
	recv:      VecDeque<Waker>,
}

impl<T> State<T> {
	fn send(&mut self, iter: impl IntoIterator<Item = T>) {
		if self.receivers.is_empty() {
			return;
		} else if self.buf.len() == self.buf.capacity() {
			self.receivers.drain_filter(|recv| recv.strong_count() == 0);
			
			let min = self.receivers.iter()
				.filter_map(Weak::upgrade)
				.map(|v| v.load(Ordering::SeqCst))
				.min()
				.unwrap_or(self.idx);
			
			self.buf.drain(..self.buf.len() - (self.idx - min));
		}
		
		let len = self.buf.len();
		self.buf.extend(iter);
		self.idx += self.buf.len() - len;
		self.recv.drain(..).for_each(|w| w.wake());
	}
}

pub struct RecvFuture<T> {
	state: Weak<RwLock<State<T>>>,
	idx:   Arc<AtomicUsize>
}

impl<T: Clone> Future for RecvFuture<T> {
	type Output = Option<T>;
	
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let state0 = match self.state.upgrade() {
			None => return Poll::Ready(None),
			Some(state) => state
		};
		
		let state = match state0.read() {
			Err(_) => return Poll::Ready(None),
			Ok(state) => state
		};
		
		if self.idx.load(Ordering::SeqCst) == state.idx {
			std::mem::drop(state);
			
			let mut state = match state0.write() {
				Err(_) => return Poll::Ready(None),
				Ok(state) => state
			};
			
			state.recv.push_back(cx.waker().clone());
			Poll::Pending
		} else {
			Poll::Ready(Some(state.buf[state.buf.len() - (state.idx - self.idx.fetch_add(1, Ordering::SeqCst))].clone()))
		}
	}
}

#[derive(Clone, Debug)]
pub struct Sender<T> {
	state: Arc<RwLock<State<T>>>
}

impl<T> Sender<T> {
	pub fn send(&self, v: T) {
		self.send_iter(std::iter::once(v))
	}
	
	pub fn send_iter(&self, iter: impl IntoIterator<Item = T>) {
		self.state.write().unwrap().send(iter)
	}
	
	pub fn try_send(&self, v: T) -> Result<(), T> {
		self.send(v);
		Ok(())
	}
	
	pub fn receiver(&self) -> Receiver<T> {
		let mut state = self.state.write().unwrap();
		let idx = Arc::new(AtomicUsize::new(state.idx));
		state.receivers.push(Arc::downgrade(&idx));
		
		Receiver {
			state: Arc::downgrade(&self.state),
			idx
		}
	}
}

#[derive(Debug)]
pub struct Receiver<T> {
	state: Weak<RwLock<State<T>>>,
	idx:   Arc<AtomicUsize>
}

impl<T: Clone> Receiver<T> {
	pub fn recv(&self) -> Option<T> {
		block_on(self.recv_async())
	}
	
	pub fn iter(&self) -> RecvIter<T> {
		RecvIter(self)
	}
	
	pub fn try_recv(&self) -> Result<T, TryRecvError> {
		let state0 = self.state.upgrade().ok_or(TryRecvError::Disconnected)?;
		let state = state0.read().unwrap();
		
		if self.idx.load(Ordering::SeqCst) == state.idx {
			Err(TryRecvError::Empty)
		} else {
			Ok(state.buf[state.buf.len() - (state.idx - self.idx.fetch_add(1, Ordering::SeqCst))].clone())
		}
	}
	
	pub fn try_iter(&self) -> TryRecvIter<T> {
		TryRecvIter {
			//state: self.tmp.as_ref().unwrap().read().unwrap(),
			//idx:   self.idx.load(Ordering::SeqCst),
			recv:  self
		}
	}
	
	pub fn recv_async(&self) -> RecvFuture<T> {
		RecvFuture { state: self.state.clone(), idx: self.idx.clone() }
	}
}

impl<T> Clone for Receiver<T>  {
	fn clone(&self) -> Self {
		Self {
			state: self.state.clone(),
			idx:   match self.state.upgrade() {
				Some(state) => {
					let mut state = state.write().unwrap();
					let idx = Arc::new(AtomicUsize::new(state.idx));
					state.receivers.push(Arc::downgrade(&idx));
					idx
				},
				None => Arc::new(AtomicUsize::new(0))
			}
		}
	}
}

impl<T> Default for Receiver<T> {
	fn default() -> Self {
		Self {
			state: Weak::new(),
			idx:   Arc::new(AtomicUsize::new(0))
		}
	}
}

pub struct RecvIter<'a, T: Clone>(&'a Receiver<T>);

impl<'a, T: Clone> Iterator for RecvIter<'a, T> {
	type Item = T;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.recv()
	}
}

pub struct TryRecvIter<'a, T> {
	recv:   &'a Receiver<T>,
	//state:  RwLockReadGuard<'a, State<T>>,
	//idx:    usize
}

impl<'a, T: Clone> Iterator for TryRecvIter<'a, T> {
	type Item = T;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.recv.try_recv().ok()
		/*if self.idx < self.state.idx {
			self.idx += 1;
			Some(self.state.buf[self.state.buf.len() - (self.state.idx - self.idx - 1)].clone())
		} else {
			None
		}*/
	}
}

/*impl<'a, T> Drop for TryRecvIter<'a, T> {
	fn drop(&mut self) {
		self.recv.idx.store(self.idx, Ordering::SeqCst);
	}
}*/