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
	crate::*,
	core::{
		fmt,
		ptr::NonNull,
		sync::atomic::*,
		alloc::{Allocator, Layout},
		any::TypeId,
		future::Future,
		task::*,
		pin::Pin
	}
};

#[cfg(feature = "std")]
use std::alloc::Global;
#[cfg(not(feature = "std"))]
use ::alloc::alloc::Global;

pub struct Schedule<A: Allocator = Global> {
	systems_offset: usize,
	systems_stride: usize,
	systems_len:    usize,
	bitset_stride:  usize,
	data:           *mut u8,
	layout:         Layout,
	alloc:          A
}

impl Schedule {
	pub fn new<'a>(systems: impl IntoIterator<Item = (Box<dyn System<In = (), Out = ()>>, &'a [usize]), IntoIter: ExactSizeIterator>) -> Self {
		Self::new_in(systems, Global)
	}
}

impl<A: Allocator> Schedule<A> {
	const BITSET_SHIFT:           usize = 8;
	const BITSET_MASK:            u64   = 0xF;
	const SYSTEM_FIXED_LEN:       usize = 8 + 24 + 16;
	const SYSTEM_STATE_READY:     u64   = 0b01;
	const SYSTEM_STATE_EXECUTING: u64   = 0b10;
	const SYSTEM_STATE_FINISHED:  u64   = 0b11;
	
	pub fn new_in<'a>(systems: impl IntoIterator<Item = (Box<dyn System<In = (), Out = ()>>, &'a [usize]), IntoIter: ExactSizeIterator>, alloc: A) -> Self {
		unsafe {
			let mut systems    = systems.into_iter();
			let systems_len    = systems.len();
			let bitset_stride  = ((systems_len - 1) >> Self::BITSET_SHIFT) + 1;
			let systems_offset = 2 * bitset_stride;
			let systems_stride = Self::SYSTEM_FIXED_LEN + 3 * bitset_stride;
			let layout         = Layout::from_size_align(systems_offset + systems_len * systems_stride, 8).unwrap();
			let data           = alloc.allocate_zeroed(layout)
				.expect("allocation failed")
				.as_mut_ptr();
			
			let mut ptr = data.add(systems_offset);
			
			for (i, (system, dependencies)) in systems.enumerate() {
				ptr.add(8).cast::<String>().write(String::new());
				ptr.add(32).cast::<Box<dyn System<In = (), Out = ()>>>().write(system);
				
				let dependencies_ptr = ptr.add(Self::SYSTEM_FIXED_LEN + bitset_stride)
					.cast::<u64>();
				
				for dependency in dependencies {
					*dependencies_ptr.add(dependency >> Self::BITSET_SHIFT) |= 1 << (*dependency as u64 & Self::BITSET_MASK);
					*data.add(systems_offset + dependency * systems_stride + Self::SYSTEM_FIXED_LEN + 2 * bitset_stride)
						.cast::<u64>()
						.add(i >> Self::BITSET_SHIFT)
						|= 1 << (i as u64 & Self::BITSET_MASK);
				}
				
				ptr = ptr.add(systems_stride);
			}
			
			let mut self_ = Self {
				systems_offset,
				systems_stride,
				systems_len,
				bitset_stride,
				data,
				layout,
				alloc
			};
			
			self_.rebuild();
			self_
		}
	}
	
	pub fn rebuild(&mut self) {
	
	}
	
	pub async fn run(
		&mut self,
		world:     &World,
		resources: &Resources,
		executor:  impl Fn(BoxedFuture<()>)
	) {
		struct Waiter<'a> {
			waker:  &'a AtomicPtr<()>,
			wakeup: &'a AtomicBool
		}
		
		impl<'a> Future for Waiter<'a> {
			type Output = ();
			
			fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
				if self.wakeup.swap(false, Ordering::SeqCst) {
					Poll::Ready(())
				} else {
					self.waker.store(cx.waker().clone(), Ordering::SeqCst);
					Poll::Pending
				}
			}
		}
		
		unsafe {
			let mut ptr = self.data.add(self.systems_offset);
			
			for i in 0..self.systems_len {
				'dependencies: {
					let mut ptr = ptr.add(Self::SYSTEM_FIXED_LEN + self.bitset_stride);
					let end = ptr.add(self.bitset_stride);
					
					while end.offset_from(ptr) > 0 {
						if *ptr.cast::<u64>() != 0 {
							break 'dependencies;
						}
						
						ptr = ptr.add(8);
					}
					
					*self.data.cast::<u64>().add(i >> 7) |= 1 << ((i & 0xC) * 2);
				}
				
				ptr = ptr.add(self.systems_stride);
			}
			
			let waker = AtomicPtr::default();
			
			loop {
				let mut ptr = self.data.cast::<AtomicU64>();
				let end = self.data.add(self.systems_len >> Self::BITSET_SHIFT);
				
				for i in 0..self.systems_len {
					// check pre conditions
					
					self.executing[i >> Self::BITSET_SHIFT].fetch_or(1 << (i as u64 & Self::BITSET_MASK), Ordering::Acquire);
					let boxed = ;
					
					(executor)(Box::pin(async move {
						boxed.run(context, ()).await;
						self.executing[i >> Self::BITSET_SHIFT].fetch_and(!(1 << (i as u64 & Self::BITSET_MASK)), Ordering::Release);
						
						loop {
							let waker = waker.swap()
						}
						
						if let Some(waker) = self.waker.lock().take() {
							waker.wake();
						}
					}));
				}
				
				Waiter(self).await
			}
		}
	}
}

impl<A: Allocator + fmt::Debug> fmt::Debug for Schedule<A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		struct DebugSystems<'a, A: Allocator>(&'a Schedule<A>);
		
		impl<'a, A: Allocator> fmt::Debug for DebugSystems<'a, A> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				unsafe {
					let mut dbg = f.debug_list();
					let mut ptr = self.0.data.add(self.0.systems_offset);
					let mut end = ptr.add(self.0.systems_len * self.0.systems_stride);
					
					while end.offset_from(ptr) > 0 {
						dbg.entry(&DebugSystem(ptr));
						ptr = ptr.add(self.0.systems_stride);
					}
					
					dbg.finish()
				}
			}
		}
		
		struct DebugSystem(*mut u8);
		
		impl fmt::Debug for DebugSystem {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
				unsafe {
					f.debug_struct("System")
						.field("id", &*self.0.cast::<TypeId>())
						.field("name", &*self.0.add(8).cast::<String>())
						.field("conflicts", &[])
						.field("dependencies", &[])
						.field("dependants", &[])
						.finish()
				}
			}
		}
		
		f.debug_struct(core::any::type_name::<Self>())
			.field("systems", &DebugSystems(self))
			.field("layout", &self.layout)
			.field("alloc", &self.alloc)
			.finish()
	}
}

impl<A: Allocator> Drop for Schedule<A> {
	fn drop(&mut self) {
		unsafe {
			let mut ptr = self.data.add(self.systems_offset);
			let end = ptr.add(self.systems_len * self.systems_stride);
			
			while ptr.offset_from(end) > 0 {
				core::mem::forget(ptr.add(32).cast::<Box<dyn System<In = (), Out = ()>>>().read());
				ptr = ptr.add(self.systems_stride);
			}
			
			self.alloc.deallocate(NonNull::new_unchecked(self.data), self.layout);
		}
	}
}