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

#![feature(allocator_api)]
#![warn(clippy::all)]
#![allow(non_camel_case_types, clippy::missing_safety_doc, clippy::zero_prefixed_literal, dead_code)]

use std::{ptr::NonNull, os::unix::io::{FromRawFd, RawFd}};

/// Opaque top level library context object.
///
/// The context contains various general library data and state, like
/// logging level and include paths.
///
/// Objects are created in a specific context, and multiple contexts may
/// coexist simultaneously. Objects from different contexts are completely
/// separated and do not share any memory or state.
#[derive(Debug)]
pub enum XkbContext {}

impl XkbContext {
	/// Create a new context.
	pub fn new(flags: XkbContextFlags) -> Box<Self, XkbAlloc> {
		unsafe { box_from_raw((LIB_XKB_COMMON.xkb_context_new)(flags)).unwrap() }
	}
}

/*impl<'a> Clone for &'a mut XkbContext {
	fn clone(&self) -> Self {
		#[allow(clippy::cast_ref_to_mut)]
		unsafe { (LIB_XKB_COMMON.xkb_context_ref)(*(self as *const _ as *mut _)).as_mut().unwrap() }
	}
}*/

impl Drop for XkbContext {
	fn drop(&mut self) {
		unsafe { (LIB_XKB_COMMON.xkb_context_unref)(self) }
	}
}

/// Opaque compiled keymap object.
///
/// The keymap object holds all of the static keyboard information obtained
/// from compiling XKB files.
///
/// A keymap is immutable after it is created (besides reference counts, etc.);
/// if you need to change it, you must create a new one.
#[derive(Debug)]
pub enum XkbKeymap {}

impl XkbKeymap {
	pub fn new(
		context: &mut XkbContext,
		fd:      RawFd,
		size:    usize,
		format:  XkbKeymapFormat,
		flags:   XkbKeymapCompileFlags
	) -> Option<Box<Self, XkbAlloc>> {
		unsafe {
			let map = memmap::MmapOptions::new()
                .len(size as _)
                .map(&std::fs::File::from_raw_fd(fd))
                .unwrap();
			box_from_raw((LIB_XKB_COMMON.xkb_keymap_new_from_buffer)(
				context,
				map.as_ptr() as _,
				size - 1,
				format,
				flags
			))
		}
	}
}

/*impl<'a> Clone for &'a mut XkbKeymap {
	fn clone(&self) -> Self {
		#[allow(clippy::cast_ref_to_mut)]
		unsafe { (LIB_XKB_COMMON.xkb_keymap_ref)(*(self as *const _ as *mut _)).as_mut().unwrap() }
	}
}*/

impl Drop for XkbKeymap {
	fn drop(&mut self) {
		unsafe { (LIB_XKB_COMMON.xkb_keymap_unref)(self) }
	}
}

/// Opaque keyboard state object.
///
/// State objects contain the active state of a keyboard (or keyboards), such
/// as the currently effective layout and the active modifiers. It acts as a
/// simple state machine, wherein key presses and releases are the input, and
/// key symbols (keysyms) are the output.
#[derive(Debug)]
pub enum XkbState {}

impl XkbState {
	/// Create a new keyboard state object.
	pub fn new(keymap: &mut XkbKeymap) -> Box<Self, XkbAlloc> {
		unsafe { box_from_raw((LIB_XKB_COMMON.xkb_state_new)(keymap)).unwrap() }
	}

	/// Get the keymap which a keyboard state object is using.
	pub fn get_keymap(&mut self) -> Box<XkbKeymap, XkbAlloc> {
		unsafe { box_from_raw((LIB_XKB_COMMON.xkb_keymap_ref)(
			(LIB_XKB_COMMON.xkb_state_get_keymap)(self))).unwrap() }
	}

	pub fn update_mask(
		&mut self,
		depressed_mods:   XkbModMask,
		latched_mods:     XkbModMask,
		locked_mods:      XkbModMask,
		depressed_layout: XkbLayoutIndex,
		latched_layout:   XkbLayoutIndex,
		locked_layout:    XkbLayoutIndex
	) -> u32 {
		unsafe { (LIB_XKB_COMMON.xkb_state_update_mask)(
			self,
			depressed_mods,
			latched_mods,
			locked_mods,
			depressed_layout,
			latched_layout,
			locked_layout
		) }
	}

	pub fn get_key_utf8(&mut self, keycode: XkbKeycode) -> Option<String> {
		let mut buf = [0u8; 64];
		let len = unsafe { (LIB_XKB_COMMON.xkb_state_key_get_utf8)(self, keycode, buf.as_mut_ptr(), buf.len()) };
		if len <= 0 { return None; }
		Some(String::from_utf8(buf[..(len as usize).min(buf.len())].to_vec()).expect("invalid utf8"))
	}

	pub fn get_key_utf32(&mut self, keycode: XkbKeycode) -> u32 {
		unsafe { (LIB_XKB_COMMON.xkb_state_key_get_utf32)(self, keycode) }
	}

	pub fn get_key_one_sym(&mut self, keycode: XkbKeycode) -> XkbKeySym {
		unsafe { (LIB_XKB_COMMON.xkb_state_key_get_one_sym)(self, keycode) }
	}
}

/*impl<'a> Clone for &'a mut XkbState {
	fn clone(&self) -> Self {
		#[allow(clippy::cast_ref_to_mut)]
		unsafe { (LIB_XKB_COMMON.xkb_state_ref)(*(self as *const _ as *mut _)).as_mut().unwrap() }
	}
}*/

impl Drop for XkbState {
	fn drop(&mut self) {
		unsafe { (LIB_XKB_COMMON.xkb_state_unref)(self) }
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct XkbKeySym(pub u32);

impl XkbKeySym {
	pub fn get_name(self) -> String {
		let mut buf = [0u8; 64];
		let len = unsafe { (LIB_XKB_COMMON.xkb_keysym_get_name)(self, buf.as_mut_ptr(), buf.len()) };
		if len == -1 { panic!("invalid keysym"); }
		String::from_utf8(buf[..len as usize].to_vec()).expect("invalid utf8")
	}

	pub fn from_name(name: &str, flags: XkbKeySymFlags) -> Self {
		if name.ends_with('\0') {
			unsafe { (LIB_XKB_COMMON.xkb_keysym_from_name)(name.as_ptr(), flags) }
		} else {
			let mut str = name.to_string();
			str.push('\0');
			unsafe { (LIB_XKB_COMMON.xkb_keysym_from_name)(str.as_ptr(), flags) }
		}
	}

	pub fn into_utf8(self) -> Option<String> {
		let mut buf = [0u8; 64];
		let len = unsafe { (LIB_XKB_COMMON.xkb_keysym_to_utf8)(self, buf.as_mut_ptr(), buf.len()) };
		if len <= 0 { return None; }
		Some(String::from_utf8(buf[..(len as usize).min(buf.len())].to_vec()).expect("invalid utf8"))
	}

	pub fn into_utf32(self) -> u32 {
		unsafe { (LIB_XKB_COMMON.xkb_keysym_to_utf32)(self) }
	}

	pub fn to_upper(self) -> Self {
		unsafe { (LIB_XKB_COMMON.xkb_keysym_to_upper)(self) }
	}

	pub fn to_lower(self) -> Self {
		unsafe { (LIB_XKB_COMMON.xkb_keysym_to_lower)(self) }
	}
}

pub type XkbKeySymFlags = u32;
pub const XKB_KEYSYM_NO_FLAGS:         XkbKeySymFlags = 0x0;
pub const XKB_KEYSYM_CASE_INSENSITIVE: XkbKeySymFlags = 0x1;

pub type XkbContextFlags = u32;
pub const XKB_CONTEXT_NO_FLAGS:             XkbContextFlags = 0x0;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES:  XkbContextFlags = 0x1;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: XkbContextFlags = 0x2;

pub type XkbKeymapCompileFlags = u32;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: XkbKeymapCompileFlags = 0x0;

pub type XkbKeycode     = u32;
pub type XkbModMask     = u32;
pub type XkbLayoutIndex = u32;
pub type XkbLevelIndex  = u32;
pub type XkbModIndex    = u32;
pub type XkbLedIndex    = u32;

#[repr(u32)]
pub enum XkbKeymapFormat {
	TextV1      = 1,
	UseOriginal = 0xFFFF_FFFF
}

#[repr(u32)]
pub enum XkbKeyDirection {
	Up,
	Down
}

static mut LIB_XKB_COMMON: LibXkbCommon = unsafe { LibXkbCommon::uninit() };

struct LibXkbCommon {
	lib:                        Option<libloading::Library>,
	xkb_context_new:            extern "C" fn(XkbContextFlags) -> *mut XkbContext,
	xkb_context_ref:            extern "C" fn(*mut XkbContext) -> *mut XkbContext,
	xkb_context_unref:          extern "C" fn(*mut XkbContext),
	xkb_keymap_new_from_buffer: extern "C" fn(context: *mut XkbContext, buffer: *const u8, size: usize, format: XkbKeymapFormat, flags: XkbKeymapCompileFlags) -> *mut XkbKeymap,
	xkb_keymap_ref:             extern "C" fn(*mut XkbKeymap) -> *mut XkbKeymap,
	xkb_keymap_unref:           extern "C" fn(*mut XkbKeymap),
	xkb_state_new:              extern "C" fn(*mut XkbKeymap) -> *mut XkbState,
	xkb_state_ref:              extern "C" fn(*mut XkbState) -> *mut XkbState,
	xkb_state_unref:            extern "C" fn(*mut XkbState),
	xkb_state_get_keymap:       extern "C" fn(*mut XkbState) -> *mut XkbKeymap,
	xkb_state_update_mask:      extern "C" fn(*mut XkbState, XkbModMask, XkbModMask, XkbModMask, XkbLayoutIndex, XkbLayoutIndex, XkbLayoutIndex) -> u32,
	xkb_state_key_get_utf8:     extern "C" fn(*mut XkbState, XkbKeycode, *mut u8, usize) -> i32,
	xkb_state_key_get_utf32:    extern "C" fn(*mut XkbState, XkbKeycode) -> u32,
	xkb_state_key_get_one_sym:  extern "C" fn(*mut XkbState, XkbKeycode) -> XkbKeySym,
	//xkb_state_get_layout:       extern "C" fn(*mut XkbState, XkbKeycode) -> XkbLayoutIndex,
	//xkb_state_get_level:        extern "C" fn(*mut XkbState, XkbKeycode, XkbLayoutIndex) -> XkbLevelIndex,
	xkb_keysym_get_name:        extern "C" fn(XkbKeySym, *mut u8, usize) -> i32,
	xkb_keysym_from_name:       extern "C" fn(*const u8, XkbKeySymFlags) -> XkbKeySym,
	xkb_keysym_to_utf8:         extern "C" fn(XkbKeySym, *mut u8, usize) -> i32,
	xkb_keysym_to_utf32:        extern "C" fn(XkbKeySym) -> u32,
	xkb_keysym_to_upper:        extern "C" fn(XkbKeySym) -> XkbKeySym,
	xkb_keysym_to_lower:        extern "C" fn(XkbKeySym) -> XkbKeySym
}

impl LibXkbCommon {
	const unsafe fn uninit() -> Self {
		extern fn abort() { panic!("libxkbcommon has not been loaded") }

		Self {
			lib:  None,
			xkb_context_new:            {
				extern "C" fn load(flags: XkbContextFlags) -> *mut XkbContext {
					unsafe { LIB_XKB_COMMON.load(); (LIB_XKB_COMMON.xkb_context_new)(flags) }
				}
				load
			},
			xkb_context_ref:            std::mem::transmute(abort as extern fn()),
			xkb_context_unref:          std::mem::transmute(abort as extern fn()),
			xkb_keymap_new_from_buffer: std::mem::transmute(abort as extern fn()),
			xkb_keymap_ref:             std::mem::transmute(abort as extern fn()),
			xkb_keymap_unref:           std::mem::transmute(abort as extern fn()),
			xkb_state_new:              std::mem::transmute(abort as extern fn()),
			xkb_state_ref:              std::mem::transmute(abort as extern fn()),
			xkb_state_unref:            std::mem::transmute(abort as extern fn()),
			xkb_state_get_keymap:       std::mem::transmute(abort as extern fn()),
			xkb_state_update_mask:      std::mem::transmute(abort as extern fn()),
			xkb_state_key_get_utf8:     std::mem::transmute(abort as extern fn()),
			xkb_state_key_get_utf32:    std::mem::transmute(abort as extern fn()),
			xkb_state_key_get_one_sym:  std::mem::transmute(abort as extern fn()),
			xkb_keysym_get_name:        std::mem::transmute(abort as extern fn()),
			xkb_keysym_from_name:       std::mem::transmute(abort as extern fn()),
			xkb_keysym_to_utf8:         std::mem::transmute(abort as extern fn()),
			xkb_keysym_to_utf32:        std::mem::transmute(abort as extern fn()),
			xkb_keysym_to_upper:        std::mem::transmute(abort as extern fn()),
			xkb_keysym_to_lower:        std::mem::transmute(abort as extern fn())
		}
	}

	unsafe fn load(&mut self) {
		let lib                         = libloading::Library::new("libxkbcommon.so.0").expect("failed to load libxkbcommon");
		self.xkb_context_new            = *lib.get(b"xkb_context_new\0").expect("failed to load `xkb_context_new`");
		self.xkb_context_ref            = *lib.get(b"xkb_context_ref\0").expect("failed to load `xkb_context_ref`");
		self.xkb_context_unref          = *lib.get(b"xkb_context_unref\0").expect("failed to load `xkb_context_unref`");
		self.xkb_keymap_new_from_buffer = *lib.get(b"xkb_keymap_new_from_buffer\0").expect("failed to load `xkb_keymap_new_from_buffer`");
		self.xkb_keymap_ref             = *lib.get(b"xkb_keymap_ref\0").expect("failed to load `xkb_keymap_ref`");
		self.xkb_keymap_unref           = *lib.get(b"xkb_keymap_unref\0").expect("failed to load `xkb_keymap_unref`");
		self.xkb_state_new              = *lib.get(b"xkb_state_new\0").expect("failed to load `xkb_state_new`");
		self.xkb_state_ref              = *lib.get(b"xkb_state_ref\0").expect("failed to load `xkb_state_ref`");
		self.xkb_state_unref            = *lib.get(b"xkb_state_unref\0").expect("failed to load `xkb_state_unref`");
		self.xkb_state_get_keymap       = *lib.get(b"xkb_state_get_keymap\0").expect("failed to load `xkb_state_get_keymap`");
		self.xkb_state_update_mask      = *lib.get(b"xkb_state_update_mask\0").expect("failed to load `xkb_state_update_mask`");
		self.xkb_state_key_get_utf8     = *lib.get(b"xkb_state_key_get_utf8\0").expect("failed to load `xkb_state_key_get_utf8`");
		self.xkb_state_key_get_utf32    = *lib.get(b"xkb_state_key_get_utf32\0").expect("failed to load `xkb_state_key_get_utf32`");
		self.xkb_state_key_get_one_sym  = *lib.get(b"xkb_state_key_get_one_sym\0").expect("failed to load `xkb_state_key_get_one_sym`");
		self.xkb_keysym_get_name        = *lib.get(b"xkb_keysym_get_name\0").expect("failed to load `xkb_keysym_get_name`");
		self.xkb_keysym_from_name       = *lib.get(b"xkb_keysym_from_name\0").expect("failed to load `xkb_keysym_from_name`");
		self.xkb_keysym_to_utf8         = *lib.get(b"xkb_keysym_to_utf8\0").expect("failed to load `xkb_keysym_to_utf8`");
		self.xkb_keysym_to_utf32        = *lib.get(b"xkb_keysym_to_utf32\0").expect("failed to load `xkb_keysym_to_utf32`");
		self.xkb_keysym_to_upper        = *lib.get(b"xkb_keysym_to_upper\0").expect("failed to load `xkb_keysym_to_upper`");
		self.xkb_keysym_to_lower        = *lib.get(b"xkb_keysym_to_lower\0").expect("failed to load `xkb_keysym_to_lower`");
		self.lib                        = Some(lib);
		log::trace!("loaded libxkbcommon");
	}
}

pub struct XkbAlloc;

unsafe impl std::alloc::Allocator for XkbAlloc {
	fn allocate(&self, _: std::alloc::Layout) -> std::result::Result<NonNull<[u8]>, std::alloc::AllocError> {
		log::error!("attempted alloc on noop allocator");
		Err(std::alloc::AllocError)
	}

	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: std::alloc::Layout) {
		log::trace!("noop dealloc: {:?} with layout {:?}", ptr.as_ptr(), layout);
	}
}

unsafe fn box_from_raw<T: ?Sized>(ptr: *mut T) -> Option<Box<T, XkbAlloc>> {
	if ptr.is_null() {
		None
	} else {
		Some(Box::from_raw_in(ptr, XkbAlloc))
	}
}