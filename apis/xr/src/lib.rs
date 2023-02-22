//! MACHINE GENERATED FILE, DO NOT EDIT
#![feature(try_trait_v2, stmt_expr_attributes)]
#![warn(clippy::all)]
#![allow(
	non_snake_case,
	non_upper_case_globals,
	non_camel_case_types,
	unused_attributes,
	unused_parens,
	invalid_value,
	clippy::unused_unit,
	clippy::too_many_arguments,
	clippy::enum_clike_unportable_variant,
	clippy::unnecessary_cast,
	clippy::missing_safety_doc,
	clippy::from_over_into,
	clippy::upper_case_acronyms
)]

use {
	core::{mem::{transmute, MaybeUninit}, ops, fmt},
	std::sync::Arc,
	self::external::*,
	vk::*,
};
pub use types::*;

/// Contains some useful types
mod types {
	#[derive(Copy, Clone, Debug)]
	#[repr(transparent)]
	pub struct XrAnyRef<'b>(&'b ());
	
	impl<'a> XrAnyRef<'a> {
		#[allow(clippy::transmute_ptr_to_ptr)]
		pub const fn new<T>(v: &'a T) -> Self {
			Self(unsafe { std::mem::transmute(v) })
		}
	}
	
	impl<'a, T> From<&'a T> for XrAnyRef<'a> {
		fn from(v: &'a T) -> Self {
			unsafe { Self((v as *const T as *const ()).as_ref().unwrap()) }
		}
	}
	
	impl<'a, T> Into<*const T> for XrAnyRef<'a> {
		fn into(self) -> *const T {
			self.0 as *const () as *const T
		}
	}
	
	#[derive(Debug)]
	#[repr(transparent)]
	pub struct XrAnyMut<'b>(&'b mut ());
	
	impl<'a> XrAnyMut<'a> {
		pub fn new<T>(v: &'a mut T) -> Self {
			Self(unsafe { &mut*(v as *mut T as *mut ()) })
		}
	}
	
	impl<'a, T> From<&'a mut T> for XrAnyMut<'a> {
		fn from(v: &'a mut T) -> Self {
			unsafe { Self((v as *mut T as *mut ()).as_mut().unwrap()) }
		}
	}
	
	impl<'a, T> Into<*mut T> for XrAnyMut<'a> {
		fn into(self) -> *mut T {
			self.0 as *mut () as *mut T
		}
	}
}

pub mod external {
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum wl_display {}
	
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum xcb_connection_t {}
	pub type xcb_window_t       = u32;
	pub type xcb_visualid_t     = u32;
	pub type xcb_glx_fbconfig_t = u32;
	pub type xcb_glx_drawable_t = u32;
	pub type xcb_glx_context_t  = u32;
	
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum Display {}
	pub type GLXFBConfig = u64;
	pub type GLXDrawable = u64;
	pub type GLXContext  = u64;
	
	pub type EGLDisplay = u64;
	pub type EGLConfig  = u64;
	pub type EGLContext = u64;
	pub type PFNEGLGETPROCADDRESSPROC = extern fn();
	
	pub type HDC   = *mut ();
	pub type HGLRC = *mut ();
	pub type LUID  = u64;
	
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum ID3D11Device {}
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum ID3D11Texture2D {}
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum ID3D12Device {}
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum ID3D12CommandQueue {}
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum ID3D12Resource {}
	#[repr(C)]
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum D3D_FEATURE_LEVEL { Variant }
	
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
	pub enum IUnknown {}

	/// This typedef only exists in vulkan.h, but not in the Vulkan registry, so it is defined here.
	pub type PFN_vkGetInstanceProcAddr = extern fn(vk::VkInstance, *const u8) -> vk::PFN_vkVoidFunction;
}

pub const XR_NULL_HANDLE          : u64        = 0;
pub const XR_CURRENT_API_VERSION  : XrVersion  = XR_MAKE_VERSION(1, 0, 5);
pub const XR_NULL_PATH            : XrPath     = 0;
pub const XR_NO_DURATION          : XrDuration = 0;
pub const XR_INFINITE_DURATION    : XrDuration = 0x7fffffffffffffff;
pub const XR_MIN_HAPTIC_DURATION  : XrDuration = -1;
pub const XR_FREQUENCY_UNSPECIFIED: i32        = 0;
pub const XR_MAX_EVENT_DATA_SIZE  : usize      = std::mem::size_of::<XrEventDataBuffer>();

#[cfg(unix)]
const LIB: &str = "libopenxr.so.1";
#[cfg(windows)]
const LIB: &str = "openxr-1.dll";

impl<T, E: From<XrResult>> std::ops::FromResidual<XrResult> for Result<T, E> {
	fn from_residual(r: XrResult) -> Self {
		Err(r.into())
	}
}

impl<T, E: Into<XrResult>> std::ops::FromResidual<Result<T, E>> for XrResult {
	fn from_residual(r: Result<T, E>) -> Self {
		match r {
			Ok(_) => XR_SUCCESS,
			Err(v) => v.into()
		}
	}
}

impl ops::FromResidual<XrResult> for XrResult {
	fn from_residual(residual: XrResult) -> Self {
		residual
	}
}

impl ops::Try for XrResult {
	type Output = Self;
	type Residual = Self;
	
	fn from_output(output: Self::Output) -> Self {
		output
	}
	
	fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
		if self as i32 >= 0 {
			ops::ControlFlow::Continue(self)
		} else {
			ops::ControlFlow::Break(self)
		}
	}
}

impl std::error::Error for XrResult {}

impl fmt::Display for XrResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[inline] pub const fn XR_MAKE_VERSION(major: u64, minor: u64, patch: u64) -> XrVersion { (major & 0xffff << 48) | (minor & 0xffff << 32) | (patch & 0xffffffff) }
#[inline] pub const fn XR_VERSION_MAJOR(version: XrVersion) -> u64 { version >> 48 & 0xffff }
#[inline] pub const fn XR_VERSION_MINOR(version: XrVersion) -> u64 { version >> 32 & 0xffff }
#[inline] pub const fn XR_VERSION_PATCH(version: XrVersion) -> u64 { version & 0xffffffff }
#[inline] pub const fn XR_SUCCEEDED(result: XrResult) -> bool { result as i32 >= 0 }
#[inline] pub const fn XR_UNQUALIFIED_SUCCESS(result: XrResult) -> bool { result as i32 == 0 }
#[inline] pub const fn XR_FAILED(result: XrResult) -> bool { (result as i32) < 0 }

pub type PFN_xrVoidFunction                            = Option<extern fn(
) -> ()>
;

pub type PFN_xrDebugUtilsMessengerCallbackEXT          = Option<extern fn(
	messageSeverity                              : XrDebugUtilsMessageSeverityFlagsEXT,
	messageTypes                                 : XrDebugUtilsMessageTypeFlagsEXT,
	callbackData                                 : &XrDebugUtilsMessengerCallbackDataEXT,
	pUserData                                    : &u8,
) -> XrBool32>
;
// Copyright (c) 2017-2021, The Khronos Group Inc.
// 
// SPDX-License-Identifier: Apache-2.0 OR MIT
// 
// ------------------------------------------------------------------------
// 
// This file, xr.xml, is the OpenXR API Registry. It is a critically important
// and normative part of the OpenXR Specification, including a canonical
// machine-readable definition of the API, parameter and member validation
// language incorporated into the Specification and reference pages, and other
// material which is registered by Khronos, such as tags used by extension and
// layer authors. The only authoritative version of xr.xml is the one
// maintained in the default branch of the Khronos OpenXR GitHub project.

pub type XrBool32                                      = u32;

pub type XrFlags64                                     = u64;

pub type XrTime                                        = i64;

pub type XrDuration                                    = i64;

pub type XrVersion                                     = u64;

pub type XrPath                                        = u64;

pub type XrSystemId                                    = u64;

pub type XrControllerModelKeyMSFT                      = u64;

pub type XrInstanceCreateFlags                         = XrFlags64;

pub type XrSessionCreateFlags                          = XrFlags64;

pub type XrSwapchainCreateFlags                        = XrFlags64;

pub type XrSwapchainUsageFlags                         = XrFlags64;

pub type XrViewStateFlags                              = XrFlags64;

pub type XrCompositionLayerFlags                       = XrFlags64;

pub type XrSpaceLocationFlags                          = XrFlags64;

pub type XrSpaceVelocityFlags                          = XrFlags64;

pub type XrInputSourceLocalizedNameFlags               = XrFlags64;

pub type XrVulkanInstanceCreateFlagsKHR                = XrFlags64;

pub type XrVulkanDeviceCreateFlagsKHR                  = XrFlags64;

pub type XrDebugUtilsMessageSeverityFlagsEXT           = XrFlags64;

pub type XrDebugUtilsMessageTypeFlagsEXT               = XrFlags64;

pub type XrOverlayMainSessionFlagsEXTX                 = XrFlags64;

pub type XrOverlaySessionCreateFlagsEXTX               = XrFlags64;

pub type XrAndroidSurfaceSwapchainFlagsFB              = XrFlags64;

pub type XrInstance                                    = u64;

pub type XrSession                                     = u64;

pub type XrActionSet                                   = u64;

pub type XrAction                                      = u64;

pub type XrSwapchain                                   = u64;

pub type XrSpace                                       = u64;

pub type XrDebugUtilsMessengerEXT                      = u64;

pub type XrSpatialAnchorMSFT                           = u64;

pub type XrHandTrackerEXT                              = u64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrVector2f {
	pub x                                            : f32,
	pub y                                            : f32,
}

unsafe impl Send for XrVector2f {}
unsafe impl Sync for XrVector2f {}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrVector3f {
	pub x                                            : f32,
	pub y                                            : f32,
	pub z                                            : f32,
}

unsafe impl Send for XrVector3f {}
unsafe impl Sync for XrVector3f {}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrVector4f {
	pub x                                            : f32,
	pub y                                            : f32,
	pub z                                            : f32,
	pub w                                            : f32,
}

unsafe impl Send for XrVector4f {}
unsafe impl Sync for XrVector4f {}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrColor4f {
	pub r                                            : f32,
	pub g                                            : f32,
	pub b                                            : f32,
	pub a                                            : f32,
}

unsafe impl Send for XrColor4f {}
unsafe impl Sync for XrColor4f {}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrQuaternionf {
	pub x                                            : f32,
	pub y                                            : f32,
	pub z                                            : f32,
	pub w                                            : f32,
}

unsafe impl Send for XrQuaternionf {}
unsafe impl Sync for XrQuaternionf {}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrPosef {
	pub orientation                                  : XrQuaternionf,
	pub position                                     : XrVector3f,
}

unsafe impl Send for XrPosef {}
unsafe impl Sync for XrPosef {}

impl fmt::Debug for XrPosef {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrPosef").finish()
	}
}

impl Default for XrPosef { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrOffset2Df {
	pub x                                            : f32,
	pub y                                            : f32,
}

unsafe impl Send for XrOffset2Df {}
unsafe impl Sync for XrOffset2Df {}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrExtent2Df {
	pub width                                        : f32,
	pub height                                       : f32,
}

unsafe impl Send for XrExtent2Df {}
unsafe impl Sync for XrExtent2Df {}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrRect2Df {
	pub offset                                       : XrOffset2Df,
	pub extent                                       : XrExtent2Df,
}

unsafe impl Send for XrRect2Df {}
unsafe impl Sync for XrRect2Df {}

impl fmt::Debug for XrRect2Df {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrRect2Df").finish()
	}
}

impl Default for XrRect2Df { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrOffset2Di {
	pub x                                            : i32,
	pub y                                            : i32,
}

unsafe impl Send for XrOffset2Di {}
unsafe impl Sync for XrOffset2Di {}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrExtent2Di {
	pub width                                        : i32,
	pub height                                       : i32,
}

unsafe impl Send for XrExtent2Di {}
unsafe impl Sync for XrExtent2Di {}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrRect2Di {
	pub offset                                       : XrOffset2Di,
	pub extent                                       : XrExtent2Di,
}

unsafe impl Send for XrRect2Di {}
unsafe impl Sync for XrRect2Di {}

impl fmt::Debug for XrRect2Di {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrRect2Di").finish()
	}
}

impl Default for XrRect2Di { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}
// XrBaseInStructure and XrBaseOutStructure use "struct" in their member definitions
// because they are recursive structures and this is easier than modifying the tooling
// to output forward declarations.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrBaseInStructure<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : &'a XrBaseInStructure<'a>,
}

unsafe impl Send for XrBaseInStructure<'_> {}
unsafe impl Sync for XrBaseInStructure<'_> {}

impl fmt::Debug for XrBaseInStructure<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrBaseInStructure").finish()
	}
}

impl Default for XrBaseInStructure<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrBaseOutStructure<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : &'a mut XrBaseOutStructure<'a>,
}

unsafe impl Send for XrBaseOutStructure<'_> {}
unsafe impl Sync for XrBaseOutStructure<'_> {}

impl fmt::Debug for XrBaseOutStructure<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrBaseOutStructure").finish()
	}
}

impl Default for XrBaseOutStructure<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrApiLayerProperties<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub layerName                                    : [u8;  XR_MAX_API_LAYER_NAME_SIZE as usize],
	pub specVersion                                  : XrVersion,
	pub layerVersion                                 : u32,
	pub description                                  : [u8;  XR_MAX_API_LAYER_DESCRIPTION_SIZE as usize],
}

unsafe impl Send for XrApiLayerProperties<'_> {}
unsafe impl Sync for XrApiLayerProperties<'_> {}

impl fmt::Debug for XrApiLayerProperties<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrApiLayerProperties").finish()
	}
}

impl Default for XrApiLayerProperties<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrExtensionProperties<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub extensionName                                : [u8;  XR_MAX_EXTENSION_NAME_SIZE as usize],
	pub extensionVersion                             : u32,
}

unsafe impl Send for XrExtensionProperties<'_> {}
unsafe impl Sync for XrExtensionProperties<'_> {}

impl fmt::Debug for XrExtensionProperties<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrExtensionProperties").finish()
	}
}

impl Default for XrExtensionProperties<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrApplicationInfo {
	pub applicationName                              : [u8;  XR_MAX_APPLICATION_NAME_SIZE as usize],
	pub applicationVersion                           : u32,
	pub engineName                                   : [u8;  XR_MAX_ENGINE_NAME_SIZE as usize],
	pub engineVersion                                : u32,
	pub apiVersion                                   : XrVersion,
}

unsafe impl Send for XrApplicationInfo {}
unsafe impl Sync for XrApplicationInfo {}

impl fmt::Debug for XrApplicationInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrApplicationInfo").finish()
	}
}

impl Default for XrApplicationInfo { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrInstanceCreateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub createFlags                                  : XrInstanceCreateFlags,
	pub applicationInfo                              : XrApplicationInfo,
	pub enabledApiLayerCount                         : u32,
	pub enabledApiLayerNames                         : *const *const u8,
	pub enabledExtensionCount                        : u32,
	pub enabledExtensionNames                        : *const *const u8,
}

unsafe impl Send for XrInstanceCreateInfo<'_> {}
unsafe impl Sync for XrInstanceCreateInfo<'_> {}

impl fmt::Debug for XrInstanceCreateInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrInstanceCreateInfo").finish()
	}
}

impl Default for XrInstanceCreateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrInstanceProperties<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub runtimeVersion                               : XrVersion,
	pub runtimeName                                  : [u8;  XR_MAX_RUNTIME_NAME_SIZE as usize],
}

unsafe impl Send for XrInstanceProperties<'_> {}
unsafe impl Sync for XrInstanceProperties<'_> {}

impl fmt::Debug for XrInstanceProperties<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrInstanceProperties").finish()
	}
}

impl Default for XrInstanceProperties<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSystemGetInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub formFactor                                   : XrFormFactor,
}

unsafe impl Send for XrSystemGetInfo<'_> {}
unsafe impl Sync for XrSystemGetInfo<'_> {}

impl Default for XrSystemGetInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrSystemProperties<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub systemId                                     : XrSystemId,
	pub vendorId                                     : u32,
	pub systemName                                   : [u8;  XR_MAX_SYSTEM_NAME_SIZE as usize],
	pub graphicsProperties                           : XrSystemGraphicsProperties,
	pub trackingProperties                           : XrSystemTrackingProperties,
}

unsafe impl Send for XrSystemProperties<'_> {}
unsafe impl Sync for XrSystemProperties<'_> {}

impl fmt::Debug for XrSystemProperties<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSystemProperties").finish()
	}
}

impl Default for XrSystemProperties<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrSystemGraphicsProperties {
	pub maxSwapchainImageHeight                      : u32,
	pub maxSwapchainImageWidth                       : u32,
	pub maxLayerCount                                : u32,
}

unsafe impl Send for XrSystemGraphicsProperties {}
unsafe impl Sync for XrSystemGraphicsProperties {}


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrSystemTrackingProperties {
	pub orientationTracking                          : XrBool32,
	pub positionTracking                             : XrBool32,
}

unsafe impl Send for XrSystemTrackingProperties {}
unsafe impl Sync for XrSystemTrackingProperties {}


#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingOpenGLWin32KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub hDC                                          : HDC,
	pub hGLRC                                        : HGLRC,
}

unsafe impl Send for XrGraphicsBindingOpenGLWin32KHR<'_> {}
unsafe impl Sync for XrGraphicsBindingOpenGLWin32KHR<'_> {}

impl fmt::Debug for XrGraphicsBindingOpenGLWin32KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingOpenGLWin32KHR").finish()
	}
}

impl Default for XrGraphicsBindingOpenGLWin32KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingOpenGLXlibKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub xDisplay                                     : &'a mut Display,
	pub visualid                                     : u32,
	pub glxFBConfig                                  : GLXFBConfig,
	pub glxDrawable                                  : GLXDrawable,
	pub glxContext                                   : GLXContext,
}

unsafe impl Send for XrGraphicsBindingOpenGLXlibKHR<'_> {}
unsafe impl Sync for XrGraphicsBindingOpenGLXlibKHR<'_> {}

impl fmt::Debug for XrGraphicsBindingOpenGLXlibKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingOpenGLXlibKHR").finish()
	}
}

impl Default for XrGraphicsBindingOpenGLXlibKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingOpenGLXcbKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub connection                                   : &'a mut xcb_connection_t,
	pub screenNumber                                 : u32,
	pub fbconfigid                                   : xcb_glx_fbconfig_t,
	pub visualid                                     : xcb_visualid_t,
	pub glxDrawable                                  : xcb_glx_drawable_t,
	pub glxContext                                   : xcb_glx_context_t,
}

unsafe impl Send for XrGraphicsBindingOpenGLXcbKHR<'_> {}
unsafe impl Sync for XrGraphicsBindingOpenGLXcbKHR<'_> {}

impl fmt::Debug for XrGraphicsBindingOpenGLXcbKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingOpenGLXcbKHR").finish()
	}
}

impl Default for XrGraphicsBindingOpenGLXcbKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingOpenGLWaylandKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub display                                      : &'a mut wl_display,
}

unsafe impl Send for XrGraphicsBindingOpenGLWaylandKHR<'_> {}
unsafe impl Sync for XrGraphicsBindingOpenGLWaylandKHR<'_> {}

impl fmt::Debug for XrGraphicsBindingOpenGLWaylandKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingOpenGLWaylandKHR").finish()
	}
}

impl Default for XrGraphicsBindingOpenGLWaylandKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingD3D11KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub device                                       : &'a mut ID3D11Device,
}

unsafe impl Send for XrGraphicsBindingD3D11KHR<'_> {}
unsafe impl Sync for XrGraphicsBindingD3D11KHR<'_> {}

impl fmt::Debug for XrGraphicsBindingD3D11KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingD3D11KHR").finish()
	}
}

impl Default for XrGraphicsBindingD3D11KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingD3D12KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub device                                       : &'a mut ID3D12Device,
	pub queue                                        : &'a mut ID3D12CommandQueue,
}

unsafe impl Send for XrGraphicsBindingD3D12KHR<'_> {}
unsafe impl Sync for XrGraphicsBindingD3D12KHR<'_> {}

impl fmt::Debug for XrGraphicsBindingD3D12KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingD3D12KHR").finish()
	}
}

impl Default for XrGraphicsBindingD3D12KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingOpenGLESAndroidKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub display                                      : EGLDisplay,
	pub config                                       : EGLConfig,
	pub context                                      : EGLContext,
}

unsafe impl Send for XrGraphicsBindingOpenGLESAndroidKHR<'_> {}
unsafe impl Sync for XrGraphicsBindingOpenGLESAndroidKHR<'_> {}

impl fmt::Debug for XrGraphicsBindingOpenGLESAndroidKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingOpenGLESAndroidKHR").finish()
	}
}

impl Default for XrGraphicsBindingOpenGLESAndroidKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingVulkanKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub instance                                     : VkInstance,
	pub physicalDevice                               : VkPhysicalDevice,
	pub device                                       : VkDevice,
	pub queueFamilyIndex                             : u32,
	pub queueIndex                                   : u32,
}

unsafe impl Send for XrGraphicsBindingVulkanKHR<'_> {}
unsafe impl Sync for XrGraphicsBindingVulkanKHR<'_> {}

impl fmt::Debug for XrGraphicsBindingVulkanKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingVulkanKHR").finish()
	}
}

impl Default for XrGraphicsBindingVulkanKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSessionCreateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub createFlags                                  : XrSessionCreateFlags,
	pub systemId                                     : XrSystemId,
}

unsafe impl Send for XrSessionCreateInfo<'_> {}
unsafe impl Sync for XrSessionCreateInfo<'_> {}

impl Default for XrSessionCreateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSessionBeginInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub primaryViewConfigurationType                 : XrViewConfigurationType,
}

unsafe impl Send for XrSessionBeginInfo<'_> {}
unsafe impl Sync for XrSessionBeginInfo<'_> {}

impl Default for XrSessionBeginInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSwapchainCreateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub createFlags                                  : XrSwapchainCreateFlags,
	pub usageFlags                                   : XrSwapchainUsageFlags,
	pub format                                       : i64,
	pub sampleCount                                  : u32,
	pub width                                        : u32,
	pub height                                       : u32,
	pub faceCount                                    : u32,
	pub arraySize                                    : u32,
	pub mipCount                                     : u32,
}

unsafe impl Send for XrSwapchainCreateInfo<'_> {}
unsafe impl Sync for XrSwapchainCreateInfo<'_> {}

impl Default for XrSwapchainCreateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSwapchainImageBaseHeader<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
}

unsafe impl Send for XrSwapchainImageBaseHeader<'_> {}
unsafe impl Sync for XrSwapchainImageBaseHeader<'_> {}

impl Default for XrSwapchainImageBaseHeader<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSwapchainImageOpenGLKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub image                                        : u32,
}

unsafe impl Send for XrSwapchainImageOpenGLKHR<'_> {}
unsafe impl Sync for XrSwapchainImageOpenGLKHR<'_> {}

impl Default for XrSwapchainImageOpenGLKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSwapchainImageOpenGLESKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub image                                        : u32,
}

unsafe impl Send for XrSwapchainImageOpenGLESKHR<'_> {}
unsafe impl Sync for XrSwapchainImageOpenGLESKHR<'_> {}

impl Default for XrSwapchainImageOpenGLESKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrSwapchainImageVulkanKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub image                                        : VkImage,
}

unsafe impl Send for XrSwapchainImageVulkanKHR<'_> {}
unsafe impl Sync for XrSwapchainImageVulkanKHR<'_> {}

impl fmt::Debug for XrSwapchainImageVulkanKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSwapchainImageVulkanKHR").finish()
	}
}

impl Default for XrSwapchainImageVulkanKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrSwapchainImageD3D11KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub texture                                      : &'a mut ID3D11Texture2D,
}

unsafe impl Send for XrSwapchainImageD3D11KHR<'_> {}
unsafe impl Sync for XrSwapchainImageD3D11KHR<'_> {}

impl fmt::Debug for XrSwapchainImageD3D11KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSwapchainImageD3D11KHR").finish()
	}
}

impl Default for XrSwapchainImageD3D11KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrSwapchainImageD3D12KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub texture                                      : &'a mut ID3D12Resource,
}

unsafe impl Send for XrSwapchainImageD3D12KHR<'_> {}
unsafe impl Sync for XrSwapchainImageD3D12KHR<'_> {}

impl fmt::Debug for XrSwapchainImageD3D12KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSwapchainImageD3D12KHR").finish()
	}
}

impl Default for XrSwapchainImageD3D12KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSwapchainImageAcquireInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrSwapchainImageAcquireInfo<'_> {}
unsafe impl Sync for XrSwapchainImageAcquireInfo<'_> {}

impl Default for XrSwapchainImageAcquireInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSwapchainImageWaitInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub timeout                                      : XrDuration,
}

unsafe impl Send for XrSwapchainImageWaitInfo<'_> {}
unsafe impl Sync for XrSwapchainImageWaitInfo<'_> {}

impl Default for XrSwapchainImageWaitInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSwapchainImageReleaseInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrSwapchainImageReleaseInfo<'_> {}
unsafe impl Sync for XrSwapchainImageReleaseInfo<'_> {}

impl Default for XrSwapchainImageReleaseInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrReferenceSpaceCreateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub referenceSpaceType                           : XrReferenceSpaceType,
	pub poseInReferenceSpace                         : XrPosef,
}

unsafe impl Send for XrReferenceSpaceCreateInfo<'_> {}
unsafe impl Sync for XrReferenceSpaceCreateInfo<'_> {}

impl fmt::Debug for XrReferenceSpaceCreateInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrReferenceSpaceCreateInfo").finish()
	}
}

impl Default for XrReferenceSpaceCreateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrActionSpaceCreateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub action                                       : XrAction,
	pub subactionPath                                : XrPath,
	pub poseInActionSpace                            : XrPosef,
}

unsafe impl Send for XrActionSpaceCreateInfo<'_> {}
unsafe impl Sync for XrActionSpaceCreateInfo<'_> {}

impl fmt::Debug for XrActionSpaceCreateInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrActionSpaceCreateInfo").finish()
	}
}

impl Default for XrActionSpaceCreateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrSpaceLocation<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub locationFlags                                : XrSpaceLocationFlags,
	pub pose                                         : XrPosef,
}

unsafe impl Send for XrSpaceLocation<'_> {}
unsafe impl Sync for XrSpaceLocation<'_> {}

impl fmt::Debug for XrSpaceLocation<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSpaceLocation").finish()
	}
}

impl Default for XrSpaceLocation<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrSpaceVelocity<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub velocityFlags                                : XrSpaceVelocityFlags,
	pub linearVelocity                               : XrVector3f,
	pub angularVelocity                              : XrVector3f,
}

unsafe impl Send for XrSpaceVelocity<'_> {}
unsafe impl Sync for XrSpaceVelocity<'_> {}

impl fmt::Debug for XrSpaceVelocity<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSpaceVelocity").finish()
	}
}

impl Default for XrSpaceVelocity<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrFovf {
	pub angleLeft                                    : f32,
	pub angleRight                                   : f32,
	pub angleUp                                      : f32,
	pub angleDown                                    : f32,
}

unsafe impl Send for XrFovf {}
unsafe impl Sync for XrFovf {}


#[repr(C)]
#[derive()]
pub struct XrView<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub pose                                         : XrPosef,
	pub fov                                          : XrFovf,
}

unsafe impl Send for XrView<'_> {}
unsafe impl Sync for XrView<'_> {}

impl fmt::Debug for XrView<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrView").finish()
	}
}

impl Default for XrView<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrViewLocateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub viewConfigurationType                        : XrViewConfigurationType,
	pub displayTime                                  : XrTime,
	pub space                                        : XrSpace,
}

unsafe impl Send for XrViewLocateInfo<'_> {}
unsafe impl Sync for XrViewLocateInfo<'_> {}

impl Default for XrViewLocateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrViewState<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub viewStateFlags                               : XrViewStateFlags,
}

unsafe impl Send for XrViewState<'_> {}
unsafe impl Sync for XrViewState<'_> {}

impl Default for XrViewState<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrViewConfigurationView<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub recommendedImageRectWidth                    : u32,
	pub maxImageRectWidth                            : u32,
	pub recommendedImageRectHeight                   : u32,
	pub maxImageRectHeight                           : u32,
	pub recommendedSwapchainSampleCount              : u32,
	pub maxSwapchainSampleCount                      : u32,
}

unsafe impl Send for XrViewConfigurationView<'_> {}
unsafe impl Sync for XrViewConfigurationView<'_> {}

impl Default for XrViewConfigurationView<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrSwapchainSubImage {
	pub swapchain                                    : XrSwapchain,
	pub imageRect                                    : XrRect2Di,
	pub imageArrayIndex                              : u32,
}

unsafe impl Send for XrSwapchainSubImage {}
unsafe impl Sync for XrSwapchainSubImage {}

impl fmt::Debug for XrSwapchainSubImage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSwapchainSubImage").finish()
	}
}

impl Default for XrSwapchainSubImage { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrCompositionLayerBaseHeader<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub layerFlags                                   : XrCompositionLayerFlags,
	pub space                                        : XrSpace,
}

unsafe impl Send for XrCompositionLayerBaseHeader<'_> {}
unsafe impl Sync for XrCompositionLayerBaseHeader<'_> {}

impl Default for XrCompositionLayerBaseHeader<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerProjectionView<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub pose                                         : XrPosef,
	pub fov                                          : XrFovf,
	pub subImage                                     : XrSwapchainSubImage,
}

unsafe impl Send for XrCompositionLayerProjectionView<'_> {}
unsafe impl Sync for XrCompositionLayerProjectionView<'_> {}

impl fmt::Debug for XrCompositionLayerProjectionView<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerProjectionView").finish()
	}
}

impl Default for XrCompositionLayerProjectionView<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerProjection<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub layerFlags                                   : XrCompositionLayerFlags,
	pub space                                        : XrSpace,
	pub viewCount                                    : u32,
	pub views                                        : *const XrCompositionLayerProjectionView<'a>,
}

unsafe impl Send for XrCompositionLayerProjection<'_> {}
unsafe impl Sync for XrCompositionLayerProjection<'_> {}

impl fmt::Debug for XrCompositionLayerProjection<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerProjection").finish()
	}
}

impl Default for XrCompositionLayerProjection<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerQuad<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub layerFlags                                   : XrCompositionLayerFlags,
	pub space                                        : XrSpace,
	pub eyeVisibility                                : XrEyeVisibility,
	pub subImage                                     : XrSwapchainSubImage,
	pub pose                                         : XrPosef,
	pub size                                         : XrExtent2Df,
}

unsafe impl Send for XrCompositionLayerQuad<'_> {}
unsafe impl Sync for XrCompositionLayerQuad<'_> {}

impl fmt::Debug for XrCompositionLayerQuad<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerQuad").finish()
	}
}

impl Default for XrCompositionLayerQuad<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerCylinderKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub layerFlags                                   : XrCompositionLayerFlags,
	pub space                                        : XrSpace,
	pub eyeVisibility                                : XrEyeVisibility,
	pub subImage                                     : XrSwapchainSubImage,
	pub pose                                         : XrPosef,
	pub radius                                       : f32,
	pub centralAngle                                 : f32,
	pub aspectRatio                                  : f32,
}

unsafe impl Send for XrCompositionLayerCylinderKHR<'_> {}
unsafe impl Sync for XrCompositionLayerCylinderKHR<'_> {}

impl fmt::Debug for XrCompositionLayerCylinderKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerCylinderKHR").finish()
	}
}

impl Default for XrCompositionLayerCylinderKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerCubeKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub layerFlags                                   : XrCompositionLayerFlags,
	pub space                                        : XrSpace,
	pub eyeVisibility                                : XrEyeVisibility,
	pub swapchain                                    : XrSwapchain,
	pub imageArrayIndex                              : u32,
	pub orientation                                  : XrQuaternionf,
}

unsafe impl Send for XrCompositionLayerCubeKHR<'_> {}
unsafe impl Sync for XrCompositionLayerCubeKHR<'_> {}

impl fmt::Debug for XrCompositionLayerCubeKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerCubeKHR").finish()
	}
}

impl Default for XrCompositionLayerCubeKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerEquirectKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub layerFlags                                   : XrCompositionLayerFlags,
	pub space                                        : XrSpace,
	pub eyeVisibility                                : XrEyeVisibility,
	pub subImage                                     : XrSwapchainSubImage,
	pub pose                                         : XrPosef,
	pub radius                                       : f32,
	pub scale                                        : XrVector2f,
	pub bias                                         : XrVector2f,
}

unsafe impl Send for XrCompositionLayerEquirectKHR<'_> {}
unsafe impl Sync for XrCompositionLayerEquirectKHR<'_> {}

impl fmt::Debug for XrCompositionLayerEquirectKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerEquirectKHR").finish()
	}
}

impl Default for XrCompositionLayerEquirectKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerDepthInfoKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub subImage                                     : XrSwapchainSubImage,
	pub minDepth                                     : f32,
	pub maxDepth                                     : f32,
	pub nearZ                                        : f32,
	pub farZ                                         : f32,
}

unsafe impl Send for XrCompositionLayerDepthInfoKHR<'_> {}
unsafe impl Sync for XrCompositionLayerDepthInfoKHR<'_> {}

impl fmt::Debug for XrCompositionLayerDepthInfoKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerDepthInfoKHR").finish()
	}
}

impl Default for XrCompositionLayerDepthInfoKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrFrameBeginInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrFrameBeginInfo<'_> {}
unsafe impl Sync for XrFrameBeginInfo<'_> {}

impl Default for XrFrameBeginInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrFrameEndInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub displayTime                                  : XrTime,
	pub environmentBlendMode                         : XrEnvironmentBlendMode,
	pub layerCount                                   : u32,
	pub layers                                       : *const *const XrCompositionLayerBaseHeader<'a>,
}

unsafe impl Send for XrFrameEndInfo<'_> {}
unsafe impl Sync for XrFrameEndInfo<'_> {}

impl fmt::Debug for XrFrameEndInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrFrameEndInfo").finish()
	}
}

impl Default for XrFrameEndInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrFrameWaitInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrFrameWaitInfo<'_> {}
unsafe impl Sync for XrFrameWaitInfo<'_> {}

impl Default for XrFrameWaitInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrFrameState<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub predictedDisplayTime                         : XrTime,
	pub predictedDisplayPeriod                       : XrDuration,
	pub shouldRender                                 : XrBool32,
}

unsafe impl Send for XrFrameState<'_> {}
unsafe impl Sync for XrFrameState<'_> {}

impl Default for XrFrameState<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrHapticBaseHeader<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrHapticBaseHeader<'_> {}
unsafe impl Sync for XrHapticBaseHeader<'_> {}

impl Default for XrHapticBaseHeader<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrHapticVibration<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub duration                                     : XrDuration,
	pub frequency                                    : f32,
	pub amplitude                                    : f32,
}

unsafe impl Send for XrHapticVibration<'_> {}
unsafe impl Sync for XrHapticVibration<'_> {}

impl Default for XrHapticVibration<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataBaseHeader<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrEventDataBaseHeader<'_> {}
unsafe impl Sync for XrEventDataBaseHeader<'_> {}

impl Default for XrEventDataBaseHeader<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrEventDataBuffer<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub varying                                      : [u8;  4000 as usize],
}

unsafe impl Send for XrEventDataBuffer<'_> {}
unsafe impl Sync for XrEventDataBuffer<'_> {}

impl fmt::Debug for XrEventDataBuffer<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrEventDataBuffer").finish()
	}
}

impl Default for XrEventDataBuffer<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataEventsLost<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub lostEventCount                               : u32,
}

unsafe impl Send for XrEventDataEventsLost<'_> {}
unsafe impl Sync for XrEventDataEventsLost<'_> {}

impl Default for XrEventDataEventsLost<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataInstanceLossPending<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub lossTime                                     : XrTime,
}

unsafe impl Send for XrEventDataInstanceLossPending<'_> {}
unsafe impl Sync for XrEventDataInstanceLossPending<'_> {}

impl Default for XrEventDataInstanceLossPending<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataSessionStateChanged<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub session                                      : XrSession,
	pub state                                        : XrSessionState,
	pub time                                         : XrTime,
}

unsafe impl Send for XrEventDataSessionStateChanged<'_> {}
unsafe impl Sync for XrEventDataSessionStateChanged<'_> {}

impl Default for XrEventDataSessionStateChanged<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrEventDataReferenceSpaceChangePending<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub session                                      : XrSession,
	pub referenceSpaceType                           : XrReferenceSpaceType,
	pub changeTime                                   : XrTime,
	pub poseValid                                    : XrBool32,
	pub poseInPreviousSpace                          : XrPosef,
}

unsafe impl Send for XrEventDataReferenceSpaceChangePending<'_> {}
unsafe impl Sync for XrEventDataReferenceSpaceChangePending<'_> {}

impl fmt::Debug for XrEventDataReferenceSpaceChangePending<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrEventDataReferenceSpaceChangePending").finish()
	}
}

impl Default for XrEventDataReferenceSpaceChangePending<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataPerfSettingsEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub domain                                       : XrPerfSettingsDomainEXT,
	pub subDomain                                    : XrPerfSettingsSubDomainEXT,
	pub fromLevel                                    : XrPerfSettingsNotificationLevelEXT,
	pub toLevel                                      : XrPerfSettingsNotificationLevelEXT,
}

unsafe impl Send for XrEventDataPerfSettingsEXT<'_> {}
unsafe impl Sync for XrEventDataPerfSettingsEXT<'_> {}

impl Default for XrEventDataPerfSettingsEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataVisibilityMaskChangedKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub session                                      : XrSession,
	pub viewConfigurationType                        : XrViewConfigurationType,
	pub viewIndex                                    : u32,
}

unsafe impl Send for XrEventDataVisibilityMaskChangedKHR<'_> {}
unsafe impl Sync for XrEventDataVisibilityMaskChangedKHR<'_> {}

impl Default for XrEventDataVisibilityMaskChangedKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrViewConfigurationProperties<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub viewConfigurationType                        : XrViewConfigurationType,
	pub fovMutable                                   : XrBool32,
}

unsafe impl Send for XrViewConfigurationProperties<'_> {}
unsafe impl Sync for XrViewConfigurationProperties<'_> {}

impl Default for XrViewConfigurationProperties<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrActionStateBoolean<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub currentState                                 : XrBool32,
	pub changedSinceLastSync                         : XrBool32,
	pub lastChangeTime                               : XrTime,
	pub isActive                                     : XrBool32,
}

unsafe impl Send for XrActionStateBoolean<'_> {}
unsafe impl Sync for XrActionStateBoolean<'_> {}

impl Default for XrActionStateBoolean<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrActionStateFloat<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub currentState                                 : f32,
	pub changedSinceLastSync                         : XrBool32,
	pub lastChangeTime                               : XrTime,
	pub isActive                                     : XrBool32,
}

unsafe impl Send for XrActionStateFloat<'_> {}
unsafe impl Sync for XrActionStateFloat<'_> {}

impl Default for XrActionStateFloat<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrActionStateVector2f<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub currentState                                 : XrVector2f,
	pub changedSinceLastSync                         : XrBool32,
	pub lastChangeTime                               : XrTime,
	pub isActive                                     : XrBool32,
}

unsafe impl Send for XrActionStateVector2f<'_> {}
unsafe impl Sync for XrActionStateVector2f<'_> {}

impl fmt::Debug for XrActionStateVector2f<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrActionStateVector2f").finish()
	}
}

impl Default for XrActionStateVector2f<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrActionStatePose<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub isActive                                     : XrBool32,
}

unsafe impl Send for XrActionStatePose<'_> {}
unsafe impl Sync for XrActionStatePose<'_> {}

impl Default for XrActionStatePose<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrActionStateGetInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub action                                       : XrAction,
	pub subactionPath                                : XrPath,
}

unsafe impl Send for XrActionStateGetInfo<'_> {}
unsafe impl Sync for XrActionStateGetInfo<'_> {}

impl Default for XrActionStateGetInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrHapticActionInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub action                                       : XrAction,
	pub subactionPath                                : XrPath,
}

unsafe impl Send for XrHapticActionInfo<'_> {}
unsafe impl Sync for XrHapticActionInfo<'_> {}

impl Default for XrHapticActionInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrActionSetCreateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub actionSetName                                : [u8;  XR_MAX_ACTION_SET_NAME_SIZE as usize],
	pub localizedActionSetName                       : [u8;  XR_MAX_LOCALIZED_ACTION_SET_NAME_SIZE as usize],
	pub priority                                     : u32,
}

unsafe impl Send for XrActionSetCreateInfo<'_> {}
unsafe impl Sync for XrActionSetCreateInfo<'_> {}

impl fmt::Debug for XrActionSetCreateInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrActionSetCreateInfo").finish()
	}
}

impl Default for XrActionSetCreateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrActionSuggestedBinding {
	pub action                                       : XrAction,
	pub binding                                      : XrPath,
}

unsafe impl Send for XrActionSuggestedBinding {}
unsafe impl Sync for XrActionSuggestedBinding {}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrInteractionProfileSuggestedBinding<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub interactionProfile                           : XrPath,
	pub countSuggestedBindings                       : u32,
	pub suggestedBindings                            : *const XrActionSuggestedBinding,
}

unsafe impl Send for XrInteractionProfileSuggestedBinding<'_> {}
unsafe impl Sync for XrInteractionProfileSuggestedBinding<'_> {}

impl fmt::Debug for XrInteractionProfileSuggestedBinding<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrInteractionProfileSuggestedBinding").finish()
	}
}

impl Default for XrInteractionProfileSuggestedBinding<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct XrActiveActionSet {
	pub actionSet                                    : XrActionSet,
	pub subactionPath                                : XrPath,
}

unsafe impl Send for XrActiveActionSet {}
unsafe impl Sync for XrActiveActionSet {}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSessionActionSetsAttachInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub countActionSets                              : u32,
	pub actionSets                                   : *const XrActionSet,
}

unsafe impl Send for XrSessionActionSetsAttachInfo<'_> {}
unsafe impl Sync for XrSessionActionSetsAttachInfo<'_> {}

impl Default for XrSessionActionSetsAttachInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrActionsSyncInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub countActiveActionSets                        : u32,
	pub activeActionSets                             : *const XrActiveActionSet,
}

unsafe impl Send for XrActionsSyncInfo<'_> {}
unsafe impl Sync for XrActionsSyncInfo<'_> {}

impl fmt::Debug for XrActionsSyncInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrActionsSyncInfo").finish()
	}
}

impl Default for XrActionsSyncInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrBoundSourcesForActionEnumerateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub action                                       : XrAction,
}

unsafe impl Send for XrBoundSourcesForActionEnumerateInfo<'_> {}
unsafe impl Sync for XrBoundSourcesForActionEnumerateInfo<'_> {}

impl Default for XrBoundSourcesForActionEnumerateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrInputSourceLocalizedNameGetInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub sourcePath                                   : XrPath,
	pub whichComponents                              : XrInputSourceLocalizedNameFlags,
}

unsafe impl Send for XrInputSourceLocalizedNameGetInfo<'_> {}
unsafe impl Sync for XrInputSourceLocalizedNameGetInfo<'_> {}

impl Default for XrInputSourceLocalizedNameGetInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataInteractionProfileChanged<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub session                                      : XrSession,
}

unsafe impl Send for XrEventDataInteractionProfileChanged<'_> {}
unsafe impl Sync for XrEventDataInteractionProfileChanged<'_> {}

impl Default for XrEventDataInteractionProfileChanged<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrInteractionProfileState<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub interactionProfile                           : XrPath,
}

unsafe impl Send for XrInteractionProfileState<'_> {}
unsafe impl Sync for XrInteractionProfileState<'_> {}

impl Default for XrInteractionProfileState<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrActionCreateInfo<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub actionName                                   : [u8;  XR_MAX_ACTION_NAME_SIZE as usize],
	pub actionType                                   : XrActionType,
	pub countSubactionPaths                          : u32,
	pub subactionPaths                               : *const XrPath,
	pub localizedActionName                          : [u8;  XR_MAX_LOCALIZED_ACTION_NAME_SIZE as usize],
}

unsafe impl Send for XrActionCreateInfo<'_> {}
unsafe impl Sync for XrActionCreateInfo<'_> {}

impl fmt::Debug for XrActionCreateInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrActionCreateInfo").finish()
	}
}

impl Default for XrActionCreateInfo<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrInstanceCreateInfoAndroidKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub applicationVM                                : XrAnyMut<'a>,
	pub applicationActivity                          : XrAnyMut<'a>,
}

unsafe impl Send for XrInstanceCreateInfoAndroidKHR<'_> {}
unsafe impl Sync for XrInstanceCreateInfoAndroidKHR<'_> {}

impl Default for XrInstanceCreateInfoAndroidKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrVulkanSwapchainFormatListCreateInfoKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub viewFormatCount                              : u32,
	pub viewFormats                                  : *const VkFormat,
}

unsafe impl Send for XrVulkanSwapchainFormatListCreateInfoKHR<'_> {}
unsafe impl Sync for XrVulkanSwapchainFormatListCreateInfoKHR<'_> {}

impl fmt::Debug for XrVulkanSwapchainFormatListCreateInfoKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrVulkanSwapchainFormatListCreateInfoKHR").finish()
	}
}

impl Default for XrVulkanSwapchainFormatListCreateInfoKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrDebugUtilsObjectNameInfoEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub objectType                                   : XrObjectType,
	pub objectHandle                                 : u64,
	pub objectName                                   : *const u8,
}

unsafe impl Send for XrDebugUtilsObjectNameInfoEXT<'_> {}
unsafe impl Sync for XrDebugUtilsObjectNameInfoEXT<'_> {}

impl Default for XrDebugUtilsObjectNameInfoEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrDebugUtilsLabelEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub labelName                                    : *const u8,
}

unsafe impl Send for XrDebugUtilsLabelEXT<'_> {}
unsafe impl Sync for XrDebugUtilsLabelEXT<'_> {}

impl Default for XrDebugUtilsLabelEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrDebugUtilsMessengerCallbackDataEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub messageId                                    : *const u8,
	pub functionName                                 : *const u8,
	pub message                                      : *const u8,
	pub objectCount                                  : u32,
	pub objects                                      : *mut XrDebugUtilsObjectNameInfoEXT<'a>,
	pub sessionLabelCount                            : u32,
	pub sessionLabels                                : *mut XrDebugUtilsLabelEXT<'a>,
}

unsafe impl Send for XrDebugUtilsMessengerCallbackDataEXT<'_> {}
unsafe impl Sync for XrDebugUtilsMessengerCallbackDataEXT<'_> {}

impl fmt::Debug for XrDebugUtilsMessengerCallbackDataEXT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrDebugUtilsMessengerCallbackDataEXT").finish()
	}
}

impl Default for XrDebugUtilsMessengerCallbackDataEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrDebugUtilsMessengerCreateInfoEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub messageSeverities                            : XrDebugUtilsMessageSeverityFlagsEXT,
	pub messageTypes                                 : XrDebugUtilsMessageTypeFlagsEXT,
	pub userCallback                                 : PFN_xrDebugUtilsMessengerCallbackEXT,
	pub userData                                     : Option<XrAnyMut<'a>>,
}

unsafe impl Send for XrDebugUtilsMessengerCreateInfoEXT<'_> {}
unsafe impl Sync for XrDebugUtilsMessengerCreateInfoEXT<'_> {}

impl fmt::Debug for XrDebugUtilsMessengerCreateInfoEXT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrDebugUtilsMessengerCreateInfoEXT").finish()
	}
}

impl Default for XrDebugUtilsMessengerCreateInfoEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrVisibilityMaskKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub vertexCapacityInput                          : u32,
	pub vertexCountOutput                            : u32,
	pub vertices                                     : *mut XrVector2f,
	pub indexCapacityInput                           : u32,
	pub indexCountOutput                             : u32,
	pub indices                                      : *mut u32,
}

unsafe impl Send for XrVisibilityMaskKHR<'_> {}
unsafe impl Sync for XrVisibilityMaskKHR<'_> {}

impl fmt::Debug for XrVisibilityMaskKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrVisibilityMaskKHR").finish()
	}
}

impl Default for XrVisibilityMaskKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrGraphicsRequirementsOpenGLKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub minApiVersionSupported                       : XrVersion,
	pub maxApiVersionSupported                       : XrVersion,
}

unsafe impl Send for XrGraphicsRequirementsOpenGLKHR<'_> {}
unsafe impl Sync for XrGraphicsRequirementsOpenGLKHR<'_> {}

impl Default for XrGraphicsRequirementsOpenGLKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrGraphicsRequirementsOpenGLESKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub minApiVersionSupported                       : XrVersion,
	pub maxApiVersionSupported                       : XrVersion,
}

unsafe impl Send for XrGraphicsRequirementsOpenGLESKHR<'_> {}
unsafe impl Sync for XrGraphicsRequirementsOpenGLESKHR<'_> {}

impl Default for XrGraphicsRequirementsOpenGLESKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrGraphicsRequirementsVulkanKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub minApiVersionSupported                       : XrVersion,
	pub maxApiVersionSupported                       : XrVersion,
}

unsafe impl Send for XrGraphicsRequirementsVulkanKHR<'_> {}
unsafe impl Sync for XrGraphicsRequirementsVulkanKHR<'_> {}

impl Default for XrGraphicsRequirementsVulkanKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsRequirementsD3D11KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub adapterLuid                                  : LUID,
	pub minFeatureLevel                              : D3D_FEATURE_LEVEL,
}

unsafe impl Send for XrGraphicsRequirementsD3D11KHR<'_> {}
unsafe impl Sync for XrGraphicsRequirementsD3D11KHR<'_> {}

impl fmt::Debug for XrGraphicsRequirementsD3D11KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsRequirementsD3D11KHR").finish()
	}
}

impl Default for XrGraphicsRequirementsD3D11KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsRequirementsD3D12KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub adapterLuid                                  : LUID,
	pub minFeatureLevel                              : D3D_FEATURE_LEVEL,
}

unsafe impl Send for XrGraphicsRequirementsD3D12KHR<'_> {}
unsafe impl Sync for XrGraphicsRequirementsD3D12KHR<'_> {}

impl fmt::Debug for XrGraphicsRequirementsD3D12KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsRequirementsD3D12KHR").finish()
	}
}

impl Default for XrGraphicsRequirementsD3D12KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrVulkanInstanceCreateInfoKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub systemId                                     : XrSystemId,
	pub createFlags                                  : XrVulkanInstanceCreateFlagsKHR,
	pub pfnGetInstanceProcAddr                       : PFN_vkGetInstanceProcAddr,
	pub vulkanCreateInfo                             : &'a VkInstanceCreateInfo<'a>,
	pub vulkanAllocator                              : Option<&'a VkAllocationCallbacks<'a>>,
}

unsafe impl Send for XrVulkanInstanceCreateInfoKHR<'_> {}
unsafe impl Sync for XrVulkanInstanceCreateInfoKHR<'_> {}

impl fmt::Debug for XrVulkanInstanceCreateInfoKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrVulkanInstanceCreateInfoKHR").finish()
	}
}

impl Default for XrVulkanInstanceCreateInfoKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrVulkanDeviceCreateInfoKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub systemId                                     : XrSystemId,
	pub createFlags                                  : XrVulkanDeviceCreateFlagsKHR,
	pub pfnGetInstanceProcAddr                       : PFN_vkGetInstanceProcAddr,
	pub vulkanPhysicalDevice                         : VkPhysicalDevice,
	pub vulkanCreateInfo                             : &'a VkDeviceCreateInfo<'a>,
	pub vulkanAllocator                              : Option<&'a VkAllocationCallbacks<'a>>,
}

unsafe impl Send for XrVulkanDeviceCreateInfoKHR<'_> {}
unsafe impl Sync for XrVulkanDeviceCreateInfoKHR<'_> {}

impl fmt::Debug for XrVulkanDeviceCreateInfoKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrVulkanDeviceCreateInfoKHR").finish()
	}
}

impl Default for XrVulkanDeviceCreateInfoKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

pub type XrGraphicsBindingVulkan2KHR                  <'a> = XrGraphicsBindingVulkanKHR<'a>;

#[repr(C)]
#[derive()]
pub struct XrVulkanGraphicsDeviceGetInfoKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub systemId                                     : XrSystemId,
	pub vulkanInstance                               : VkInstance,
}

unsafe impl Send for XrVulkanGraphicsDeviceGetInfoKHR<'_> {}
unsafe impl Sync for XrVulkanGraphicsDeviceGetInfoKHR<'_> {}

impl fmt::Debug for XrVulkanGraphicsDeviceGetInfoKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrVulkanGraphicsDeviceGetInfoKHR").finish()
	}
}

impl Default for XrVulkanGraphicsDeviceGetInfoKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

pub type XrSwapchainImageVulkan2KHR                   <'a> = XrSwapchainImageVulkanKHR<'a>;

pub type XrGraphicsRequirementsVulkan2KHR             <'a> = XrGraphicsRequirementsVulkanKHR<'a>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSessionCreateInfoOverlayEXTX<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub createFlags                                  : XrOverlaySessionCreateFlagsEXTX,
	pub sessionLayersPlacement                       : u32,
}

unsafe impl Send for XrSessionCreateInfoOverlayEXTX<'_> {}
unsafe impl Sync for XrSessionCreateInfoOverlayEXTX<'_> {}

impl Default for XrSessionCreateInfoOverlayEXTX<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataMainSessionVisibilityChangedEXTX<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub visible                                      : XrBool32,
	pub flags                                        : XrOverlayMainSessionFlagsEXTX,
}

unsafe impl Send for XrEventDataMainSessionVisibilityChangedEXTX<'_> {}
unsafe impl Sync for XrEventDataMainSessionVisibilityChangedEXTX<'_> {}

impl Default for XrEventDataMainSessionVisibilityChangedEXTX<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrEventDataDisplayRefreshRateChangedFB<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub fromDisplayRefreshRate                       : f32,
	pub toDisplayRefreshRate                         : f32,
}

unsafe impl Send for XrEventDataDisplayRefreshRateChangedFB<'_> {}
unsafe impl Sync for XrEventDataDisplayRefreshRateChangedFB<'_> {}

impl Default for XrEventDataDisplayRefreshRateChangedFB<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrViewConfigurationDepthRangeEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub recommendedNearZ                             : f32,
	pub minNearZ                                     : f32,
	pub recommendedFarZ                              : f32,
	pub maxFarZ                                      : f32,
}

unsafe impl Send for XrViewConfigurationDepthRangeEXT<'_> {}
unsafe impl Sync for XrViewConfigurationDepthRangeEXT<'_> {}

impl Default for XrViewConfigurationDepthRangeEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrViewConfigurationViewFovEPIC<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub recommendedFov                               : XrFovf,
	pub maxMutableFov                                : XrFovf,
}

unsafe impl Send for XrViewConfigurationViewFovEPIC<'_> {}
unsafe impl Sync for XrViewConfigurationViewFovEPIC<'_> {}

impl fmt::Debug for XrViewConfigurationViewFovEPIC<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrViewConfigurationViewFovEPIC").finish()
	}
}

impl Default for XrViewConfigurationViewFovEPIC<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrInteractionProfileAnalogThresholdVALVE<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub action                                       : XrAction,
	pub binding                                      : XrPath,
	pub onThreshold                                  : f32,
	pub offThreshold                                 : f32,
	pub onHaptic                                     : Option<&'a XrHapticBaseHeader<'a>>,
	pub offHaptic                                    : Option<&'a XrHapticBaseHeader<'a>>,
}

unsafe impl Send for XrInteractionProfileAnalogThresholdVALVE<'_> {}
unsafe impl Sync for XrInteractionProfileAnalogThresholdVALVE<'_> {}

impl fmt::Debug for XrInteractionProfileAnalogThresholdVALVE<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrInteractionProfileAnalogThresholdVALVE").finish()
	}
}

impl Default for XrInteractionProfileAnalogThresholdVALVE<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrBindingModificationsKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub bindingModificationCount                     : u32,
	pub bindingModifications                         : *const *const XrBindingModificationBaseHeaderKHR<'a>,
}

unsafe impl Send for XrBindingModificationsKHR<'_> {}
unsafe impl Sync for XrBindingModificationsKHR<'_> {}

impl fmt::Debug for XrBindingModificationsKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrBindingModificationsKHR").finish()
	}
}

impl Default for XrBindingModificationsKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrBindingModificationBaseHeaderKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrBindingModificationBaseHeaderKHR<'_> {}
unsafe impl Sync for XrBindingModificationBaseHeaderKHR<'_> {}

impl Default for XrBindingModificationBaseHeaderKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSystemEyeGazeInteractionPropertiesEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub supportsEyeGazeInteraction                   : XrBool32,
}

unsafe impl Send for XrSystemEyeGazeInteractionPropertiesEXT<'_> {}
unsafe impl Sync for XrSystemEyeGazeInteractionPropertiesEXT<'_> {}

impl Default for XrSystemEyeGazeInteractionPropertiesEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrEyeGazeSampleTimeEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub time                                         : XrTime,
}

unsafe impl Send for XrEyeGazeSampleTimeEXT<'_> {}
unsafe impl Sync for XrEyeGazeSampleTimeEXT<'_> {}

impl Default for XrEyeGazeSampleTimeEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrSpatialAnchorCreateInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub space                                        : XrSpace,
	pub pose                                         : XrPosef,
	pub time                                         : XrTime,
}

unsafe impl Send for XrSpatialAnchorCreateInfoMSFT<'_> {}
unsafe impl Sync for XrSpatialAnchorCreateInfoMSFT<'_> {}

impl fmt::Debug for XrSpatialAnchorCreateInfoMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSpatialAnchorCreateInfoMSFT").finish()
	}
}

impl Default for XrSpatialAnchorCreateInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrSpatialAnchorSpaceCreateInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub anchor                                       : XrSpatialAnchorMSFT,
	pub poseInAnchorSpace                            : XrPosef,
}

unsafe impl Send for XrSpatialAnchorSpaceCreateInfoMSFT<'_> {}
unsafe impl Sync for XrSpatialAnchorSpaceCreateInfoMSFT<'_> {}

impl fmt::Debug for XrSpatialAnchorSpaceCreateInfoMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSpatialAnchorSpaceCreateInfoMSFT").finish()
	}
}

impl Default for XrSpatialAnchorSpaceCreateInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrGraphicsBindingEGLMNDX<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub getProcAddress                               : PFNEGLGETPROCADDRESSPROC,
	pub display                                      : EGLDisplay,
	pub config                                       : EGLConfig,
	pub context                                      : EGLContext,
}

unsafe impl Send for XrGraphicsBindingEGLMNDX<'_> {}
unsafe impl Sync for XrGraphicsBindingEGLMNDX<'_> {}

impl fmt::Debug for XrGraphicsBindingEGLMNDX<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrGraphicsBindingEGLMNDX").finish()
	}
}

impl Default for XrGraphicsBindingEGLMNDX<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrSpatialGraphNodeSpaceCreateInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub nodeType                                     : XrSpatialGraphNodeTypeMSFT,
	pub nodeId                                       : [u8;  16 as usize],
	pub pose                                         : XrPosef,
}

unsafe impl Send for XrSpatialGraphNodeSpaceCreateInfoMSFT<'_> {}
unsafe impl Sync for XrSpatialGraphNodeSpaceCreateInfoMSFT<'_> {}

impl fmt::Debug for XrSpatialGraphNodeSpaceCreateInfoMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSpatialGraphNodeSpaceCreateInfoMSFT").finish()
	}
}

impl Default for XrSpatialGraphNodeSpaceCreateInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSystemHandTrackingPropertiesEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub supportsHandTracking                         : XrBool32,
}

unsafe impl Send for XrSystemHandTrackingPropertiesEXT<'_> {}
unsafe impl Sync for XrSystemHandTrackingPropertiesEXT<'_> {}

impl Default for XrSystemHandTrackingPropertiesEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrHandTrackerCreateInfoEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub hand                                         : XrHandEXT,
	pub handJointSet                                 : XrHandJointSetEXT,
}

unsafe impl Send for XrHandTrackerCreateInfoEXT<'_> {}
unsafe impl Sync for XrHandTrackerCreateInfoEXT<'_> {}

impl Default for XrHandTrackerCreateInfoEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrHandJointsLocateInfoEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub baseSpace                                    : XrSpace,
	pub time                                         : XrTime,
}

unsafe impl Send for XrHandJointsLocateInfoEXT<'_> {}
unsafe impl Sync for XrHandJointsLocateInfoEXT<'_> {}

impl Default for XrHandJointsLocateInfoEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrHandJointLocationEXT {
	pub locationFlags                                : XrSpaceLocationFlags,
	pub pose                                         : XrPosef,
	pub radius                                       : f32,
}

unsafe impl Send for XrHandJointLocationEXT {}
unsafe impl Sync for XrHandJointLocationEXT {}

impl fmt::Debug for XrHandJointLocationEXT {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandJointLocationEXT").finish()
	}
}

impl Default for XrHandJointLocationEXT { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrHandJointVelocityEXT {
	pub velocityFlags                                : XrSpaceVelocityFlags,
	pub linearVelocity                               : XrVector3f,
	pub angularVelocity                              : XrVector3f,
}

unsafe impl Send for XrHandJointVelocityEXT {}
unsafe impl Sync for XrHandJointVelocityEXT {}

impl fmt::Debug for XrHandJointVelocityEXT {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandJointVelocityEXT").finish()
	}
}

impl Default for XrHandJointVelocityEXT { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrHandJointLocationsEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub isActive                                     : XrBool32,
	pub jointCount                                   : u32,
	pub jointLocations                               : *mut XrHandJointLocationEXT,
}

unsafe impl Send for XrHandJointLocationsEXT<'_> {}
unsafe impl Sync for XrHandJointLocationsEXT<'_> {}

impl fmt::Debug for XrHandJointLocationsEXT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandJointLocationsEXT").finish()
	}
}

impl Default for XrHandJointLocationsEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrHandJointVelocitiesEXT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub jointCount                                   : u32,
	pub jointVelocities                              : *mut XrHandJointVelocityEXT,
}

unsafe impl Send for XrHandJointVelocitiesEXT<'_> {}
unsafe impl Sync for XrHandJointVelocitiesEXT<'_> {}

impl fmt::Debug for XrHandJointVelocitiesEXT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandJointVelocitiesEXT").finish()
	}
}

impl Default for XrHandJointVelocitiesEXT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrHandMeshSpaceCreateInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub handPoseType                                 : XrHandPoseTypeMSFT,
	pub poseInHandMeshSpace                          : XrPosef,
}

unsafe impl Send for XrHandMeshSpaceCreateInfoMSFT<'_> {}
unsafe impl Sync for XrHandMeshSpaceCreateInfoMSFT<'_> {}

impl fmt::Debug for XrHandMeshSpaceCreateInfoMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandMeshSpaceCreateInfoMSFT").finish()
	}
}

impl Default for XrHandMeshSpaceCreateInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrHandMeshUpdateInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub time                                         : XrTime,
	pub handPoseType                                 : XrHandPoseTypeMSFT,
}

unsafe impl Send for XrHandMeshUpdateInfoMSFT<'_> {}
unsafe impl Sync for XrHandMeshUpdateInfoMSFT<'_> {}

impl Default for XrHandMeshUpdateInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrHandMeshMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub isActive                                     : XrBool32,
	pub indexBufferChanged                           : XrBool32,
	pub vertexBufferChanged                          : XrBool32,
	pub indexBuffer                                  : XrHandMeshIndexBufferMSFT,
	pub vertexBuffer                                 : XrHandMeshVertexBufferMSFT,
}

unsafe impl Send for XrHandMeshMSFT<'_> {}
unsafe impl Sync for XrHandMeshMSFT<'_> {}

impl fmt::Debug for XrHandMeshMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandMeshMSFT").finish()
	}
}

impl Default for XrHandMeshMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrHandMeshIndexBufferMSFT {
	pub indexBufferKey                               : u32,
	pub indexCapacityInput                           : u32,
	pub indexCountOutput                             : u32,
	pub indices                                      : *mut u32,
}

unsafe impl Send for XrHandMeshIndexBufferMSFT {}
unsafe impl Sync for XrHandMeshIndexBufferMSFT {}

impl Default for XrHandMeshIndexBufferMSFT { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrHandMeshVertexBufferMSFT {
	pub vertexUpdateTime                             : XrTime,
	pub vertexCapacityInput                          : u32,
	pub vertexCountOutput                            : u32,
	pub vertices                                     : *mut XrHandMeshVertexMSFT,
}

unsafe impl Send for XrHandMeshVertexBufferMSFT {}
unsafe impl Sync for XrHandMeshVertexBufferMSFT {}

impl fmt::Debug for XrHandMeshVertexBufferMSFT {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandMeshVertexBufferMSFT").finish()
	}
}

impl Default for XrHandMeshVertexBufferMSFT { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrHandMeshVertexMSFT {
	pub position                                     : XrVector3f,
	pub normal                                       : XrVector3f,
}

unsafe impl Send for XrHandMeshVertexMSFT {}
unsafe impl Sync for XrHandMeshVertexMSFT {}

impl fmt::Debug for XrHandMeshVertexMSFT {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHandMeshVertexMSFT").finish()
	}
}

impl Default for XrHandMeshVertexMSFT { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSystemHandTrackingMeshPropertiesMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub supportsHandTrackingMesh                     : XrBool32,
	pub maxHandMeshIndexCount                        : u32,
	pub maxHandMeshVertexCount                       : u32,
}

unsafe impl Send for XrSystemHandTrackingMeshPropertiesMSFT<'_> {}
unsafe impl Sync for XrSystemHandTrackingMeshPropertiesMSFT<'_> {}

impl Default for XrSystemHandTrackingMeshPropertiesMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrHandPoseTypeInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub handPoseType                                 : XrHandPoseTypeMSFT,
}

unsafe impl Send for XrHandPoseTypeInfoMSFT<'_> {}
unsafe impl Sync for XrHandPoseTypeInfoMSFT<'_> {}

impl Default for XrHandPoseTypeInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSecondaryViewConfigurationSessionBeginInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub viewConfigurationCount                       : u32,
	pub enabledViewConfigurationTypes                : *const XrViewConfigurationType,
}

unsafe impl Send for XrSecondaryViewConfigurationSessionBeginInfoMSFT<'_> {}
unsafe impl Sync for XrSecondaryViewConfigurationSessionBeginInfoMSFT<'_> {}

impl Default for XrSecondaryViewConfigurationSessionBeginInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSecondaryViewConfigurationStateMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub viewConfigurationType                        : XrViewConfigurationType,
	pub active                                       : XrBool32,
}

unsafe impl Send for XrSecondaryViewConfigurationStateMSFT<'_> {}
unsafe impl Sync for XrSecondaryViewConfigurationStateMSFT<'_> {}

impl Default for XrSecondaryViewConfigurationStateMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrSecondaryViewConfigurationFrameStateMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub viewConfigurationCount                       : u32,
	pub viewConfigurationStates                      : *mut XrSecondaryViewConfigurationStateMSFT<'a>,
}

unsafe impl Send for XrSecondaryViewConfigurationFrameStateMSFT<'_> {}
unsafe impl Sync for XrSecondaryViewConfigurationFrameStateMSFT<'_> {}

impl fmt::Debug for XrSecondaryViewConfigurationFrameStateMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSecondaryViewConfigurationFrameStateMSFT").finish()
	}
}

impl Default for XrSecondaryViewConfigurationFrameStateMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrSecondaryViewConfigurationFrameEndInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub viewConfigurationCount                       : u32,
	pub viewConfigurationLayersInfo                  : *const XrSecondaryViewConfigurationLayerInfoMSFT<'a>,
}

unsafe impl Send for XrSecondaryViewConfigurationFrameEndInfoMSFT<'_> {}
unsafe impl Sync for XrSecondaryViewConfigurationFrameEndInfoMSFT<'_> {}

impl fmt::Debug for XrSecondaryViewConfigurationFrameEndInfoMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSecondaryViewConfigurationFrameEndInfoMSFT").finish()
	}
}

impl Default for XrSecondaryViewConfigurationFrameEndInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrSecondaryViewConfigurationLayerInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub viewConfigurationType                        : XrViewConfigurationType,
	pub environmentBlendMode                         : XrEnvironmentBlendMode,
	pub layerCount                                   : u32,
	pub layers                                       : *const *const XrCompositionLayerBaseHeader<'a>,
}

unsafe impl Send for XrSecondaryViewConfigurationLayerInfoMSFT<'_> {}
unsafe impl Sync for XrSecondaryViewConfigurationLayerInfoMSFT<'_> {}

impl fmt::Debug for XrSecondaryViewConfigurationLayerInfoMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrSecondaryViewConfigurationLayerInfoMSFT").finish()
	}
}

impl Default for XrSecondaryViewConfigurationLayerInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrSecondaryViewConfigurationSwapchainCreateInfoMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub viewConfigurationType                        : XrViewConfigurationType,
}

unsafe impl Send for XrSecondaryViewConfigurationSwapchainCreateInfoMSFT<'_> {}
unsafe impl Sync for XrSecondaryViewConfigurationSwapchainCreateInfoMSFT<'_> {}

impl Default for XrSecondaryViewConfigurationSwapchainCreateInfoMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrHolographicWindowAttachmentMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub holographicSpace                             : &'a mut IUnknown,
	pub coreWindow                                   : &'a mut IUnknown,
}

unsafe impl Send for XrHolographicWindowAttachmentMSFT<'_> {}
unsafe impl Sync for XrHolographicWindowAttachmentMSFT<'_> {}

impl fmt::Debug for XrHolographicWindowAttachmentMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrHolographicWindowAttachmentMSFT").finish()
	}
}

impl Default for XrHolographicWindowAttachmentMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrAndroidSurfaceSwapchainCreateInfoFB<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub createFlags                                  : XrAndroidSurfaceSwapchainFlagsFB,
}

unsafe impl Send for XrAndroidSurfaceSwapchainCreateInfoFB<'_> {}
unsafe impl Sync for XrAndroidSurfaceSwapchainCreateInfoFB<'_> {}

impl Default for XrAndroidSurfaceSwapchainCreateInfoFB<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrLoaderInitInfoBaseHeaderKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
}

unsafe impl Send for XrLoaderInitInfoBaseHeaderKHR<'_> {}
unsafe impl Sync for XrLoaderInitInfoBaseHeaderKHR<'_> {}

impl Default for XrLoaderInitInfoBaseHeaderKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrLoaderInitInfoAndroidKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub applicationVM                                : XrAnyMut<'a>,
	pub applicationContext                           : XrAnyMut<'a>,
}

unsafe impl Send for XrLoaderInitInfoAndroidKHR<'_> {}
unsafe impl Sync for XrLoaderInitInfoAndroidKHR<'_> {}

impl Default for XrLoaderInitInfoAndroidKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerEquirect2KHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub layerFlags                                   : XrCompositionLayerFlags,
	pub space                                        : XrSpace,
	pub eyeVisibility                                : XrEyeVisibility,
	pub subImage                                     : XrSwapchainSubImage,
	pub pose                                         : XrPosef,
	pub radius                                       : f32,
	pub centralHorizontalAngle                       : f32,
	pub upperVerticalAngle                           : f32,
	pub lowerVerticalAngle                           : f32,
}

unsafe impl Send for XrCompositionLayerEquirect2KHR<'_> {}
unsafe impl Sync for XrCompositionLayerEquirect2KHR<'_> {}

impl fmt::Debug for XrCompositionLayerEquirect2KHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerEquirect2KHR").finish()
	}
}

impl Default for XrCompositionLayerEquirect2KHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrCompositionLayerColorScaleBiasKHR<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyRef<'a>,
	pub colorScale                                   : XrColor4f,
	pub colorBias                                    : XrColor4f,
}

unsafe impl Send for XrCompositionLayerColorScaleBiasKHR<'_> {}
unsafe impl Sync for XrCompositionLayerColorScaleBiasKHR<'_> {}

impl fmt::Debug for XrCompositionLayerColorScaleBiasKHR<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrCompositionLayerColorScaleBiasKHR").finish()
	}
}

impl Default for XrCompositionLayerColorScaleBiasKHR<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrControllerModelKeyStateMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub modelKey                                     : XrControllerModelKeyMSFT,
}

unsafe impl Send for XrControllerModelKeyStateMSFT<'_> {}
unsafe impl Sync for XrControllerModelKeyStateMSFT<'_> {}

impl Default for XrControllerModelKeyStateMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrControllerModelNodePropertiesMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub parentNodeName                               : [u8;  XR_MAX_CONTROLLER_MODEL_NODE_NAME_SIZE_MSFT as usize],
	pub nodeName                                     : [u8;  XR_MAX_CONTROLLER_MODEL_NODE_NAME_SIZE_MSFT as usize],
}

unsafe impl Send for XrControllerModelNodePropertiesMSFT<'_> {}
unsafe impl Sync for XrControllerModelNodePropertiesMSFT<'_> {}

impl fmt::Debug for XrControllerModelNodePropertiesMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrControllerModelNodePropertiesMSFT").finish()
	}
}

impl Default for XrControllerModelNodePropertiesMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrControllerModelPropertiesMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub nodeCapacityInput                            : u32,
	pub nodeCountOutput                              : u32,
	pub nodeProperties                               : *mut XrControllerModelNodePropertiesMSFT<'a>,
}

unsafe impl Send for XrControllerModelPropertiesMSFT<'_> {}
unsafe impl Sync for XrControllerModelPropertiesMSFT<'_> {}

impl fmt::Debug for XrControllerModelPropertiesMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrControllerModelPropertiesMSFT").finish()
	}
}

impl Default for XrControllerModelPropertiesMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrControllerModelNodeStateMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub nodePose                                     : XrPosef,
}

unsafe impl Send for XrControllerModelNodeStateMSFT<'_> {}
unsafe impl Sync for XrControllerModelNodeStateMSFT<'_> {}

impl fmt::Debug for XrControllerModelNodeStateMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrControllerModelNodeStateMSFT").finish()
	}
}

impl Default for XrControllerModelNodeStateMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive()]
pub struct XrControllerModelStateMSFT<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub nodeCapacityInput                            : u32,
	pub nodeCountOutput                              : u32,
	pub nodeStates                                   : *mut XrControllerModelNodeStateMSFT<'a>,
}

unsafe impl Send for XrControllerModelStateMSFT<'_> {}
unsafe impl Sync for XrControllerModelStateMSFT<'_> {}

impl fmt::Debug for XrControllerModelStateMSFT<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrControllerModelStateMSFT").finish()
	}
}

impl Default for XrControllerModelStateMSFT<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct XrSystemColorSpacePropertiesFB<'a> {
	pub r#type                                       : XrStructureType,
	pub next                                         : XrAnyMut<'a>,
	pub colorSpace                                   : XrColorSpaceFB,
}

unsafe impl Send for XrSystemColorSpacePropertiesFB<'_> {}
unsafe impl Sync for XrSystemColorSpacePropertiesFB<'_> {}

impl Default for XrSystemColorSpacePropertiesFB<'_> { 
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

/// Misc. hardcoded constants - not an enumerated type

pub const XR_TRUE                                           : u32 = 1u32;
pub const XR_FALSE                                          : u32 = 0u32;
pub const XR_MAX_EXTENSION_NAME_SIZE                        : i32 = 128;
pub const XR_MAX_API_LAYER_NAME_SIZE                        : i32 = 256;
pub const XR_MAX_API_LAYER_DESCRIPTION_SIZE                 : i32 = 256;
pub const XR_MAX_SYSTEM_NAME_SIZE                           : i32 = 256;
pub const XR_MAX_APPLICATION_NAME_SIZE                      : i32 = 128;
pub const XR_MAX_ENGINE_NAME_SIZE                           : i32 = 128;
pub const XR_MAX_RUNTIME_NAME_SIZE                          : i32 = 128;
pub const XR_MAX_PATH_LENGTH                                : i32 = 256;
pub const XR_MAX_STRUCTURE_NAME_SIZE                        : i32 = 64;
pub const XR_MAX_RESULT_STRING_SIZE                         : i32 = 64;
pub const XR_MAX_GRAPHICS_APIS_SUPPORTED                    : i32 = 32;
pub const XR_MAX_ACTION_SET_NAME_SIZE                       : i32 = 64;
pub const XR_MAX_ACTION_NAME_SIZE                           : i32 = 64;
pub const XR_MAX_LOCALIZED_ACTION_SET_NAME_SIZE             : i32 = 128;
pub const XR_MAX_LOCALIZED_ACTION_NAME_SIZE                 : i32 = 128;
pub const XR_MIN_COMPOSITION_LAYERS_SUPPORTED               : i32 = 16;

/// Structure type enumerant
pub use self::XrStructureType::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrStructureType {
	XR_TYPE_UNKNOWN                                                                  = 0,
	XR_TYPE_API_LAYER_PROPERTIES                                                     = 1,
	XR_TYPE_EXTENSION_PROPERTIES                                                     = 2,
	XR_TYPE_INSTANCE_CREATE_INFO                                                     = 3,
	XR_TYPE_SYSTEM_GET_INFO                                                          = 4,
	XR_TYPE_SYSTEM_PROPERTIES                                                        = 5,
	XR_TYPE_VIEW_LOCATE_INFO                                                         = 6,
	XR_TYPE_VIEW                                                                     = 7,
	XR_TYPE_SESSION_CREATE_INFO                                                      = 8,
	XR_TYPE_SWAPCHAIN_CREATE_INFO                                                    = 9,
	XR_TYPE_SESSION_BEGIN_INFO                                                       = 10,
	XR_TYPE_VIEW_STATE                                                               = 11,
	XR_TYPE_FRAME_END_INFO                                                           = 12,
	XR_TYPE_HAPTIC_VIBRATION                                                         = 13,
	XR_TYPE_EVENT_DATA_BUFFER                                                        = 16,
	XR_TYPE_EVENT_DATA_INSTANCE_LOSS_PENDING                                         = 17,
	XR_TYPE_EVENT_DATA_SESSION_STATE_CHANGED                                         = 18,
	XR_TYPE_ACTION_STATE_BOOLEAN                                                     = 23,
	XR_TYPE_ACTION_STATE_FLOAT                                                       = 24,
	XR_TYPE_ACTION_STATE_VECTOR2F                                                    = 25,
	XR_TYPE_ACTION_STATE_POSE                                                        = 27,
	XR_TYPE_ACTION_SET_CREATE_INFO                                                   = 28,
	XR_TYPE_ACTION_CREATE_INFO                                                       = 29,
	XR_TYPE_INSTANCE_PROPERTIES                                                      = 32,
	XR_TYPE_FRAME_WAIT_INFO                                                          = 33,
	XR_TYPE_COMPOSITION_LAYER_PROJECTION                                             = 35,
	XR_TYPE_COMPOSITION_LAYER_QUAD                                                   = 36,
	XR_TYPE_REFERENCE_SPACE_CREATE_INFO                                              = 37,
	XR_TYPE_ACTION_SPACE_CREATE_INFO                                                 = 38,
	XR_TYPE_EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING                                = 40,
	XR_TYPE_VIEW_CONFIGURATION_VIEW                                                  = 41,
	XR_TYPE_SPACE_LOCATION                                                           = 42,
	XR_TYPE_SPACE_VELOCITY                                                           = 43,
	XR_TYPE_FRAME_STATE                                                              = 44,
	XR_TYPE_VIEW_CONFIGURATION_PROPERTIES                                            = 45,
	XR_TYPE_FRAME_BEGIN_INFO                                                         = 46,
	XR_TYPE_COMPOSITION_LAYER_PROJECTION_VIEW                                        = 48,
	XR_TYPE_EVENT_DATA_EVENTS_LOST                                                   = 49,
	XR_TYPE_INTERACTION_PROFILE_SUGGESTED_BINDING                                    = 51,
	XR_TYPE_EVENT_DATA_INTERACTION_PROFILE_CHANGED                                   = 52,
	XR_TYPE_INTERACTION_PROFILE_STATE                                                = 53,
	XR_TYPE_SWAPCHAIN_IMAGE_ACQUIRE_INFO                                             = 55,
	XR_TYPE_SWAPCHAIN_IMAGE_WAIT_INFO                                                = 56,
	XR_TYPE_SWAPCHAIN_IMAGE_RELEASE_INFO                                             = 57,
	XR_TYPE_ACTION_STATE_GET_INFO                                                    = 58,
	XR_TYPE_HAPTIC_ACTION_INFO                                                       = 59,
	XR_TYPE_SESSION_ACTION_SETS_ATTACH_INFO                                          = 60,
	XR_TYPE_ACTIONS_SYNC_INFO                                                        = 61,
	XR_TYPE_BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO                                  = 62,
	XR_TYPE_INPUT_SOURCE_LOCALIZED_NAME_GET_INFO                                     = 63,
	XR_TYPE_COMPOSITION_LAYER_CUBE_KHR                                               = 1000006000,
	XR_TYPE_INSTANCE_CREATE_INFO_ANDROID_KHR                                         = 1000008000,
	XR_TYPE_COMPOSITION_LAYER_DEPTH_INFO_KHR                                         = 1000010000,
	XR_TYPE_VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR                             = 1000014000,
	XR_TYPE_EVENT_DATA_PERF_SETTINGS_EXT                                             = 1000015000,
	XR_TYPE_COMPOSITION_LAYER_CYLINDER_KHR                                           = 1000017000,
	XR_TYPE_COMPOSITION_LAYER_EQUIRECT_KHR                                           = 1000018000,
	XR_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT                                         = 1000019000,
	XR_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT                                  = 1000019001,
	XR_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT                                    = 1000019002,
	XR_TYPE_DEBUG_UTILS_LABEL_EXT                                                    = 1000019003,
	XR_TYPE_GRAPHICS_BINDING_OPENGL_WIN32_KHR                                        = 1000023000,
	XR_TYPE_GRAPHICS_BINDING_OPENGL_XLIB_KHR                                         = 1000023001,
	XR_TYPE_GRAPHICS_BINDING_OPENGL_XCB_KHR                                          = 1000023002,
	XR_TYPE_GRAPHICS_BINDING_OPENGL_WAYLAND_KHR                                      = 1000023003,
	XR_TYPE_SWAPCHAIN_IMAGE_OPENGL_KHR                                               = 1000023004,
	XR_TYPE_GRAPHICS_REQUIREMENTS_OPENGL_KHR                                         = 1000023005,
	XR_TYPE_GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR                                   = 1000024001,
	XR_TYPE_SWAPCHAIN_IMAGE_OPENGL_ES_KHR                                            = 1000024002,
	XR_TYPE_GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR                                      = 1000024003,
	XR_TYPE_GRAPHICS_BINDING_VULKAN_KHR                                              = 1000025000,
	XR_TYPE_SWAPCHAIN_IMAGE_VULKAN_KHR                                               = 1000025001,
	XR_TYPE_GRAPHICS_REQUIREMENTS_VULKAN_KHR                                         = 1000025002,
	XR_TYPE_GRAPHICS_BINDING_D3D11_KHR                                               = 1000027000,
	XR_TYPE_SWAPCHAIN_IMAGE_D3D11_KHR                                                = 1000027001,
	XR_TYPE_GRAPHICS_REQUIREMENTS_D3D11_KHR                                          = 1000027002,
	XR_TYPE_GRAPHICS_BINDING_D3D12_KHR                                               = 1000028000,
	XR_TYPE_SWAPCHAIN_IMAGE_D3D12_KHR                                                = 1000028001,
	XR_TYPE_GRAPHICS_REQUIREMENTS_D3D12_KHR                                          = 1000028002,
	XR_TYPE_SYSTEM_EYE_GAZE_INTERACTION_PROPERTIES_EXT                               = 1000030000,
	XR_TYPE_EYE_GAZE_SAMPLE_TIME_EXT                                                 = 1000030001,
	XR_TYPE_VISIBILITY_MASK_KHR                                                      = 1000031000,
	XR_TYPE_EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR                                   = 1000031001,
	XR_TYPE_SESSION_CREATE_INFO_OVERLAY_EXTX                                         = 1000033000,
	XR_TYPE_EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX                          = 1000033003,
	XR_TYPE_COMPOSITION_LAYER_COLOR_SCALE_BIAS_KHR                                   = 1000034000,
	XR_TYPE_SPATIAL_ANCHOR_CREATE_INFO_MSFT                                          = 1000039000,
	XR_TYPE_SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT                                    = 1000039001,
	XR_TYPE_VIEW_CONFIGURATION_DEPTH_RANGE_EXT                                       = 1000046000,
	XR_TYPE_GRAPHICS_BINDING_EGL_MNDX                                                = 1000048004,
	XR_TYPE_SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT                                = 1000049000,
	XR_TYPE_SYSTEM_HAND_TRACKING_PROPERTIES_EXT                                      = 1000051000,
	XR_TYPE_HAND_TRACKER_CREATE_INFO_EXT                                             = 1000051001,
	XR_TYPE_HAND_JOINTS_LOCATE_INFO_EXT                                              = 1000051002,
	XR_TYPE_HAND_JOINT_LOCATIONS_EXT                                                 = 1000051003,
	XR_TYPE_HAND_JOINT_VELOCITIES_EXT                                                = 1000051004,
	XR_TYPE_SYSTEM_HAND_TRACKING_MESH_PROPERTIES_MSFT                                = 1000052000,
	XR_TYPE_HAND_MESH_SPACE_CREATE_INFO_MSFT                                         = 1000052001,
	XR_TYPE_HAND_MESH_UPDATE_INFO_MSFT                                               = 1000052002,
	XR_TYPE_HAND_MESH_MSFT                                                           = 1000052003,
	XR_TYPE_HAND_POSE_TYPE_INFO_MSFT                                                 = 1000052004,
	XR_TYPE_SECONDARY_VIEW_CONFIGURATION_SESSION_BEGIN_INFO_MSFT                     = 1000053000,
	XR_TYPE_SECONDARY_VIEW_CONFIGURATION_STATE_MSFT                                  = 1000053001,
	XR_TYPE_SECONDARY_VIEW_CONFIGURATION_FRAME_STATE_MSFT                            = 1000053002,
	XR_TYPE_SECONDARY_VIEW_CONFIGURATION_FRAME_END_INFO_MSFT                         = 1000053003,
	XR_TYPE_SECONDARY_VIEW_CONFIGURATION_LAYER_INFO_MSFT                             = 1000053004,
	XR_TYPE_SECONDARY_VIEW_CONFIGURATION_SWAPCHAIN_CREATE_INFO_MSFT                  = 1000053005,
	XR_TYPE_CONTROLLER_MODEL_KEY_STATE_MSFT                                          = 1000055000,
	XR_TYPE_CONTROLLER_MODEL_NODE_PROPERTIES_MSFT                                    = 1000055001,
	XR_TYPE_CONTROLLER_MODEL_PROPERTIES_MSFT                                         = 1000055002,
	XR_TYPE_CONTROLLER_MODEL_NODE_STATE_MSFT                                         = 1000055003,
	XR_TYPE_CONTROLLER_MODEL_STATE_MSFT                                              = 1000055004,
	XR_TYPE_VIEW_CONFIGURATION_VIEW_FOV_EPIC                                         = 1000059000,
	XR_TYPE_HOLOGRAPHIC_WINDOW_ATTACHMENT_MSFT                                       = 1000063000,
	XR_TYPE_ANDROID_SURFACE_SWAPCHAIN_CREATE_INFO_FB                                 = 1000070000,
	XR_TYPE_INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE                               = 1000079000,
	XR_TYPE_LOADER_INIT_INFO_ANDROID_KHR                                             = 1000089000,
	XR_TYPE_VULKAN_INSTANCE_CREATE_INFO_KHR                                          = 1000090000,
	XR_TYPE_VULKAN_DEVICE_CREATE_INFO_KHR                                            = 1000090001,
	XR_TYPE_VULKAN_GRAPHICS_DEVICE_GET_INFO_KHR                                      = 1000090003,
	XR_TYPE_COMPOSITION_LAYER_EQUIRECT2_KHR                                          = 1000091000,
	XR_TYPE_EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB                               = 1000101000,
	XR_TYPE_SYSTEM_COLOR_SPACE_PROPERTIES_FB                                         = 1000108000,
	XR_TYPE_BINDING_MODIFICATIONS_KHR                                                = 1000120000,
}

impl Default for XrStructureType {
	fn default() -> Self {
		Self::XR_TYPE_UNKNOWN
	}
}

pub const XR_TYPE_GRAPHICS_BINDING_VULKAN2_KHR                                            : XrStructureType = XR_TYPE_GRAPHICS_BINDING_VULKAN_KHR;
pub const XR_TYPE_SWAPCHAIN_IMAGE_VULKAN2_KHR                                             : XrStructureType = XR_TYPE_SWAPCHAIN_IMAGE_VULKAN_KHR;
pub const XR_TYPE_GRAPHICS_REQUIREMENTS_VULKAN2_KHR                                       : XrStructureType = XR_TYPE_GRAPHICS_REQUIREMENTS_VULKAN_KHR;

/// Error and return codes
pub use self::XrResult::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrResult {
	 /// Function successfully completed.
	XR_SUCCESS                                                                       = 0,
	 /// The specified timeout time occurred before the operation could complete.
	XR_TIMEOUT_EXPIRED                                                               = 1,
	 /// The session will be lost soon.
	XR_SESSION_LOSS_PENDING                                                          = 3,
	 /// No event was available.
	XR_EVENT_UNAVAILABLE                                                             = 4,
	 /// The space's bounds are not known at the moment.
	XR_SPACE_BOUNDS_UNAVAILABLE                                                      = 7,
	 /// The session is not in the focused state.
	XR_SESSION_NOT_FOCUSED                                                           = 8,
	 /// A frame has been discarded from composition.
	XR_FRAME_DISCARDED                                                               = 9,
	 /// The function usage was invalid in some way.
	XR_ERROR_VALIDATION_FAILURE                                                      = -1,
	 /// The runtime failed to handle the function in an unexpected way that is not covered by another error result.
	XR_ERROR_RUNTIME_FAILURE                                                         = -2,
	 /// A memory allocation has failed.
	XR_ERROR_OUT_OF_MEMORY                                                           = -3,
	 /// The runtime does not support the requested API version.
	XR_ERROR_API_VERSION_UNSUPPORTED                                                 = -4,
	 /// Initialization of object could not be completed.
	XR_ERROR_INITIALIZATION_FAILED                                                   = -6,
	 /// The requested function was not found or is otherwise unsupported.
	XR_ERROR_FUNCTION_UNSUPPORTED                                                    = -7,
	 /// The requested feature is not supported.
	XR_ERROR_FEATURE_UNSUPPORTED                                                     = -8,
	 /// A requested extension is not supported.
	XR_ERROR_EXTENSION_NOT_PRESENT                                                   = -9,
	 /// The runtime supports no more of the requested resource.
	XR_ERROR_LIMIT_REACHED                                                           = -10,
	 /// The supplied size was smaller than required.
	XR_ERROR_SIZE_INSUFFICIENT                                                       = -11,
	 /// A supplied object handle was invalid.
	XR_ERROR_HANDLE_INVALID                                                          = -12,
	 /// The slink:XrInstance was lost or could not be found. It will need to be destroyed and optionally recreated.
	XR_ERROR_INSTANCE_LOST                                                           = -13,
	 /// The session <<session_running, is already running>>.
	XR_ERROR_SESSION_RUNNING                                                         = -14,
	 /// The session <<session_not_running, is not yet running>>.
	XR_ERROR_SESSION_NOT_RUNNING                                                     = -16,
	 /// The slink:XrSession was lost. It will need to be destroyed and optionally recreated.
	XR_ERROR_SESSION_LOST                                                            = -17,
	 /// The provided basetype:XrSystemId was invalid.
	XR_ERROR_SYSTEM_INVALID                                                          = -18,
	 /// The provided basetype:XrPath was not valid.
	XR_ERROR_PATH_INVALID                                                            = -19,
	 /// The maximum number of supported semantic paths has been reached.
	XR_ERROR_PATH_COUNT_EXCEEDED                                                     = -20,
	 /// The semantic path character format is invalid.
	XR_ERROR_PATH_FORMAT_INVALID                                                     = -21,
	 /// The semantic path is unsupported.
	XR_ERROR_PATH_UNSUPPORTED                                                        = -22,
	 /// The layer was NULL or otherwise invalid.
	XR_ERROR_LAYER_INVALID                                                           = -23,
	 /// The number of specified layers is greater than the supported number.
	XR_ERROR_LAYER_LIMIT_EXCEEDED                                                    = -24,
	 /// The image rect was negatively sized or otherwise invalid.
	XR_ERROR_SWAPCHAIN_RECT_INVALID                                                  = -25,
	 /// The image format is not supported by the runtime or platform.
	XR_ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED                                            = -26,
	 /// The API used to retrieve an action's state does not match the action's type.
	XR_ERROR_ACTION_TYPE_MISMATCH                                                    = -27,
	 /// The session is not in the ready state.
	XR_ERROR_SESSION_NOT_READY                                                       = -28,
	 /// The session is not in the stopping state.
	XR_ERROR_SESSION_NOT_STOPPING                                                    = -29,
	 /// The provided basetype:XrTime was zero, negative, or out of range.
	XR_ERROR_TIME_INVALID                                                            = -30,
	 /// The specified reference space is not supported by the runtime or system.
	XR_ERROR_REFERENCE_SPACE_UNSUPPORTED                                             = -31,
	 /// The file could not be accessed.
	XR_ERROR_FILE_ACCESS_ERROR                                                       = -32,
	 /// The file's contents were invalid.
	XR_ERROR_FILE_CONTENTS_INVALID                                                   = -33,
	 /// The specified form factor is not supported by the current runtime or platform.
	XR_ERROR_FORM_FACTOR_UNSUPPORTED                                                 = -34,
	 /// The specified form factor is supported, but the device is currently not available, e.g. not plugged in or powered off.
	XR_ERROR_FORM_FACTOR_UNAVAILABLE                                                 = -35,
	 /// A requested API layer is not present or could not be loaded.
	XR_ERROR_API_LAYER_NOT_PRESENT                                                   = -36,
	 /// The call was made without having made a previously required call.
	XR_ERROR_CALL_ORDER_INVALID                                                      = -37,
	 /// The given graphics device is not in a valid state. The graphics device could be lost or initialized without meeting graphics requirements.
	XR_ERROR_GRAPHICS_DEVICE_INVALID                                                 = -38,
	 /// The supplied pose was invalid with respect to the requirements.
	XR_ERROR_POSE_INVALID                                                            = -39,
	 /// The supplied index was outside the range of valid indices.
	XR_ERROR_INDEX_OUT_OF_RANGE                                                      = -40,
	 /// The specified view configuration type is not supported by the runtime or platform.
	XR_ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED                                     = -41,
	 /// The specified environment blend mode is not supported by the runtime or platform.
	XR_ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED                                      = -42,
	 /// The name provided was a duplicate of an already-existing resource.
	XR_ERROR_NAME_DUPLICATED                                                         = -44,
	 /// The name provided was invalid.
	XR_ERROR_NAME_INVALID                                                            = -45,
	 /// A referenced action set is not attached to the session.
	XR_ERROR_ACTIONSET_NOT_ATTACHED                                                  = -46,
	 /// The session already has attached action sets.
	XR_ERROR_ACTIONSETS_ALREADY_ATTACHED                                             = -47,
	 /// The localized name provided was a duplicate of an already-existing resource.
	XR_ERROR_LOCALIZED_NAME_DUPLICATED                                               = -48,
	 /// The localized name provided was invalid.
	XR_ERROR_LOCALIZED_NAME_INVALID                                                  = -49,
	 /// The fname:xrGetGraphicsRequirements* call was not made before calling fname:xrCreateSession.
	XR_ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING                                      = -50,
	XR_ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR                                  = -1000003000,
	XR_ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR                                     = -1000003001,
	XR_ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT                                       = -1000039001,
	XR_ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT                      = -1000053000,
	XR_ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT                                       = -1000055000,
	XR_ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB                                     = -1000101000,
	XR_ERROR_COLOR_SPACE_UNSUPPORTED_FB                                              = -1000108000,
}

impl Default for XrResult {
	fn default() -> Self {
		Self::XR_SUCCESS
	}
}

/// Enums to track objects of various types
pub use self::XrObjectType::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrObjectType {
	XR_OBJECT_TYPE_UNKNOWN                                                           = 0,
	 /// XrInstance
	XR_OBJECT_TYPE_INSTANCE                                                          = 1,
	 /// XrSession
	XR_OBJECT_TYPE_SESSION                                                           = 2,
	 /// XrSwapchain
	XR_OBJECT_TYPE_SWAPCHAIN                                                         = 3,
	 /// XrSpace
	XR_OBJECT_TYPE_SPACE                                                             = 4,
	 /// XrActionSet
	XR_OBJECT_TYPE_ACTION_SET                                                        = 5,
	 /// XrAction
	XR_OBJECT_TYPE_ACTION                                                            = 6,
	XR_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT                                         = 1000019000,
	XR_OBJECT_TYPE_SPATIAL_ANCHOR_MSFT                                               = 1000039000,
	XR_OBJECT_TYPE_HAND_TRACKER_EXT                                                  = 1000051000,
}

impl Default for XrObjectType {
	fn default() -> Self {
		Self::XR_OBJECT_TYPE_UNKNOWN
	}
}

/// Android Thread Types
pub use self::XrAndroidThreadTypeKHR::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrAndroidThreadTypeKHR {
	XR_ANDROID_THREAD_TYPE_APPLICATION_MAIN_KHR                                      = 1,
	XR_ANDROID_THREAD_TYPE_APPLICATION_WORKER_KHR                                    = 2,
	XR_ANDROID_THREAD_TYPE_RENDERER_MAIN_KHR                                         = 3,
	XR_ANDROID_THREAD_TYPE_RENDERER_WORKER_KHR                                       = 4,
}

impl Default for XrAndroidThreadTypeKHR {
	fn default() -> Self {
		Self::XR_ANDROID_THREAD_TYPE_APPLICATION_MAIN_KHR
	}
}

/// eye visibility selector
pub use self::XrEyeVisibility::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrEyeVisibility {
	 /// Display in both eyes.
	XR_EYE_VISIBILITY_BOTH                                                           = 0,
	 /// Display in the left eye only.
	XR_EYE_VISIBILITY_LEFT                                                           = 1,
	 /// Display in the right eye only.
	XR_EYE_VISIBILITY_RIGHT                                                          = 2,
}

impl Default for XrEyeVisibility {
	fn default() -> Self {
		Self::XR_EYE_VISIBILITY_BOTH
	}
}

pub use self::XrActionType::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrActionType {
	XR_ACTION_TYPE_BOOLEAN_INPUT                                                     = 1,
	XR_ACTION_TYPE_FLOAT_INPUT                                                       = 2,
	XR_ACTION_TYPE_VECTOR2F_INPUT                                                    = 3,
	XR_ACTION_TYPE_POSE_INPUT                                                        = 4,
	XR_ACTION_TYPE_VIBRATION_OUTPUT                                                  = 100,
}

impl Default for XrActionType {
	fn default() -> Self {
		Self::XR_ACTION_TYPE_BOOLEAN_INPUT
	}
}

pub use self::XrReferenceSpaceType::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrReferenceSpaceType {
	XR_REFERENCE_SPACE_TYPE_VIEW                                                     = 1,
	XR_REFERENCE_SPACE_TYPE_LOCAL                                                    = 2,
	XR_REFERENCE_SPACE_TYPE_STAGE                                                    = 3,
	XR_REFERENCE_SPACE_TYPE_UNBOUNDED_MSFT                                           = 1000038000,
}

impl Default for XrReferenceSpaceType {
	fn default() -> Self {
		Self::XR_REFERENCE_SPACE_TYPE_VIEW
	}
}

pub use self::XrFormFactor::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrFormFactor {
	XR_FORM_FACTOR_HEAD_MOUNTED_DISPLAY                                              = 1,
	XR_FORM_FACTOR_HANDHELD_DISPLAY                                                  = 2,
}

impl Default for XrFormFactor {
	fn default() -> Self {
		Self::XR_FORM_FACTOR_HEAD_MOUNTED_DISPLAY
	}
}

pub use self::XrViewConfigurationType::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrViewConfigurationType {
	XR_VIEW_CONFIGURATION_TYPE_PRIMARY_MONO                                          = 1,
	XR_VIEW_CONFIGURATION_TYPE_PRIMARY_STEREO                                        = 2,
	XR_VIEW_CONFIGURATION_TYPE_PRIMARY_QUAD_VARJO                                    = 1000037000,
	XR_VIEW_CONFIGURATION_TYPE_SECONDARY_MONO_FIRST_PERSON_OBSERVER_MSFT             = 1000054000,
}

impl Default for XrViewConfigurationType {
	fn default() -> Self {
		Self::XR_VIEW_CONFIGURATION_TYPE_PRIMARY_MONO
	}
}

pub use self::XrEnvironmentBlendMode::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrEnvironmentBlendMode {
	XR_ENVIRONMENT_BLEND_MODE_OPAQUE                                                 = 1,
	XR_ENVIRONMENT_BLEND_MODE_ADDITIVE                                               = 2,
	XR_ENVIRONMENT_BLEND_MODE_ALPHA_BLEND                                            = 3,
}

impl Default for XrEnvironmentBlendMode {
	fn default() -> Self {
		Self::XR_ENVIRONMENT_BLEND_MODE_OPAQUE
	}
}

pub use self::XrSessionState::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrSessionState {
	XR_SESSION_STATE_UNKNOWN                                                         = 0,
	XR_SESSION_STATE_IDLE                                                            = 1,
	XR_SESSION_STATE_READY                                                           = 2,
	XR_SESSION_STATE_SYNCHRONIZED                                                    = 3,
	XR_SESSION_STATE_VISIBLE                                                         = 4,
	XR_SESSION_STATE_FOCUSED                                                         = 5,
	XR_SESSION_STATE_STOPPING                                                        = 6,
	XR_SESSION_STATE_LOSS_PENDING                                                    = 7,
	XR_SESSION_STATE_EXITING                                                         = 8,
}

impl Default for XrSessionState {
	fn default() -> Self {
		Self::XR_SESSION_STATE_UNKNOWN
	}
}

pub use self::XrPerfSettingsLevelEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrPerfSettingsLevelEXT {
	 /// Performance settings hint used by the application to indicate that it enters a non-XR
	 /// section (head-locked / static screen), during which power savings are to be prioritized
	XR_PERF_SETTINGS_LEVEL_POWER_SAVINGS_EXT                                         = 0,
	 /// Performance settings hint used by the application to indicate that it enters a low
	 /// and stable complexity section, during which reducing power is more important than
	 /// occasional late rendering frames
	XR_PERF_SETTINGS_LEVEL_SUSTAINED_LOW_EXT                                         = 25,
	 /// Performance settings hint used by the application to indicate that it enters
	 /// a high or dynamic complexity section, during which the XR Runtime strives for consistent
	 /// XR compositing and frame rendering within a thermally sustainable range
	XR_PERF_SETTINGS_LEVEL_SUSTAINED_HIGH_EXT                                        = 50,
	 /// Performance settings hint used by the application to indicate that the application enters
	 /// a section with very high complexity, during which the XR Runtime is allowed to step
	 /// up beyond the thermally sustainable range
	XR_PERF_SETTINGS_LEVEL_BOOST_EXT                                                 = 75,
}

impl Default for XrPerfSettingsLevelEXT {
	fn default() -> Self {
		Self::XR_PERF_SETTINGS_LEVEL_POWER_SAVINGS_EXT
	}
}

pub use self::XrPerfSettingsDomainEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrPerfSettingsDomainEXT {
	 /// Indicates that the performance settings or notification applies to CPU domain
	XR_PERF_SETTINGS_DOMAIN_CPU_EXT                                                  = 1,
	 /// Indicates that the performance settings or notification applies to GPU domain
	XR_PERF_SETTINGS_DOMAIN_GPU_EXT                                                  = 2,
}

impl Default for XrPerfSettingsDomainEXT {
	fn default() -> Self {
		Self::XR_PERF_SETTINGS_DOMAIN_CPU_EXT
	}
}

pub use self::XrPerfSettingsSubDomainEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrPerfSettingsSubDomainEXT {
	 /// Indicates that the performance notification originates from the COMPOSITING sub-domain
	XR_PERF_SETTINGS_SUB_DOMAIN_COMPOSITING_EXT                                      = 1,
	 /// Indicates that the performance notification originates from the RENDERING sub-domain
	XR_PERF_SETTINGS_SUB_DOMAIN_RENDERING_EXT                                        = 2,
	 /// Indicates that the performance notification originates from the THERMAL sub-domain
	XR_PERF_SETTINGS_SUB_DOMAIN_THERMAL_EXT                                          = 3,
}

impl Default for XrPerfSettingsSubDomainEXT {
	fn default() -> Self {
		Self::XR_PERF_SETTINGS_SUB_DOMAIN_COMPOSITING_EXT
	}
}

pub use self::XrPerfSettingsNotificationLevelEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrPerfSettingsNotificationLevelEXT {
	 /// Notifies that the sub-domain has reached a level
	 /// where no further actions other than currently applied are necessary
	XR_PERF_SETTINGS_NOTIF_LEVEL_NORMAL_EXT                                          = 0,
	 /// Notifies that the sub-domain has reached an early warning level
	 /// where the application should start proactive mitigation actions
	 /// with the goal to return to the ename:XR_PERF_NOTIF_LEVEL_NORMAL level
	XR_PERF_SETTINGS_NOTIF_LEVEL_WARNING_EXT                                         = 25,
	 /// Notifies that the sub-domain has reached a critical
	 /// level with significant performance degradation.
	 /// The application should take drastic mitigation action
	XR_PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED_EXT                                        = 75,
}

impl Default for XrPerfSettingsNotificationLevelEXT {
	fn default() -> Self {
		Self::XR_PERF_SETTINGS_NOTIF_LEVEL_NORMAL_EXT
	}
}

pub use self::XrVisibilityMaskTypeKHR::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrVisibilityMaskTypeKHR {
	 /// exclusive mesh; indicates that which the viewer cannot see.
	XR_VISIBILITY_MASK_TYPE_HIDDEN_TRIANGLE_MESH_KHR                                 = 1,
	 /// inclusive mesh; indicates strictly that which the viewer can see.
	XR_VISIBILITY_MASK_TYPE_VISIBLE_TRIANGLE_MESH_KHR                                = 2,
	 /// line loop; traces the outline of the area the viewer can see.
	XR_VISIBILITY_MASK_TYPE_LINE_LOOP_KHR                                            = 3,
}

impl Default for XrVisibilityMaskTypeKHR {
	fn default() -> Self {
		Self::XR_VISIBILITY_MASK_TYPE_HIDDEN_TRIANGLE_MESH_KHR
	}
}

pub use self::XrHandEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrHandEXT {
	XR_HAND_LEFT_EXT                                                                 = 1,
	XR_HAND_RIGHT_EXT                                                                = 2,
}

impl Default for XrHandEXT {
	fn default() -> Self {
		Self::XR_HAND_LEFT_EXT
	}
}

pub use self::XrHandJointEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrHandJointEXT {
	XR_HAND_JOINT_PALM_EXT                                                           = 0,
	XR_HAND_JOINT_WRIST_EXT                                                          = 1,
	XR_HAND_JOINT_THUMB_METACARPAL_EXT                                               = 2,
	XR_HAND_JOINT_THUMB_PROXIMAL_EXT                                                 = 3,
	XR_HAND_JOINT_THUMB_DISTAL_EXT                                                   = 4,
	XR_HAND_JOINT_THUMB_TIP_EXT                                                      = 5,
	XR_HAND_JOINT_INDEX_METACARPAL_EXT                                               = 6,
	XR_HAND_JOINT_INDEX_PROXIMAL_EXT                                                 = 7,
	XR_HAND_JOINT_INDEX_INTERMEDIATE_EXT                                             = 8,
	XR_HAND_JOINT_INDEX_DISTAL_EXT                                                   = 9,
	XR_HAND_JOINT_INDEX_TIP_EXT                                                      = 10,
	XR_HAND_JOINT_MIDDLE_METACARPAL_EXT                                              = 11,
	XR_HAND_JOINT_MIDDLE_PROXIMAL_EXT                                                = 12,
	XR_HAND_JOINT_MIDDLE_INTERMEDIATE_EXT                                            = 13,
	XR_HAND_JOINT_MIDDLE_DISTAL_EXT                                                  = 14,
	XR_HAND_JOINT_MIDDLE_TIP_EXT                                                     = 15,
	XR_HAND_JOINT_RING_METACARPAL_EXT                                                = 16,
	XR_HAND_JOINT_RING_PROXIMAL_EXT                                                  = 17,
	XR_HAND_JOINT_RING_INTERMEDIATE_EXT                                              = 18,
	XR_HAND_JOINT_RING_DISTAL_EXT                                                    = 19,
	XR_HAND_JOINT_RING_TIP_EXT                                                       = 20,
	XR_HAND_JOINT_LITTLE_METACARPAL_EXT                                              = 21,
	XR_HAND_JOINT_LITTLE_PROXIMAL_EXT                                                = 22,
	XR_HAND_JOINT_LITTLE_INTERMEDIATE_EXT                                            = 23,
	XR_HAND_JOINT_LITTLE_DISTAL_EXT                                                  = 24,
	XR_HAND_JOINT_LITTLE_TIP_EXT                                                     = 25,
}

impl Default for XrHandJointEXT {
	fn default() -> Self {
		Self::XR_HAND_JOINT_PALM_EXT
	}
}

pub use self::XrHandJointSetEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrHandJointSetEXT {
	XR_HAND_JOINT_SET_DEFAULT_EXT                                                    = 0,
}

impl Default for XrHandJointSetEXT {
	fn default() -> Self {
		Self::XR_HAND_JOINT_SET_DEFAULT_EXT
	}
}

pub use self::XrHandPoseTypeMSFT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrHandPoseTypeMSFT {
	XR_HAND_POSE_TYPE_TRACKED_MSFT                                                   = 0,
	XR_HAND_POSE_TYPE_REFERENCE_OPEN_PALM_MSFT                                       = 1,
}

impl Default for XrHandPoseTypeMSFT {
	fn default() -> Self {
		Self::XR_HAND_POSE_TYPE_TRACKED_MSFT
	}
}

pub use self::XrColorSpaceFB::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrColorSpaceFB {
	XR_COLOR_SPACE_UNMANAGED_FB                                                      = 0,
	XR_COLOR_SPACE_REC2020_FB                                                        = 1,
	XR_COLOR_SPACE_REC709_FB                                                         = 2,
	XR_COLOR_SPACE_RIFT_CV1_FB                                                       = 3,
	XR_COLOR_SPACE_RIFT_S_FB                                                         = 4,
	XR_COLOR_SPACE_QUEST_FB                                                          = 5,
	XR_COLOR_SPACE_P3_FB                                                             = 6,
	XR_COLOR_SPACE_ADOBE_RGB_FB                                                      = 7,
}

impl Default for XrColorSpaceFB {
	fn default() -> Self {
		Self::XR_COLOR_SPACE_UNMANAGED_FB
	}
}

pub use self::XrInstanceCreateFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrInstanceCreateFlagBits {
	__Default__ = 0,
}

impl Default for XrInstanceCreateFlagBits {
	fn default() -> Self {
		Self::__Default__
	}
}

pub use self::XrSessionCreateFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrSessionCreateFlagBits {
	__Default__ = 0,
}

impl Default for XrSessionCreateFlagBits {
	fn default() -> Self {
		Self::__Default__
	}
}

pub use self::XrSwapchainCreateFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrSwapchainCreateFlagBits {
	 /// Content will be protected from CPU access
	XR_SWAPCHAIN_CREATE_PROTECTED_CONTENT_BIT                                        = 0x1,
	 /// Only one image will be acquired from this swapchain over its lifetime
	XR_SWAPCHAIN_CREATE_STATIC_IMAGE_BIT                                             = 0x2,
}

impl Default for XrSwapchainCreateFlagBits {
	fn default() -> Self {
		Self::XR_SWAPCHAIN_CREATE_PROTECTED_CONTENT_BIT
	}
}

pub use self::XrSwapchainUsageFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrSwapchainUsageFlagBits {
	 /// Specifies that the image can: be a color rendering target.
	XR_SWAPCHAIN_USAGE_COLOR_ATTACHMENT_BIT                                          = 0x1,
	 /// Specifies that the image can: be a depth/stencil rendering target.
	XR_SWAPCHAIN_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT                                  = 0x2,
	 /// Specifies that the image can: be used as data/compute.
	XR_SWAPCHAIN_USAGE_UNORDERED_ACCESS_BIT                                          = 0x4,
	 /// Specifies that the image can: be used as the source of a transfer command.
	XR_SWAPCHAIN_USAGE_TRANSFER_SRC_BIT                                              = 0x8,
	 /// Specifies that the image can: be used as the destination of a transfer command.
	XR_SWAPCHAIN_USAGE_TRANSFER_DST_BIT                                              = 0x10,
	 /// Specifies that the image can: be sampled by a shader.
	XR_SWAPCHAIN_USAGE_SAMPLED_BIT                                                   = 0x20,
	 /// Specifies that the image can: be reinterpreted as another image format.
	XR_SWAPCHAIN_USAGE_MUTABLE_FORMAT_BIT                                            = 0x40,
	XR_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_MND                                      = 0x80,
}

impl Default for XrSwapchainUsageFlagBits {
	fn default() -> Self {
		Self::XR_SWAPCHAIN_USAGE_COLOR_ATTACHMENT_BIT
	}
}

pub use self::XrViewStateFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrViewStateFlagBits {
	 /// Indicates validity of all XrView orientations
	XR_VIEW_STATE_ORIENTATION_VALID_BIT                                              = 0x1,
	 /// Indicates validity of all XrView positions
	XR_VIEW_STATE_POSITION_VALID_BIT                                                 = 0x2,
	 /// Indicates whether all XrView orientations are actively tracked
	XR_VIEW_STATE_ORIENTATION_TRACKED_BIT                                            = 0x4,
	 /// Indicates whether all XrView positions are actively tracked
	XR_VIEW_STATE_POSITION_TRACKED_BIT                                               = 0x8,
}

impl Default for XrViewStateFlagBits {
	fn default() -> Self {
		Self::XR_VIEW_STATE_ORIENTATION_VALID_BIT
	}
}

pub use self::XrCompositionLayerFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrCompositionLayerFlagBits {
	 /// Enables chromatic aberration correction when not done by default.
	XR_COMPOSITION_LAYER_CORRECT_CHROMATIC_ABERRATION_BIT                            = 0x1,
	 /// Enables the layer texture alpha channel.
	XR_COMPOSITION_LAYER_BLEND_TEXTURE_SOURCE_ALPHA_BIT                              = 0x2,
	 /// Indicates the texture color channels have not been premultiplied by the texture alpha channel.
	XR_COMPOSITION_LAYER_UNPREMULTIPLIED_ALPHA_BIT                                   = 0x4,
}

impl Default for XrCompositionLayerFlagBits {
	fn default() -> Self {
		Self::XR_COMPOSITION_LAYER_CORRECT_CHROMATIC_ABERRATION_BIT
	}
}

pub use self::XrSpaceLocationFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrSpaceLocationFlagBits {
	 /// Indicates validity of orientation member
	XR_SPACE_LOCATION_ORIENTATION_VALID_BIT                                          = 0x1,
	 /// Indicates validity of position member
	XR_SPACE_LOCATION_POSITION_VALID_BIT                                             = 0x2,
	 /// Indicates whether pose member contains an actively tracked orientation
	XR_SPACE_LOCATION_ORIENTATION_TRACKED_BIT                                        = 0x4,
	 /// Indicates whether pose member contains an actively tracked position
	XR_SPACE_LOCATION_POSITION_TRACKED_BIT                                           = 0x8,
}

impl Default for XrSpaceLocationFlagBits {
	fn default() -> Self {
		Self::XR_SPACE_LOCATION_ORIENTATION_VALID_BIT
	}
}

pub use self::XrSpaceVelocityFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrSpaceVelocityFlagBits {
	 /// Indicates validity of linearVelocity member
	XR_SPACE_VELOCITY_LINEAR_VALID_BIT                                               = 0x1,
	 /// Indicates validity of angularVelocity member
	XR_SPACE_VELOCITY_ANGULAR_VALID_BIT                                              = 0x2,
}

impl Default for XrSpaceVelocityFlagBits {
	fn default() -> Self {
		Self::XR_SPACE_VELOCITY_LINEAR_VALID_BIT
	}
}

pub use self::XrInputSourceLocalizedNameFlagBits::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrInputSourceLocalizedNameFlagBits {
	 /// Asks for the part of the string which indicates the top level user path the source represents
	XR_INPUT_SOURCE_LOCALIZED_NAME_USER_PATH_BIT                                     = 0x1,
	 /// Asks for the part of the string which represents the interaction profile of the source
	XR_INPUT_SOURCE_LOCALIZED_NAME_INTERACTION_PROFILE_BIT                           = 0x2,
	 /// Asks for the part of the string which represents the component on the device which needs to be interacted with
	XR_INPUT_SOURCE_LOCALIZED_NAME_COMPONENT_BIT                                     = 0x4,
}

impl Default for XrInputSourceLocalizedNameFlagBits {
	fn default() -> Self {
		Self::XR_INPUT_SOURCE_LOCALIZED_NAME_USER_PATH_BIT
	}
}

pub use self::XrVulkanInstanceCreateFlagBitsKHR::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrVulkanInstanceCreateFlagBitsKHR {
	__Default__ = 0,
}

impl Default for XrVulkanInstanceCreateFlagBitsKHR {
	fn default() -> Self {
		Self::__Default__
	}
}

pub use self::XrVulkanDeviceCreateFlagBitsKHR::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrVulkanDeviceCreateFlagBitsKHR {
	__Default__ = 0,
}

impl Default for XrVulkanDeviceCreateFlagBitsKHR {
	fn default() -> Self {
		Self::__Default__
	}
}

pub use self::XrDebugUtilsMessageSeverityFlagBitsEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrDebugUtilsMessageSeverityFlagBitsEXT {
	 /// Most verbose output severity, typically used for debugging.
	XR_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT                                  = 0x1,
	 /// General info message
	XR_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT                                     = 0x10,
	 /// Indicates the item may be the cause of issues.
	XR_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT                                  = 0x100,
	 /// Indicates that the item is definitely related to erroneous behavior.
	XR_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT                                    = 0x1000,
}

impl Default for XrDebugUtilsMessageSeverityFlagBitsEXT {
	fn default() -> Self {
		Self::XR_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT
	}
}

pub use self::XrDebugUtilsMessageTypeFlagBitsEXT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrDebugUtilsMessageTypeFlagBitsEXT {
	 /// Indicates this is a general message
	XR_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT                                      = 0x1,
	 /// Indicates the message is related to a validation message
	XR_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT                                   = 0x2,
	 /// Indicates the message is related to a potential performance situation
	XR_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT                                  = 0x4,
	 /// Indicates the message is related to a non-conformant runtime result
	XR_DEBUG_UTILS_MESSAGE_TYPE_CONFORMANCE_BIT_EXT                                  = 0x8,
}

impl Default for XrDebugUtilsMessageTypeFlagBitsEXT {
	fn default() -> Self {
		Self::XR_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
	}
}

pub use self::XrOverlayMainSessionFlagBitsEXTX::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrOverlayMainSessionFlagBitsEXTX {
	 /// Indicates the main session enabled XR_KHR_extra_layer_info_depth
	XR_OVERLAY_MAIN_SESSION_ENABLED_COMPOSITION_LAYER_INFO_DEPTH_BIT_EXTX            = 0x1,
}

impl Default for XrOverlayMainSessionFlagBitsEXTX {
	fn default() -> Self {
		Self::XR_OVERLAY_MAIN_SESSION_ENABLED_COMPOSITION_LAYER_INFO_DEPTH_BIT_EXTX
	}
}

pub use self::XrOverlaySessionCreateFlagBitsEXTX::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrOverlaySessionCreateFlagBitsEXTX {
	 /// Indicates the runtime does not need to attempt to lock the overlay session displayTime to the main session displayTime
	XR_OVERLAY_SESSION_CREATE_RELAXED_DISPLAY_TIME_BIT_EXTX                          = 0x1,
}

impl Default for XrOverlaySessionCreateFlagBitsEXTX {
	fn default() -> Self {
		Self::XR_OVERLAY_SESSION_CREATE_RELAXED_DISPLAY_TIME_BIT_EXTX
	}
}

pub use self::XrSpatialGraphNodeTypeMSFT::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrSpatialGraphNodeTypeMSFT {
	XR_SPATIAL_GRAPH_NODE_TYPE_STATIC_MSFT                                           = 1,
	XR_SPATIAL_GRAPH_NODE_TYPE_DYNAMIC_MSFT                                          = 2,
}

impl Default for XrSpatialGraphNodeTypeMSFT {
	fn default() -> Self {
		Self::XR_SPATIAL_GRAPH_NODE_TYPE_STATIC_MSFT
	}
}

pub use self::XrAndroidSurfaceSwapchainFlagBitsFB::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrAndroidSurfaceSwapchainFlagBitsFB {
	 /// Create the underlying BufferQueue in synchronous mode
	XR_ANDROID_SURFACE_SWAPCHAIN_SYNCHRONOUS_BIT_FB                                  = 0x1,
	 /// Acquire most recent buffer whose presentation timestamp is not greater than display time of final composited frame
	XR_ANDROID_SURFACE_SWAPCHAIN_USE_TIMESTAMPS_BIT_FB                               = 0x2,
}

impl Default for XrAndroidSurfaceSwapchainFlagBitsFB {
	fn default() -> Self {
		Self::XR_ANDROID_SURFACE_SWAPCHAIN_SYNCHRONOUS_BIT_FB
	}
}


pub const XR_KHR_android_thread_settings_SPEC_VERSION       : i32 = 5;
pub const XR_KHR_ANDROID_THREAD_SETTINGS_EXTENSION_NAME     : *const u8 = b"XR_KHR_android_thread_settings\0".as_ptr();
pub const XR_KHR_android_surface_swapchain_SPEC_VERSION     : i32 = 4;
pub const XR_KHR_ANDROID_SURFACE_SWAPCHAIN_EXTENSION_NAME   : *const u8 = b"XR_KHR_android_surface_swapchain\0".as_ptr();
pub const XR_KHR_composition_layer_cube_SPEC_VERSION        : i32 = 8;
pub const XR_KHR_COMPOSITION_LAYER_CUBE_EXTENSION_NAME      : *const u8 = b"XR_KHR_composition_layer_cube\0".as_ptr();
pub const XR_KHR_android_create_instance_SPEC_VERSION       : i32 = 3;
pub const XR_KHR_ANDROID_CREATE_INSTANCE_EXTENSION_NAME     : *const u8 = b"XR_KHR_android_create_instance\0".as_ptr();
pub const XR_KHR_composition_layer_depth_SPEC_VERSION       : i32 = 5;
pub const XR_KHR_COMPOSITION_LAYER_DEPTH_EXTENSION_NAME     : *const u8 = b"XR_KHR_composition_layer_depth\0".as_ptr();
pub const XR_KHR_vulkan_swapchain_format_list_SPEC_VERSION  : i32 = 3;
pub const XR_KHR_VULKAN_SWAPCHAIN_FORMAT_LIST_EXTENSION_NAME: *const u8 = b"XR_KHR_vulkan_swapchain_format_list\0".as_ptr();
pub const XR_EXT_performance_settings_SPEC_VERSION          : i32 = 1;
pub const XR_EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME        : *const u8 = b"XR_EXT_performance_settings\0".as_ptr();
pub const XR_EXT_thermal_query_SPEC_VERSION                 : i32 = 1;
pub const XR_EXT_THERMAL_QUERY_EXTENSION_NAME               : *const u8 = b"XR_EXT_thermal_query\0".as_ptr();
pub const XR_KHR_composition_layer_cylinder_SPEC_VERSION    : i32 = 4;
pub const XR_KHR_COMPOSITION_LAYER_CYLINDER_EXTENSION_NAME  : *const u8 = b"XR_KHR_composition_layer_cylinder\0".as_ptr();
pub const XR_KHR_composition_layer_equirect_SPEC_VERSION    : i32 = 3;
pub const XR_KHR_COMPOSITION_LAYER_EQUIRECT_EXTENSION_NAME  : *const u8 = b"XR_KHR_composition_layer_equirect\0".as_ptr();
pub const XR_EXT_debug_utils_SPEC_VERSION                   : i32 = 3;
pub const XR_EXT_DEBUG_UTILS_EXTENSION_NAME                 : *const u8 = b"XR_EXT_debug_utils\0".as_ptr();
pub const XR_KHR_opengl_enable_SPEC_VERSION                 : i32 = 9;
pub const XR_KHR_OPENGL_ENABLE_EXTENSION_NAME               : *const u8 = b"XR_KHR_opengl_enable\0".as_ptr();
pub const XR_KHR_opengl_es_enable_SPEC_VERSION              : i32 = 7;
pub const XR_KHR_OPENGL_ES_ENABLE_EXTENSION_NAME            : *const u8 = b"XR_KHR_opengl_es_enable\0".as_ptr();
pub const XR_KHR_vulkan_enable_SPEC_VERSION                 : i32 = 7;
pub const XR_KHR_VULKAN_ENABLE_EXTENSION_NAME               : *const u8 = b"XR_KHR_vulkan_enable\0".as_ptr();
pub const XR_KHR_D3D11_enable_SPEC_VERSION                  : i32 = 5;
pub const XR_KHR_D3D11_ENABLE_EXTENSION_NAME                : *const u8 = b"XR_KHR_D3D11_enable\0".as_ptr();
pub const XR_KHR_D3D12_enable_SPEC_VERSION                  : i32 = 7;
pub const XR_KHR_D3D12_ENABLE_EXTENSION_NAME                : *const u8 = b"XR_KHR_D3D12_enable\0".as_ptr();
pub const XR_EXT_eye_gaze_interaction_SPEC_VERSION          : i32 = 1;
pub const XR_EXT_EYE_GAZE_INTERACTION_EXTENSION_NAME        : *const u8 = b"XR_EXT_eye_gaze_interaction\0".as_ptr();
pub const XR_KHR_visibility_mask_SPEC_VERSION               : i32 = 2;
pub const XR_KHR_VISIBILITY_MASK_EXTENSION_NAME             : *const u8 = b"XR_KHR_visibility_mask\0".as_ptr();
pub const XR_EXTX_overlay_SPEC_VERSION                      : i32 = 4;
pub const XR_EXTX_OVERLAY_EXTENSION_NAME                    : *const u8 = b"XR_EXTX_overlay\0".as_ptr();
pub const XR_KHR_composition_layer_color_scale_bias_SPEC_VERSION: i32 = 5;
pub const XR_KHR_COMPOSITION_LAYER_COLOR_SCALE_BIAS_EXTENSION_NAME: *const u8 = b"XR_KHR_composition_layer_color_scale_bias\0".as_ptr();
pub const XR_KHR_win32_convert_performance_counter_time_SPEC_VERSION: i32 = 1;
pub const XR_KHR_WIN32_CONVERT_PERFORMANCE_COUNTER_TIME_EXTENSION_NAME: *const u8 = b"XR_KHR_win32_convert_performance_counter_time\0".as_ptr();
pub const XR_KHR_convert_timespec_time_SPEC_VERSION         : i32 = 1;
pub const XR_KHR_CONVERT_TIMESPEC_TIME_EXTENSION_NAME       : *const u8 = b"XR_KHR_convert_timespec_time\0".as_ptr();
pub const XR_VARJO_quad_views_SPEC_VERSION                  : i32 = 1;
pub const XR_VARJO_QUAD_VIEWS_EXTENSION_NAME                : *const u8 = b"XR_VARJO_quad_views\0".as_ptr();
pub const XR_MSFT_unbounded_reference_space_SPEC_VERSION    : i32 = 1;
pub const XR_MSFT_UNBOUNDED_REFERENCE_SPACE_EXTENSION_NAME  : *const u8 = b"XR_MSFT_unbounded_reference_space\0".as_ptr();
pub const XR_MSFT_spatial_anchor_SPEC_VERSION               : i32 = 1;
pub const XR_MSFT_SPATIAL_ANCHOR_EXTENSION_NAME             : *const u8 = b"XR_MSFT_spatial_anchor\0".as_ptr();
pub const XR_MND_headless_SPEC_VERSION                      : i32 = 2;
pub const XR_MND_HEADLESS_EXTENSION_NAME                    : *const u8 = b"XR_MND_headless\0".as_ptr();
pub const XR_OCULUS_android_session_state_enable_SPEC_VERSION: i32 = 1;
pub const XR_OCULUS_ANDROID_SESSION_STATE_ENABLE_EXTENSION_NAME: *const u8 = b"XR_OCULUS_android_session_state_enable\0".as_ptr();
pub const XR_EXT_view_configuration_depth_range_SPEC_VERSION: i32 = 1;
pub const XR_EXT_VIEW_CONFIGURATION_DEPTH_RANGE_EXTENSION_NAME: *const u8 = b"XR_EXT_view_configuration_depth_range\0".as_ptr();
pub const XR_EXT_conformance_automation_SPEC_VERSION        : i32 = 1;
pub const XR_EXT_CONFORMANCE_AUTOMATION_EXTENSION_NAME      : *const u8 = b"XR_EXT_conformance_automation\0".as_ptr();
pub const XR_MNDX_egl_enable_SPEC_VERSION                   : i32 = 1;
pub const XR_MNDX_EGL_ENABLE_EXTENSION_NAME                 : *const u8 = b"XR_MNDX_egl_enable\0".as_ptr();
pub const XR_MSFT_spatial_graph_bridge_SPEC_VERSION         : i32 = 1;
pub const XR_MSFT_SPATIAL_GRAPH_BRIDGE_EXTENSION_NAME       : *const u8 = b"XR_MSFT_spatial_graph_bridge\0".as_ptr();
pub const XR_MSFT_hand_interaction_SPEC_VERSION             : i32 = 1;
pub const XR_MSFT_HAND_INTERACTION_EXTENSION_NAME           : *const u8 = b"XR_MSFT_hand_interaction\0".as_ptr();
pub const XR_EXT_hand_tracking_SPEC_VERSION                 : i32 = 2;
pub const XR_EXT_HAND_TRACKING_EXTENSION_NAME               : *const u8 = b"XR_EXT_hand_tracking\0".as_ptr();
pub const XR_MSFT_hand_tracking_mesh_SPEC_VERSION           : i32 = 2;
pub const XR_MSFT_HAND_TRACKING_MESH_EXTENSION_NAME         : *const u8 = b"XR_MSFT_hand_tracking_mesh\0".as_ptr();
pub const XR_MSFT_secondary_view_configuration_SPEC_VERSION : i32 = 1;
pub const XR_MSFT_SECONDARY_VIEW_CONFIGURATION_EXTENSION_NAME: *const u8 = b"XR_MSFT_secondary_view_configuration\0".as_ptr();
pub const XR_MSFT_first_person_observer_SPEC_VERSION        : i32 = 1;
pub const XR_MSFT_FIRST_PERSON_OBSERVER_EXTENSION_NAME      : *const u8 = b"XR_MSFT_first_person_observer\0".as_ptr();
pub const XR_MSFT_controller_model_SPEC_VERSION             : i32 = 2;
pub const XR_MSFT_CONTROLLER_MODEL_EXTENSION_NAME           : *const u8 = b"XR_MSFT_controller_model\0".as_ptr();
pub const XR_MAX_CONTROLLER_MODEL_NODE_NAME_SIZE_MSFT       : i32 = 64;
pub const XR_MSFT_perception_anchor_interop_SPEC_VERSION    : i32 = 1;
pub const XR_MSFT_PERCEPTION_ANCHOR_INTEROP_EXTENSION_NAME  : *const u8 = b"XR_MSFT_perception_anchor_interop\0".as_ptr();
pub const XR_EXT_win32_appcontainer_compatible_SPEC_VERSION : i32 = 1;
pub const XR_EXT_WIN32_APPCONTAINER_COMPATIBLE_EXTENSION_NAME: *const u8 = b"XR_EXT_win32_appcontainer_compatible\0".as_ptr();
pub const XR_EPIC_view_configuration_fov_SPEC_VERSION       : i32 = 2;
pub const XR_EPIC_VIEW_CONFIGURATION_FOV_EXTENSION_NAME     : *const u8 = b"XR_EPIC_view_configuration_fov\0".as_ptr();
pub const XR_MSFT_holographic_window_attachment_SPEC_VERSION: i32 = 1;
pub const XR_MSFT_HOLOGRAPHIC_WINDOW_ATTACHMENT_EXTENSION_NAME: *const u8 = b"XR_MSFT_holographic_window_attachment\0".as_ptr();
pub const XR_HUAWEI_controller_interaction_SPEC_VERSION     : i32 = 1;
pub const XR_HUAWEI_CONTROLLER_INTERACTION_EXTENSION_NAME   : *const u8 = b"XR_HUAWEI_controller_interaction\0".as_ptr();
pub const XR_FB_android_surface_swapchain_create_SPEC_VERSION: i32 = 1;
pub const XR_FB_ANDROID_SURFACE_SWAPCHAIN_CREATE_EXTENSION_NAME: *const u8 = b"XR_FB_android_surface_swapchain_create\0".as_ptr();
pub const XR_VALVE_analog_threshold_SPEC_VERSION            : i32 = 1;
pub const XR_VALVE_ANALOG_THRESHOLD_EXTENSION_NAME          : *const u8 = b"XR_VALVE_analog_threshold\0".as_ptr();
pub const XR_KHR_loader_init_SPEC_VERSION                   : i32 = 1;
pub const XR_KHR_LOADER_INIT_EXTENSION_NAME                 : *const u8 = b"XR_KHR_loader_init\0".as_ptr();
pub const XR_KHR_loader_init_android_SPEC_VERSION           : i32 = 1;
pub const XR_KHR_LOADER_INIT_ANDROID_EXTENSION_NAME         : *const u8 = b"XR_KHR_loader_init_android\0".as_ptr();
pub const XR_KHR_vulkan_enable2_SPEC_VERSION                : i32 = 1;
pub const XR_KHR_VULKAN_ENABLE2_EXTENSION_NAME              : *const u8 = b"XR_KHR_vulkan_enable2\0".as_ptr();
pub const XR_KHR_composition_layer_equirect2_SPEC_VERSION   : i32 = 1;
pub const XR_KHR_COMPOSITION_LAYER_EQUIRECT2_EXTENSION_NAME : *const u8 = b"XR_KHR_composition_layer_equirect2\0".as_ptr();
pub const XR_EXT_samsung_odyssey_controller_SPEC_VERSION    : i32 = 1;
pub const XR_EXT_SAMSUNG_ODYSSEY_CONTROLLER_EXTENSION_NAME  : *const u8 = b"XR_EXT_samsung_odyssey_controller\0".as_ptr();
pub const XR_EXT_hp_mixed_reality_controller_SPEC_VERSION   : i32 = 1;
pub const XR_EXT_HP_MIXED_REALITY_CONTROLLER_EXTENSION_NAME : *const u8 = b"XR_EXT_hp_mixed_reality_controller\0".as_ptr();
pub const XR_MND_swapchain_usage_input_attachment_bit_SPEC_VERSION: i32 = 2;
pub const XR_MND_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME: *const u8 = b"XR_MND_swapchain_usage_input_attachment_bit\0".as_ptr();
pub const XR_FB_display_refresh_rate_SPEC_VERSION           : i32 = 1;
pub const XR_FB_DISPLAY_REFRESH_RATE_EXTENSION_NAME         : *const u8 = b"XR_FB_display_refresh_rate\0".as_ptr();
pub const XR_HTC_vive_cosmos_controller_interaction_SPEC_VERSION: i32 = 1;
pub const XR_HTC_VIVE_COSMOS_CONTROLLER_INTERACTION_EXTENSION_NAME: *const u8 = b"XR_HTC_vive_cosmos_controller_interaction\0".as_ptr();
pub const XR_FB_color_space_SPEC_VERSION                    : i32 = 1;
pub const XR_FB_COLOR_SPACE_EXTENSION_NAME                  : *const u8 = b"XR_FB_color_space\0".as_ptr();
pub const XR_KHR_binding_modification_SPEC_VERSION          : i32 = 1;
pub const XR_KHR_BINDING_MODIFICATION_EXTENSION_NAME        : *const u8 = b"XR_KHR_binding_modification\0".as_ptr();

/// Wrapper for a `XrSpatialAnchorMSFT` handle
#[derive(Clone, Debug)]
pub struct XrSpatialAnchorMSFTImpl {
	pub handle: XrSpatialAnchorMSFT,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrSpatialAnchorMSFTImpl {
	type Target = XrSpatialAnchorMSFT;
	
	#[inline]
	fn deref(&self) -> &XrSpatialAnchorMSFT { &self.handle }
}

impl fmt::Display for XrSpatialAnchorMSFTImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrSpatialAnchorMSFTImpl {
	pub fn new(handle: XrSpatialAnchorMSFT, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_MSFT_spatial_anchor")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroySpatialAnchorMSFT)(self.handle)
	}
}

/// Wrapper for a `XrSession` handle
#[derive(Clone, Debug)]
pub struct XrSessionImpl {
	pub handle: XrSession,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrSessionImpl {
	type Target = XrSession;
	
	#[inline]
	fn deref(&self) -> &XrSession { &self.handle }
}

impl fmt::Display for XrSessionImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrSessionImpl {
	pub fn new(handle: XrSession, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroySession)(self.handle)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateSwapchainFormats(
		&self,
		formatCountOutput                            : &mut u32,
		formats                                      : Option<&mut [i64]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateSwapchainFormats)(self.handle, formats.as_ref().map_or(0, |v| v.len() as _), formatCountOutput as _, formats.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn createSwapchain(
		&self,
		createInfo                                   : &XrSwapchainCreateInfo,
		swapchain                                    : &mut XrSwapchain
	) -> XrResult {
		(self.table.pfn_xrCreateSwapchain)(self.handle, createInfo as _, swapchain as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn begin(
		&self,
		beginInfo                                    : &XrSessionBeginInfo
	) -> XrResult {
		(self.table.pfn_xrBeginSession)(self.handle, beginInfo as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn end(
		&self
	) -> XrResult {
		(self.table.pfn_xrEndSession)(self.handle)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn requestExit(
		&self
	) -> XrResult {
		(self.table.pfn_xrRequestExitSession)(self.handle)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateReferenceSpaces(
		&self,
		spaceCountOutput                             : &mut u32,
		spaces                                       : Option<&mut [XrReferenceSpaceType]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateReferenceSpaces)(self.handle, spaces.as_ref().map_or(0, |v| v.len() as _), spaceCountOutput as _, spaces.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn createReferenceSpace(
		&self,
		createInfo                                   : &XrReferenceSpaceCreateInfo,
		space                                        : &mut XrSpace
	) -> XrResult {
		(self.table.pfn_xrCreateReferenceSpace)(self.handle, createInfo as _, space as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn createActionSpace(
		&self,
		createInfo                                   : &XrActionSpaceCreateInfo,
		space                                        : &mut XrSpace
	) -> XrResult {
		(self.table.pfn_xrCreateActionSpace)(self.handle, createInfo as _, space as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn beginFrame(
		&self,
		frameBeginInfo                               : Option<&XrFrameBeginInfo>
	) -> XrResult {
		(self.table.pfn_xrBeginFrame)(self.handle, frameBeginInfo.map_or(std::ptr::null(), |v| v as _) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn locateViews(
		&self,
		viewLocateInfo                               : &XrViewLocateInfo,
		viewState                                    : &mut XrViewState,
		viewCountOutput                              : &mut u32,
		views                                        : Option<&mut [XrView]>
	) -> XrResult {
		(self.table.pfn_xrLocateViews)(self.handle, viewLocateInfo as _, viewState as _, views.as_ref().map_or(0, |v| v.len() as _), viewCountOutput as _, views.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn endFrame(
		&self,
		frameEndInfo                                 : &XrFrameEndInfo
	) -> XrResult {
		(self.table.pfn_xrEndFrame)(self.handle, frameEndInfo as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn waitFrame(
		&self,
		frameWaitInfo                                : Option<&XrFrameWaitInfo>,
		frameState                                   : &mut XrFrameState
	) -> XrResult {
		(self.table.pfn_xrWaitFrame)(self.handle, frameWaitInfo.map_or(std::ptr::null(), |v| v as _) as _, frameState as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn applyHapticFeedback(
		&self,
		hapticActionInfo                             : &XrHapticActionInfo,
		hapticFeedback                               : &XrHapticBaseHeader
	) -> XrResult {
		(self.table.pfn_xrApplyHapticFeedback)(self.handle, hapticActionInfo as _, hapticFeedback as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn stopHapticFeedback(
		&self,
		hapticActionInfo                             : &XrHapticActionInfo
	) -> XrResult {
		(self.table.pfn_xrStopHapticFeedback)(self.handle, hapticActionInfo as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getReferenceSpaceBoundsRect(
		&self,
		referenceSpaceType                           : XrReferenceSpaceType,
		bounds                                       : &mut XrExtent2Df
	) -> XrResult {
		(self.table.pfn_xrGetReferenceSpaceBoundsRect)(self.handle, referenceSpaceType, bounds as _)
	}

	#[cfg(feature = "XR_KHR_android_thread_settings")]
	pub fn setAndroidApplicationThreadKHR(
		&self,
		threadType                                   : XrAndroidThreadTypeKHR,
		threadId                                     : u32
	) -> XrResult {
		(self.table.pfn_xrSetAndroidApplicationThreadKHR)(self.handle, threadType, threadId)
	}

	#[cfg(feature = "XR_KHR_android_surface_swapchain")]
	pub fn createSwapchainAndroidSurfaceKHR(
		&self,
		info                                         : &XrSwapchainCreateInfo,
		swapchain                                    : &mut XrSwapchain,
		surface                                      : &mut jobject
	) -> XrResult {
		(self.table.pfn_xrCreateSwapchainAndroidSurfaceKHR)(self.handle, info as _, swapchain as _, surface as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getActionStateBoolean(
		&self,
		getInfo                                      : &XrActionStateGetInfo,
		state                                        : &mut XrActionStateBoolean
	) -> XrResult {
		(self.table.pfn_xrGetActionStateBoolean)(self.handle, getInfo as _, state as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getActionStateFloat(
		&self,
		getInfo                                      : &XrActionStateGetInfo,
		state                                        : &mut XrActionStateFloat
	) -> XrResult {
		(self.table.pfn_xrGetActionStateFloat)(self.handle, getInfo as _, state as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getActionStateVector2f(
		&self,
		getInfo                                      : &XrActionStateGetInfo,
		state                                        : &mut XrActionStateVector2f
	) -> XrResult {
		(self.table.pfn_xrGetActionStateVector2f)(self.handle, getInfo as _, state as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getActionStatePose(
		&self,
		getInfo                                      : &XrActionStateGetInfo,
		state                                        : &mut XrActionStatePose
	) -> XrResult {
		(self.table.pfn_xrGetActionStatePose)(self.handle, getInfo as _, state as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn attachActionSets(
		&self,
		attachInfo                                   : &XrSessionActionSetsAttachInfo
	) -> XrResult {
		(self.table.pfn_xrAttachSessionActionSets)(self.handle, attachInfo as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getCurrentInteractionProfile(
		&self,
		topLevelUserPath                             : XrPath,
		interactionProfile                           : &mut XrInteractionProfileState
	) -> XrResult {
		(self.table.pfn_xrGetCurrentInteractionProfile)(self.handle, topLevelUserPath, interactionProfile as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn syncActions(
		&self,
		syncInfo                                     : &XrActionsSyncInfo
	) -> XrResult {
		(self.table.pfn_xrSyncActions)(self.handle, syncInfo as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateBoundSourcesForAction(
		&self,
		enumerateInfo                                : &XrBoundSourcesForActionEnumerateInfo,
		sourceCountOutput                            : &mut u32,
		sources                                      : Option<&mut [XrPath]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateBoundSourcesForAction)(self.handle, enumerateInfo as _, sources.as_ref().map_or(0, |v| v.len() as _), sourceCountOutput as _, sources.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getInputSourceLocalizedName(
		&self,
		getInfo                                      : &XrInputSourceLocalizedNameGetInfo,
		bufferCountOutput                            : &mut u32,
		buffer                                       : Option<&mut [u8]>
	) -> XrResult {
		(self.table.pfn_xrGetInputSourceLocalizedName)(self.handle, getInfo as _, buffer.as_ref().map_or(0, |v| v.len() as _), bufferCountOutput as _, buffer.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_EXT_performance_settings")]
	pub fn perfSettingsSetPerformanceLevelEXT(
		&self,
		domain                                       : XrPerfSettingsDomainEXT,
		level                                        : XrPerfSettingsLevelEXT
	) -> XrResult {
		(self.table.pfn_xrPerfSettingsSetPerformanceLevelEXT)(self.handle, domain, level)
	}

	#[cfg(feature = "XR_EXT_thermal_query")]
	pub fn thermalGetTemperatureTrendEXT(
		&self,
		domain                                       : XrPerfSettingsDomainEXT,
		notificationLevel                            : &mut XrPerfSettingsNotificationLevelEXT,
		tempHeadroom                                 : &mut f32,
		tempSlope                                    : &mut f32
	) -> XrResult {
		(self.table.pfn_xrThermalGetTemperatureTrendEXT)(self.handle, domain, notificationLevel as _, tempHeadroom as _, tempSlope as _)
	}

	#[cfg(feature = "XR_EXT_debug_utils")]
	pub fn beginDebugUtilsLabelRegionEXT(
		&self,
		labelInfo                                    : &XrDebugUtilsLabelEXT
	) -> XrResult {
		(self.table.pfn_xrSessionBeginDebugUtilsLabelRegionEXT)(self.handle, labelInfo as _)
	}

	#[cfg(feature = "XR_EXT_debug_utils")]
	pub fn endDebugUtilsLabelRegionEXT(
		&self
	) -> XrResult {
		(self.table.pfn_xrSessionEndDebugUtilsLabelRegionEXT)(self.handle)
	}

	#[cfg(feature = "XR_EXT_debug_utils")]
	pub fn insertDebugUtilsLabelEXT(
		&self,
		labelInfo                                    : &XrDebugUtilsLabelEXT
	) -> XrResult {
		(self.table.pfn_xrSessionInsertDebugUtilsLabelEXT)(self.handle, labelInfo as _)
	}

	#[cfg(feature = "XR_KHR_visibility_mask")]
	pub fn getVisibilityMaskKHR(
		&self,
		viewConfigurationType                        : XrViewConfigurationType,
		viewIndex                                    : u32,
		visibilityMaskType                           : XrVisibilityMaskTypeKHR,
		visibilityMask                               : &mut XrVisibilityMaskKHR
	) -> XrResult {
		(self.table.pfn_xrGetVisibilityMaskKHR)(self.handle, viewConfigurationType, viewIndex, visibilityMaskType, visibilityMask as _)
	}

	#[cfg(feature = "XR_MSFT_spatial_anchor")]
	pub fn createSpatialAnchorMSFT(
		&self,
		createInfo                                   : &XrSpatialAnchorCreateInfoMSFT,
		anchor                                       : &mut XrSpatialAnchorMSFT
	) -> XrResult {
		(self.table.pfn_xrCreateSpatialAnchorMSFT)(self.handle, createInfo as _, anchor as _)
	}

	#[cfg(feature = "XR_MSFT_spatial_anchor")]
	pub fn createSpatialAnchorSpaceMSFT(
		&self,
		createInfo                                   : &XrSpatialAnchorSpaceCreateInfoMSFT,
		space                                        : &mut XrSpace
	) -> XrResult {
		(self.table.pfn_xrCreateSpatialAnchorSpaceMSFT)(self.handle, createInfo as _, space as _)
	}

	#[cfg(feature = "XR_EXT_conformance_automation")]
	pub fn setInputDeviceActiveEXT(
		&self,
		interactionProfile                           : XrPath,
		topLevelPath                                 : XrPath,
		isActive                                     : XrBool32
	) -> XrResult {
		(self.table.pfn_xrSetInputDeviceActiveEXT)(self.handle, interactionProfile, topLevelPath, isActive)
	}

	#[cfg(feature = "XR_EXT_conformance_automation")]
	pub fn setInputDeviceStateBoolEXT(
		&self,
		topLevelPath                                 : XrPath,
		inputSourcePath                              : XrPath,
		state                                        : XrBool32
	) -> XrResult {
		(self.table.pfn_xrSetInputDeviceStateBoolEXT)(self.handle, topLevelPath, inputSourcePath, state)
	}

	#[cfg(feature = "XR_EXT_conformance_automation")]
	pub fn setInputDeviceStateFloatEXT(
		&self,
		topLevelPath                                 : XrPath,
		inputSourcePath                              : XrPath,
		state                                        : f32
	) -> XrResult {
		(self.table.pfn_xrSetInputDeviceStateFloatEXT)(self.handle, topLevelPath, inputSourcePath, state)
	}

	#[cfg(feature = "XR_EXT_conformance_automation")]
	pub fn setInputDeviceStateVector2fEXT(
		&self,
		topLevelPath                                 : XrPath,
		inputSourcePath                              : XrPath,
		state                                        : XrVector2f
	) -> XrResult {
		(self.table.pfn_xrSetInputDeviceStateVector2fEXT)(self.handle, topLevelPath, inputSourcePath, state)
	}

	#[cfg(feature = "XR_EXT_conformance_automation")]
	pub fn setInputDeviceLocationEXT(
		&self,
		topLevelPath                                 : XrPath,
		inputSourcePath                              : XrPath,
		space                                        : XrSpace,
		pose                                         : XrPosef
	) -> XrResult {
		(self.table.pfn_xrSetInputDeviceLocationEXT)(self.handle, topLevelPath, inputSourcePath, space, pose)
	}

	#[cfg(feature = "XR_MSFT_spatial_graph_bridge")]
	pub fn createSpatialGraphNodeSpaceMSFT(
		&self,
		createInfo                                   : &XrSpatialGraphNodeSpaceCreateInfoMSFT,
		space                                        : &mut XrSpace
	) -> XrResult {
		(self.table.pfn_xrCreateSpatialGraphNodeSpaceMSFT)(self.handle, createInfo as _, space as _)
	}

	#[cfg(feature = "XR_EXT_hand_tracking")]
	pub fn createHandTrackerEXT(
		&self,
		createInfo                                   : &XrHandTrackerCreateInfoEXT,
		handTracker                                  : &mut XrHandTrackerEXT
	) -> XrResult {
		(self.table.pfn_xrCreateHandTrackerEXT)(self.handle, createInfo as _, handTracker as _)
	}

	#[cfg(feature = "XR_MSFT_controller_model")]
	pub fn getControllerModelKeyMSFT(
		&self,
		topLevelUserPath                             : XrPath,
		controllerModelKeyState                      : &mut XrControllerModelKeyStateMSFT
	) -> XrResult {
		(self.table.pfn_xrGetControllerModelKeyMSFT)(self.handle, topLevelUserPath, controllerModelKeyState as _)
	}

	#[cfg(feature = "XR_MSFT_controller_model")]
	pub fn loadControllerModelMSFT(
		&self,
		modelKey                                     : XrControllerModelKeyMSFT,
		bufferCountOutput                            : &mut u32,
		buffer                                       : Option<&mut [u8]>
	) -> XrResult {
		(self.table.pfn_xrLoadControllerModelMSFT)(self.handle, modelKey, buffer.as_ref().map_or(0, |v| v.len() as _), bufferCountOutput as _, buffer.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_MSFT_controller_model")]
	pub fn getControllerModelPropertiesMSFT(
		&self,
		modelKey                                     : XrControllerModelKeyMSFT,
		properties                                   : &mut XrControllerModelPropertiesMSFT
	) -> XrResult {
		(self.table.pfn_xrGetControllerModelPropertiesMSFT)(self.handle, modelKey, properties as _)
	}

	#[cfg(feature = "XR_MSFT_controller_model")]
	pub fn getControllerModelStateMSFT(
		&self,
		modelKey                                     : XrControllerModelKeyMSFT,
		state                                        : &mut XrControllerModelStateMSFT
	) -> XrResult {
		(self.table.pfn_xrGetControllerModelStateMSFT)(self.handle, modelKey, state as _)
	}

	#[cfg(feature = "XR_FB_display_refresh_rate")]
	pub fn enumerateDisplayRefreshRatesFB(
		&self,
		displayRefreshRateCountOutput                : &mut u32,
		displayRefreshRates                          : Option<&mut [f32]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateDisplayRefreshRatesFB)(self.handle, displayRefreshRates.as_ref().map_or(0, |v| v.len() as _), displayRefreshRateCountOutput as _, displayRefreshRates.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_FB_display_refresh_rate")]
	pub fn getDisplayRefreshRateFB(
		&self,
		displayRefreshRate                           : &mut f32
	) -> XrResult {
		(self.table.pfn_xrGetDisplayRefreshRateFB)(self.handle, displayRefreshRate as _)
	}

	#[cfg(feature = "XR_FB_display_refresh_rate")]
	pub fn requestDisplayRefreshRateFB(
		&self,
		displayRefreshRate                           : f32
	) -> XrResult {
		(self.table.pfn_xrRequestDisplayRefreshRateFB)(self.handle, displayRefreshRate)
	}

	#[cfg(feature = "XR_MSFT_perception_anchor_interop")]
	pub fn createSpatialAnchorFromPerceptionAnchorMSFT(
		&self,
		perceptionAnchor                             : &mut IUnknown,
		anchor                                       : &mut XrSpatialAnchorMSFT
	) -> XrResult {
		(self.table.pfn_xrCreateSpatialAnchorFromPerceptionAnchorMSFT)(self.handle, perceptionAnchor as _, anchor as _)
	}

	#[cfg(feature = "XR_MSFT_perception_anchor_interop")]
	pub fn tryGetPerceptionAnchorFromSpatialAnchorMSFT(
		&self,
		anchor                                       : XrSpatialAnchorMSFT,
		perceptionAnchor                             : &mut &mut IUnknown
	) -> XrResult {
		(self.table.pfn_xrTryGetPerceptionAnchorFromSpatialAnchorMSFT)(self.handle, anchor, perceptionAnchor as _)
	}

	#[cfg(feature = "XR_FB_color_space")]
	pub fn enumerateColorSpacesFB(
		&self,
		colorSpaceCountOutput                        : &mut u32,
		colorSpaces                                  : Option<&mut [XrColorSpaceFB]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateColorSpacesFB)(self.handle, colorSpaces.as_ref().map_or(0, |v| v.len() as _), colorSpaceCountOutput as _, colorSpaces.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_FB_color_space")]
	pub fn setColorSpaceFB(
		&self,
		colorspace                                   : XrColorSpaceFB
	) -> XrResult {
		(self.table.pfn_xrSetColorSpaceFB)(self.handle, colorspace)
	}
}

/// Wrapper for a `XrAction` handle
#[derive(Clone, Debug)]
pub struct XrActionImpl {
	pub handle: XrAction,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrActionImpl {
	type Target = XrAction;
	
	#[inline]
	fn deref(&self) -> &XrAction { &self.handle }
}

impl fmt::Display for XrActionImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrActionImpl {
	pub fn new(handle: XrAction, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroyAction)(self.handle)
	}
}

/// Wrapper for a `XrSpace` handle
#[derive(Clone, Debug)]
pub struct XrSpaceImpl {
	pub handle: XrSpace,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrSpaceImpl {
	type Target = XrSpace;
	
	#[inline]
	fn deref(&self) -> &XrSpace { &self.handle }
}

impl fmt::Display for XrSpaceImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrSpaceImpl {
	pub fn new(handle: XrSpace, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroySpace)(self.handle)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn locate(
		&self,
		baseSpace                                    : XrSpace,
		time                                         : XrTime,
		location                                     : &mut XrSpaceLocation
	) -> XrResult {
		(self.table.pfn_xrLocateSpace)(self.handle, baseSpace, time, location as _)
	}
}

/// Wrapper for a `XrDebugUtilsMessengerEXT` handle
#[derive(Clone, Debug)]
pub struct XrDebugUtilsMessengerEXTImpl {
	pub handle: XrDebugUtilsMessengerEXT,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrDebugUtilsMessengerEXTImpl {
	type Target = XrDebugUtilsMessengerEXT;
	
	#[inline]
	fn deref(&self) -> &XrDebugUtilsMessengerEXT { &self.handle }
}

impl fmt::Display for XrDebugUtilsMessengerEXTImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrDebugUtilsMessengerEXTImpl {
	pub fn new(handle: XrDebugUtilsMessengerEXT, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_EXT_debug_utils")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroyDebugUtilsMessengerEXT)(self.handle)
	}
}
impl XrInstanceImpl {

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateApiLayerProperties(
		propertyCountOutput                          : &mut u32,
		properties                                   : Option<&mut [XrApiLayerProperties]>
	) -> XrResult {
		unsafe { (LIB_OPENXR.pfn_xrEnumerateApiLayerProperties)(properties.as_ref().map_or(0, |v| v.len() as _), propertyCountOutput as _, properties.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _) }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateExtensionProperties(
		layerName                                    : Option<&[u8]>,
		propertyCountOutput                          : &mut u32,
		properties                                   : Option<&mut [XrExtensionProperties]>
	) -> XrResult {
		unsafe { (LIB_OPENXR.pfn_xrEnumerateInstanceExtensionProperties)(layerName.map_or(std::ptr::null(), <[_]>::as_ptr) as _, properties.as_ref().map_or(0, |v| v.len() as _), propertyCountOutput as _, properties.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _) }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn create(
		createInfo                                   : &XrInstanceCreateInfo,
		instance                                     : &mut XrInstance
	) -> XrResult {
		unsafe { (LIB_OPENXR.pfn_xrCreateInstance)(createInfo as _, instance as _) }
	}

	#[cfg(feature = "XR_KHR_loader_init")]
	pub fn initializeLoaderKHR(
		loaderInitInfo                               : &XrLoaderInitInfoBaseHeaderKHR
	) -> XrResult {
		unsafe { (LIB_OPENXR.pfn_xrInitializeLoaderKHR)(loaderInitInfo as _) }
	}
}

/// Wrapper for a `XrHandTrackerEXT` handle
#[derive(Clone, Debug)]
pub struct XrHandTrackerEXTImpl {
	pub handle: XrHandTrackerEXT,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrHandTrackerEXTImpl {
	type Target = XrHandTrackerEXT;
	
	#[inline]
	fn deref(&self) -> &XrHandTrackerEXT { &self.handle }
}

impl fmt::Display for XrHandTrackerEXTImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrHandTrackerEXTImpl {
	pub fn new(handle: XrHandTrackerEXT, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_EXT_hand_tracking")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroyHandTrackerEXT)(self.handle)
	}

	#[cfg(feature = "XR_EXT_hand_tracking")]
	pub fn locateHandJointsEXT(
		&self,
		locateInfo                                   : &XrHandJointsLocateInfoEXT,
		locations                                    : &mut XrHandJointLocationsEXT
	) -> XrResult {
		(self.table.pfn_xrLocateHandJointsEXT)(self.handle, locateInfo as _, locations as _)
	}

	#[cfg(feature = "XR_MSFT_hand_tracking_mesh")]
	pub fn createHandMeshSpaceMSFT(
		&self,
		createInfo                                   : &XrHandMeshSpaceCreateInfoMSFT,
		space                                        : &mut XrSpace
	) -> XrResult {
		(self.table.pfn_xrCreateHandMeshSpaceMSFT)(self.handle, createInfo as _, space as _)
	}

	#[cfg(feature = "XR_MSFT_hand_tracking_mesh")]
	pub fn updateHandMeshMSFT(
		&self,
		updateInfo                                   : &XrHandMeshUpdateInfoMSFT,
		handMesh                                     : &mut XrHandMeshMSFT
	) -> XrResult {
		(self.table.pfn_xrUpdateHandMeshMSFT)(self.handle, updateInfo as _, handMesh as _)
	}
}

/// Wrapper for a `XrSwapchain` handle
#[derive(Clone, Debug)]
pub struct XrSwapchainImpl {
	pub handle: XrSwapchain,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrSwapchainImpl {
	type Target = XrSwapchain;
	
	#[inline]
	fn deref(&self) -> &XrSwapchain { &self.handle }
}

impl fmt::Display for XrSwapchainImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrSwapchainImpl {
	pub fn new(handle: XrSwapchain, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroySwapchain)(self.handle)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateImages(
		&self,
		imageCountOutput                             : &mut u32,
		images                                       : Option<&mut [XrSwapchainImageBaseHeader]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateSwapchainImages)(self.handle, images.as_ref().map_or(0, |v| v.len() as _), imageCountOutput as _, images.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn acquireImage(
		&self,
		acquireInfo                                  : Option<&XrSwapchainImageAcquireInfo>,
		index                                        : &mut u32
	) -> XrResult {
		(self.table.pfn_xrAcquireSwapchainImage)(self.handle, acquireInfo.map_or(std::ptr::null(), |v| v as _) as _, index as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn waitImage(
		&self,
		waitInfo                                     : &XrSwapchainImageWaitInfo
	) -> XrResult {
		(self.table.pfn_xrWaitSwapchainImage)(self.handle, waitInfo as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn releaseImage(
		&self,
		releaseInfo                                  : Option<&XrSwapchainImageReleaseInfo>
	) -> XrResult {
		(self.table.pfn_xrReleaseSwapchainImage)(self.handle, releaseInfo.map_or(std::ptr::null(), |v| v as _) as _)
	}
}

/// Wrapper for a `XrInstance` handle
#[derive(Clone, Debug)]
pub struct XrInstanceImpl {
	pub handle: XrInstance,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrInstanceImpl {
	type Target = XrInstance;
	
	#[inline]
	fn deref(&self) -> &XrInstance { &self.handle }
}

impl fmt::Display for XrInstanceImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrInstanceImpl {
	pub fn new(handle: XrInstance) -> Self {
		Self { handle, table: XrInstanceTable::load(handle) }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroyInstance)(self.handle)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn resultToString(
		&self,
		value                                        : XrResult,
		buffer                                       : [u8;  XR_MAX_RESULT_STRING_SIZE as usize]
	) -> XrResult {
		(self.table.pfn_xrResultToString)(self.handle, value, buffer)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn structureTypeToString(
		&self,
		value                                        : XrStructureType,
		buffer                                       : [u8;  XR_MAX_STRUCTURE_NAME_SIZE as usize]
	) -> XrResult {
		(self.table.pfn_xrStructureTypeToString)(self.handle, value, buffer)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getProperties(
		&self,
		instanceProperties                           : &mut XrInstanceProperties
	) -> XrResult {
		(self.table.pfn_xrGetInstanceProperties)(self.handle, instanceProperties as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getSystem(
		&self,
		getInfo                                      : &XrSystemGetInfo,
		systemId                                     : &mut XrSystemId
	) -> XrResult {
		(self.table.pfn_xrGetSystem)(self.handle, getInfo as _, systemId as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getSystemProperties(
		&self,
		systemId                                     : XrSystemId,
		properties                                   : &mut XrSystemProperties
	) -> XrResult {
		(self.table.pfn_xrGetSystemProperties)(self.handle, systemId, properties as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn createSession(
		&self,
		createInfo                                   : &XrSessionCreateInfo,
		session                                      : &mut XrSession
	) -> XrResult {
		(self.table.pfn_xrCreateSession)(self.handle, createInfo as _, session as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateViewConfigurations(
		&self,
		systemId                                     : XrSystemId,
		viewConfigurationTypeCountOutput             : &mut u32,
		viewConfigurationTypes                       : Option<&mut [XrViewConfigurationType]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateViewConfigurations)(self.handle, systemId, viewConfigurationTypes.as_ref().map_or(0, |v| v.len() as _), viewConfigurationTypeCountOutput as _, viewConfigurationTypes.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateEnvironmentBlendModes(
		&self,
		systemId                                     : XrSystemId,
		viewConfigurationType                        : XrViewConfigurationType,
		environmentBlendModeCountOutput              : &mut u32,
		environmentBlendModes                        : Option<&mut [XrEnvironmentBlendMode]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateEnvironmentBlendModes)(self.handle, systemId, viewConfigurationType, environmentBlendModes.as_ref().map_or(0, |v| v.len() as _), environmentBlendModeCountOutput as _, environmentBlendModes.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn getViewConfigurationProperties(
		&self,
		systemId                                     : XrSystemId,
		viewConfigurationType                        : XrViewConfigurationType,
		configurationProperties                      : &mut XrViewConfigurationProperties
	) -> XrResult {
		(self.table.pfn_xrGetViewConfigurationProperties)(self.handle, systemId, viewConfigurationType, configurationProperties as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn enumerateViewConfigurationViews(
		&self,
		systemId                                     : XrSystemId,
		viewConfigurationType                        : XrViewConfigurationType,
		viewCountOutput                              : &mut u32,
		views                                        : Option<&mut [XrViewConfigurationView]>
	) -> XrResult {
		(self.table.pfn_xrEnumerateViewConfigurationViews)(self.handle, systemId, viewConfigurationType, views.as_ref().map_or(0, |v| v.len() as _), viewCountOutput as _, views.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn pollEvent(
		&self,
		eventData                                    : &mut XrEventDataBuffer
	) -> XrResult {
		(self.table.pfn_xrPollEvent)(self.handle, eventData as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn stringToPath(
		&self,
		pathString                                   : &[u8],
		path                                         : &mut XrPath
	) -> XrResult {
		(self.table.pfn_xrStringToPath)(self.handle, pathString.as_ptr() as _, path as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn pathToString(
		&self,
		path                                         : XrPath,
		bufferCountOutput                            : &mut u32,
		buffer                                       : Option<&mut [u8]>
	) -> XrResult {
		(self.table.pfn_xrPathToString)(self.handle, path, buffer.as_ref().map_or(0, |v| v.len() as _), bufferCountOutput as _, buffer.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn createActionSet(
		&self,
		createInfo                                   : &XrActionSetCreateInfo,
		actionSet                                    : &mut XrActionSet
	) -> XrResult {
		(self.table.pfn_xrCreateActionSet)(self.handle, createInfo as _, actionSet as _)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn suggestInteractionProfileBindings(
		&self,
		suggestedBindings                            : &XrInteractionProfileSuggestedBinding
	) -> XrResult {
		(self.table.pfn_xrSuggestInteractionProfileBindings)(self.handle, suggestedBindings as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pub fn getVulkanExtensionsKHR(
		&self,
		systemId                                     : XrSystemId,
		bufferCapacityInput                          : u32,
		bufferCountOutput                            : &mut u32,
		buffer                                       : Option<&mut [u8]>
	) -> XrResult {
		(self.table.pfn_xrGetVulkanInstanceExtensionsKHR)(self.handle, systemId, bufferCapacityInput, bufferCountOutput as _, buffer.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pub fn getVulkanDeviceExtensionsKHR(
		&self,
		systemId                                     : XrSystemId,
		bufferCapacityInput                          : u32,
		bufferCountOutput                            : &mut u32,
		buffer                                       : Option<&mut [u8]>
	) -> XrResult {
		(self.table.pfn_xrGetVulkanDeviceExtensionsKHR)(self.handle, systemId, bufferCapacityInput, bufferCountOutput as _, buffer.map_or(std::ptr::null_mut(), <[_]>::as_mut_ptr) as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pub fn getVulkanGraphicsDeviceKHR(
		&self,
		systemId                                     : XrSystemId,
		vkInstance                                   : VkInstance,
		vkPhysicalDevice                             : &mut VkPhysicalDevice
	) -> XrResult {
		(self.table.pfn_xrGetVulkanGraphicsDeviceKHR)(self.handle, systemId, vkInstance, vkPhysicalDevice as _)
	}

	#[cfg(feature = "XR_KHR_opengl_enable")]
	pub fn getOpenGLGraphicsRequirementsKHR(
		&self,
		systemId                                     : XrSystemId,
		graphicsRequirements                         : &mut XrGraphicsRequirementsOpenGLKHR
	) -> XrResult {
		(self.table.pfn_xrGetOpenGLGraphicsRequirementsKHR)(self.handle, systemId, graphicsRequirements as _)
	}

	#[cfg(feature = "XR_KHR_opengl_es_enable")]
	pub fn getOpenGLESGraphicsRequirementsKHR(
		&self,
		systemId                                     : XrSystemId,
		graphicsRequirements                         : &mut XrGraphicsRequirementsOpenGLESKHR
	) -> XrResult {
		(self.table.pfn_xrGetOpenGLESGraphicsRequirementsKHR)(self.handle, systemId, graphicsRequirements as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pub fn getVulkanGraphicsRequirementsKHR(
		&self,
		systemId                                     : XrSystemId,
		graphicsRequirements                         : &mut XrGraphicsRequirementsVulkanKHR
	) -> XrResult {
		(self.table.pfn_xrGetVulkanGraphicsRequirementsKHR)(self.handle, systemId, graphicsRequirements as _)
	}

	#[cfg(feature = "XR_KHR_D3D11_enable")]
	pub fn getD3D11GraphicsRequirementsKHR(
		&self,
		systemId                                     : XrSystemId,
		graphicsRequirements                         : &mut XrGraphicsRequirementsD3D11KHR
	) -> XrResult {
		(self.table.pfn_xrGetD3D11GraphicsRequirementsKHR)(self.handle, systemId, graphicsRequirements as _)
	}

	#[cfg(feature = "XR_KHR_D3D12_enable")]
	pub fn getD3D12GraphicsRequirementsKHR(
		&self,
		systemId                                     : XrSystemId,
		graphicsRequirements                         : &mut XrGraphicsRequirementsD3D12KHR
	) -> XrResult {
		(self.table.pfn_xrGetD3D12GraphicsRequirementsKHR)(self.handle, systemId, graphicsRequirements as _)
	}

	#[cfg(feature = "XR_EXT_debug_utils")]
	pub fn setDebugUtilsObjectNameEXT(
		&self,
		nameInfo                                     : &XrDebugUtilsObjectNameInfoEXT
	) -> XrResult {
		(self.table.pfn_xrSetDebugUtilsObjectNameEXT)(self.handle, nameInfo as _)
	}

	#[cfg(feature = "XR_EXT_debug_utils")]
	pub fn createDebugUtilsMessengerEXT(
		&self,
		createInfo                                   : &XrDebugUtilsMessengerCreateInfoEXT,
		messenger                                    : &mut XrDebugUtilsMessengerEXT
	) -> XrResult {
		(self.table.pfn_xrCreateDebugUtilsMessengerEXT)(self.handle, createInfo as _, messenger as _)
	}

	#[cfg(feature = "XR_EXT_debug_utils")]
	pub fn submitDebugUtilsMessageEXT(
		&self,
		messageSeverity                              : XrDebugUtilsMessageSeverityFlagsEXT,
		messageTypes                                 : XrDebugUtilsMessageTypeFlagsEXT,
		callbackData                                 : &XrDebugUtilsMessengerCallbackDataEXT
	) -> XrResult {
		(self.table.pfn_xrSubmitDebugUtilsMessageEXT)(self.handle, messageSeverity, messageTypes, callbackData as _)
	}

	#[cfg(feature = "XR_KHR_win32_convert_performance_counter_time")]
	pub fn convertTimeToWin32PerformanceCounterKHR(
		&self,
		time                                         : XrTime,
		performanceCounter                           : &mut LARGE_INTEGER
	) -> XrResult {
		(self.table.pfn_xrConvertTimeToWin32PerformanceCounterKHR)(self.handle, time, performanceCounter as _)
	}

	#[cfg(feature = "XR_KHR_win32_convert_performance_counter_time")]
	pub fn convertWin32PerformanceCounterToTimeKHR(
		&self,
		performanceCounter                           : &LARGE_INTEGER,
		time                                         : &mut XrTime
	) -> XrResult {
		(self.table.pfn_xrConvertWin32PerformanceCounterToTimeKHR)(self.handle, performanceCounter as _, time as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pub fn createVulkanKHR(
		&self,
		createInfo                                   : &XrVulkanInstanceCreateInfoKHR,
		vulkanInstance                               : &mut VkInstance,
		vulkanResult                                 : &mut VkResult
	) -> XrResult {
		(self.table.pfn_xrCreateVulkanInstanceKHR)(self.handle, createInfo as _, vulkanInstance as _, vulkanResult as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pub fn createVulkanDeviceKHR(
		&self,
		createInfo                                   : &XrVulkanDeviceCreateInfoKHR,
		vulkanDevice                                 : &mut VkDevice,
		vulkanResult                                 : &mut VkResult
	) -> XrResult {
		(self.table.pfn_xrCreateVulkanDeviceKHR)(self.handle, createInfo as _, vulkanDevice as _, vulkanResult as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pub fn getVulkanGraphicsDevice2KHR(
		&self,
		getInfo                                      : &XrVulkanGraphicsDeviceGetInfoKHR,
		vulkanPhysicalDevice                         : &mut VkPhysicalDevice
	) -> XrResult {
		(self.table.pfn_xrGetVulkanGraphicsDevice2KHR)(self.handle, getInfo as _, vulkanPhysicalDevice as _)
	}

	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pub fn getVulkanGraphicsRequirements2KHR(
		&self,
		systemId                                     : XrSystemId,
		graphicsRequirements                         : &mut XrGraphicsRequirementsVulkanKHR
	) -> XrResult {
		(self.table.pfn_xrGetVulkanGraphicsRequirements2KHR)(self.handle, systemId, graphicsRequirements as _)
	}

	#[cfg(feature = "XR_KHR_convert_timespec_time")]
	pub fn convertTimeToTimespecTimeKHR(
		&self,
		time                                         : XrTime,
		timespecTime                                 : &mut timespec
	) -> XrResult {
		(self.table.pfn_xrConvertTimeToTimespecTimeKHR)(self.handle, time, timespecTime as _)
	}

	#[cfg(feature = "XR_KHR_convert_timespec_time")]
	pub fn convertTimespecTimeToTimeKHR(
		&self,
		timespecTime                                 : &timespec,
		time                                         : &mut XrTime
	) -> XrResult {
		(self.table.pfn_xrConvertTimespecTimeToTimeKHR)(self.handle, timespecTime as _, time as _)
	}
}

/// Wrapper for a `XrActionSet` handle
#[derive(Clone, Debug)]
pub struct XrActionSetImpl {
	pub handle: XrActionSet,
	pub table:  Arc<XrInstanceTable>
}

impl ops::Deref for XrActionSetImpl {
	type Target = XrActionSet;
	
	#[inline]
	fn deref(&self) -> &XrActionSet { &self.handle }
}

impl fmt::Display for XrActionSetImpl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#x}", self.handle)
	}
}

impl XrActionSetImpl {
	pub fn new(handle: XrActionSet, parent: &XrInstanceImpl) -> Self {
		Self { handle, table: parent.table.clone() }
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn destroy(
		&self
	) -> XrResult {
		(self.table.pfn_xrDestroyActionSet)(self.handle)
	}

	#[cfg(feature = "XR_VERSION_1_0")]
	pub fn createAction(
		&self,
		createInfo                                   : &XrActionCreateInfo,
		action                                       : &mut XrAction
	) -> XrResult {
		(self.table.pfn_xrCreateAction)(self.handle, createInfo as _, action as _)
	}
}

pub struct XrInstanceTable {
	
	#[cfg(feature = "XR_MSFT_spatial_anchor")]
	pfn_xrDestroySpatialAnchorMSFT              : extern "C" fn(XrSpatialAnchorMSFT) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrDestroySession                        : extern "C" fn(XrSession) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateSwapchainFormats             : extern "C" fn(XrSession, u32, *mut u32, *mut i64) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateSwapchain                       : extern "C" fn(XrSession, *const XrSwapchainCreateInfo, *mut XrSwapchain) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrBeginSession                          : extern "C" fn(XrSession, *const XrSessionBeginInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEndSession                            : extern "C" fn(XrSession) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrRequestExitSession                    : extern "C" fn(XrSession) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateReferenceSpaces              : extern "C" fn(XrSession, u32, *mut u32, *mut XrReferenceSpaceType) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateReferenceSpace                  : extern "C" fn(XrSession, *const XrReferenceSpaceCreateInfo, *mut XrSpace) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateActionSpace                     : extern "C" fn(XrSession, *const XrActionSpaceCreateInfo, *mut XrSpace) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrBeginFrame                            : extern "C" fn(XrSession, *const XrFrameBeginInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrLocateViews                           : extern "C" fn(XrSession, *const XrViewLocateInfo, *mut XrViewState, u32, *mut u32, *mut XrView) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEndFrame                              : extern "C" fn(XrSession, *const XrFrameEndInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrWaitFrame                             : extern "C" fn(XrSession, *const XrFrameWaitInfo, *mut XrFrameState) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrApplyHapticFeedback                   : extern "C" fn(XrSession, *const XrHapticActionInfo, *const XrHapticBaseHeader) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrStopHapticFeedback                    : extern "C" fn(XrSession, *const XrHapticActionInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetReferenceSpaceBoundsRect           : extern "C" fn(XrSession, XrReferenceSpaceType, *mut XrExtent2Df) -> XrResult,
	#[cfg(feature = "XR_KHR_android_thread_settings")]
	pfn_xrSetAndroidApplicationThreadKHR        : extern "C" fn(XrSession, XrAndroidThreadTypeKHR, u32) -> XrResult,
	#[cfg(feature = "XR_KHR_android_surface_swapchain")]
	pfn_xrCreateSwapchainAndroidSurfaceKHR      : extern "C" fn(XrSession, *const XrSwapchainCreateInfo, *mut XrSwapchain, *mut jobject) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetActionStateBoolean                 : extern "C" fn(XrSession, *const XrActionStateGetInfo, *mut XrActionStateBoolean) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetActionStateFloat                   : extern "C" fn(XrSession, *const XrActionStateGetInfo, *mut XrActionStateFloat) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetActionStateVector2f                : extern "C" fn(XrSession, *const XrActionStateGetInfo, *mut XrActionStateVector2f) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetActionStatePose                    : extern "C" fn(XrSession, *const XrActionStateGetInfo, *mut XrActionStatePose) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrAttachSessionActionSets               : extern "C" fn(XrSession, *const XrSessionActionSetsAttachInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetCurrentInteractionProfile          : extern "C" fn(XrSession, XrPath, *mut XrInteractionProfileState) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrSyncActions                           : extern "C" fn(XrSession, *const XrActionsSyncInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateBoundSourcesForAction        : extern "C" fn(XrSession, *const XrBoundSourcesForActionEnumerateInfo, u32, *mut u32, *mut XrPath) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetInputSourceLocalizedName           : extern "C" fn(XrSession, *const XrInputSourceLocalizedNameGetInfo, u32, *mut u32, *mut u8) -> XrResult,
	#[cfg(feature = "XR_EXT_performance_settings")]
	pfn_xrPerfSettingsSetPerformanceLevelEXT    : extern "C" fn(XrSession, XrPerfSettingsDomainEXT, XrPerfSettingsLevelEXT) -> XrResult,
	#[cfg(feature = "XR_EXT_thermal_query")]
	pfn_xrThermalGetTemperatureTrendEXT         : extern "C" fn(XrSession, XrPerfSettingsDomainEXT, *mut XrPerfSettingsNotificationLevelEXT, *mut f32, *mut f32) -> XrResult,
	#[cfg(feature = "XR_EXT_debug_utils")]
	pfn_xrSessionBeginDebugUtilsLabelRegionEXT  : extern "C" fn(XrSession, *const XrDebugUtilsLabelEXT) -> XrResult,
	#[cfg(feature = "XR_EXT_debug_utils")]
	pfn_xrSessionEndDebugUtilsLabelRegionEXT    : extern "C" fn(XrSession) -> XrResult,
	#[cfg(feature = "XR_EXT_debug_utils")]
	pfn_xrSessionInsertDebugUtilsLabelEXT       : extern "C" fn(XrSession, *const XrDebugUtilsLabelEXT) -> XrResult,
	#[cfg(feature = "XR_KHR_visibility_mask")]
	pfn_xrGetVisibilityMaskKHR                  : extern "C" fn(XrSession, XrViewConfigurationType, u32, XrVisibilityMaskTypeKHR, *mut XrVisibilityMaskKHR) -> XrResult,
	#[cfg(feature = "XR_MSFT_spatial_anchor")]
	pfn_xrCreateSpatialAnchorMSFT               : extern "C" fn(XrSession, *const XrSpatialAnchorCreateInfoMSFT, *mut XrSpatialAnchorMSFT) -> XrResult,
	#[cfg(feature = "XR_MSFT_spatial_anchor")]
	pfn_xrCreateSpatialAnchorSpaceMSFT          : extern "C" fn(XrSession, *const XrSpatialAnchorSpaceCreateInfoMSFT, *mut XrSpace) -> XrResult,
	#[cfg(feature = "XR_EXT_conformance_automation")]
	pfn_xrSetInputDeviceActiveEXT               : extern "C" fn(XrSession, XrPath, XrPath, XrBool32) -> XrResult,
	#[cfg(feature = "XR_EXT_conformance_automation")]
	pfn_xrSetInputDeviceStateBoolEXT            : extern "C" fn(XrSession, XrPath, XrPath, XrBool32) -> XrResult,
	#[cfg(feature = "XR_EXT_conformance_automation")]
	pfn_xrSetInputDeviceStateFloatEXT           : extern "C" fn(XrSession, XrPath, XrPath, f32) -> XrResult,
	#[cfg(feature = "XR_EXT_conformance_automation")]
	pfn_xrSetInputDeviceStateVector2fEXT        : extern "C" fn(XrSession, XrPath, XrPath, XrVector2f) -> XrResult,
	#[cfg(feature = "XR_EXT_conformance_automation")]
	pfn_xrSetInputDeviceLocationEXT             : extern "C" fn(XrSession, XrPath, XrPath, XrSpace, XrPosef) -> XrResult,
	#[cfg(feature = "XR_MSFT_spatial_graph_bridge")]
	pfn_xrCreateSpatialGraphNodeSpaceMSFT       : extern "C" fn(XrSession, *const XrSpatialGraphNodeSpaceCreateInfoMSFT, *mut XrSpace) -> XrResult,
	#[cfg(feature = "XR_EXT_hand_tracking")]
	pfn_xrCreateHandTrackerEXT                  : extern "C" fn(XrSession, *const XrHandTrackerCreateInfoEXT, *mut XrHandTrackerEXT) -> XrResult,
	#[cfg(feature = "XR_MSFT_controller_model")]
	pfn_xrGetControllerModelKeyMSFT             : extern "C" fn(XrSession, XrPath, *mut XrControllerModelKeyStateMSFT) -> XrResult,
	#[cfg(feature = "XR_MSFT_controller_model")]
	pfn_xrLoadControllerModelMSFT               : extern "C" fn(XrSession, XrControllerModelKeyMSFT, u32, *mut u32, *mut u8) -> XrResult,
	#[cfg(feature = "XR_MSFT_controller_model")]
	pfn_xrGetControllerModelPropertiesMSFT      : extern "C" fn(XrSession, XrControllerModelKeyMSFT, *mut XrControllerModelPropertiesMSFT) -> XrResult,
	#[cfg(feature = "XR_MSFT_controller_model")]
	pfn_xrGetControllerModelStateMSFT           : extern "C" fn(XrSession, XrControllerModelKeyMSFT, *mut XrControllerModelStateMSFT) -> XrResult,
	#[cfg(feature = "XR_FB_display_refresh_rate")]
	pfn_xrEnumerateDisplayRefreshRatesFB        : extern "C" fn(XrSession, u32, *mut u32, *mut f32) -> XrResult,
	#[cfg(feature = "XR_FB_display_refresh_rate")]
	pfn_xrGetDisplayRefreshRateFB               : extern "C" fn(XrSession, *mut f32) -> XrResult,
	#[cfg(feature = "XR_FB_display_refresh_rate")]
	pfn_xrRequestDisplayRefreshRateFB           : extern "C" fn(XrSession, f32) -> XrResult,
	#[cfg(feature = "XR_MSFT_perception_anchor_interop")]
	pfn_xrCreateSpatialAnchorFromPerceptionAnchorMSFT: extern "C" fn(XrSession, *mut IUnknown, *mut XrSpatialAnchorMSFT) -> XrResult,
	#[cfg(feature = "XR_MSFT_perception_anchor_interop")]
	pfn_xrTryGetPerceptionAnchorFromSpatialAnchorMSFT: extern "C" fn(XrSession, XrSpatialAnchorMSFT, *mut *mut IUnknown) -> XrResult,
	#[cfg(feature = "XR_FB_color_space")]
	pfn_xrEnumerateColorSpacesFB                : extern "C" fn(XrSession, u32, *mut u32, *mut XrColorSpaceFB) -> XrResult,
	#[cfg(feature = "XR_FB_color_space")]
	pfn_xrSetColorSpaceFB                       : extern "C" fn(XrSession, XrColorSpaceFB) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrDestroyAction                         : extern "C" fn(XrAction) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrDestroySpace                          : extern "C" fn(XrSpace) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrLocateSpace                           : extern "C" fn(XrSpace, XrSpace, XrTime, *mut XrSpaceLocation) -> XrResult,
	#[cfg(feature = "XR_EXT_debug_utils")]
	pfn_xrDestroyDebugUtilsMessengerEXT         : extern "C" fn(XrDebugUtilsMessengerEXT) -> XrResult,
	#[cfg(feature = "XR_EXT_hand_tracking")]
	pfn_xrDestroyHandTrackerEXT                 : extern "C" fn(XrHandTrackerEXT) -> XrResult,
	#[cfg(feature = "XR_EXT_hand_tracking")]
	pfn_xrLocateHandJointsEXT                   : extern "C" fn(XrHandTrackerEXT, *const XrHandJointsLocateInfoEXT, *mut XrHandJointLocationsEXT) -> XrResult,
	#[cfg(feature = "XR_MSFT_hand_tracking_mesh")]
	pfn_xrCreateHandMeshSpaceMSFT               : extern "C" fn(XrHandTrackerEXT, *const XrHandMeshSpaceCreateInfoMSFT, *mut XrSpace) -> XrResult,
	#[cfg(feature = "XR_MSFT_hand_tracking_mesh")]
	pfn_xrUpdateHandMeshMSFT                    : extern "C" fn(XrHandTrackerEXT, *const XrHandMeshUpdateInfoMSFT, *mut XrHandMeshMSFT) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrDestroySwapchain                      : extern "C" fn(XrSwapchain) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateSwapchainImages              : extern "C" fn(XrSwapchain, u32, *mut u32, *mut XrSwapchainImageBaseHeader) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrAcquireSwapchainImage                 : extern "C" fn(XrSwapchain, *const XrSwapchainImageAcquireInfo, *mut u32) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrWaitSwapchainImage                    : extern "C" fn(XrSwapchain, *const XrSwapchainImageWaitInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrReleaseSwapchainImage                 : extern "C" fn(XrSwapchain, *const XrSwapchainImageReleaseInfo) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrDestroyInstance                       : extern "C" fn(XrInstance) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrResultToString                        : extern "C" fn(XrInstance, XrResult, [u8;  XR_MAX_RESULT_STRING_SIZE as usize]) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrStructureTypeToString                 : extern "C" fn(XrInstance, XrStructureType, [u8;  XR_MAX_STRUCTURE_NAME_SIZE as usize]) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetInstanceProperties                 : extern "C" fn(XrInstance, *mut XrInstanceProperties) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetSystem                             : extern "C" fn(XrInstance, *const XrSystemGetInfo, *mut XrSystemId) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetSystemProperties                   : extern "C" fn(XrInstance, XrSystemId, *mut XrSystemProperties) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateSession                         : extern "C" fn(XrInstance, *const XrSessionCreateInfo, *mut XrSession) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateViewConfigurations           : extern "C" fn(XrInstance, XrSystemId, u32, *mut u32, *mut XrViewConfigurationType) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateEnvironmentBlendModes        : extern "C" fn(XrInstance, XrSystemId, XrViewConfigurationType, u32, *mut u32, *mut XrEnvironmentBlendMode) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrGetViewConfigurationProperties        : extern "C" fn(XrInstance, XrSystemId, XrViewConfigurationType, *mut XrViewConfigurationProperties) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateViewConfigurationViews       : extern "C" fn(XrInstance, XrSystemId, XrViewConfigurationType, u32, *mut u32, *mut XrViewConfigurationView) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrPollEvent                             : extern "C" fn(XrInstance, *mut XrEventDataBuffer) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrStringToPath                          : extern "C" fn(XrInstance, *const u8, *mut XrPath) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrPathToString                          : extern "C" fn(XrInstance, XrPath, u32, *mut u32, *mut u8) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateActionSet                       : extern "C" fn(XrInstance, *const XrActionSetCreateInfo, *mut XrActionSet) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrSuggestInteractionProfileBindings     : extern "C" fn(XrInstance, *const XrInteractionProfileSuggestedBinding) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pfn_xrGetVulkanInstanceExtensionsKHR        : extern "C" fn(XrInstance, XrSystemId, u32, *mut u32, *mut u8) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pfn_xrGetVulkanDeviceExtensionsKHR          : extern "C" fn(XrInstance, XrSystemId, u32, *mut u32, *mut u8) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pfn_xrGetVulkanGraphicsDeviceKHR            : extern "C" fn(XrInstance, XrSystemId, VkInstance, *mut VkPhysicalDevice) -> XrResult,
	#[cfg(feature = "XR_KHR_opengl_enable")]
	pfn_xrGetOpenGLGraphicsRequirementsKHR      : extern "C" fn(XrInstance, XrSystemId, *mut XrGraphicsRequirementsOpenGLKHR) -> XrResult,
	#[cfg(feature = "XR_KHR_opengl_es_enable")]
	pfn_xrGetOpenGLESGraphicsRequirementsKHR    : extern "C" fn(XrInstance, XrSystemId, *mut XrGraphicsRequirementsOpenGLESKHR) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable")]
	pfn_xrGetVulkanGraphicsRequirementsKHR      : extern "C" fn(XrInstance, XrSystemId, *mut XrGraphicsRequirementsVulkanKHR) -> XrResult,
	#[cfg(feature = "XR_KHR_D3D11_enable")]
	pfn_xrGetD3D11GraphicsRequirementsKHR       : extern "C" fn(XrInstance, XrSystemId, *mut XrGraphicsRequirementsD3D11KHR) -> XrResult,
	#[cfg(feature = "XR_KHR_D3D12_enable")]
	pfn_xrGetD3D12GraphicsRequirementsKHR       : extern "C" fn(XrInstance, XrSystemId, *mut XrGraphicsRequirementsD3D12KHR) -> XrResult,
	#[cfg(feature = "XR_EXT_debug_utils")]
	pfn_xrSetDebugUtilsObjectNameEXT            : extern "C" fn(XrInstance, *const XrDebugUtilsObjectNameInfoEXT) -> XrResult,
	#[cfg(feature = "XR_EXT_debug_utils")]
	pfn_xrCreateDebugUtilsMessengerEXT          : extern "C" fn(XrInstance, *const XrDebugUtilsMessengerCreateInfoEXT, *mut XrDebugUtilsMessengerEXT) -> XrResult,
	#[cfg(feature = "XR_EXT_debug_utils")]
	pfn_xrSubmitDebugUtilsMessageEXT            : extern "C" fn(XrInstance, XrDebugUtilsMessageSeverityFlagsEXT, XrDebugUtilsMessageTypeFlagsEXT, *const XrDebugUtilsMessengerCallbackDataEXT) -> XrResult,
	#[cfg(feature = "XR_KHR_win32_convert_performance_counter_time")]
	pfn_xrConvertTimeToWin32PerformanceCounterKHR: extern "C" fn(XrInstance, XrTime, *mut LARGE_INTEGER) -> XrResult,
	#[cfg(feature = "XR_KHR_win32_convert_performance_counter_time")]
	pfn_xrConvertWin32PerformanceCounterToTimeKHR: extern "C" fn(XrInstance, *const LARGE_INTEGER, *mut XrTime) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pfn_xrCreateVulkanInstanceKHR               : extern "C" fn(XrInstance, *const XrVulkanInstanceCreateInfoKHR, *mut VkInstance, *mut VkResult) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pfn_xrCreateVulkanDeviceKHR                 : extern "C" fn(XrInstance, *const XrVulkanDeviceCreateInfoKHR, *mut VkDevice, *mut VkResult) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pfn_xrGetVulkanGraphicsDevice2KHR           : extern "C" fn(XrInstance, *const XrVulkanGraphicsDeviceGetInfoKHR, *mut VkPhysicalDevice) -> XrResult,
	#[cfg(feature = "XR_KHR_vulkan_enable2")]
	pfn_xrGetVulkanGraphicsRequirements2KHR     : extern "C" fn(XrInstance, XrSystemId, *mut XrGraphicsRequirementsVulkanKHR) -> XrResult,
	#[cfg(feature = "XR_KHR_convert_timespec_time")]
	pfn_xrConvertTimeToTimespecTimeKHR          : extern "C" fn(XrInstance, XrTime, *mut timespec) -> XrResult,
	#[cfg(feature = "XR_KHR_convert_timespec_time")]
	pfn_xrConvertTimespecTimeToTimeKHR          : extern "C" fn(XrInstance, *const timespec, *mut XrTime) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrDestroyActionSet                      : extern "C" fn(XrActionSet) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateAction                          : extern "C" fn(XrActionSet, *const XrActionCreateInfo, *mut XrAction) -> XrResult,
}

impl XrInstanceTable {
	fn load(handle: XrInstance) -> Arc<Self> {
		let table = unsafe { &LIB_OPENXR };
		Arc::new(unsafe { Self {
			
			#[cfg(feature = "XR_MSFT_spatial_anchor")]
			pfn_xrDestroySpatialAnchorMSFT                                  : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroySpatialAnchorMSFT\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrDestroySession                                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroySession\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEnumerateSwapchainFormats                                 : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateSwapchainFormats\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrCreateSwapchain                                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateSwapchain\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrBeginSession                                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrBeginSession\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEndSession                                                : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEndSession\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrRequestExitSession                                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrRequestExitSession\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEnumerateReferenceSpaces                                  : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateReferenceSpaces\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrCreateReferenceSpace                                      : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateReferenceSpace\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrCreateActionSpace                                         : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateActionSpace\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrBeginFrame                                                : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrBeginFrame\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrLocateViews                                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrLocateViews\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEndFrame                                                  : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEndFrame\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrWaitFrame                                                 : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrWaitFrame\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrApplyHapticFeedback                                       : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrApplyHapticFeedback\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrStopHapticFeedback                                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrStopHapticFeedback\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetReferenceSpaceBoundsRect                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetReferenceSpaceBoundsRect\0".as_ptr())),
			#[cfg(feature = "XR_KHR_android_thread_settings")]
			pfn_xrSetAndroidApplicationThreadKHR                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetAndroidApplicationThreadKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_android_surface_swapchain")]
			pfn_xrCreateSwapchainAndroidSurfaceKHR                          : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateSwapchainAndroidSurfaceKHR\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetActionStateBoolean                                     : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetActionStateBoolean\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetActionStateFloat                                       : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetActionStateFloat\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetActionStateVector2f                                    : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetActionStateVector2f\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetActionStatePose                                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetActionStatePose\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrAttachSessionActionSets                                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrAttachSessionActionSets\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetCurrentInteractionProfile                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetCurrentInteractionProfile\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrSyncActions                                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSyncActions\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEnumerateBoundSourcesForAction                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateBoundSourcesForAction\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetInputSourceLocalizedName                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetInputSourceLocalizedName\0".as_ptr())),
			#[cfg(feature = "XR_EXT_performance_settings")]
			pfn_xrPerfSettingsSetPerformanceLevelEXT                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrPerfSettingsSetPerformanceLevelEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_thermal_query")]
			pfn_xrThermalGetTemperatureTrendEXT                             : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrThermalGetTemperatureTrendEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_debug_utils")]
			pfn_xrSessionBeginDebugUtilsLabelRegionEXT                      : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSessionBeginDebugUtilsLabelRegionEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_debug_utils")]
			pfn_xrSessionEndDebugUtilsLabelRegionEXT                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSessionEndDebugUtilsLabelRegionEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_debug_utils")]
			pfn_xrSessionInsertDebugUtilsLabelEXT                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSessionInsertDebugUtilsLabelEXT\0".as_ptr())),
			#[cfg(feature = "XR_KHR_visibility_mask")]
			pfn_xrGetVisibilityMaskKHR                                      : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetVisibilityMaskKHR\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_spatial_anchor")]
			pfn_xrCreateSpatialAnchorMSFT                                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateSpatialAnchorMSFT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_spatial_anchor")]
			pfn_xrCreateSpatialAnchorSpaceMSFT                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateSpatialAnchorSpaceMSFT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_conformance_automation")]
			pfn_xrSetInputDeviceActiveEXT                                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetInputDeviceActiveEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_conformance_automation")]
			pfn_xrSetInputDeviceStateBoolEXT                                : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetInputDeviceStateBoolEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_conformance_automation")]
			pfn_xrSetInputDeviceStateFloatEXT                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetInputDeviceStateFloatEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_conformance_automation")]
			pfn_xrSetInputDeviceStateVector2fEXT                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetInputDeviceStateVector2fEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_conformance_automation")]
			pfn_xrSetInputDeviceLocationEXT                                 : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetInputDeviceLocationEXT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_spatial_graph_bridge")]
			pfn_xrCreateSpatialGraphNodeSpaceMSFT                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateSpatialGraphNodeSpaceMSFT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_hand_tracking")]
			pfn_xrCreateHandTrackerEXT                                      : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateHandTrackerEXT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_controller_model")]
			pfn_xrGetControllerModelKeyMSFT                                 : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetControllerModelKeyMSFT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_controller_model")]
			pfn_xrLoadControllerModelMSFT                                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrLoadControllerModelMSFT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_controller_model")]
			pfn_xrGetControllerModelPropertiesMSFT                          : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetControllerModelPropertiesMSFT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_controller_model")]
			pfn_xrGetControllerModelStateMSFT                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetControllerModelStateMSFT\0".as_ptr())),
			#[cfg(feature = "XR_FB_display_refresh_rate")]
			pfn_xrEnumerateDisplayRefreshRatesFB                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateDisplayRefreshRatesFB\0".as_ptr())),
			#[cfg(feature = "XR_FB_display_refresh_rate")]
			pfn_xrGetDisplayRefreshRateFB                                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetDisplayRefreshRateFB\0".as_ptr())),
			#[cfg(feature = "XR_FB_display_refresh_rate")]
			pfn_xrRequestDisplayRefreshRateFB                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrRequestDisplayRefreshRateFB\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_perception_anchor_interop")]
			pfn_xrCreateSpatialAnchorFromPerceptionAnchorMSFT               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateSpatialAnchorFromPerceptionAnchorMSFT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_perception_anchor_interop")]
			pfn_xrTryGetPerceptionAnchorFromSpatialAnchorMSFT               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrTryGetPerceptionAnchorFromSpatialAnchorMSFT\0".as_ptr())),
			#[cfg(feature = "XR_FB_color_space")]
			pfn_xrEnumerateColorSpacesFB                                    : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateColorSpacesFB\0".as_ptr())),
			#[cfg(feature = "XR_FB_color_space")]
			pfn_xrSetColorSpaceFB                                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetColorSpaceFB\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrDestroyAction                                             : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroyAction\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrDestroySpace                                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroySpace\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrLocateSpace                                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrLocateSpace\0".as_ptr())),
			#[cfg(feature = "XR_EXT_debug_utils")]
			pfn_xrDestroyDebugUtilsMessengerEXT                             : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroyDebugUtilsMessengerEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_hand_tracking")]
			pfn_xrDestroyHandTrackerEXT                                     : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroyHandTrackerEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_hand_tracking")]
			pfn_xrLocateHandJointsEXT                                       : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrLocateHandJointsEXT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_hand_tracking_mesh")]
			pfn_xrCreateHandMeshSpaceMSFT                                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateHandMeshSpaceMSFT\0".as_ptr())),
			#[cfg(feature = "XR_MSFT_hand_tracking_mesh")]
			pfn_xrUpdateHandMeshMSFT                                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrUpdateHandMeshMSFT\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrDestroySwapchain                                          : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroySwapchain\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEnumerateSwapchainImages                                  : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateSwapchainImages\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrAcquireSwapchainImage                                     : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrAcquireSwapchainImage\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrWaitSwapchainImage                                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrWaitSwapchainImage\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrReleaseSwapchainImage                                     : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrReleaseSwapchainImage\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrDestroyInstance                                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroyInstance\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrResultToString                                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrResultToString\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrStructureTypeToString                                     : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrStructureTypeToString\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetInstanceProperties                                     : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetInstanceProperties\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetSystem                                                 : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetSystem\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetSystemProperties                                       : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetSystemProperties\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrCreateSession                                             : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateSession\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEnumerateViewConfigurations                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateViewConfigurations\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEnumerateEnvironmentBlendModes                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateEnvironmentBlendModes\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrGetViewConfigurationProperties                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetViewConfigurationProperties\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrEnumerateViewConfigurationViews                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrEnumerateViewConfigurationViews\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrPollEvent                                                 : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrPollEvent\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrStringToPath                                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrStringToPath\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrPathToString                                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrPathToString\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrCreateActionSet                                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateActionSet\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrSuggestInteractionProfileBindings                         : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSuggestInteractionProfileBindings\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable")]
			pfn_xrGetVulkanInstanceExtensionsKHR                            : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetVulkanInstanceExtensionsKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable")]
			pfn_xrGetVulkanDeviceExtensionsKHR                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetVulkanDeviceExtensionsKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable")]
			pfn_xrGetVulkanGraphicsDeviceKHR                                : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetVulkanGraphicsDeviceKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_opengl_enable")]
			pfn_xrGetOpenGLGraphicsRequirementsKHR                          : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetOpenGLGraphicsRequirementsKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_opengl_es_enable")]
			pfn_xrGetOpenGLESGraphicsRequirementsKHR                        : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetOpenGLESGraphicsRequirementsKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable")]
			pfn_xrGetVulkanGraphicsRequirementsKHR                          : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetVulkanGraphicsRequirementsKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_D3D11_enable")]
			pfn_xrGetD3D11GraphicsRequirementsKHR                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetD3D11GraphicsRequirementsKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_D3D12_enable")]
			pfn_xrGetD3D12GraphicsRequirementsKHR                           : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetD3D12GraphicsRequirementsKHR\0".as_ptr())),
			#[cfg(feature = "XR_EXT_debug_utils")]
			pfn_xrSetDebugUtilsObjectNameEXT                                : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSetDebugUtilsObjectNameEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_debug_utils")]
			pfn_xrCreateDebugUtilsMessengerEXT                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateDebugUtilsMessengerEXT\0".as_ptr())),
			#[cfg(feature = "XR_EXT_debug_utils")]
			pfn_xrSubmitDebugUtilsMessageEXT                                : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrSubmitDebugUtilsMessageEXT\0".as_ptr())),
			#[cfg(feature = "XR_KHR_win32_convert_performance_counter_time")]
			pfn_xrConvertTimeToWin32PerformanceCounterKHR                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrConvertTimeToWin32PerformanceCounterKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_win32_convert_performance_counter_time")]
			pfn_xrConvertWin32PerformanceCounterToTimeKHR                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrConvertWin32PerformanceCounterToTimeKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable2")]
			pfn_xrCreateVulkanInstanceKHR                                   : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateVulkanInstanceKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable2")]
			pfn_xrCreateVulkanDeviceKHR                                     : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateVulkanDeviceKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable2")]
			pfn_xrGetVulkanGraphicsDevice2KHR                               : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetVulkanGraphicsDevice2KHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_vulkan_enable2")]
			pfn_xrGetVulkanGraphicsRequirements2KHR                         : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrGetVulkanGraphicsRequirements2KHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_convert_timespec_time")]
			pfn_xrConvertTimeToTimespecTimeKHR                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrConvertTimeToTimespecTimeKHR\0".as_ptr())),
			#[cfg(feature = "XR_KHR_convert_timespec_time")]
			pfn_xrConvertTimespecTimeToTimeKHR                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrConvertTimespecTimeToTimeKHR\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrDestroyActionSet                                          : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrDestroyActionSet\0".as_ptr())),
			#[cfg(feature = "XR_VERSION_1_0")]
			pfn_xrCreateAction                                              : transmute((table.pfn_xrGetInstanceProcAddr)(handle, b"xrCreateAction\0".as_ptr())),
		} })
	}
}

impl fmt::Debug for XrInstanceTable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("XrInstanceTable").finish()
	}
}


static mut LIB_OPENXR: LibOpenXr = LibOpenXr {
	lib: None,
	pfn_xrGetInstanceProcAddr: { extern fn load(handle: XrInstance, name: *const u8) -> PFN_xrVoidFunction { unsafe { LIB_OPENXR.load(); (LIB_OPENXR.pfn_xrGetInstanceProcAddr)(handle, name) } } load },
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateApiLayerProperties           : { extern "C" fn load(propertyCapacityInput: u32, propertyCountOutput: *mut u32, properties: *mut XrApiLayerProperties, ) -> XrResult { unsafe { LIB_OPENXR.load(); (LIB_OPENXR.pfn_xrEnumerateApiLayerProperties)(propertyCapacityInput, propertyCountOutput, properties, ) } } load },
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateInstanceExtensionProperties  : { extern "C" fn load(layerName: *const u8, propertyCapacityInput: u32, propertyCountOutput: *mut u32, properties: *mut XrExtensionProperties, ) -> XrResult { unsafe { LIB_OPENXR.load(); (LIB_OPENXR.pfn_xrEnumerateInstanceExtensionProperties)(layerName, propertyCapacityInput, propertyCountOutput, properties, ) } } load },
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateInstance                        : { extern "C" fn load(createInfo: *const XrInstanceCreateInfo, instance: *mut XrInstance, ) -> XrResult { unsafe { LIB_OPENXR.load(); (LIB_OPENXR.pfn_xrCreateInstance)(createInfo, instance, ) } } load },
	#[cfg(feature = "XR_KHR_loader_init")]
	pfn_xrInitializeLoaderKHR                   : { extern "C" fn load(loaderInitInfo: *const XrLoaderInitInfoBaseHeaderKHR, ) -> XrResult { unsafe { LIB_OPENXR.load(); (LIB_OPENXR.pfn_xrInitializeLoaderKHR)(loaderInitInfo, ) } } load },
};

pub struct LibOpenXr {
	#[allow(dead_code)]
	lib: Option<libloading::Library>,
	pfn_xrGetInstanceProcAddr: extern fn(XrInstance, *const u8) -> PFN_xrVoidFunction,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateApiLayerProperties           : extern "C" fn(u32, *mut u32, *mut XrApiLayerProperties) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrEnumerateInstanceExtensionProperties  : extern "C" fn(*const u8, u32, *mut u32, *mut XrExtensionProperties) -> XrResult,
	#[cfg(feature = "XR_VERSION_1_0")]
	pfn_xrCreateInstance                        : extern "C" fn(*const XrInstanceCreateInfo, *mut XrInstance) -> XrResult,
	#[cfg(feature = "XR_KHR_loader_init")]
	pfn_xrInitializeLoaderKHR                   : extern "C" fn(*const XrLoaderInitInfoBaseHeaderKHR) -> XrResult,
}

impl LibOpenXr {
	unsafe fn load(&mut self) {
		let lib                                          = libloading::Library::new(LIB).expect("failed to load `libopenxr`");
		self.pfn_xrGetInstanceProcAddr                   = *lib.get(b"xrGetInstanceProcAddr\0").expect("failed to load `xrGetInstanceProcAddr`");
		self.lib                                         = Some(lib);
		
		#[cfg(feature = "XR_VERSION_1_0")]
		{ 
			self.pfn_xrEnumerateApiLayerProperties           = transmute((self.pfn_xrGetInstanceProcAddr)(XR_NULL_HANDLE, b"xrEnumerateApiLayerProperties\0".as_ptr()));
		}
		#[cfg(feature = "XR_VERSION_1_0")]
		{ 
			self.pfn_xrEnumerateInstanceExtensionProperties  = transmute((self.pfn_xrGetInstanceProcAddr)(XR_NULL_HANDLE, b"xrEnumerateInstanceExtensionProperties\0".as_ptr()));
		}
		#[cfg(feature = "XR_VERSION_1_0")]
		{ 
			self.pfn_xrCreateInstance                        = transmute((self.pfn_xrGetInstanceProcAddr)(XR_NULL_HANDLE, b"xrCreateInstance\0".as_ptr()));
		}
		#[cfg(feature = "XR_KHR_loader_init")]
		{ 
			self.pfn_xrInitializeLoaderKHR                   = transmute((self.pfn_xrGetInstanceProcAddr)(XR_NULL_HANDLE, b"xrInitializeLoaderKHR\0".as_ptr()));
		}
	}
}

