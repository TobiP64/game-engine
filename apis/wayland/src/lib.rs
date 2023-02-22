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

#![feature(
	arbitrary_self_types,
	c_variadic,
	allocator_api
)]
#![warn(clippy::all)]
#![allow(
	clippy::unreadable_literal,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::type_complexity,
	clippy::zero_prefixed_literal,
	clippy::missing_safety_doc,
	clippy::single_match,
	clippy::match_single_binding,
	clippy::too_many_arguments,
	clippy::from_over_into,
	clippy::result_unit_err,
	unused_variables,
	dead_code,
	deprecated
)]

use {std::os::unix::io::RawFd, libloading::*, std::ptr::NonNull};

pub use protocols::*;

pub mod protocols;

pub enum WlProxy {}

impl WlProxy {
	pub unsafe fn get_class(self: *const Self) -> *const u8 {
		(LIB_WAYLAND.wl_proxy_get_class)(self)
	}

	pub unsafe fn get_version(self: *const Self) -> u32 {
		(LIB_WAYLAND.wl_proxy_get_version)(self)
	}

	pub unsafe fn get_id(self: *const Self) -> u32 {
		(LIB_WAYLAND.wl_proxy_get_id)(self)
	}

	pub unsafe fn set_user_data(self: *mut Self, data: *mut u8) {
		(LIB_WAYLAND.wl_proxy_set_user_data)(self, data)
	}

	pub unsafe fn get_user_data(self: *const Self) -> *const u8 {
		(LIB_WAYLAND.wl_proxy_get_user_data)(self)
	}

	pub unsafe fn add_dispatcher(self: *mut Self, dispatcher: WlDispatcherFunc, implementation: *mut u8, user_data: *mut u8) -> u32 {
		(LIB_WAYLAND.wl_proxy_add_dispatcher)(self, dispatcher, implementation, user_data)
	}

	pub unsafe fn destroy(self: *mut Self) {
		(LIB_WAYLAND.wl_proxy_destroy)(self);
	}
}

impl std::fmt::Debug for WlProxy {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct(std::any::type_name::<Self>())
			.field("id", unsafe { &Self::get_id(self) })
			.field("class", &unsafe { std::ffi::CStr::from_ptr(Self::get_class(self) as _) }.to_string_lossy().as_ref())
			.field("version", unsafe { &Self::get_version(self) })
			.finish()
	}
}

pub enum WlEventQueue {}

pub type Result<T> = std::result::Result<T, ()>;
pub type WlLogFunc = extern fn(*const u8, args: std::ffi::VaList);
pub type WlDispatcherFunc = extern fn(*const u8, *mut WlProxy, u32, *const WlMessage, *mut WlArgument);

impl WlDisplay {
	pub fn set_log_handler(func: WlLogFunc) {
		unsafe { (LIB_WAYLAND.wl_log_set_handler_client)(func); }
	}

	pub fn connect_to_fd(fd: RawFd) -> Result<Box<WlDisplay, WlAlloc>> {
		unsafe { box_from_raw((LIB_WAYLAND.wl_display_connect_to_fd)(fd as _)) }
	}

	/// Connect to the Wayland display named `name`. If `name` is `None`, its value will be
	/// replaced e `WAYLAND_DISPLAY` environment variable if it is set, otherwise display
	/// `"wayland-0"` will be used.
	///
	/// If `WAYLAND_SOCKET` is set, it's interpreted as a file descriptor number referring to an
	/// already opened socket. In this case, the socket is used as-is and `name` is ignored.
	///
	/// If `name` is a relative path, then the socket is opened relative to the `XDG_RUNTIME_DIR`
	/// directory.
	///
	/// If `name` is an absolute path, then that path is used as-is for the location of the socket
	/// at which the Wayland server is listening; no qualification inside `XDG_RUNTIME_DIR` is
	/// attempted.
	///
	/// If `name` is `None` and the `WAYLAND_DISPLAY` environment variable is set to an absolute
	/// pathname, then that pathname is used as-is for the socket in the same manner as if `name`
	/// held an absolute path. Support for absolute paths in `name` and `WAYLAND_DISPLAY` is
	/// present since Wayland version 1.15.
	///
	/// `name` mut be null-terminated.
	pub fn connect(name: Option<&str>) -> Result<Box<WlDisplay, WlAlloc>> {
		unsafe { box_from_raw((LIB_WAYLAND.wl_display_connect)(name.map_or(std::ptr::null(), str::as_ptr))) }
	}

	/// Return the file descriptor associated with a display so it can be integrated into the
	/// client's main loop.
	pub fn get_fd(&self) -> RawFd {
		(unsafe { LIB_WAYLAND.wl_display_get_fd })(self)
	}

	/// Send all buffered data on the client side to the server. Clients should always call this
	/// function before blocking on input from the display fd. On success, the number of bytes sent
	/// to the server is returned. On failure, this function returns `-1` and errno is set
	/// appropriately.
	///
	/// `WlDisplay::flush()` never blocks. It will write as much data as possible, but if all data
	/// could not be written, errno will be set to EAGAIN and -1 returned. In that case, use poll
	/// on the display file descriptor to wait for it to become writable again.
	pub fn flush(&self) -> Result<usize> {
		match (unsafe { LIB_WAYLAND.wl_display_flush })(self) {
			-1 => Err(()),
			n  => Ok(n as _)
		}
	}

	/// This function blocks until the server has processed all currently issued requests by
	/// sending a request to the display server and waiting for a reply before returning.
	///
	/// This function uses `dispatch_queue()` internally. It is not allowed to call this
	/// function while the thread is being prepared for reading events, and doing so will cause a
	/// dead lock.
	///
	/// Note: This function may dispatch other events being received on the default queue.
	pub fn roundtrip(&self) -> Result<usize> {
		match (unsafe { LIB_WAYLAND.wl_display_roundtrip })(self) {
			-1 => Err(()),
			n  => Ok(n as _)
		}
	}

	/// Dispatch events on the default event queue.
	///
	/// If the default event queue is empty, this function blocks until there are events to be read
	/// from the display fd. Events are read and queued on the appropriate event queues. Finally,
	/// events on the default event queue are dispatched. On failure `Err(())` is returned and
	/// errno set appropriately.
	///
	/// In a multi threaded environment, do not manually wait using `poll()` (or equivalent) before
	/// calling this function, as doing so might cause a dead lock. If external reliance on `poll()`
	/// (or equivalent) is required, see `wl_display_prepare_read_queue()` of how to do so.
	///
	/// This function is thread safe as long as it dispatches the right queue on the right thread.
	/// It is also compatible with the multi thread event reading preparation API (see
	/// `wl_display_prepare_read_queue()`), and uses the equivalent functionality internally. It
	/// is not allowed to call this function while the thread is being prepared for reading events,
	/// and doing so will cause a dead lock.
	pub fn dispatch(&self) -> Result<usize> {
		match (unsafe { LIB_WAYLAND.wl_display_dispatch })(self) {
			-1 => Err(()),
			n  => Ok(n as _)
		}
	}

	/// This function dispatches events on the main event queue. It does not attempt to read the
	/// display fd and simply returns zero if the main queue is empty, i.e., it doesn't block.
	pub fn dispatch_pending(&self) -> Result<usize> {
		match (unsafe { LIB_WAYLAND.wl_display_dispatch_pending })(self) {
			-1 => Err(()),
			n  => Ok(n as _)
		}
	}

	/// Close the connection to display and free all resources associated with it.
	pub fn disconnect(&self) {
		(unsafe { LIB_WAYLAND.wl_display_disconnect })(self)
	}
}

#[repr(C)]
pub struct WlMessage {
	pub name:      *const u8,
	pub signature: *const u8,
	pub types:     *const *const WlInterface
}

unsafe impl Sync for WlMessage {}

#[repr(C)]
pub struct WlInterface {
	pub name:         *const u8,
	pub version:      u32,
	pub method_count: u32,
	pub methods:      *const WlMessage,
	pub event_count:  u32,
	pub events:       *const WlMessage
}

unsafe impl Sync for WlInterface {}

#[repr(C)]
pub union WlArgument {
	pub i: i32,
	pub u: u32,
	pub f: WlFixed,
	pub s: *const u8,
	pub o: *mut u8,
	pub n: u32,
	pub a: *mut WlArray,
	pub h: RawFd
}

#[repr(C)]
pub struct WlList {
	prev: *mut Self,
	next: *mut Self
}

#[repr(C)]
pub struct WlArray {
	pub size:  usize,
	pub alloc: usize,
	pub data:  *mut u8
}

impl WlArray {
	pub fn new() -> Self {
		let mut  array = Self {
			size:  0,
			alloc: 0,
			data:  std::ptr::null_mut()
		};
		unsafe { (LIB_WAYLAND.wl_array_init)(&mut array); }
		array
	}

	pub fn add(&mut self, size: usize) -> *mut u8 {
		unsafe { (LIB_WAYLAND.wl_array_add)(self, size) }
	}

	pub fn copy(&mut self, other: &mut Self) -> bool {
		unsafe { (LIB_WAYLAND.wl_array_copy)(self, other) == 0 }
	}
}

impl Default for WlArray {
	fn default() -> Self {
		Self::new()
	}
}

impl std::fmt::Debug for WlArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list()
			.finish()
	}
}

impl std::ops::Drop for WlArray {
	fn drop(&mut self) {
		unsafe { (LIB_WAYLAND.wl_array_release)(self); }
	}
}

#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WlFixed(i32);

impl From<f64> for WlFixed {
	fn from(v: f64) -> Self {
		union U { d: f64, i: i64 }
		Self(unsafe { U { d: v + (3i64 << (51 - 8)) as f64 }.i as _ })
	}
}

impl From<i32> for WlFixed {
	fn from(v: i32) -> Self {
		Self(v * 256)
	}
}

impl Into<f64> for WlFixed {
	fn into(self) -> f64 {
		union U { d: f64, i: i64 }
		unsafe { U { i: ((1023i64 + 44i64) << 52) + (1i64 << 51) + self.0 as i64 }.d - (3i64 << 43) as f64 }
	}
}

impl Into<i32> for WlFixed {
	fn into(self) -> i32 {
		self.0 / 256
	}
}

static mut LIB_WAYLAND: LibWayland = unsafe { LibWayland::uninit() };

struct LibWayland {
	lib:                                          Option<Library>,
	wl_display_connect_to_fd:                     extern fn(RawFd) -> *mut WlDisplay,
	wl_display_connect:                           extern fn(*const u8) -> *mut WlDisplay,
	wl_display_disconnect:                        extern fn(*const WlDisplay),
	wl_display_get_fd:                            extern fn(*const WlDisplay) -> RawFd,
	wl_display_roundtrip:                         extern fn(*const WlDisplay) -> i32,
	//wl_display_read_events:                       extern fn(*mut WlDisplay) -> u32,
	//wl_display_prepare_read:                      extern fn(*mut WlDisplay) -> u32,
	//wl_display_cancel_read:                       extern fn(*mut WlDisplay),
	wl_display_dispatch:                          extern fn(*const WlDisplay) -> i32,
	wl_display_dispatch_pending:                  extern fn(*const WlDisplay) -> i32,
	//wl_display_get_error:                         extern fn(*mut WlDisplay) -> u32,
	//wl_display_get_protocol_error:                extern fn(*mut WlDisplay, *mut *mut WlInterface, *mut u32) -> u32,
	wl_display_flush:                             extern fn(*const WlDisplay) -> i32,
	//wl_event_queue_destroy:                       extern fn(*mut WlEventQueue),
	//wl_display_create_queue:                      extern fn(*mut WlDisplay) -> *mut WlEventQueue,
	//wl_display_roundtrip_queue:                   extern fn(*mut WlDisplay, *mut WlEventQueue) -> u32,
	//wl_display_prepare_read_queue:                extern fn(*mut WlDisplay, *mut WlEventQueue) -> u32,
	//wl_display_dispatch_queue:                    extern fn(*mut WlDisplay, *mut WlEventQueue) -> u32,
	//wl_display_dispatch_queue_pending:            extern fn(*mut WlDisplay, *mut WlEventQueue) -> u32,
	//wl_proxy_create:                              extern fn(*mut WlProxy, *const WlInterface) -> *mut WlProxy,
	wl_proxy_destroy:                             extern fn(*mut WlProxy),
	wl_proxy_add_listener:                        extern fn(*mut WlProxy, *mut fn(), *mut u8) -> u32,
	wl_proxy_get_listener:                        extern fn(*const WlProxy) -> *const u8,
	wl_proxy_add_dispatcher:                      extern fn(*mut WlProxy, WlDispatcherFunc, *const u8, *mut u8) -> u32,
	wl_proxy_marshal:                             extern fn(*mut WlProxy, u32, ...),
	wl_proxy_marshal_constructor:                 extern fn(*mut WlProxy, u32, *const WlInterface, ...) -> *mut WlProxy,
	wl_proxy_marshal_constructor_versioned:       extern fn(*mut WlProxy, u32, *const WlInterface, u32, ...) -> *mut WlProxy,
	//wl_proxy_marshal_array:                       extern fn(*mut WlProxy, u32, *mut WlArgument),
	//wl_proxy_marshal_array_constructor:           extern fn(*mut WlProxy, u32, *mut WlArgument, *const WlInterface) -> *mut WlProxy,
	//wl_proxy_marshal_array_constructor_versioned: extern fn(*mut WlProxy, u32, *mut WlArgument, *const WlInterface, u32) -> *mut WlProxy,
	wl_proxy_set_user_data:                       extern fn(*mut WlProxy, *mut u8),
	wl_proxy_get_user_data:                       extern fn(*const WlProxy) -> *mut u8,
	wl_proxy_get_id:                              extern fn(*const WlProxy) -> u32,
	wl_proxy_get_class:                           extern fn(*const WlProxy) -> *const u8,
	//wl_proxy_set_queue:                           extern fn(*mut WlProxy, *mut WlEventQueue),
	wl_proxy_get_version:                         extern fn(*const WlProxy) -> u32,
	//wl_proxy_create_wrapper:                      extern fn(*mut WlProxy) -> *mut WlProxy,
	//wl_proxy_wrapper_destroy:                     extern fn(*mut WlProxy),
	wl_log_set_handler_client:                    extern fn(WlLogFunc),
	wl_array_init:                                extern fn(*mut WlArray),
	wl_array_release:                             extern fn(*mut WlArray),
	wl_array_add:                                 extern fn(*mut WlArray, usize) -> *mut u8,
	wl_array_copy:                                extern fn(*mut WlArray, *mut WlArray) -> i32,
}

impl LibWayland {
	const unsafe fn uninit() -> Self {
		extern fn abort() { panic!("libwayland has not been loaded") }

		Self {
			lib:                                    None,
			wl_display_connect_to_fd:               {
				extern fn load(fd: RawFd) -> *mut WlDisplay {
					unsafe { LIB_WAYLAND.load(); (LIB_WAYLAND.wl_display_connect_to_fd)(fd) }
				}
				load
			},
			wl_display_connect:                     {
				extern fn load(fd: *const u8) -> *mut WlDisplay {
					unsafe { LIB_WAYLAND.load(); (LIB_WAYLAND.wl_display_connect)(fd) }
				}
				load
			},
			wl_display_disconnect:                  std::mem::transmute(abort as extern fn()),
			wl_display_get_fd:                      std::mem::transmute(abort as extern fn()),
			wl_display_roundtrip:                   std::mem::transmute(abort as extern fn()),
			wl_display_dispatch:                    std::mem::transmute(abort as extern fn()),
			wl_display_dispatch_pending:            std::mem::transmute(abort as extern fn()),
			wl_display_flush:                       std::mem::transmute(abort as extern fn()),
			wl_proxy_destroy:                       std::mem::transmute(abort as extern fn()),
			wl_proxy_add_listener:                  std::mem::transmute(abort as extern fn()),
			wl_proxy_get_listener:                  std::mem::transmute(abort as extern fn()),
			wl_proxy_add_dispatcher:                std::mem::transmute(abort as extern fn()),
			wl_proxy_set_user_data:                 std::mem::transmute(abort as extern fn()),
			wl_proxy_get_user_data:                 std::mem::transmute(abort as extern fn()),
			wl_proxy_get_id:                        std::mem::transmute(abort as extern fn()),
			wl_proxy_get_class:                     std::mem::transmute(abort as extern fn()),
			wl_proxy_get_version:                   std::mem::transmute(abort as extern fn()),
			wl_proxy_marshal_constructor:           std::mem::transmute(abort as extern fn()),
			wl_proxy_marshal_constructor_versioned: std::mem::transmute(abort as extern fn()),
			wl_proxy_marshal:                       std::mem::transmute(abort as extern fn()),
			wl_log_set_handler_client:              {
				extern fn load(f: WlLogFunc) {
					unsafe { LIB_WAYLAND.load(); (LIB_WAYLAND.wl_log_set_handler_client)(f) }
				}
				load
			},
			wl_array_init:                          std::mem::transmute(abort as extern fn()),
			wl_array_release:                       std::mem::transmute(abort as extern fn()),
			wl_array_add:                           std::mem::transmute(abort as extern fn()),
			wl_array_copy:                          std::mem::transmute(abort as extern fn())
		}
	}

	unsafe fn load(&mut self) {
		if self.lib.is_some() { return; }

		let lib                             = Library::new("/home/tobias/Downloads/bin/lib64/libwayland-client.so.0").expect("failed to load libwayland");
		self.wl_display_connect_to_fd               = *lib.get(b"wl_display_connect_to_fd\0").expect("failed to load `wl_display_connect_to_fd`");
		self.wl_display_connect                     = *lib.get(b"wl_display_connect\0").expect("failed to load `wl_display_connect`");
		self.wl_display_disconnect                  = *lib.get(b"wl_display_disconnect\0").expect("failed to load `wl_display_disconnect`");
		self.wl_display_get_fd                      = *lib.get(b"wl_display_get_fd\0").expect("failed to load `wl_display_get_fd`");
		self.wl_display_roundtrip                   = *lib.get(b"wl_display_roundtrip\0").expect("failed to load `wl_display_roundtrip`");
		self.wl_display_dispatch                    = *lib.get(b"wl_display_dispatch\0").expect("failed to load `wl_display_dispatch`");
		self.wl_display_dispatch_pending            = *lib.get(b"wl_display_dispatch_pending\0").expect("failed to load `wl_display_dispatch_pending`");
		self.wl_display_flush                       = *lib.get(b"wl_display_flush\0").expect("failed to load `wl_display_flush`");
		self.wl_proxy_destroy                       = *lib.get(b"wl_proxy_destroy\0").expect("failed to load `wl_proxy_destroy`");
		self.wl_proxy_add_listener                  = *lib.get(b"wl_proxy_add_listener\0").expect("failed to load `wl_proxy_add_listener`");
		self.wl_proxy_get_listener                  = *lib.get(b"wl_proxy_get_listener\0").expect("failed to load `wl_proxy_get_listener`");
		self.wl_proxy_add_dispatcher                = *lib.get(b"wl_proxy_add_dispatcher\0").expect("failed to load `wl_proxy_add_dispatcher`");
		self.wl_proxy_set_user_data                 = *lib.get(b"wl_proxy_set_user_data\0").expect("failed to load `wl_proxy_set_user_data`");
		self.wl_proxy_get_user_data                 = *lib.get(b"wl_proxy_get_user_data\0").expect("failed to load `wl_proxy_get_user_data`");
		self.wl_proxy_get_id                        = *lib.get(b"wl_proxy_get_id\0").expect("failed to load `wl_proxy_get_id`");
		self.wl_proxy_get_class                     = *lib.get(b"wl_proxy_get_class\0").expect("failed to load `wl_proxy_get_class`");
		self.wl_proxy_get_version                   = *lib.get(b"wl_proxy_get_version\0").expect("failed to load `wl_proxy_get_version`");
		self.wl_proxy_marshal_constructor           = *lib.get(b"wl_proxy_marshal_constructor\0").expect("failed to load `wl_proxy_marshal_constructor`");
		self.wl_proxy_marshal_constructor_versioned = *lib.get(b"wl_proxy_marshal_constructor_versioned\0").expect("failed to load `wl_proxy_marshal_constructor_versioned`");
		self.wl_proxy_marshal                       = *lib.get(b"wl_proxy_marshal\0").expect("failed to load `wl_proxy_marshal`");
		self.wl_log_set_handler_client              = *lib.get(b"wl_log_set_handler_client\0").expect("failed to load `wl_log_set_handler_client`");
		self.wl_array_init                          = *lib.get(b"wl_array_init\0").expect("failed to load `wl_array_init`");
		self.wl_array_release                       = *lib.get(b"wl_array_release\0").expect("failed to load `wl_array_release`");
		self.wl_array_add                           = *lib.get(b"wl_array_add\0").expect("failed to load `wl_array_add`");
		self.wl_array_copy                          = *lib.get(b"wl_array_copy\0").expect("failed to load `wl_array_copy`");
		self.lib                                    = Some(lib);
		log::trace!("loaded libwayland");
	}
}

pub struct WlAlloc;

unsafe impl std::alloc::Allocator for WlAlloc {
	fn allocate(&self, _: std::alloc::Layout) -> std::result::Result<NonNull<[u8]>, std::alloc::AllocError> {
		log::error!("attempted alloc on noop allocator");
		Err(std::alloc::AllocError)
	}

	unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
}

unsafe fn box_from_raw<T: ?Sized>(ptr: *mut T) -> Result<Box<T, WlAlloc>> {
	if ptr.is_null() {
		Err(())
	} else {
		Ok(Box::from_raw_in(ptr, WlAlloc))
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TraitObject {
	data:   *mut (),
	vtable: *mut (),
}

/*
//! Custom Rust-only implementation of wayland
mod custom {
	use nix::sys::socket::*;
	use std::sync::{Arc, Mutex};
	use std::collections::VecDeque;

	struct WlDisplay {
		connection:     WlConnection,
		client_objects: Vec<WlObject>,
		server_objects: Vec<WlObject>,
	}

	struct WlProxy {
		connection: Arc<Mutex<Connection>>,
		id: u32
	}

	struct WlConnection {
		data_in:  VecDeque<u8>,
		data_out: VecDeque<u8>,
		fds_in:   VecDeque<RawFd>,
		fds_out:  VecDeque<RawFd>,
		fd:       RawFd,
	}

	enum Error {
		Nix(nix::Error),
		Var(std::env::VarError)
	}

	impl WlConnection {
		fn connect() -> Result<Self, Error> {
			let mut dir = std::env::var("XDG_RUNTIME_DIR")?;
			dir.push_str(&std::env::var("WAYLAND_DISPLAY")
				.unwrap_or("wayland-0".to_string()));
			let fd = socket(AddressFamily::Unix, SockType::Stream, 0 as _, SockProtocol::Udp)?;
			connect(fd, &SockAddr::new_unix(&dir)?)?;
			Ok(Self::from_fd(fd))
		}

		fn from_fd(fd: RawFd) -> Self {
			Self {
				data_in:  VecDeque::with_capacity(4096),
				data_out: VecDeque::with_capacity(4096),
				fds_in:   VecDeque::with_capacity(16),
				fds_out:  VecDeque::with_capacity(16),
				fd
			}
		}

		fn recv(&mut self, buf: &mut [u8]) -> Result<usize, nix::Error> {
			recv(self.fd, buf, 0 as _)
		}

		fn send(&mut self, buf: &[u8]) -> Result<usize, nix::Error> {
			send(self.fd, buf, 0 as _)
		}
	}
}*/

#[cfg(test)]
mod tests {
	use crate::WlDisplay;

	#[test]
	fn connect() {
		WlDisplay::connect(None)
			.unwrap().disconnect();
	}
}
