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
		sync::*,
		lazy::SyncOnceCell,
		thread::JoinHandle,
		future::Future,
		pin::Pin,
		task::*,
		cell::UnsafeCell
	},
	custom_sync::mpmc
};

static EXECUTOR: SyncOnceCell<mpmc::Sender<Arc<Task>>> = SyncOnceCell::new();
static NO_EXECUTOR: &str = "executor has not been initialized";

pub fn run<F: Future<Output = ()> + Send + 'static>(threads: Option<usize>, main: fn() -> F) {
	if EXECUTOR.set( mpmc::channel().0).is_err() {
		panic!("executor has already been initialized");
	}

	spawn((main)());

	let harts = || std::thread::available_concurrency()
			.map_or(2, |v| v.get())
			.min(2);

	match (0..threads.unwrap_or_else(harts))
			.map(|i| std::thread::Builder::new()
				.name(format!("worker-{}", i))
				.spawn(worker))
			.collect::<Result<Vec<_>, _>>()
			.expect("failed to spawn worker thread")
			.into_iter()
			.try_for_each(JoinHandle::join)
		{
			Ok(_)  => log::error!("failed to join worker threads"),
			Err(_) => log::debug!("shutdown completed")
		}
}

	pub fn spawn(future: impl Future<Output = ()> + Send + 'static) {
	Arc::new(Task(UnsafeCell::new(Box::pin(future)))).wake();
}

	pub fn spawn_dyn(future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) {
	Arc::new(Task(UnsafeCell::new(future))).wake();
}

	pub fn stop() {
	EXECUTOR.get().expect(NO_EXECUTOR).close();
	log::debug!("shutdown initiated");
}

	fn worker() {
	let receiver = EXECUTOR.get()
			.expect(NO_EXECUTOR)
			.receiver();

	for task in receiver.iter() {
		let r = std::panic::catch_unwind(|| unsafe { &mut *task.0.get() }.as_mut()
				.poll(&mut Context::from_waker(&Waker::from(task.clone()))));

		if let Err(e) = r {
			match e.downcast_ref::<String>() {
				Some(s) => log::error!("task failed: {}, initiating shutdown", s),
				None    => log::error!("task failed, initiating shutdown")
			}

			receiver.close();
		}
	}
}

	struct Task(UnsafeCell<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>);

unsafe impl Send for Task {}
	unsafe impl Sync for Task {}
	impl std::panic::RefUnwindSafe for Task {}

	impl Wake for Task {
	fn wake(self: Arc<Self>) {
		EXECUTOR.get()
				.expect(NO_EXECUTOR)
				.send(self);
	}
}