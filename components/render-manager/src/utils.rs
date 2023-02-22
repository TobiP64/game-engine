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

use std::io;

pub fn align<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Rem<Output = T> + std::cmp::Eq + Copy>(
	offset: T, align: T
) -> T {
	#[allow(clippy::eq_op)]
	let zero = offset - offset;
	offset + match offset % align {
		v if v == zero => zero,
		v => align - v
	}
}

pub fn generic_seek(seek: io::SeekFrom, pos: &mut usize, size: usize) -> io::Result<u64> {
	use io::SeekFrom::*;
	
	// calc new pos
	let pos_ = match seek {
		Start(offset)   => offset as i64,
		Current(offset) => *pos as i64 + offset,
		End(offset)     => size as i64 + offset,
	};
	
	// check new pos
	if pos_ >= 0 && pos_ <= size as i64 {
		*pos = pos_ as _;
		Ok(*pos as _)
	} else {
		Err(io::Error::new(io::ErrorKind::Other, "seek failed"))
	}
}

pub fn vk_features_to_bools(features: &vk::VkPhysicalDeviceFeatures) -> &[vk::VkBool32; 55] {
	if cfg!(feature = "trace-unsafe") { log::trace!("TRACE-UNSAFE") }
	unsafe { &*(features as *const vk::VkPhysicalDeviceFeatures as *const _) }
}

pub fn vk_features_to_bools_mut(features: &mut vk::VkPhysicalDeviceFeatures) -> &mut [vk::VkBool32; 55] {
	if cfg!(feature = "trace-unsafe") { log::trace!("TRACE-UNSAFE") }
	unsafe { &mut*(features as *mut vk::VkPhysicalDeviceFeatures as *mut _) }
}

/// # Safety
///
/// The passed pointer must be valid, otherwise calling this function is UB.
pub unsafe fn str_convert<'a>(ptr: *const u8) -> &'a str {
	try_str_convert(ptr).expect("pointer was null")
}

/// # Safety
///
/// The passed pointer must be valid, otherwise calling this function is UB.
pub unsafe fn try_str_convert<'a>(ptr: *const u8) -> Option<&'a str> {
	(!ptr.is_null())
		.then(|| std::ffi::CStr::from_ptr(ptr as _))
		.and_then(|s| s.to_str().ok())
}

pub fn string_convert(ptr: *const u8) -> String {
	(!ptr.is_null())
		.then(|| unsafe { std::ffi::CStr::from_ptr(ptr as _) })
		.and_then(|s| std::str::from_utf8(s.to_bytes()).ok())
		.expect("invalid pointer")
		.to_string()
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn ffi_str_eq(s1: *const u8, s2: *const u8) -> bool {
	if s1.is_null() && s2.is_null() {
		true
	} else if s1.is_null() || s2.is_null() {
		false
	} else {
		let mut i = 0;
		unsafe {
			while *s1.offset(i) == *s2.offset(i) {
				if *s1.offset(i) == 0 {
					return true;
				}
				i += 1;
			}
		}
		
		false
	}
}

pub trait ContainsFfiStr<T> {
	fn contains_ffi_str(&self, s: *const T) -> bool;
}

impl<T> ContainsFfiStr<T> for [*const T] {
	fn contains_ffi_str(&self, s: *const T) -> bool {
		let str = unsafe { std::ffi::CStr::from_ptr(s as *const _) };
		for s in self {
			if unsafe { std::ffi::CStr::from_ptr(*s as *const _) } == str {
				return true;
			}
		}
		false
	}
}