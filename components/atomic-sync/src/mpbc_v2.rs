use std::{
	ptr::{null, null_mut},
	sync::{Arc, Mutex, atomic::*},
	collections::VecDeque,
	thread::Thread,
	task::{Waker, Context, Poll},
	future::Future,
	pin::Pin
};

const DEFAULT_BLOCK_LEN: usize = 64;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
	let mut block = Box::new([unsafe { std::mem::zeroed() }; DEFAULT_BLOCK_LEN]);
	block[0].refs = AtomicUsize::new(1);
	
	for i in 1..DEFAULT_BLOCK_LEN - 1 {
		block[i].next = AtomicPtr::new(&mut block[i + 1]);
	}
	
	let state = Arc::new(State {
		head:  AtomicPtr::new(&mut block[0]),
		cache: AtomicPtr::new(&mut block[1]),
		refs:  AtomicUsize::new(1),
		fixed: false,
		send:  ConcurrentQueue::default(),
		recv:  ConcurrentQueue::default()
	});

	(Sender { state: state.clone() }, Receiver { state, next: &mut Box::leak(block)[0], ghost: true })
}

#[derive(Default)]
struct State<T> {
	head:  AtomicPtr<Node<T>>,
	tail:  AtomicPtr<Node<T>>,
	refs:  AtomicUsize,
	fixed: bool,
	send:  ConcurrentQueue<Waker>,
	recv:  ConcurrentQueue<Waker>,
}

struct Node<T> {
	next:  AtomicPtr<T>,
	refs:  AtomicUsize,
	value: T
}

pub struct Sender<T> {
	state: Arc<State<T>>
}

impl<T> Sender<T> {
	pub fn try_send(&mut self, v: T) -> Option<T> {
		let node = loop {
			let ptr = self.state.cache.load(Ordering::SeqCst);
			
			if ptr.is_null() {
				if self.state.fixed {
					return None;
				}
				
				// TODO alloc
			}
			
			let ptr = unsafe { ptr.as_mut() }.unwrap();
			let new_ptr = ptr.next.load(Ordering::SeqCst);
			
			if self.state.cache.compare_and_swap(ptr, new_ptr, Ordering::SeqCst) == ptr {
				break ptr;
			}
		};
		
		node.value = v;
		
		loop {
			let top = self.state.top.load(Ordering::SeqCst);
			let top_ref = unsafe { top.as_ref() }.expect("invalid state: state.top was null");
			
			if !top_ref.next.compare_and_swap(null_mut(), node, Ordering::SeqCst).is_null() {
				continue;
			}
			
			if self.state.top.compare_and_swap(top, node, Ordering::SeqCst) == top {
				return None;
			}
		}
	}
	
	pub fn blocking_send(&mut self, mut v: T) {

	}
	
	pub fn async_send(&mut self, v: T) -> SendFuture<T> {
		SendFuture { sender: self, val: Some(v) }
	}
}

impl<T> Drop for Sender<T> {
	fn drop(&mut self) {
		unimplemented!()
	}
}

pub struct Receiver<T> {
	state: Arc<State<T>>,
	next:  *const Node<T>,
	ghost: bool
}

impl<T> Receiver<T> {
	pub fn try_recv(&mut self) -> Option<&T> {
		let next = unsafe { self.next.as_ref().expect("invalid state: next was null") };
		let next_next = next.next.load(Ordering::SeqCst);
		
		if !next_next.is_null() {
			self.next = next_next;
		} else {
			self.ghost = true;
		}
		
		if next.refs.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |refs| (refs != 0).then(|| refs - 1)).is_ok() {
			Some(&next.value)
		} else {
			None
		}
	}

	pub fn blocking_recv(&mut self) -> &T {
		unimplemented!()
	}

	pub fn async_recv(&mut self) -> RecvFuture<T> {
		RecvFuture { recv: self }
	}
}

impl<T> Clone for Receiver<T> {
	fn clone(&self) -> Self {
		let next = self.state.top.load(Ordering::SeqCst);
		Self {
			state: self.state.clone(),
			next,
			ghost: self.ghost && self.next == next
		}
	}
}

impl<T> Drop for Receiver<T> {
	fn drop(&mut self) {
		self.state.refs.fetch_sub(1, Ordering::SeqCst);
		
		unimplemented!()
	}
}

pub struct SendFuture<'a, T> {
	sender: &'a mut Sender<T>,
	val: Option<T>,
}

impl<'a, T> Future for SendFuture<'a, T> {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		while let Some(val) = self.sender.try_send(self.val.take().unwrap()) {
			let blocked = self.sender.state.send.lock().expect("failed to acquire lock");
			if let Some(val) = self.sender.try_send(val) {
				blocked.wake.push_back(cx.waker().clone());
				self.val = Some(val);
				return Poll::Pending;
			} else {
				return Poll::Ready(());
			}
		}
		
		return Poll::Ready(());
	}
}

pub struct RecvFuture<'a, T> {
	recv: &'a mut Receiver<T>
}

impl<'a, T> Future for RecvFuture<'a, T> {
	type Output = T;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		unimplemented!()
	}
}