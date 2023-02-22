
/// # Protocol for grabbing the keyboard from Xwayland
///
/// This protocol is application-specific to meet the needs of the X11
/// protocol through Xwayland. It provides a way for Xwayland to request
/// all keyboard events to be forwarded to a surface even when the
/// surface does not have keyboard focus.
///
/// In the X11 protocol, a client may request an "active grab" on the
/// keyboard. On success, all key events are reported only to the
/// grabbing X11 client. For details, see XGrabKeyboard(3).
///
/// The core Wayland protocol does not have a notion of an active
/// keyboard grab. When running in Xwayland, X11 applications may
/// acquire an active grab inside Xwayland but that cannot be translated
/// to the Wayland compositor who may set the input focus to some other
/// surface. In doing so, it breaks the X11 client assumption that all
/// key events are reported to the grabbing client.
///
/// This protocol specifies a way for Xwayland to request all keyboard
/// be directed to the given surface. The protocol does not guarantee
/// that the compositor will honor this request and it does not
/// prescribe user interfaces on how to handle the respond. For example,
/// a compositor may inform the user that all key events are now
/// forwarded to the given client surface, or it may ask the user for
/// permission to do so.
///
/// Compositors are required to restrict access to this application
/// specific protocol to Xwayland alone.
///
/// Warning! The protocol described in this file is experimental and
/// backward incompatible changes may be made. Backward compatible
/// changes may be added together with the corresponding interface
/// version bump.
/// Backward incompatible changes are done by bumping the version
/// number in the protocol and interface names and resetting the
/// interface version. Once the protocol is to be declared stable,
/// the 'z' prefix and the version number in the protocol and
/// interface names are removed and the interface version number is
/// reset.
pub use xwayland_keyboard_grab_unstable_v1::*;
mod xwayland_keyboard_grab_unstable_v1 {
	use crate::*;
	
	// Copyright © 2017 Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_XWAYLAND_KEYBOARD_GRAB_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_xwayland_keyboard_grab_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "grab_keyboard\0".as_ptr(),
				signature: "noo\0".as_ptr(),
				types:     [&ZWP_XWAYLAND_KEYBOARD_GRAB_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # context object for keyboard grab manager
	///
	/// A global interface used for grabbing the keyboard.
	pub struct ZwpXwaylandKeyboardGrabManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpXwaylandKeyboardGrabManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpXwaylandKeyboardGrabManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpXwaylandKeyboardGrabManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpXwaylandKeyboardGrabManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpXwaylandKeyboardGrabManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the keyboard grab manager
		///
		/// Destroy the keyboard grab manager.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # grab the keyboard to a surface
		///
		/// The grab_keyboard request asks for a grab of the keyboard, forcing
		/// the keyboard focus for the given seat upon the given surface.
		///
		/// The protocol provides no guarantee that the grab is ever satisfied,
		/// and does not require the compositor to send an error if the grab
		/// cannot ever be satisfied. It is thus possible to request a keyboard
		/// grab that will never be effective.
		///
		/// The protocol:
		///
		/// * does not guarantee that the grab itself is applied for a surface,
		/// the grab request may be silently ignored by the compositor,
		/// * does not guarantee that any events are sent to this client even
		/// if the grab is applied to a surface,
		/// * does not guarantee that events sent to this client are exhaustive,
		/// a compositor may filter some events for its own consumption,
		/// * does not guarantee that events sent to this client are continuous,
		/// a compositor may change and reroute keyboard events while the grab
		/// is nominally active.
		pub fn grab_keyboard(
			&self,
			surface         : &WlSurface,
			seat            : &WlSeat
		) -> Result<Box<ZwpXwaylandKeyboardGrabV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_XWAYLAND_KEYBOARD_GRAB_V1_INTERFACE, std::ptr::null::<u8>(), surface, seat) as *mut ZwpXwaylandKeyboardGrabV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZWP_XWAYLAND_KEYBOARD_GRAB_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_xwayland_keyboard_grab_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # interface for grabbing the keyboard
	///
	/// A global interface used for grabbing the keyboard.
	pub struct ZwpXwaylandKeyboardGrabV1(WlProxy);
	
	impl std::ops::Deref for ZwpXwaylandKeyboardGrabV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpXwaylandKeyboardGrabV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpXwaylandKeyboardGrabV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpXwaylandKeyboardGrabV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpXwaylandKeyboardGrabV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the grabbed keyboard object
		///
		/// Destroy the grabbed keyboard object. If applicable, the compositor
		/// will ungrab the keyboard.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
}
pub use xdg_shell_unstable_v5::*;
mod xdg_shell_unstable_v5 {
	use crate::*;
	
	// Copyright © 2008-2013 Kristian Høgsberg
	// Copyright © 2013      Rafael Antognolli
	// Copyright © 2013      Jasper St. Pierre
	// Copyright © 2010-2013 Intel Corporation
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static XDG_SHELL_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_shell\0".as_ptr(),
		version:      1,
		method_count: 5,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "use_unstable_version\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_xdg_surface\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&XDG_SURFACE_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_xdg_popup\0".as_ptr(),
				signature: "nooouii\0".as_ptr(),
				types:     [&XDG_POPUP_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "pong\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "ping\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # create desktop-style surfaces
	///
	/// xdg_shell allows clients to turn a wl_surface into a "real window"
	/// which can be dragged, resized, stacked, and moved around by the
	/// user. Everything about this interface is suited towards traditional
	/// desktop environments.
	pub struct XdgShell(WlProxy);
	
	impl std::ops::Deref for XdgShell {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgShell {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgShell {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgShell")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgShell {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl XdgShellListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn XdgShellListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.ping((proxy as *mut XdgShell).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `ping` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn XdgShellListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy xdg_shell
		///
		/// Destroy this xdg_shell object.
		///
		/// Destroying a bound xdg_shell object while there are surfaces
		/// still alive created by this xdg_shell object instance is illegal
		/// and will result in a protocol error.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # enable use of this unstable version
		///
		/// Negotiate the unstable version of the interface.  This
		/// mechanism is in place to ensure client and server agree on the
		/// unstable versions of the protocol that they speak or exit
		/// cleanly if they don't agree.  This request will go away once
		/// the xdg-shell protocol is stable.
		pub fn use_unstable_version(
			&self,
			version         : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, version); }
		}
		
		/// # create a shell surface from a surface
		///
		/// This creates an xdg_surface for the given surface and gives it the
		/// xdg_surface role. A wl_surface can only be given an xdg_surface role
		/// once. If get_xdg_surface is called with a wl_surface that already has
		/// an active xdg_surface associated with it, or if it had any other role,
		/// an error is raised.
		///
		/// See the documentation of xdg_surface for more details about what an
		/// xdg_surface is and how it is used.
		pub fn get_xdg_surface(
			&self,
			surface         : &WlSurface
		) -> Result<Box<XdgSurface, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &XDG_SURFACE_INTERFACE, std::ptr::null::<u8>(), surface) as *mut XdgSurface };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # create a popup for a surface
		///
		/// This creates an xdg_popup for the given surface and gives it the
		/// xdg_popup role. A wl_surface can only be given an xdg_popup role
		/// once. If get_xdg_popup is called with a wl_surface that already has
		/// an active xdg_popup associated with it, or if it had any other role,
		/// an error is raised.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event.
		///
		/// See the documentation of xdg_popup for more details about what an
		/// xdg_popup is and how it is used.
		pub fn get_xdg_popup(
			&self,
			surface         : &WlSurface,
			parent          : &WlSurface,
			seat            : &WlSeat,
			serial          : u32,
			x               : i32,
			y               : i32
		) -> Result<Box<XdgPopup, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 3, &XDG_POPUP_INTERFACE, std::ptr::null::<u8>(), surface, parent, seat, serial, x, y) as *mut XdgPopup };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # respond to a ping event
		///
		/// A client must respond to a ping event with a pong request or
		/// the client may be deemed unresponsive.
		pub fn pong(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, serial); }
		}
	}
	
	
	pub trait XdgShellListener: std::any::Any {
		
		/// # check if the client is alive
		///
		/// The ping event asks the client if it's still alive. Pass the
		/// serial specified in the event back to the compositor by sending
		/// a "pong" request back with the specified serial.
		///
		/// Compositors can use this to determine if the client is still
		/// alive. It's unspecified what will happen if the client doesn't
		/// respond to the ping request, or in what timeframe. Clients should
		/// try to respond in a reasonable amount of time.
		///
		/// A compositor is free to ping in any way it wants, but a client must
		/// always respond to any xdg_shell object it created.
		fn ping(
			&self,
			proxy: &mut XdgShell,
			serial          : u32,
		);
	}
	
	/// # latest protocol version
	///
	/// The 'current' member of this enum gives the version of the
	/// protocol.  Implementations can compare this to the version
	/// they implement using static_assert to ensure the protocol and
	/// implementation versions match.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgShellVersion {
		/// Always the latest version
		Current = 5,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgShellError {
		/// given wl_surface has another role
		Role = 0,
		/// xdg_shell was destroyed before children
		DefunctSurfaces = 1,
		/// the client tried to map or destroy a non-topmost popup
		NotTheTopmostPopup = 2,
		/// the client specified an invalid popup parent surface
		InvalidPopupParent = 3,
	}
	
	pub static XDG_SURFACE_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_surface\0".as_ptr(),
		version:      1,
		method_count: 14,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_parent\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&XDG_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_title\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_app_id\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "show_window_menu\0".as_ptr(),
				signature: "ouii\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "move\0".as_ptr(),
				signature: "ou\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "resize\0".as_ptr(),
				signature: "ouu\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "ack_configure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_window_geometry\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_maximized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "unset_maximized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_fullscreen\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "unset_fullscreen\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_minimized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "iiau\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "close\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # A desktop window
	///
	/// An interface that may be implemented by a wl_surface, for
	/// implementations that provide a desktop-style user interface.
	///
	/// It provides requests to treat surfaces like windows, allowing to set
	/// properties like maximized, fullscreen, minimized, and to move and resize
	/// them, and associate metadata like title and app id.
	///
	/// The client must call wl_surface.commit on the corresponding wl_surface
	/// for the xdg_surface state to take effect. Prior to committing the new
	/// state, it can set up initial configuration, such as maximizing or setting
	/// a window geometry.
	///
	/// Even without attaching a buffer the compositor must respond to initial
	/// committed configuration, for instance sending a configure event with
	/// expected window geometry if the client maximized its surface during
	/// initialization.
	///
	/// For a surface to be mapped by the compositor the client must have
	/// committed both an xdg_surface state and a buffer.
	pub struct XdgSurface(WlProxy);
	
	impl std::ops::Deref for XdgSurface {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgSurface {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgSurface {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgSurface")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgSurface {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl XdgSurfaceListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn XdgSurfaceListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut XdgSurface).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).a.as_ref().unwrap(), (*args.add(3)).u, ),
						1 => listener.close((proxy as *mut XdgSurface).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: width: {:?}, height: {:?}, states: {:?}, serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).a.as_ref().unwrap(), (*args.add(3)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `close` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn XdgSurfaceListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # Destroy the xdg_surface
		///
		/// Unmap and destroy the window. The window will be effectively
		/// hidden from the user's point of view, and all state like
		/// maximization, fullscreen, and so on, will be lost.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the parent of this surface
		///
		/// Set the "parent" of this surface. This window should be stacked
		/// above a parent. The parent surface must be mapped as long as this
		/// surface is mapped.
		///
		/// Parent windows should be set on dialogs, toolboxes, or other
		/// "auxiliary" surfaces, so that the parent is raised when the dialog
		/// is raised.
		pub fn set_parent(
			&self,
			parent          : Option<&XdgSurface>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, parent.map_or(std::ptr::null_mut(), |r| r as *const XdgSurface as *mut XdgSurface)); }
		}
		
		/// # set surface title
		///
		/// Set a short title for the surface.
		///
		/// This string may be used to identify the surface in a task bar,
		/// window list, or other user interface elements provided by the
		/// compositor.
		///
		/// The string must be encoded in UTF-8.
		pub fn set_title(
			&self,
			title           : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, title.as_ptr()); }
		}
		
		/// # set application ID
		///
		/// Set an application identifier for the surface.
		///
		/// The app ID identifies the general class of applications to which
		/// the surface belongs. The compositor can use this to group multiple
		/// surfaces together, or to determine how to launch a new application.
		///
		/// For D-Bus activatable applications, the app ID is used as the D-Bus
		/// service name.
		///
		/// The compositor shell will try to group application surfaces together
		/// by their app ID.  As a best practice, it is suggested to select app
		/// ID's that match the basename of the application's .desktop file.
		/// For example, "org.freedesktop.FooViewer" where the .desktop file is
		/// "org.freedesktop.FooViewer.desktop".
		///
		/// See the desktop-entry specification [0] for more details on
		/// application identifiers and how they relate to well-known D-Bus
		/// names and .desktop files.
		///
		/// [0] http://standards.freedesktop.org/desktop-entry-spec/
		pub fn set_app_id(
			&self,
			app_id          : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, app_id.as_ptr()); }
		}
		
		/// # show the window menu
		///
		/// Clients implementing client-side decorations might want to show
		/// a context menu when right-clicking on the decorations, giving the
		/// user a menu that they can use to maximize or minimize the window.
		///
		/// This request asks the compositor to pop up such a window menu at
		/// the given position, relative to the local surface coordinates of
		/// the parent surface. There are no guarantees as to what menu items
		/// the window menu contains.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event.
		pub fn show_window_menu(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			x               : i32,
			y               : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, seat, serial, x, y); }
		}
		
		/// # start an interactive move
		///
		/// Start an interactive, user-driven move of the surface.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event. The passed
		/// serial is used to determine the type of interactive move (touch,
		/// pointer, etc).
		///
		/// The server may ignore move requests depending on the state of
		/// the surface (e.g. fullscreen or maximized), or if the passed serial
		/// is no longer valid.
		///
		/// If triggered, the surface will lose the focus of the device
		/// (wl_pointer, wl_touch, etc) used for the move. It is up to the
		/// compositor to visually indicate that the move is taking place, such as
		/// updating a pointer cursor, during the move. There is no guarantee
		/// that the device focus will return when the move is completed.
		pub fn r#move(
			&self,
			seat            : &WlSeat,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, seat, serial); }
		}
		
		/// # start an interactive resize
		///
		/// Start a user-driven, interactive resize of the surface.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event. The passed
		/// serial is used to determine the type of interactive resize (touch,
		/// pointer, etc).
		///
		/// The server may ignore resize requests depending on the state of
		/// the surface (e.g. fullscreen or maximized).
		///
		/// If triggered, the client will receive configure events with the
		/// "resize" state enum value and the expected sizes. See the "resize"
		/// enum value for more details about what is required. The client
		/// must also acknowledge configure events using "ack_configure". After
		/// the resize is completed, the client will receive another "configure"
		/// event without the resize state.
		///
		/// If triggered, the surface also will lose the focus of the device
		/// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
		/// compositor to visually indicate that the resize is taking place,
		/// such as updating a pointer cursor, during the resize. There is no
		/// guarantee that the device focus will return when the resize is
		/// completed.
		///
		/// The edges parameter specifies how the surface should be resized,
		/// and is one of the values of the resize_edge enum. The compositor
		/// may use this information to update the surface position for
		/// example when dragging the top left corner. The compositor may also
		/// use this information to adapt its behavior, e.g. choose an
		/// appropriate cursor image.
		pub fn resize(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			edges           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, seat, serial, edges); }
		}
		
		/// # ack a configure event
		///
		/// When a configure event is received, if a client commits the
		/// surface in response to the configure event, then the client
		/// must make an ack_configure request sometime before the commit
		/// request, passing along the serial of the configure event.
		///
		/// For instance, the compositor might use this information to move
		/// a surface to the top left only when the client has drawn itself
		/// for the maximized or fullscreen state.
		///
		/// If the client receives multiple configure events before it
		/// can respond to one, it only has to ack the last configure event.
		///
		/// A client is not required to commit immediately after sending
		/// an ack_configure request - it may even ack_configure several times
		/// before its next surface commit.
		///
		/// The compositor expects that the most recently received
		/// ack_configure request at the time of a commit indicates which
		/// configure event the client is responding to.
		pub fn ack_configure(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7, serial); }
		}
		
		/// # set the new window geometry
		///
		/// The window geometry of a window is its "visible bounds" from the
		/// user's perspective. Client-side decorations often have invisible
		/// portions like drop-shadows which should be ignored for the
		/// purposes of aligning, placing and constraining windows.
		///
		/// The window geometry is double buffered, and will be applied at the
		/// time wl_surface.commit of the corresponding wl_surface is called.
		///
		/// Once the window geometry of the surface is set once, it is not
		/// possible to unset it, and it will remain the same until
		/// set_window_geometry is called again, even if a new subsurface or
		/// buffer is attached.
		///
		/// If never set, the value is the full bounds of the surface,
		/// including any subsurfaces. This updates dynamically on every
		/// commit. This unset mode is meant for extremely simple clients.
		///
		/// If responding to a configure event, the window geometry in here
		/// must respect the sizing negotiations specified by the states in
		/// the configure event.
		///
		/// The arguments are given in the surface local coordinate space of
		/// the wl_surface associated with this xdg_surface.
		///
		/// The width and height must be greater than zero.
		pub fn set_window_geometry(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 8, x, y, width, height); }
		}
		
		/// # maximize the window
		///
		/// Maximize the surface.
		///
		/// After requesting that the surface should be maximized, the compositor
		/// will respond by emitting a configure event with the "maximized" state
		/// and the required window geometry. The client should then update its
		/// content, drawing it in a maximized state, i.e. without shadow or other
		/// decoration outside of the window geometry. The client must also
		/// acknowledge the configure when committing the new content (see
		/// ack_configure).
		///
		/// It is up to the compositor to decide how and where to maximize the
		/// surface, for example which output and what region of the screen should
		/// be used.
		///
		/// If the surface was already maximized, the compositor will still emit
		/// a configure event with the "maximized" state.
		pub fn set_maximized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 9); }
		}
		
		/// # unmaximize the window
		///
		/// Unmaximize the surface.
		///
		/// After requesting that the surface should be unmaximized, the compositor
		/// will respond by emitting a configure event without the "maximized"
		/// state. If available, the compositor will include the window geometry
		/// dimensions the window had prior to being maximized in the configure
		/// request. The client must then update its content, drawing it in a
		/// regular state, i.e. potentially with shadow, etc. The client must also
		/// acknowledge the configure when committing the new content (see
		/// ack_configure).
		///
		/// It is up to the compositor to position the surface after it was
		/// unmaximized; usually the position the surface had before maximizing, if
		/// applicable.
		///
		/// If the surface was already not maximized, the compositor will still
		/// emit a configure event without the "maximized" state.
		pub fn unset_maximized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 10); }
		}
		
		/// # set the window as fullscreen on a monitor
		///
		/// Make the surface fullscreen.
		///
		/// You can specify an output that you would prefer to be fullscreen.
		/// If this value is NULL, it's up to the compositor to choose which
		/// display will be used to map this surface.
		///
		/// If the surface doesn't cover the whole output, the compositor will
		/// position the surface in the center of the output and compensate with
		/// black borders filling the rest of the output.
		pub fn set_fullscreen(
			&self,
			output          : Option<&WlOutput>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 11, output.map_or(std::ptr::null_mut(), |r| r as *const WlOutput as *mut WlOutput)); }
		}
		pub fn unset_fullscreen(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 12); }
		}
		
		/// # set the window as minimized
		///
		/// Request that the compositor minimize your surface. There is no
		/// way to know if the surface is currently minimized, nor is there
		/// any way to unset minimization on this surface.
		///
		/// If you are looking to throttle redrawing when minimized, please
		/// instead use the wl_surface.frame event for this, as this will
		/// also work with live previews on windows in Alt-Tab, Expose or
		/// similar compositor features.
		pub fn set_minimized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 13); }
		}
	}
	
	
	pub trait XdgSurfaceListener: std::any::Any {
		
		/// # suggest a surface change
		///
		/// The configure event asks the client to resize its surface or to
		/// change its state.
		///
		/// The width and height arguments specify a hint to the window
		/// about how its surface should be resized in window geometry
		/// coordinates. See set_window_geometry.
		///
		/// If the width or height arguments are zero, it means the client
		/// should decide its own window dimension. This may happen when the
		/// compositor need to configure the state of the surface but doesn't
		/// have any information about any previous or expected dimension.
		///
		/// The states listed in the event specify how the width/height
		/// arguments should be interpreted, and possibly how it should be
		/// drawn.
		///
		/// Clients should arrange their surface for the new size and
		/// states, and then send a ack_configure request with the serial
		/// sent in this configure event at some point before committing
		/// the new surface.
		///
		/// If the client receives multiple configure events before it
		/// can respond to one, it is free to discard all but the last
		/// event it received.
		fn configure(
			&self,
			proxy: &mut XdgSurface,
			width           : i32,
			height          : i32,
			states          : &WlArray,
			serial          : u32,
		);
		
		/// # surface wants to be closed
		///
		/// The close event is sent by the compositor when the user
		/// wants the surface to be closed. This should be equivalent to
		/// the user clicking the close button in client-side decorations,
		/// if your application has any...
		///
		/// This is only a request that the user intends to close your
		/// window. The client may choose to ignore this request, or show
		/// a dialog to ask the user to save their data...
		fn close(
			&self,
			proxy: &mut XdgSurface,
		);
	}
	
	/// # edge values for resizing
	///
	/// These values are used to indicate which edge of a surface
	/// is being dragged in a resize operation.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgSurfaceResizeEdge {
		///
		None = 0,
		///
		Top = 1,
		///
		Bottom = 2,
		///
		Left = 4,
		///
		TopLeft = 5,
		///
		BottomLeft = 6,
		///
		Right = 8,
		///
		TopRight = 9,
		///
		BottomRight = 10,
	}
	
	/// # types of state on the surface
	///
	/// The different state values used on the surface. This is designed for
	/// state values like maximized, fullscreen. It is paired with the
	/// configure event to ensure that both the client and the compositor
	/// setting the state can be synchronized.
	///
	/// States set in this way are double-buffered. They will get applied on
	/// the next commit.
	///
	/// Desktop environments may extend this enum by taking up a range of
	/// values and documenting the range they chose in this description.
	/// They are not required to document the values for the range that they
	/// chose. Ideally, any good extensions from a desktop environment should
	/// make its way into standardization into this enum.
	///
	/// The current reserved ranges are:
	///
	/// 0x0000 - 0x0FFF: xdg-shell core values, documented below.
	/// 0x1000 - 0x1FFF: GNOME
	/// 0x2000 - 0x2FFF: EFL
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgSurfaceState {
		/// the surface is maximized
		Maximized = 1,
		/// the surface is fullscreen
		Fullscreen = 2,
		/// the surface is being resized
		Resizing = 3,
		/// the surface is now activated
		Activated = 4,
	}
	
	pub static XDG_POPUP_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_popup\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "popup_done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # short-lived, popup surfaces for menus
	///
	/// A popup surface is a short-lived, temporary surface that can be
	/// used to implement menus. It takes an explicit grab on the surface
	/// that will be dismissed when the user dismisses the popup. This can
	/// be done by the user clicking outside the surface, using the keyboard,
	/// or even locking the screen through closing the lid or a timeout.
	///
	/// When the popup is dismissed, a popup_done event will be sent out,
	/// and at the same time the surface will be unmapped. The xdg_popup
	/// object is now inert and cannot be reactivated, so clients should
	/// destroy it. Explicitly destroying the xdg_popup object will also
	/// dismiss the popup and unmap the surface.
	///
	/// Clients will receive events for all their surfaces during this
	/// grab (which is an "owner-events" grab in X11 parlance). This is
	/// done so that users can navigate through submenus and other
	/// "nested" popup windows without having to dismiss the topmost
	/// popup.
	///
	/// Clients that want to dismiss the popup when another surface of
	/// their own is clicked should dismiss the popup using the destroy
	/// request.
	///
	/// The parent surface must have either an xdg_surface or xdg_popup
	/// role.
	///
	/// Specifying an xdg_popup for the parent means that the popups are
	/// nested, with this popup now being the topmost popup. Nested
	/// popups must be destroyed in the reverse order they were created
	/// in, e.g. the only popup you are allowed to destroy at all times
	/// is the topmost one.
	///
	/// If there is an existing popup when creating a new popup, the
	/// parent must be the current topmost popup.
	///
	/// A parent surface must be mapped before the new popup is mapped.
	///
	/// When compositors choose to dismiss a popup, they will likely
	/// dismiss every nested popup as well. When a compositor dismisses
	/// popups, it will follow the same dismissing order as required
	/// from the client.
	///
	/// The x and y arguments passed when creating the popup object specify
	/// where the top left of the popup should be placed, relative to the
	/// local surface coordinates of the parent surface. See
	/// xdg_shell.get_xdg_popup.
	///
	/// The client must call wl_surface.commit on the corresponding wl_surface
	/// for the xdg_popup state to take effect.
	///
	/// For a surface to be mapped by the compositor the client must have
	/// committed both the xdg_popup state and a buffer.
	pub struct XdgPopup(WlProxy);
	
	impl std::ops::Deref for XdgPopup {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgPopup {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgPopup {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgPopup")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgPopup {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl XdgPopupListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn XdgPopupListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.popup_done((proxy as *mut XdgPopup).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `popup_done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn XdgPopupListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # remove xdg_popup interface
		///
		/// This destroys the popup. Explicitly destroying the xdg_popup
		/// object will also dismiss the popup, and unmap the surface.
		///
		/// If this xdg_popup is not the "topmost" popup, a protocol error
		/// will be sent.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait XdgPopupListener: std::any::Any {
		
		/// # popup interaction is done
		///
		/// The popup_done event is sent out when a popup is dismissed by the
		/// compositor. The client should destroy the xdg_popup object at this
		/// point.
		fn popup_done(
			&self,
			proxy: &mut XdgPopup,
		);
	}
}
pub use xdg_shell_unstable_v6::*;
mod xdg_shell_unstable_v6 {
	use crate::*;
	
	// Copyright © 2008-2013 Kristian Høgsberg
	// Copyright © 2013      Rafael Antognolli
	// Copyright © 2013      Jasper St. Pierre
	// Copyright © 2010-2013 Intel Corporation
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZXDG_SHELL_V6_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_shell_v6\0".as_ptr(),
		version:      1,
		method_count: 4,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "create_positioner\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZXDG_POSITIONER_V6_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_xdg_surface\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZXDG_SURFACE_V6_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "pong\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "ping\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # create desktop-style surfaces
	///
	/// xdg_shell allows clients to turn a wl_surface into a "real window"
	/// which can be dragged, resized, stacked, and moved around by the
	/// user. Everything about this interface is suited towards traditional
	/// desktop environments.
	pub struct ZxdgShellV6(WlProxy);
	
	impl std::ops::Deref for ZxdgShellV6 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgShellV6 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgShellV6 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgShellV6")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgShellV6 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgShellV6Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgShellV6Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.ping((proxy as *mut ZxdgShellV6).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `ping` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgShellV6Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy xdg_shell
		///
		/// Destroy this xdg_shell object.
		///
		/// Destroying a bound xdg_shell object while there are surfaces
		/// still alive created by this xdg_shell object instance is illegal
		/// and will result in a protocol error.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a positioner object
		///
		/// Create a positioner object. A positioner object is used to position
		/// surfaces relative to some parent surface. See the interface description
		/// and xdg_surface.get_popup for details.
		pub fn create_positioner(
			&self
		) -> Result<Box<ZxdgPositionerV6, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_POSITIONER_V6_INTERFACE, std::ptr::null::<u8>()) as *mut ZxdgPositionerV6 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # create a shell surface from a surface
		///
		/// This creates an xdg_surface for the given surface. While xdg_surface
		/// itself is not a role, the corresponding surface may only be assigned
		/// a role extending xdg_surface, such as xdg_toplevel or xdg_popup.
		///
		/// This creates an xdg_surface for the given surface. An xdg_surface is
		/// used as basis to define a role to a given surface, such as xdg_toplevel
		/// or xdg_popup. It also manages functionality shared between xdg_surface
		/// based surface roles.
		///
		/// See the documentation of xdg_surface for more details about what an
		/// xdg_surface is and how it is used.
		pub fn get_xdg_surface(
			&self,
			surface         : &WlSurface
		) -> Result<Box<ZxdgSurfaceV6, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &ZXDG_SURFACE_V6_INTERFACE, std::ptr::null::<u8>(), surface) as *mut ZxdgSurfaceV6 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # respond to a ping event
		///
		/// A client must respond to a ping event with a pong request or
		/// the client may be deemed unresponsive. See xdg_shell.ping.
		pub fn pong(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, serial); }
		}
	}
	
	
	pub trait ZxdgShellV6Listener: std::any::Any {
		
		/// # check if the client is alive
		///
		/// The ping event asks the client if it's still alive. Pass the
		/// serial specified in the event back to the compositor by sending
		/// a "pong" request back with the specified serial. See xdg_shell.ping.
		///
		/// Compositors can use this to determine if the client is still
		/// alive. It's unspecified what will happen if the client doesn't
		/// respond to the ping request, or in what timeframe. Clients should
		/// try to respond in a reasonable amount of time.
		///
		/// A compositor is free to ping in any way it wants, but a client must
		/// always respond to any xdg_shell object it created.
		fn ping(
			&self,
			proxy: &mut ZxdgShellV6,
			serial          : u32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgShellV6Error {
		/// given wl_surface has another role
		Role = 0,
		/// xdg_shell was destroyed before children
		DefunctSurfaces = 1,
		/// the client tried to map or destroy a non-topmost popup
		NotTheTopmostPopup = 2,
		/// the client specified an invalid popup parent surface
		InvalidPopupParent = 3,
		/// the client provided an invalid surface state
		InvalidSurfaceState = 4,
		/// the client provided an invalid positioner
		InvalidPositioner = 5,
	}
	
	pub static ZXDG_POSITIONER_V6_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_positioner_v6\0".as_ptr(),
		version:      1,
		method_count: 7,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_size\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_anchor_rect\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_anchor\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_gravity\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_constraint_adjustment\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_offset\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # child surface positioner
	///
	/// The xdg_positioner provides a collection of rules for the placement of a
	/// child surface relative to a parent surface. Rules can be defined to ensure
	/// the child surface remains within the visible area's borders, and to
	/// specify how the child surface changes its position, such as sliding along
	/// an axis, or flipping around a rectangle. These positioner-created rules are
	/// constrained by the requirement that a child surface must intersect with or
	/// be at least partially adjacent to its parent surface.
	///
	/// See the various requests for details about possible rules.
	///
	/// At the time of the request, the compositor makes a copy of the rules
	/// specified by the xdg_positioner. Thus, after the request is complete the
	/// xdg_positioner object can be destroyed or reused; further changes to the
	/// object will have no effect on previous usages.
	///
	/// For an xdg_positioner object to be considered complete, it must have a
	/// non-zero size set by set_size, and a non-zero anchor rectangle set by
	/// set_anchor_rect. Passing an incomplete xdg_positioner object when
	/// positioning a surface raises an error.
	pub struct ZxdgPositionerV6(WlProxy);
	
	impl std::ops::Deref for ZxdgPositionerV6 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgPositionerV6 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgPositionerV6 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgPositionerV6")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgPositionerV6 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the xdg_positioner object
		///
		/// Notify the compositor that the xdg_positioner will no longer be used.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the size of the to-be positioned rectangle
		///
		/// Set the size of the surface that is to be positioned with the positioner
		/// object. The size is in surface-local coordinates and corresponds to the
		/// window geometry. See xdg_surface.set_window_geometry.
		///
		/// If a zero or negative size is set the invalid_input error is raised.
		pub fn set_size(
			&self,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, width, height); }
		}
		
		/// # set the anchor rectangle within the parent surface
		///
		/// Specify the anchor rectangle within the parent surface that the child
		/// surface will be placed relative to. The rectangle is relative to the
		/// window geometry as defined by xdg_surface.set_window_geometry of the
		/// parent surface. The rectangle must be at least 1x1 large.
		///
		/// When the xdg_positioner object is used to position a child surface, the
		/// anchor rectangle may not extend outside the window geometry of the
		/// positioned child's parent surface.
		///
		/// If a zero or negative size is set the invalid_input error is raised.
		pub fn set_anchor_rect(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, x, y, width, height); }
		}
		
		/// # set anchor rectangle anchor edges
		///
		/// Defines a set of edges for the anchor rectangle. These are used to
		/// derive an anchor point that the child surface will be positioned
		/// relative to. If two orthogonal edges are specified (e.g. 'top' and
		/// 'left'), then the anchor point will be the intersection of the edges
		/// (e.g. the top left position of the rectangle); otherwise, the derived
		/// anchor point will be centered on the specified edge, or in the center of
		/// the anchor rectangle if no edge is specified.
		///
		/// If two parallel anchor edges are specified (e.g. 'left' and 'right'),
		/// the invalid_input error is raised.
		pub fn set_anchor(
			&self,
			anchor          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, anchor); }
		}
		
		/// # set child surface gravity
		///
		/// Defines in what direction a surface should be positioned, relative to
		/// the anchor point of the parent surface. If two orthogonal gravities are
		/// specified (e.g. 'bottom' and 'right'), then the child surface will be
		/// placed in the specified direction; otherwise, the child surface will be
		/// centered over the anchor point on any axis that had no gravity
		/// specified.
		///
		/// If two parallel gravities are specified (e.g. 'left' and 'right'), the
		/// invalid_input error is raised.
		pub fn set_gravity(
			&self,
			gravity         : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, gravity); }
		}
		
		/// # set the adjustment to be done when constrained
		///
		/// Specify how the window should be positioned if the originally intended
		/// position caused the surface to be constrained, meaning at least
		/// partially outside positioning boundaries set by the compositor. The
		/// adjustment is set by constructing a bitmask describing the adjustment to
		/// be made when the surface is constrained on that axis.
		///
		/// If no bit for one axis is set, the compositor will assume that the child
		/// surface should not change its position on that axis when constrained.
		///
		/// If more than one bit for one axis is set, the order of how adjustments
		/// are applied is specified in the corresponding adjustment descriptions.
		///
		/// The default adjustment is none.
		pub fn set_constraint_adjustment(
			&self,
			constraint_adjustment: u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, constraint_adjustment); }
		}
		
		/// # set surface position offset
		///
		/// Specify the surface position offset relative to the position of the
		/// anchor on the anchor rectangle and the anchor on the surface. For
		/// example if the anchor of the anchor rectangle is at (x, y), the surface
		/// has the gravity bottom|right, and the offset is (ox, oy), the calculated
		/// surface position will be (x + ox, y + oy). The offset position of the
		/// surface is the one used for constraint testing. See
		/// set_constraint_adjustment.
		///
		/// An example use case is placing a popup menu on top of a user interface
		/// element, while aligning the user interface element of the parent surface
		/// with some user interface element placed somewhere in the popup surface.
		pub fn set_offset(
			&self,
			x               : i32,
			y               : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, x, y); }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgPositionerV6Error {
		/// invalid input provided
		InvalidInput = 0,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgPositionerV6Anchor {
		/// the center of the anchor rectangle
		None = 0,
		/// the top edge of the anchor rectangle
		Top = 1,
		/// the bottom edge of the anchor rectangle
		Bottom = 2,
		/// the left edge of the anchor rectangle
		Left = 4,
		/// the right edge of the anchor rectangle
		Right = 8,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgPositionerV6Gravity {
		/// center over the anchor edge
		None = 0,
		/// position above the anchor edge
		Top = 1,
		/// position below the anchor edge
		Bottom = 2,
		/// position to the left of the anchor edge
		Left = 4,
		/// position to the right of the anchor edge
		Right = 8,
	}
	
	/// # constraint adjustments
	///
	/// The constraint adjustment value define ways the compositor will adjust
	/// the position of the surface, if the unadjusted position would result
	/// in the surface being partly constrained.
	///
	/// Whether a surface is considered 'constrained' is left to the compositor
	/// to determine. For example, the surface may be partly outside the
	/// compositor's defined 'work area', thus necessitating the child surface's
	/// position be adjusted until it is entirely inside the work area.
	///
	/// The adjustments can be combined, according to a defined precedence: 1)
	/// Flip, 2) Slide, 3) Resize.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgPositionerV6ConstraintAdjustment {
		///
		None = 0,
		///
		SlideX = 1,
		///
		SlideY = 2,
		///
		FlipX = 4,
		///
		FlipY = 8,
		///
		ResizeX = 16,
		///
		ResizeY = 32,
	}
	
	pub static ZXDG_SURFACE_V6_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_surface_v6\0".as_ptr(),
		version:      1,
		method_count: 5,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_toplevel\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZXDG_TOPLEVEL_V6_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_popup\0".as_ptr(),
				signature: "noo\0".as_ptr(),
				types:     [&ZXDG_POPUP_V6_INTERFACE as _, &ZXDG_SURFACE_V6_INTERFACE as _, &ZXDG_POSITIONER_V6_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_window_geometry\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "ack_configure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # desktop user interface surface base interface
	///
	/// An interface that may be implemented by a wl_surface, for
	/// implementations that provide a desktop-style user interface.
	///
	/// It provides a base set of functionality required to construct user
	/// interface elements requiring management by the compositor, such as
	/// toplevel windows, menus, etc. The types of functionality are split into
	/// xdg_surface roles.
	///
	/// Creating an xdg_surface does not set the role for a wl_surface. In order
	/// to map an xdg_surface, the client must create a role-specific object
	/// using, e.g., get_toplevel, get_popup. The wl_surface for any given
	/// xdg_surface can have at most one role, and may not be assigned any role
	/// not based on xdg_surface.
	///
	/// A role must be assigned before any other requests are made to the
	/// xdg_surface object.
	///
	/// The client must call wl_surface.commit on the corresponding wl_surface
	/// for the xdg_surface state to take effect.
	///
	/// Creating an xdg_surface from a wl_surface which has a buffer attached or
	/// committed is a client error, and any attempts by a client to attach or
	/// manipulate a buffer prior to the first xdg_surface.configure call must
	/// also be treated as errors.
	///
	/// For a surface to be mapped by the compositor, the following conditions
	/// must be met: (1) the client has assigned a xdg_surface based role to the
	/// surface, (2) the client has set and committed the xdg_surface state and
	/// the role dependent state to the surface and (3) the client has committed a
	/// buffer to the surface.
	pub struct ZxdgSurfaceV6(WlProxy);
	
	impl std::ops::Deref for ZxdgSurfaceV6 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgSurfaceV6 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgSurfaceV6 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgSurfaceV6")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgSurfaceV6 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgSurfaceV6Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgSurfaceV6Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut ZxdgSurfaceV6).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgSurfaceV6Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the xdg_surface
		///
		/// Destroy the xdg_surface object. An xdg_surface must only be destroyed
		/// after its role object has been destroyed.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # assign the xdg_toplevel surface role
		///
		/// This creates an xdg_toplevel object for the given xdg_surface and gives
		/// the associated wl_surface the xdg_toplevel role.
		///
		/// See the documentation of xdg_toplevel for more details about what an
		/// xdg_toplevel is and how it is used.
		pub fn get_toplevel(
			&self
		) -> Result<Box<ZxdgToplevelV6, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_TOPLEVEL_V6_INTERFACE, std::ptr::null::<u8>()) as *mut ZxdgToplevelV6 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # assign the xdg_popup surface role
		///
		/// This creates an xdg_popup object for the given xdg_surface and gives the
		/// associated wl_surface the xdg_popup role.
		///
		/// See the documentation of xdg_popup for more details about what an
		/// xdg_popup is and how it is used.
		pub fn get_popup(
			&self,
			parent          : &ZxdgSurfaceV6,
			positioner      : &ZxdgPositionerV6
		) -> Result<Box<ZxdgPopupV6, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &ZXDG_POPUP_V6_INTERFACE, std::ptr::null::<u8>(), parent, positioner) as *mut ZxdgPopupV6 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # set the new window geometry
		///
		/// The window geometry of a surface is its "visible bounds" from the
		/// user's perspective. Client-side decorations often have invisible
		/// portions like drop-shadows which should be ignored for the
		/// purposes of aligning, placing and constraining windows.
		///
		/// The window geometry is double buffered, and will be applied at the
		/// time wl_surface.commit of the corresponding wl_surface is called.
		///
		/// Once the window geometry of the surface is set, it is not possible to
		/// unset it, and it will remain the same until set_window_geometry is
		/// called again, even if a new subsurface or buffer is attached.
		///
		/// If never set, the value is the full bounds of the surface,
		/// including any subsurfaces. This updates dynamically on every
		/// commit. This unset is meant for extremely simple clients.
		///
		/// The arguments are given in the surface-local coordinate space of
		/// the wl_surface associated with this xdg_surface.
		///
		/// The width and height must be greater than zero. Setting an invalid size
		/// will raise an error. When applied, the effective window geometry will be
		/// the set window geometry clamped to the bounding rectangle of the
		/// combined geometry of the surface of the xdg_surface and the associated
		/// subsurfaces.
		pub fn set_window_geometry(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, x, y, width, height); }
		}
		
		/// # ack a configure event
		///
		/// When a configure event is received, if a client commits the
		/// surface in response to the configure event, then the client
		/// must make an ack_configure request sometime before the commit
		/// request, passing along the serial of the configure event.
		///
		/// For instance, for toplevel surfaces the compositor might use this
		/// information to move a surface to the top left only when the client has
		/// drawn itself for the maximized or fullscreen state.
		///
		/// If the client receives multiple configure events before it
		/// can respond to one, it only has to ack the last configure event.
		///
		/// A client is not required to commit immediately after sending
		/// an ack_configure request - it may even ack_configure several times
		/// before its next surface commit.
		///
		/// A client may send multiple ack_configure requests before committing, but
		/// only the last request sent before a commit indicates which configure
		/// event the client really is responding to.
		pub fn ack_configure(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, serial); }
		}
	}
	
	
	pub trait ZxdgSurfaceV6Listener: std::any::Any {
		
		/// # suggest a surface change
		///
		/// The configure event marks the end of a configure sequence. A configure
		/// sequence is a set of one or more events configuring the state of the
		/// xdg_surface, including the final xdg_surface.configure event.
		///
		/// Where applicable, xdg_surface surface roles will during a configure
		/// sequence extend this event as a latched state sent as events before the
		/// xdg_surface.configure event. Such events should be considered to make up
		/// a set of atomically applied configuration states, where the
		/// xdg_surface.configure commits the accumulated state.
		///
		/// Clients should arrange their surface for the new states, and then send
		/// an ack_configure request with the serial sent in this configure event at
		/// some point before committing the new surface.
		///
		/// If the client receives multiple configure events before it can respond
		/// to one, it is free to discard all but the last event it received.
		fn configure(
			&self,
			proxy: &mut ZxdgSurfaceV6,
			serial          : u32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgSurfaceV6Error {
		///
		NotConstructed = 1,
		///
		AlreadyConstructed = 2,
		///
		UnconfiguredBuffer = 3,
	}
	
	pub static ZXDG_TOPLEVEL_V6_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_toplevel_v6\0".as_ptr(),
		version:      1,
		method_count: 14,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_parent\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&ZXDG_TOPLEVEL_V6_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_title\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_app_id\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "show_window_menu\0".as_ptr(),
				signature: "ouii\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "move\0".as_ptr(),
				signature: "ou\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "resize\0".as_ptr(),
				signature: "ouu\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_max_size\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_min_size\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_maximized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "unset_maximized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_fullscreen\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "unset_fullscreen\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_minimized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "iia\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "close\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # toplevel surface
	///
	/// This interface defines an xdg_surface role which allows a surface to,
	/// among other things, set window-like properties such as maximize,
	/// fullscreen, and minimize, set application-specific metadata like title and
	/// id, and well as trigger user interactive operations such as interactive
	/// resize and move.
	pub struct ZxdgToplevelV6(WlProxy);
	
	impl std::ops::Deref for ZxdgToplevelV6 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgToplevelV6 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgToplevelV6 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgToplevelV6")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgToplevelV6 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgToplevelV6Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgToplevelV6Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut ZxdgToplevelV6).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).a.as_ref().unwrap(), ),
						1 => listener.close((proxy as *mut ZxdgToplevelV6).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: width: {:?}, height: {:?}, states: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).a.as_ref().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `close` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgToplevelV6Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the xdg_toplevel
		///
		/// Unmap and destroy the window. The window will be effectively
		/// hidden from the user's point of view, and all state like
		/// maximization, fullscreen, and so on, will be lost.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the parent of this surface
		///
		/// Set the "parent" of this surface. This window should be stacked
		/// above a parent. The parent surface must be mapped as long as this
		/// surface is mapped.
		///
		/// Parent windows should be set on dialogs, toolboxes, or other
		/// "auxiliary" surfaces, so that the parent is raised when the dialog
		/// is raised.
		pub fn set_parent(
			&self,
			parent          : Option<&ZxdgToplevelV6>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, parent.map_or(std::ptr::null_mut(), |r| r as *const ZxdgToplevelV6 as *mut ZxdgToplevelV6)); }
		}
		
		/// # set surface title
		///
		/// Set a short title for the surface.
		///
		/// This string may be used to identify the surface in a task bar,
		/// window list, or other user interface elements provided by the
		/// compositor.
		///
		/// The string must be encoded in UTF-8.
		pub fn set_title(
			&self,
			title           : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, title.as_ptr()); }
		}
		
		/// # set application ID
		///
		/// Set an application identifier for the surface.
		///
		/// The app ID identifies the general class of applications to which
		/// the surface belongs. The compositor can use this to group multiple
		/// surfaces together, or to determine how to launch a new application.
		///
		/// For D-Bus activatable applications, the app ID is used as the D-Bus
		/// service name.
		///
		/// The compositor shell will try to group application surfaces together
		/// by their app ID. As a best practice, it is suggested to select app
		/// ID's that match the basename of the application's .desktop file.
		/// For example, "org.freedesktop.FooViewer" where the .desktop file is
		/// "org.freedesktop.FooViewer.desktop".
		///
		/// See the desktop-entry specification [0] for more details on
		/// application identifiers and how they relate to well-known D-Bus
		/// names and .desktop files.
		///
		/// [0] http://standards.freedesktop.org/desktop-entry-spec/
		pub fn set_app_id(
			&self,
			app_id          : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, app_id.as_ptr()); }
		}
		
		/// # show the window menu
		///
		/// Clients implementing client-side decorations might want to show
		/// a context menu when right-clicking on the decorations, giving the
		/// user a menu that they can use to maximize or minimize the window.
		///
		/// This request asks the compositor to pop up such a window menu at
		/// the given position, relative to the local surface coordinates of
		/// the parent surface. There are no guarantees as to what menu items
		/// the window menu contains.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event.
		pub fn show_window_menu(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			x               : i32,
			y               : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, seat, serial, x, y); }
		}
		
		/// # start an interactive move
		///
		/// Start an interactive, user-driven move of the surface.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event. The passed
		/// serial is used to determine the type of interactive move (touch,
		/// pointer, etc).
		///
		/// The server may ignore move requests depending on the state of
		/// the surface (e.g. fullscreen or maximized), or if the passed serial
		/// is no longer valid.
		///
		/// If triggered, the surface will lose the focus of the device
		/// (wl_pointer, wl_touch, etc) used for the move. It is up to the
		/// compositor to visually indicate that the move is taking place, such as
		/// updating a pointer cursor, during the move. There is no guarantee
		/// that the device focus will return when the move is completed.
		pub fn r#move(
			&self,
			seat            : &WlSeat,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, seat, serial); }
		}
		
		/// # start an interactive resize
		///
		/// Start a user-driven, interactive resize of the surface.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event. The passed
		/// serial is used to determine the type of interactive resize (touch,
		/// pointer, etc).
		///
		/// The server may ignore resize requests depending on the state of
		/// the surface (e.g. fullscreen or maximized).
		///
		/// If triggered, the client will receive configure events with the
		/// "resize" state enum value and the expected sizes. See the "resize"
		/// enum value for more details about what is required. The client
		/// must also acknowledge configure events using "ack_configure". After
		/// the resize is completed, the client will receive another "configure"
		/// event without the resize state.
		///
		/// If triggered, the surface also will lose the focus of the device
		/// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
		/// compositor to visually indicate that the resize is taking place,
		/// such as updating a pointer cursor, during the resize. There is no
		/// guarantee that the device focus will return when the resize is
		/// completed.
		///
		/// The edges parameter specifies how the surface should be resized,
		/// and is one of the values of the resize_edge enum. The compositor
		/// may use this information to update the surface position for
		/// example when dragging the top left corner. The compositor may also
		/// use this information to adapt its behavior, e.g. choose an
		/// appropriate cursor image.
		pub fn resize(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			edges           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, seat, serial, edges); }
		}
		
		/// # set the maximum size
		///
		/// Set a maximum size for the window.
		///
		/// The client can specify a maximum size so that the compositor does
		/// not try to configure the window beyond this size.
		///
		/// The width and height arguments are in window geometry coordinates.
		/// See xdg_surface.set_window_geometry.
		///
		/// Values set in this way are double-buffered. They will get applied
		/// on the next commit.
		///
		/// The compositor can use this information to allow or disallow
		/// different states like maximize or fullscreen and draw accurate
		/// animations.
		///
		/// Similarly, a tiling window manager may use this information to
		/// place and resize client windows in a more effective way.
		///
		/// The client should not rely on the compositor to obey the maximum
		/// size. The compositor may decide to ignore the values set by the
		/// client and request a larger size.
		///
		/// If never set, or a value of zero in the request, means that the
		/// client has no expected maximum size in the given dimension.
		/// As a result, a client wishing to reset the maximum size
		/// to an unspecified state can use zero for width and height in the
		/// request.
		///
		/// Requesting a maximum size to be smaller than the minimum size of
		/// a surface is illegal and will result in a protocol error.
		///
		/// The width and height must be greater than or equal to zero. Using
		/// strictly negative values for width and height will result in a
		/// protocol error.
		pub fn set_max_size(
			&self,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7, width, height); }
		}
		
		/// # set the minimum size
		///
		/// Set a minimum size for the window.
		///
		/// The client can specify a minimum size so that the compositor does
		/// not try to configure the window below this size.
		///
		/// The width and height arguments are in window geometry coordinates.
		/// See xdg_surface.set_window_geometry.
		///
		/// Values set in this way are double-buffered. They will get applied
		/// on the next commit.
		///
		/// The compositor can use this information to allow or disallow
		/// different states like maximize or fullscreen and draw accurate
		/// animations.
		///
		/// Similarly, a tiling window manager may use this information to
		/// place and resize client windows in a more effective way.
		///
		/// The client should not rely on the compositor to obey the minimum
		/// size. The compositor may decide to ignore the values set by the
		/// client and request a smaller size.
		///
		/// If never set, or a value of zero in the request, means that the
		/// client has no expected minimum size in the given dimension.
		/// As a result, a client wishing to reset the minimum size
		/// to an unspecified state can use zero for width and height in the
		/// request.
		///
		/// Requesting a minimum size to be larger than the maximum size of
		/// a surface is illegal and will result in a protocol error.
		///
		/// The width and height must be greater than or equal to zero. Using
		/// strictly negative values for width and height will result in a
		/// protocol error.
		pub fn set_min_size(
			&self,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 8, width, height); }
		}
		
		/// # maximize the window
		///
		/// Maximize the surface.
		///
		/// After requesting that the surface should be maximized, the compositor
		/// will respond by emitting a configure event with the "maximized" state
		/// and the required window geometry. The client should then update its
		/// content, drawing it in a maximized state, i.e. without shadow or other
		/// decoration outside of the window geometry. The client must also
		/// acknowledge the configure when committing the new content (see
		/// ack_configure).
		///
		/// It is up to the compositor to decide how and where to maximize the
		/// surface, for example which output and what region of the screen should
		/// be used.
		///
		/// If the surface was already maximized, the compositor will still emit
		/// a configure event with the "maximized" state.
		pub fn set_maximized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 9); }
		}
		
		/// # unmaximize the window
		///
		/// Unmaximize the surface.
		///
		/// After requesting that the surface should be unmaximized, the compositor
		/// will respond by emitting a configure event without the "maximized"
		/// state. If available, the compositor will include the window geometry
		/// dimensions the window had prior to being maximized in the configure
		/// request. The client must then update its content, drawing it in a
		/// regular state, i.e. potentially with shadow, etc. The client must also
		/// acknowledge the configure when committing the new content (see
		/// ack_configure).
		///
		/// It is up to the compositor to position the surface after it was
		/// unmaximized; usually the position the surface had before maximizing, if
		/// applicable.
		///
		/// If the surface was already not maximized, the compositor will still
		/// emit a configure event without the "maximized" state.
		pub fn unset_maximized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 10); }
		}
		
		/// # set the window as fullscreen on a monitor
		///
		/// Make the surface fullscreen.
		///
		/// You can specify an output that you would prefer to be fullscreen.
		/// If this value is NULL, it's up to the compositor to choose which
		/// display will be used to map this surface.
		///
		/// If the surface doesn't cover the whole output, the compositor will
		/// position the surface in the center of the output and compensate with
		/// black borders filling the rest of the output.
		pub fn set_fullscreen(
			&self,
			output          : Option<&WlOutput>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 11, output.map_or(std::ptr::null_mut(), |r| r as *const WlOutput as *mut WlOutput)); }
		}
		pub fn unset_fullscreen(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 12); }
		}
		
		/// # set the window as minimized
		///
		/// Request that the compositor minimize your surface. There is no
		/// way to know if the surface is currently minimized, nor is there
		/// any way to unset minimization on this surface.
		///
		/// If you are looking to throttle redrawing when minimized, please
		/// instead use the wl_surface.frame event for this, as this will
		/// also work with live previews on windows in Alt-Tab, Expose or
		/// similar compositor features.
		pub fn set_minimized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 13); }
		}
	}
	
	
	pub trait ZxdgToplevelV6Listener: std::any::Any {
		
		/// # suggest a surface change
		///
		/// This configure event asks the client to resize its toplevel surface or
		/// to change its state. The configured state should not be applied
		/// immediately. See xdg_surface.configure for details.
		///
		/// The width and height arguments specify a hint to the window
		/// about how its surface should be resized in window geometry
		/// coordinates. See set_window_geometry.
		///
		/// If the width or height arguments are zero, it means the client
		/// should decide its own window dimension. This may happen when the
		/// compositor needs to configure the state of the surface but doesn't
		/// have any information about any previous or expected dimension.
		///
		/// The states listed in the event specify how the width/height
		/// arguments should be interpreted, and possibly how it should be
		/// drawn.
		///
		/// Clients must send an ack_configure in response to this event. See
		/// xdg_surface.configure and xdg_surface.ack_configure for details.
		fn configure(
			&self,
			proxy: &mut ZxdgToplevelV6,
			width           : i32,
			height          : i32,
			states          : &WlArray,
		);
		
		/// # surface wants to be closed
		///
		/// The close event is sent by the compositor when the user
		/// wants the surface to be closed. This should be equivalent to
		/// the user clicking the close button in client-side decorations,
		/// if your application has any.
		///
		/// This is only a request that the user intends to close the
		/// window. The client may choose to ignore this request, or show
		/// a dialog to ask the user to save their data, etc.
		fn close(
			&self,
			proxy: &mut ZxdgToplevelV6,
		);
	}
	
	/// # edge values for resizing
	///
	/// These values are used to indicate which edge of a surface
	/// is being dragged in a resize operation.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgToplevelV6ResizeEdge {
		///
		None = 0,
		///
		Top = 1,
		///
		Bottom = 2,
		///
		Left = 4,
		///
		TopLeft = 5,
		///
		BottomLeft = 6,
		///
		Right = 8,
		///
		TopRight = 9,
		///
		BottomRight = 10,
	}
	
	/// # types of state on the surface
	///
	/// The different state values used on the surface. This is designed for
	/// state values like maximized, fullscreen. It is paired with the
	/// configure event to ensure that both the client and the compositor
	/// setting the state can be synchronized.
	///
	/// States set in this way are double-buffered. They will get applied on
	/// the next commit.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgToplevelV6State {
		/// the surface is maximized
		Maximized = 1,
		/// the surface is fullscreen
		Fullscreen = 2,
		/// the surface is being resized
		Resizing = 3,
		/// the surface is now activated
		Activated = 4,
	}
	
	pub static ZXDG_POPUP_V6_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_popup_v6\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "grab\0".as_ptr(),
				signature: "ou\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "popup_done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # short-lived, popup surfaces for menus
	///
	/// A popup surface is a short-lived, temporary surface. It can be used to
	/// implement for example menus, popovers, tooltips and other similar user
	/// interface concepts.
	///
	/// A popup can be made to take an explicit grab. See xdg_popup.grab for
	/// details.
	///
	/// When the popup is dismissed, a popup_done event will be sent out, and at
	/// the same time the surface will be unmapped. See the xdg_popup.popup_done
	/// event for details.
	///
	/// Explicitly destroying the xdg_popup object will also dismiss the popup and
	/// unmap the surface. Clients that want to dismiss the popup when another
	/// surface of their own is clicked should dismiss the popup using the destroy
	/// request.
	///
	/// The parent surface must have either the xdg_toplevel or xdg_popup surface
	/// role.
	///
	/// A newly created xdg_popup will be stacked on top of all previously created
	/// xdg_popup surfaces associated with the same xdg_toplevel.
	///
	/// The parent of an xdg_popup must be mapped (see the xdg_surface
	/// description) before the xdg_popup itself.
	///
	/// The x and y arguments passed when creating the popup object specify
	/// where the top left of the popup should be placed, relative to the
	/// local surface coordinates of the parent surface. See
	/// xdg_surface.get_popup. An xdg_popup must intersect with or be at least
	/// partially adjacent to its parent surface.
	///
	/// The client must call wl_surface.commit on the corresponding wl_surface
	/// for the xdg_popup state to take effect.
	pub struct ZxdgPopupV6(WlProxy);
	
	impl std::ops::Deref for ZxdgPopupV6 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgPopupV6 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgPopupV6 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgPopupV6")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgPopupV6 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgPopupV6Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgPopupV6Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut ZxdgPopupV6).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i, ),
						1 => listener.popup_done((proxy as *mut ZxdgPopupV6).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: x: {:?}, y: {:?}, width: {:?}, height: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `popup_done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgPopupV6Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # remove xdg_popup interface
		///
		/// This destroys the popup. Explicitly destroying the xdg_popup
		/// object will also dismiss the popup, and unmap the surface.
		///
		/// If this xdg_popup is not the "topmost" popup, a protocol error
		/// will be sent.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # make the popup take an explicit grab
		///
		/// This request makes the created popup take an explicit grab. An explicit
		/// grab will be dismissed when the user dismisses the popup, or when the
		/// client destroys the xdg_popup. This can be done by the user clicking
		/// outside the surface, using the keyboard, or even locking the screen
		/// through closing the lid or a timeout.
		///
		/// If the compositor denies the grab, the popup will be immediately
		/// dismissed.
		///
		/// This request must be used in response to some sort of user action like a
		/// button press, key press, or touch down event. The serial number of the
		/// event should be passed as 'serial'.
		///
		/// The parent of a grabbing popup must either be an xdg_toplevel surface or
		/// another xdg_popup with an explicit grab. If the parent is another
		/// xdg_popup it means that the popups are nested, with this popup now being
		/// the topmost popup.
		///
		/// Nested popups must be destroyed in the reverse order they were created
		/// in, e.g. the only popup you are allowed to destroy at all times is the
		/// topmost one.
		///
		/// When compositors choose to dismiss a popup, they may dismiss every
		/// nested grabbing popup as well. When a compositor dismisses popups, it
		/// will follow the same dismissing order as required from the client.
		///
		/// The parent of a grabbing popup must either be another xdg_popup with an
		/// active explicit grab, or an xdg_popup or xdg_toplevel, if there are no
		/// explicit grabs already taken.
		///
		/// If the topmost grabbing popup is destroyed, the grab will be returned to
		/// the parent of the popup, if that parent previously had an explicit grab.
		///
		/// If the parent is a grabbing popup which has already been dismissed, this
		/// popup will be immediately dismissed. If the parent is a popup that did
		/// not take an explicit grab, an error will be raised.
		///
		/// During a popup grab, the client owning the grab will receive pointer
		/// and touch events for all their surfaces as normal (similar to an
		/// "owner-events" grab in X11 parlance), while the top most grabbing popup
		/// will always have keyboard focus.
		pub fn grab(
			&self,
			seat            : &WlSeat,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, seat, serial); }
		}
	}
	
	
	pub trait ZxdgPopupV6Listener: std::any::Any {
		
		/// # configure the popup surface
		///
		/// This event asks the popup surface to configure itself given the
		/// configuration. The configured state should not be applied immediately.
		/// See xdg_surface.configure for details.
		///
		/// The x and y arguments represent the position the popup was placed at
		/// given the xdg_positioner rule, relative to the upper left corner of the
		/// window geometry of the parent surface.
		fn configure(
			&self,
			proxy: &mut ZxdgPopupV6,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32,
		);
		
		/// # popup interaction is done
		///
		/// The popup_done event is sent out when a popup is dismissed by the
		/// compositor. The client should destroy the xdg_popup object at this
		/// point.
		fn popup_done(
			&self,
			proxy: &mut ZxdgPopupV6,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgPopupV6Error {
		/// tried to grab after being mapped
		InvalidGrab = 0,
	}
}
pub use xdg_shell::*;
mod xdg_shell {
	use crate::*;
	
	// Copyright © 2008-2013 Kristian Høgsberg
	// Copyright © 2013      Rafael Antognolli
	// Copyright © 2013      Jasper St. Pierre
	// Copyright © 2010-2013 Intel Corporation
	// Copyright © 2015-2017 Samsung Electronics Co., Ltd
	// Copyright © 2015-2017 Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static XDG_WM_BASE_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_wm_base\0".as_ptr(),
		version:      2,
		method_count: 4,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "create_positioner\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&XDG_POSITIONER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_xdg_surface\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&XDG_SURFACE_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "pong\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "ping\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # create desktop-style surfaces
	///
	/// The xdg_wm_base interface is exposed as a global object enabling clients
	/// to turn their wl_surfaces into windows in a desktop environment. It
	/// defines the basic functionality needed for clients and the compositor to
	/// create windows that can be dragged, resized, maximized, etc, as well as
	/// creating transient windows such as popup menus.
	pub struct XdgWmBase(WlProxy);
	
	impl std::ops::Deref for XdgWmBase {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgWmBase {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgWmBase {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgWmBase")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgWmBase {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl XdgWmBaseListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn XdgWmBaseListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.ping((proxy as *mut XdgWmBase).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `ping` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn XdgWmBaseListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy xdg_wm_base
		///
		/// Destroy this xdg_wm_base object.
		///
		/// Destroying a bound xdg_wm_base object while there are surfaces
		/// still alive created by this xdg_wm_base object instance is illegal
		/// and will result in a protocol error.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a positioner object
		///
		/// Create a positioner object. A positioner object is used to position
		/// surfaces relative to some parent surface. See the interface description
		/// and xdg_surface.get_popup for details.
		pub fn create_positioner(
			&self
		) -> Result<Box<XdgPositioner, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &XDG_POSITIONER_INTERFACE, std::ptr::null::<u8>()) as *mut XdgPositioner };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # create a shell surface from a surface
		///
		/// This creates an xdg_surface for the given surface. While xdg_surface
		/// itself is not a role, the corresponding surface may only be assigned
		/// a role extending xdg_surface, such as xdg_toplevel or xdg_popup.
		///
		/// This creates an xdg_surface for the given surface. An xdg_surface is
		/// used as basis to define a role to a given surface, such as xdg_toplevel
		/// or xdg_popup. It also manages functionality shared between xdg_surface
		/// based surface roles.
		///
		/// See the documentation of xdg_surface for more details about what an
		/// xdg_surface is and how it is used.
		pub fn get_xdg_surface(
			&self,
			surface         : &WlSurface
		) -> Result<Box<XdgSurface, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &XDG_SURFACE_INTERFACE, std::ptr::null::<u8>(), surface) as *mut XdgSurface };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # respond to a ping event
		///
		/// A client must respond to a ping event with a pong request or
		/// the client may be deemed unresponsive. See xdg_wm_base.ping.
		pub fn pong(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, serial); }
		}
	}
	
	
	pub trait XdgWmBaseListener: std::any::Any {
		
		/// # check if the client is alive
		///
		/// The ping event asks the client if it's still alive. Pass the
		/// serial specified in the event back to the compositor by sending
		/// a "pong" request back with the specified serial. See xdg_wm_base.pong.
		///
		/// Compositors can use this to determine if the client is still
		/// alive. It's unspecified what will happen if the client doesn't
		/// respond to the ping request, or in what timeframe. Clients should
		/// try to respond in a reasonable amount of time.
		///
		/// A compositor is free to ping in any way it wants, but a client must
		/// always respond to any xdg_wm_base object it created.
		fn ping(
			&self,
			proxy: &mut XdgWmBase,
			serial          : u32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgWmBaseError {
		/// given wl_surface has another role
		Role = 0,
		/// xdg_wm_base was destroyed before children
		DefunctSurfaces = 1,
		/// the client tried to map or destroy a non-topmost popup
		NotTheTopmostPopup = 2,
		/// the client specified an invalid popup parent surface
		InvalidPopupParent = 3,
		/// the client provided an invalid surface state
		InvalidSurfaceState = 4,
		/// the client provided an invalid positioner
		InvalidPositioner = 5,
	}
	
	pub static XDG_POSITIONER_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_positioner\0".as_ptr(),
		version:      2,
		method_count: 7,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_size\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_anchor_rect\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_anchor\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_gravity\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_constraint_adjustment\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_offset\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # child surface positioner
	///
	/// The xdg_positioner provides a collection of rules for the placement of a
	/// child surface relative to a parent surface. Rules can be defined to ensure
	/// the child surface remains within the visible area's borders, and to
	/// specify how the child surface changes its position, such as sliding along
	/// an axis, or flipping around a rectangle. These positioner-created rules are
	/// constrained by the requirement that a child surface must intersect with or
	/// be at least partially adjacent to its parent surface.
	///
	/// See the various requests for details about possible rules.
	///
	/// At the time of the request, the compositor makes a copy of the rules
	/// specified by the xdg_positioner. Thus, after the request is complete the
	/// xdg_positioner object can be destroyed or reused; further changes to the
	/// object will have no effect on previous usages.
	///
	/// For an xdg_positioner object to be considered complete, it must have a
	/// non-zero size set by set_size, and a non-zero anchor rectangle set by
	/// set_anchor_rect. Passing an incomplete xdg_positioner object when
	/// positioning a surface raises an error.
	pub struct XdgPositioner(WlProxy);
	
	impl std::ops::Deref for XdgPositioner {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgPositioner {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgPositioner {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgPositioner")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgPositioner {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the xdg_positioner object
		///
		/// Notify the compositor that the xdg_positioner will no longer be used.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the size of the to-be positioned rectangle
		///
		/// Set the size of the surface that is to be positioned with the positioner
		/// object. The size is in surface-local coordinates and corresponds to the
		/// window geometry. See xdg_surface.set_window_geometry.
		///
		/// If a zero or negative size is set the invalid_input error is raised.
		pub fn set_size(
			&self,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, width, height); }
		}
		
		/// # set the anchor rectangle within the parent surface
		///
		/// Specify the anchor rectangle within the parent surface that the child
		/// surface will be placed relative to. The rectangle is relative to the
		/// window geometry as defined by xdg_surface.set_window_geometry of the
		/// parent surface.
		///
		/// When the xdg_positioner object is used to position a child surface, the
		/// anchor rectangle may not extend outside the window geometry of the
		/// positioned child's parent surface.
		///
		/// If a negative size is set the invalid_input error is raised.
		pub fn set_anchor_rect(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, x, y, width, height); }
		}
		
		/// # set anchor rectangle anchor
		///
		/// Defines the anchor point for the anchor rectangle. The specified anchor
		/// is used derive an anchor point that the child surface will be
		/// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
		/// 'bottom_right'), the anchor point will be at the specified corner;
		/// otherwise, the derived anchor point will be centered on the specified
		/// edge, or in the center of the anchor rectangle if no edge is specified.
		pub fn set_anchor(
			&self,
			anchor          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, anchor); }
		}
		
		/// # set child surface gravity
		///
		/// Defines in what direction a surface should be positioned, relative to
		/// the anchor point of the parent surface. If a corner gravity is
		/// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
		/// will be placed towards the specified gravity; otherwise, the child
		/// surface will be centered over the anchor point on any axis that had no
		/// gravity specified.
		pub fn set_gravity(
			&self,
			gravity         : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, gravity); }
		}
		
		/// # set the adjustment to be done when constrained
		///
		/// Specify how the window should be positioned if the originally intended
		/// position caused the surface to be constrained, meaning at least
		/// partially outside positioning boundaries set by the compositor. The
		/// adjustment is set by constructing a bitmask describing the adjustment to
		/// be made when the surface is constrained on that axis.
		///
		/// If no bit for one axis is set, the compositor will assume that the child
		/// surface should not change its position on that axis when constrained.
		///
		/// If more than one bit for one axis is set, the order of how adjustments
		/// are applied is specified in the corresponding adjustment descriptions.
		///
		/// The default adjustment is none.
		pub fn set_constraint_adjustment(
			&self,
			constraint_adjustment: u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, constraint_adjustment); }
		}
		
		/// # set surface position offset
		///
		/// Specify the surface position offset relative to the position of the
		/// anchor on the anchor rectangle and the anchor on the surface. For
		/// example if the anchor of the anchor rectangle is at (x, y), the surface
		/// has the gravity bottom|right, and the offset is (ox, oy), the calculated
		/// surface position will be (x + ox, y + oy). The offset position of the
		/// surface is the one used for constraint testing. See
		/// set_constraint_adjustment.
		///
		/// An example use case is placing a popup menu on top of a user interface
		/// element, while aligning the user interface element of the parent surface
		/// with some user interface element placed somewhere in the popup surface.
		pub fn set_offset(
			&self,
			x               : i32,
			y               : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, x, y); }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgPositionerError {
		/// invalid input provided
		InvalidInput = 0,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgPositionerAnchor {
		///
		None = 0,
		///
		Top = 1,
		///
		Bottom = 2,
		///
		Left = 3,
		///
		Right = 4,
		///
		TopLeft = 5,
		///
		BottomLeft = 6,
		///
		TopRight = 7,
		///
		BottomRight = 8,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgPositionerGravity {
		///
		None = 0,
		///
		Top = 1,
		///
		Bottom = 2,
		///
		Left = 3,
		///
		Right = 4,
		///
		TopLeft = 5,
		///
		BottomLeft = 6,
		///
		TopRight = 7,
		///
		BottomRight = 8,
	}
	
	/// # constraint adjustments
	///
	/// The constraint adjustment value define ways the compositor will adjust
	/// the position of the surface, if the unadjusted position would result
	/// in the surface being partly constrained.
	///
	/// Whether a surface is considered 'constrained' is left to the compositor
	/// to determine. For example, the surface may be partly outside the
	/// compositor's defined 'work area', thus necessitating the child surface's
	/// position be adjusted until it is entirely inside the work area.
	///
	/// The adjustments can be combined, according to a defined precedence: 1)
	/// Flip, 2) Slide, 3) Resize.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgPositionerConstraintAdjustment {
		///
		None = 0,
		///
		SlideX = 1,
		///
		SlideY = 2,
		///
		FlipX = 4,
		///
		FlipY = 8,
		///
		ResizeX = 16,
		///
		ResizeY = 32,
	}
	
	pub static XDG_SURFACE_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_surface\0".as_ptr(),
		version:      2,
		method_count: 5,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_toplevel\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&XDG_TOPLEVEL_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_popup\0".as_ptr(),
				signature: "n?oo\0".as_ptr(),
				types:     [&XDG_POPUP_INTERFACE as _, &XDG_SURFACE_INTERFACE as _, &XDG_POSITIONER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_window_geometry\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "ack_configure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # desktop user interface surface base interface
	///
	/// An interface that may be implemented by a wl_surface, for
	/// implementations that provide a desktop-style user interface.
	///
	/// It provides a base set of functionality required to construct user
	/// interface elements requiring management by the compositor, such as
	/// toplevel windows, menus, etc. The types of functionality are split into
	/// xdg_surface roles.
	///
	/// Creating an xdg_surface does not set the role for a wl_surface. In order
	/// to map an xdg_surface, the client must create a role-specific object
	/// using, e.g., get_toplevel, get_popup. The wl_surface for any given
	/// xdg_surface can have at most one role, and may not be assigned any role
	/// not based on xdg_surface.
	///
	/// A role must be assigned before any other requests are made to the
	/// xdg_surface object.
	///
	/// The client must call wl_surface.commit on the corresponding wl_surface
	/// for the xdg_surface state to take effect.
	///
	/// Creating an xdg_surface from a wl_surface which has a buffer attached or
	/// committed is a client error, and any attempts by a client to attach or
	/// manipulate a buffer prior to the first xdg_surface.configure call must
	/// also be treated as errors.
	///
	/// Mapping an xdg_surface-based role surface is defined as making it
	/// possible for the surface to be shown by the compositor. Note that
	/// a mapped surface is not guaranteed to be visible once it is mapped.
	///
	/// For an xdg_surface to be mapped by the compositor, the following
	/// conditions must be met:
	/// (1) the client has assigned an xdg_surface-based role to the surface
	/// (2) the client has set and committed the xdg_surface state and the
	/// role-dependent state to the surface
	/// (3) the client has committed a buffer to the surface
	///
	/// A newly-unmapped surface is considered to have met condition (1) out
	/// of the 3 required conditions for mapping a surface if its role surface
	/// has not been destroyed.
	pub struct XdgSurface(WlProxy);
	
	impl std::ops::Deref for XdgSurface {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgSurface {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgSurface {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgSurface")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgSurface {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl XdgSurfaceListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn XdgSurfaceListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut XdgSurface).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn XdgSurfaceListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the xdg_surface
		///
		/// Destroy the xdg_surface object. An xdg_surface must only be destroyed
		/// after its role object has been destroyed.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # assign the xdg_toplevel surface role
		///
		/// This creates an xdg_toplevel object for the given xdg_surface and gives
		/// the associated wl_surface the xdg_toplevel role.
		///
		/// See the documentation of xdg_toplevel for more details about what an
		/// xdg_toplevel is and how it is used.
		pub fn get_toplevel(
			&self
		) -> Result<Box<XdgToplevel, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &XDG_TOPLEVEL_INTERFACE, std::ptr::null::<u8>()) as *mut XdgToplevel };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # assign the xdg_popup surface role
		///
		/// This creates an xdg_popup object for the given xdg_surface and gives
		/// the associated wl_surface the xdg_popup role.
		///
		/// If null is passed as a parent, a parent surface must be specified using
		/// some other protocol, before committing the initial state.
		///
		/// See the documentation of xdg_popup for more details about what an
		/// xdg_popup is and how it is used.
		pub fn get_popup(
			&self,
			parent          : Option<&XdgSurface>,
			positioner      : &XdgPositioner
		) -> Result<Box<XdgPopup, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &XDG_POPUP_INTERFACE, std::ptr::null::<u8>(), parent.map_or(std::ptr::null_mut(), |r| r as *const XdgSurface as *mut XdgSurface), positioner) as *mut XdgPopup };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # set the new window geometry
		///
		/// The window geometry of a surface is its "visible bounds" from the
		/// user's perspective. Client-side decorations often have invisible
		/// portions like drop-shadows which should be ignored for the
		/// purposes of aligning, placing and constraining windows.
		///
		/// The window geometry is double buffered, and will be applied at the
		/// time wl_surface.commit of the corresponding wl_surface is called.
		///
		/// When maintaining a position, the compositor should treat the (x, y)
		/// coordinate of the window geometry as the top left corner of the window.
		/// A client changing the (x, y) window geometry coordinate should in
		/// general not alter the position of the window.
		///
		/// Once the window geometry of the surface is set, it is not possible to
		/// unset it, and it will remain the same until set_window_geometry is
		/// called again, even if a new subsurface or buffer is attached.
		///
		/// If never set, the value is the full bounds of the surface,
		/// including any subsurfaces. This updates dynamically on every
		/// commit. This unset is meant for extremely simple clients.
		///
		/// The arguments are given in the surface-local coordinate space of
		/// the wl_surface associated with this xdg_surface.
		///
		/// The width and height must be greater than zero. Setting an invalid size
		/// will raise an error. When applied, the effective window geometry will be
		/// the set window geometry clamped to the bounding rectangle of the
		/// combined geometry of the surface of the xdg_surface and the associated
		/// subsurfaces.
		pub fn set_window_geometry(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, x, y, width, height); }
		}
		
		/// # ack a configure event
		///
		/// When a configure event is received, if a client commits the
		/// surface in response to the configure event, then the client
		/// must make an ack_configure request sometime before the commit
		/// request, passing along the serial of the configure event.
		///
		/// For instance, for toplevel surfaces the compositor might use this
		/// information to move a surface to the top left only when the client has
		/// drawn itself for the maximized or fullscreen state.
		///
		/// If the client receives multiple configure events before it
		/// can respond to one, it only has to ack the last configure event.
		///
		/// A client is not required to commit immediately after sending
		/// an ack_configure request - it may even ack_configure several times
		/// before its next surface commit.
		///
		/// A client may send multiple ack_configure requests before committing, but
		/// only the last request sent before a commit indicates which configure
		/// event the client really is responding to.
		pub fn ack_configure(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, serial); }
		}
	}
	
	
	pub trait XdgSurfaceListener: std::any::Any {
		
		/// # suggest a surface change
		///
		/// The configure event marks the end of a configure sequence. A configure
		/// sequence is a set of one or more events configuring the state of the
		/// xdg_surface, including the final xdg_surface.configure event.
		///
		/// Where applicable, xdg_surface surface roles will during a configure
		/// sequence extend this event as a latched state sent as events before the
		/// xdg_surface.configure event. Such events should be considered to make up
		/// a set of atomically applied configuration states, where the
		/// xdg_surface.configure commits the accumulated state.
		///
		/// Clients should arrange their surface for the new states, and then send
		/// an ack_configure request with the serial sent in this configure event at
		/// some point before committing the new surface.
		///
		/// If the client receives multiple configure events before it can respond
		/// to one, it is free to discard all but the last event it received.
		fn configure(
			&self,
			proxy: &mut XdgSurface,
			serial          : u32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgSurfaceError {
		///
		NotConstructed = 1,
		///
		AlreadyConstructed = 2,
		///
		UnconfiguredBuffer = 3,
	}
	
	pub static XDG_TOPLEVEL_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_toplevel\0".as_ptr(),
		version:      2,
		method_count: 14,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_parent\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&XDG_TOPLEVEL_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_title\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_app_id\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "show_window_menu\0".as_ptr(),
				signature: "ouii\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "move\0".as_ptr(),
				signature: "ou\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "resize\0".as_ptr(),
				signature: "ouu\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_max_size\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_min_size\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_maximized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "unset_maximized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_fullscreen\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "unset_fullscreen\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_minimized\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "iia\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "close\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # toplevel surface
	///
	/// This interface defines an xdg_surface role which allows a surface to,
	/// among other things, set window-like properties such as maximize,
	/// fullscreen, and minimize, set application-specific metadata like title and
	/// id, and well as trigger user interactive operations such as interactive
	/// resize and move.
	///
	/// Unmapping an xdg_toplevel means that the surface cannot be shown
	/// by the compositor until it is explicitly mapped again.
	/// All active operations (e.g., move, resize) are canceled and all
	/// attributes (e.g. title, state, stacking, ...) are discarded for
	/// an xdg_toplevel surface when it is unmapped.
	///
	/// Attaching a null buffer to a toplevel unmaps the surface.
	pub struct XdgToplevel(WlProxy);
	
	impl std::ops::Deref for XdgToplevel {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgToplevel {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgToplevel {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgToplevel")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgToplevel {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl XdgToplevelListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn XdgToplevelListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut XdgToplevel).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).a.as_ref().unwrap(), ),
						1 => listener.close((proxy as *mut XdgToplevel).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: width: {:?}, height: {:?}, states: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).a.as_ref().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `close` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn XdgToplevelListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the xdg_toplevel
		///
		/// This request destroys the role surface and unmaps the surface;
		/// see "Unmapping" behavior in interface section for details.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the parent of this surface
		///
		/// Set the "parent" of this surface. This surface should be stacked
		/// above the parent surface and all other ancestor surfaces.
		///
		/// Parent windows should be set on dialogs, toolboxes, or other
		/// "auxiliary" surfaces, so that the parent is raised when the dialog
		/// is raised.
		///
		/// Setting a null parent for a child window removes any parent-child
		/// relationship for the child. Setting a null parent for a window which
		/// currently has no parent is a no-op.
		///
		/// If the parent is unmapped then its children are managed as
		/// though the parent of the now-unmapped parent has become the
		/// parent of this surface. If no parent exists for the now-unmapped
		/// parent then the children are managed as though they have no
		/// parent surface.
		pub fn set_parent(
			&self,
			parent          : Option<&XdgToplevel>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, parent.map_or(std::ptr::null_mut(), |r| r as *const XdgToplevel as *mut XdgToplevel)); }
		}
		
		/// # set surface title
		///
		/// Set a short title for the surface.
		///
		/// This string may be used to identify the surface in a task bar,
		/// window list, or other user interface elements provided by the
		/// compositor.
		///
		/// The string must be encoded in UTF-8.
		pub fn set_title(
			&self,
			title           : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, title.as_ptr()); }
		}
		
		/// # set application ID
		///
		/// Set an application identifier for the surface.
		///
		/// The app ID identifies the general class of applications to which
		/// the surface belongs. The compositor can use this to group multiple
		/// surfaces together, or to determine how to launch a new application.
		///
		/// For D-Bus activatable applications, the app ID is used as the D-Bus
		/// service name.
		///
		/// The compositor shell will try to group application surfaces together
		/// by their app ID. As a best practice, it is suggested to select app
		/// ID's that match the basename of the application's .desktop file.
		/// For example, "org.freedesktop.FooViewer" where the .desktop file is
		/// "org.freedesktop.FooViewer.desktop".
		///
		/// Like other properties, a set_app_id request can be sent after the
		/// xdg_toplevel has been mapped to update the property.
		///
		/// See the desktop-entry specification [0] for more details on
		/// application identifiers and how they relate to well-known D-Bus
		/// names and .desktop files.
		///
		/// [0] http://standards.freedesktop.org/desktop-entry-spec/
		pub fn set_app_id(
			&self,
			app_id          : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, app_id.as_ptr()); }
		}
		
		/// # show the window menu
		///
		/// Clients implementing client-side decorations might want to show
		/// a context menu when right-clicking on the decorations, giving the
		/// user a menu that they can use to maximize or minimize the window.
		///
		/// This request asks the compositor to pop up such a window menu at
		/// the given position, relative to the local surface coordinates of
		/// the parent surface. There are no guarantees as to what menu items
		/// the window menu contains.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event.
		pub fn show_window_menu(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			x               : i32,
			y               : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, seat, serial, x, y); }
		}
		
		/// # start an interactive move
		///
		/// Start an interactive, user-driven move of the surface.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event. The passed
		/// serial is used to determine the type of interactive move (touch,
		/// pointer, etc).
		///
		/// The server may ignore move requests depending on the state of
		/// the surface (e.g. fullscreen or maximized), or if the passed serial
		/// is no longer valid.
		///
		/// If triggered, the surface will lose the focus of the device
		/// (wl_pointer, wl_touch, etc) used for the move. It is up to the
		/// compositor to visually indicate that the move is taking place, such as
		/// updating a pointer cursor, during the move. There is no guarantee
		/// that the device focus will return when the move is completed.
		pub fn r#move(
			&self,
			seat            : &WlSeat,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, seat, serial); }
		}
		
		/// # start an interactive resize
		///
		/// Start a user-driven, interactive resize of the surface.
		///
		/// This request must be used in response to some sort of user action
		/// like a button press, key press, or touch down event. The passed
		/// serial is used to determine the type of interactive resize (touch,
		/// pointer, etc).
		///
		/// The server may ignore resize requests depending on the state of
		/// the surface (e.g. fullscreen or maximized).
		///
		/// If triggered, the client will receive configure events with the
		/// "resize" state enum value and the expected sizes. See the "resize"
		/// enum value for more details about what is required. The client
		/// must also acknowledge configure events using "ack_configure". After
		/// the resize is completed, the client will receive another "configure"
		/// event without the resize state.
		///
		/// If triggered, the surface also will lose the focus of the device
		/// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
		/// compositor to visually indicate that the resize is taking place,
		/// such as updating a pointer cursor, during the resize. There is no
		/// guarantee that the device focus will return when the resize is
		/// completed.
		///
		/// The edges parameter specifies how the surface should be resized,
		/// and is one of the values of the resize_edge enum. The compositor
		/// may use this information to update the surface position for
		/// example when dragging the top left corner. The compositor may also
		/// use this information to adapt its behavior, e.g. choose an
		/// appropriate cursor image.
		pub fn resize(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			edges           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, seat, serial, edges); }
		}
		
		/// # set the maximum size
		///
		/// Set a maximum size for the window.
		///
		/// The client can specify a maximum size so that the compositor does
		/// not try to configure the window beyond this size.
		///
		/// The width and height arguments are in window geometry coordinates.
		/// See xdg_surface.set_window_geometry.
		///
		/// Values set in this way are double-buffered. They will get applied
		/// on the next commit.
		///
		/// The compositor can use this information to allow or disallow
		/// different states like maximize or fullscreen and draw accurate
		/// animations.
		///
		/// Similarly, a tiling window manager may use this information to
		/// place and resize client windows in a more effective way.
		///
		/// The client should not rely on the compositor to obey the maximum
		/// size. The compositor may decide to ignore the values set by the
		/// client and request a larger size.
		///
		/// If never set, or a value of zero in the request, means that the
		/// client has no expected maximum size in the given dimension.
		/// As a result, a client wishing to reset the maximum size
		/// to an unspecified state can use zero for width and height in the
		/// request.
		///
		/// Requesting a maximum size to be smaller than the minimum size of
		/// a surface is illegal and will result in a protocol error.
		///
		/// The width and height must be greater than or equal to zero. Using
		/// strictly negative values for width and height will result in a
		/// protocol error.
		pub fn set_max_size(
			&self,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7, width, height); }
		}
		
		/// # set the minimum size
		///
		/// Set a minimum size for the window.
		///
		/// The client can specify a minimum size so that the compositor does
		/// not try to configure the window below this size.
		///
		/// The width and height arguments are in window geometry coordinates.
		/// See xdg_surface.set_window_geometry.
		///
		/// Values set in this way are double-buffered. They will get applied
		/// on the next commit.
		///
		/// The compositor can use this information to allow or disallow
		/// different states like maximize or fullscreen and draw accurate
		/// animations.
		///
		/// Similarly, a tiling window manager may use this information to
		/// place and resize client windows in a more effective way.
		///
		/// The client should not rely on the compositor to obey the minimum
		/// size. The compositor may decide to ignore the values set by the
		/// client and request a smaller size.
		///
		/// If never set, or a value of zero in the request, means that the
		/// client has no expected minimum size in the given dimension.
		/// As a result, a client wishing to reset the minimum size
		/// to an unspecified state can use zero for width and height in the
		/// request.
		///
		/// Requesting a minimum size to be larger than the maximum size of
		/// a surface is illegal and will result in a protocol error.
		///
		/// The width and height must be greater than or equal to zero. Using
		/// strictly negative values for width and height will result in a
		/// protocol error.
		pub fn set_min_size(
			&self,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 8, width, height); }
		}
		
		/// # maximize the window
		///
		/// Maximize the surface.
		///
		/// After requesting that the surface should be maximized, the compositor
		/// will respond by emitting a configure event. Whether this configure
		/// actually sets the window maximized is subject to compositor policies.
		/// The client must then update its content, drawing in the configured
		/// state. The client must also acknowledge the configure when committing
		/// the new content (see ack_configure).
		///
		/// It is up to the compositor to decide how and where to maximize the
		/// surface, for example which output and what region of the screen should
		/// be used.
		///
		/// If the surface was already maximized, the compositor will still emit
		/// a configure event with the "maximized" state.
		///
		/// If the surface is in a fullscreen state, this request has no direct
		/// effect. It may alter the state the surface is returned to when
		/// unmaximized unless overridden by the compositor.
		pub fn set_maximized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 9); }
		}
		
		/// # unmaximize the window
		///
		/// Unmaximize the surface.
		///
		/// After requesting that the surface should be unmaximized, the compositor
		/// will respond by emitting a configure event. Whether this actually
		/// un-maximizes the window is subject to compositor policies.
		/// If available and applicable, the compositor will include the window
		/// geometry dimensions the window had prior to being maximized in the
		/// configure event. The client must then update its content, drawing it in
		/// the configured state. The client must also acknowledge the configure
		/// when committing the new content (see ack_configure).
		///
		/// It is up to the compositor to position the surface after it was
		/// unmaximized; usually the position the surface had before maximizing, if
		/// applicable.
		///
		/// If the surface was already not maximized, the compositor will still
		/// emit a configure event without the "maximized" state.
		///
		/// If the surface is in a fullscreen state, this request has no direct
		/// effect. It may alter the state the surface is returned to when
		/// unmaximized unless overridden by the compositor.
		pub fn unset_maximized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 10); }
		}
		
		/// # set the window as fullscreen on an output
		///
		/// Make the surface fullscreen.
		///
		/// After requesting that the surface should be fullscreened, the
		/// compositor will respond by emitting a configure event. Whether the
		/// client is actually put into a fullscreen state is subject to compositor
		/// policies. The client must also acknowledge the configure when
		/// committing the new content (see ack_configure).
		///
		/// The output passed by the request indicates the client's preference as
		/// to which display it should be set fullscreen on. If this value is NULL,
		/// it's up to the compositor to choose which display will be used to map
		/// this surface.
		///
		/// If the surface doesn't cover the whole output, the compositor will
		/// position the surface in the center of the output and compensate with
		/// with border fill covering the rest of the output. The content of the
		/// border fill is undefined, but should be assumed to be in some way that
		/// attempts to blend into the surrounding area (e.g. solid black).
		///
		/// If the fullscreened surface is not opaque, the compositor must make
		/// sure that other screen content not part of the same surface tree (made
		/// up of subsurfaces, popups or similarly coupled surfaces) are not
		/// visible below the fullscreened surface.
		pub fn set_fullscreen(
			&self,
			output          : Option<&WlOutput>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 11, output.map_or(std::ptr::null_mut(), |r| r as *const WlOutput as *mut WlOutput)); }
		}
		
		/// # unset the window as fullscreen
		///
		/// Make the surface no longer fullscreen.
		///
		/// After requesting that the surface should be unfullscreened, the
		/// compositor will respond by emitting a configure event.
		/// Whether this actually removes the fullscreen state of the client is
		/// subject to compositor policies.
		///
		/// Making a surface unfullscreen sets states for the surface based on the following:
		/// * the state(s) it may have had before becoming fullscreen
		/// * any state(s) decided by the compositor
		/// * any state(s) requested by the client while the surface was fullscreen
		///
		/// The compositor may include the previous window geometry dimensions in
		/// the configure event, if applicable.
		///
		/// The client must also acknowledge the configure when committing the new
		/// content (see ack_configure).
		pub fn unset_fullscreen(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 12); }
		}
		
		/// # set the window as minimized
		///
		/// Request that the compositor minimize your surface. There is no
		/// way to know if the surface is currently minimized, nor is there
		/// any way to unset minimization on this surface.
		///
		/// If you are looking to throttle redrawing when minimized, please
		/// instead use the wl_surface.frame event for this, as this will
		/// also work with live previews on windows in Alt-Tab, Expose or
		/// similar compositor features.
		pub fn set_minimized(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 13); }
		}
	}
	
	
	pub trait XdgToplevelListener: std::any::Any {
		
		/// # suggest a surface change
		///
		/// This configure event asks the client to resize its toplevel surface or
		/// to change its state. The configured state should not be applied
		/// immediately. See xdg_surface.configure for details.
		///
		/// The width and height arguments specify a hint to the window
		/// about how its surface should be resized in window geometry
		/// coordinates. See set_window_geometry.
		///
		/// If the width or height arguments are zero, it means the client
		/// should decide its own window dimension. This may happen when the
		/// compositor needs to configure the state of the surface but doesn't
		/// have any information about any previous or expected dimension.
		///
		/// The states listed in the event specify how the width/height
		/// arguments should be interpreted, and possibly how it should be
		/// drawn.
		///
		/// Clients must send an ack_configure in response to this event. See
		/// xdg_surface.configure and xdg_surface.ack_configure for details.
		fn configure(
			&self,
			proxy: &mut XdgToplevel,
			width           : i32,
			height          : i32,
			states          : &WlArray,
		);
		
		/// # surface wants to be closed
		///
		/// The close event is sent by the compositor when the user
		/// wants the surface to be closed. This should be equivalent to
		/// the user clicking the close button in client-side decorations,
		/// if your application has any.
		///
		/// This is only a request that the user intends to close the
		/// window. The client may choose to ignore this request, or show
		/// a dialog to ask the user to save their data, etc.
		fn close(
			&self,
			proxy: &mut XdgToplevel,
		);
	}
	
	/// # edge values for resizing
	///
	/// These values are used to indicate which edge of a surface
	/// is being dragged in a resize operation.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgToplevelResizeEdge {
		///
		None = 0,
		///
		Top = 1,
		///
		Bottom = 2,
		///
		Left = 4,
		///
		TopLeft = 5,
		///
		BottomLeft = 6,
		///
		Right = 8,
		///
		TopRight = 9,
		///
		BottomRight = 10,
	}
	
	/// # types of state on the surface
	///
	/// The different state values used on the surface. This is designed for
	/// state values like maximized, fullscreen. It is paired with the
	/// configure event to ensure that both the client and the compositor
	/// setting the state can be synchronized.
	///
	/// States set in this way are double-buffered. They will get applied on
	/// the next commit.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgToplevelState {
		/// the surface is maximized
		Maximized = 1,
		/// the surface is fullscreen
		Fullscreen = 2,
		/// the surface is being resized
		Resizing = 3,
		/// the surface is now activated
		Activated = 4,
		///
		TiledLeft = 5,
		///
		TiledRight = 6,
		///
		TiledTop = 7,
		///
		TiledBottom = 8,
	}
	
	pub static XDG_POPUP_INTERFACE: WlInterface = WlInterface {
		name:         "xdg_popup\0".as_ptr(),
		version:      2,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "grab\0".as_ptr(),
				signature: "ou\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "popup_done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # short-lived, popup surfaces for menus
	///
	/// A popup surface is a short-lived, temporary surface. It can be used to
	/// implement for example menus, popovers, tooltips and other similar user
	/// interface concepts.
	///
	/// A popup can be made to take an explicit grab. See xdg_popup.grab for
	/// details.
	///
	/// When the popup is dismissed, a popup_done event will be sent out, and at
	/// the same time the surface will be unmapped. See the xdg_popup.popup_done
	/// event for details.
	///
	/// Explicitly destroying the xdg_popup object will also dismiss the popup and
	/// unmap the surface. Clients that want to dismiss the popup when another
	/// surface of their own is clicked should dismiss the popup using the destroy
	/// request.
	///
	/// A newly created xdg_popup will be stacked on top of all previously created
	/// xdg_popup surfaces associated with the same xdg_toplevel.
	///
	/// The parent of an xdg_popup must be mapped (see the xdg_surface
	/// description) before the xdg_popup itself.
	///
	/// The x and y arguments passed when creating the popup object specify
	/// where the top left of the popup should be placed, relative to the
	/// local surface coordinates of the parent surface. See
	/// xdg_surface.get_popup. An xdg_popup must intersect with or be at least
	/// partially adjacent to its parent surface.
	///
	/// The client must call wl_surface.commit on the corresponding wl_surface
	/// for the xdg_popup state to take effect.
	pub struct XdgPopup(WlProxy);
	
	impl std::ops::Deref for XdgPopup {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for XdgPopup {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for XdgPopup {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("XdgPopup")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl XdgPopup {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl XdgPopupListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn XdgPopupListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut XdgPopup).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i, ),
						1 => listener.popup_done((proxy as *mut XdgPopup).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: x: {:?}, y: {:?}, width: {:?}, height: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `popup_done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn XdgPopupListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # remove xdg_popup interface
		///
		/// This destroys the popup. Explicitly destroying the xdg_popup
		/// object will also dismiss the popup, and unmap the surface.
		///
		/// If this xdg_popup is not the "topmost" popup, a protocol error
		/// will be sent.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # make the popup take an explicit grab
		///
		/// This request makes the created popup take an explicit grab. An explicit
		/// grab will be dismissed when the user dismisses the popup, or when the
		/// client destroys the xdg_popup. This can be done by the user clicking
		/// outside the surface, using the keyboard, or even locking the screen
		/// through closing the lid or a timeout.
		///
		/// If the compositor denies the grab, the popup will be immediately
		/// dismissed.
		///
		/// This request must be used in response to some sort of user action like a
		/// button press, key press, or touch down event. The serial number of the
		/// event should be passed as 'serial'.
		///
		/// The parent of a grabbing popup must either be an xdg_toplevel surface or
		/// another xdg_popup with an explicit grab. If the parent is another
		/// xdg_popup it means that the popups are nested, with this popup now being
		/// the topmost popup.
		///
		/// Nested popups must be destroyed in the reverse order they were created
		/// in, e.g. the only popup you are allowed to destroy at all times is the
		/// topmost one.
		///
		/// When compositors choose to dismiss a popup, they may dismiss every
		/// nested grabbing popup as well. When a compositor dismisses popups, it
		/// will follow the same dismissing order as required from the client.
		///
		/// The parent of a grabbing popup must either be another xdg_popup with an
		/// active explicit grab, or an xdg_popup or xdg_toplevel, if there are no
		/// explicit grabs already taken.
		///
		/// If the topmost grabbing popup is destroyed, the grab will be returned to
		/// the parent of the popup, if that parent previously had an explicit grab.
		///
		/// If the parent is a grabbing popup which has already been dismissed, this
		/// popup will be immediately dismissed. If the parent is a popup that did
		/// not take an explicit grab, an error will be raised.
		///
		/// During a popup grab, the client owning the grab will receive pointer
		/// and touch events for all their surfaces as normal (similar to an
		/// "owner-events" grab in X11 parlance), while the top most grabbing popup
		/// will always have keyboard focus.
		pub fn grab(
			&self,
			seat            : &WlSeat,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, seat, serial); }
		}
	}
	
	
	pub trait XdgPopupListener: std::any::Any {
		
		/// # configure the popup surface
		///
		/// This event asks the popup surface to configure itself given the
		/// configuration. The configured state should not be applied immediately.
		/// See xdg_surface.configure for details.
		///
		/// The x and y arguments represent the position the popup was placed at
		/// given the xdg_positioner rule, relative to the upper left corner of the
		/// window geometry of the parent surface.
		fn configure(
			&self,
			proxy: &mut XdgPopup,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32,
		);
		
		/// # popup interaction is done
		///
		/// The popup_done event is sent out when a popup is dismissed by the
		/// compositor. The client should destroy the xdg_popup object at this
		/// point.
		fn popup_done(
			&self,
			proxy: &mut XdgPopup,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum XdgPopupError {
		/// tried to grab after being mapped
		InvalidGrab = 0,
	}
}

/// # Protocol to describe output regions
///
/// This protocol aims at describing outputs in a way which is more in line
/// with the concept of an output on desktop oriented systems.
///
/// Some information are more specific to the concept of an output for
/// a desktop oriented system and may not make sense in other applications,
/// such as IVI systems for example.
///
/// Typically, the global compositor space on a desktop system is made of
/// a contiguous or overlapping set of rectangular regions.
///
/// Some of the information provided in this protocol might be identical
/// to their counterparts already available from wl_output, in which case
/// the information provided by this protocol should be preferred to their
/// equivalent in wl_output. The goal is to move the desktop specific
/// concepts (such as output location within the global compositor space,
/// the connector name and types, etc.) out of the core wl_output protocol.
///
/// Warning! The protocol described in this file is experimental and
/// backward incompatible changes may be made. Backward compatible
/// changes may be added together with the corresponding interface
/// version bump.
/// Backward incompatible changes are done by bumping the version
/// number in the protocol and interface names and resetting the
/// interface version. Once the protocol is to be declared stable,
/// the 'z' prefix and the version number in the protocol and
/// interface names are removed and the interface version number is
/// reset.
pub use xdg_output_unstable_v1::*;
mod xdg_output_unstable_v1 {
	use crate::*;
	
	// Copyright © 2017 Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZXDG_OUTPUT_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_output_manager_v1\0".as_ptr(),
		version:      3,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_xdg_output\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZXDG_OUTPUT_V1_INTERFACE as _, &WL_OUTPUT_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # manage xdg_output objects
	///
	/// A global factory interface for xdg_output objects.
	pub struct ZxdgOutputManagerV1(WlProxy);
	
	impl std::ops::Deref for ZxdgOutputManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgOutputManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgOutputManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgOutputManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgOutputManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the xdg_output_manager object
		///
		/// Using this request a client can tell the server that it is not
		/// going to use the xdg_output_manager object anymore.
		///
		/// Any objects already created through this instance are not affected.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create an xdg output from a wl_output
		///
		/// This creates a new xdg_output object for the given wl_output.
		pub fn get_xdg_output(
			&self,
			output          : &WlOutput
		) -> Result<Box<ZxdgOutputV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_OUTPUT_V1_INTERFACE, std::ptr::null::<u8>(), output) as *mut ZxdgOutputV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZXDG_OUTPUT_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_output_v1\0".as_ptr(),
		version:      3,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  5,
		events:       [
			WlMessage {
				name:      "logical_position\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "logical_size\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "name\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "description\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # compositor logical output region
	///
	/// An xdg_output describes part of the compositor geometry.
	///
	/// This typically corresponds to a monitor that displays part of the
	/// compositor space.
	///
	/// For objects version 3 onwards, after all xdg_output properties have been
	/// sent (when the object is created and when properties are updated), a
	/// wl_output.done event is sent. This allows changes to the output
	/// properties to be seen as atomic, even if they happen via multiple events.
	pub struct ZxdgOutputV1(WlProxy);
	
	impl std::ops::Deref for ZxdgOutputV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgOutputV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgOutputV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgOutputV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgOutputV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgOutputV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgOutputV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.logical_position((proxy as *mut ZxdgOutputV1).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, ),
						1 => listener.logical_size((proxy as *mut ZxdgOutputV1).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, ),
						2 => listener.done((proxy as *mut ZxdgOutputV1).as_mut().unwrap(), ),
						3 => listener.name((proxy as *mut ZxdgOutputV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						4 => listener.description((proxy as *mut ZxdgOutputV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `logical_position` ARGS: x: {:?}, y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `logical_size` ARGS: width: {:?}, height: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `name` ARGS: name: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `description` ARGS: description: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgOutputV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the xdg_output object
		///
		/// Using this request a client can tell the server that it is not
		/// going to use the xdg_output object anymore.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZxdgOutputV1Listener: std::any::Any {
		
		/// # position of the output within the global compositor space
		///
		/// The position event describes the location of the wl_output within
		/// the global compositor space.
		///
		/// The logical_position event is sent after creating an xdg_output
		/// (see xdg_output_manager.get_xdg_output) and whenever the location
		/// of the output changes within the global compositor space.
		fn logical_position(
			&self,
			proxy: &mut ZxdgOutputV1,
			x               : i32,
			y               : i32,
		);
		
		/// # size of the output in the global compositor space
		///
		/// The logical_size event describes the size of the output in the
		/// global compositor space.
		///
		/// For example, a surface without any buffer scale, transformation
		/// nor rotation set, with the size matching the logical_size will
		/// have the same size as the corresponding output when displayed.
		///
		/// Most regular Wayland clients should not pay attention to the
		/// logical size and would rather rely on xdg_shell interfaces.
		///
		/// Some clients such as Xwayland, however, need this to configure
		/// their surfaces in the global compositor space as the compositor
		/// may apply a different scale from what is advertised by the output
		/// scaling property (to achieve fractional scaling, for example).
		///
		/// For example, for a wl_output mode 3840×2160 and a scale factor 2:
		///
		/// - A compositor not scaling the surface buffers will advertise a
		/// logical size of 3840×2160,
		///
		/// - A compositor automatically scaling the surface buffers will
		/// advertise a logical size of 1920×1080,
		///
		/// - A compositor using a fractional scale of 1.5 will advertise a
		/// logical size to 2560×1620.
		///
		/// For example, for a wl_output mode 1920×1080 and a 90 degree rotation,
		/// the compositor will advertise a logical size of 1080x1920.
		///
		/// The logical_size event is sent after creating an xdg_output
		/// (see xdg_output_manager.get_xdg_output) and whenever the logical
		/// size of the output changes, either as a result of a change in the
		/// applied scale or because of a change in the corresponding output
		/// mode(see wl_output.mode) or transform (see wl_output.transform).
		fn logical_size(
			&self,
			proxy: &mut ZxdgOutputV1,
			width           : i32,
			height          : i32,
		);
		
		/// # all information about the output have been sent
		///
		/// This event is sent after all other properties of an xdg_output
		/// have been sent.
		///
		/// This allows changes to the xdg_output properties to be seen as
		/// atomic, even if they happen via multiple events.
		///
		/// For objects version 3 onwards, this event is deprecated. Compositors
		/// are not required to send it anymore and must send wl_output.done
		/// instead.
		fn done(
			&self,
			proxy: &mut ZxdgOutputV1,
		);
		
		/// # name of this output
		///
		/// Many compositors will assign names to their outputs, show them to the
		/// user, allow them to be configured by name, etc. The client may wish to
		/// know this name as well to offer the user similar behaviors.
		///
		/// The naming convention is compositor defined, but limited to
		/// alphanumeric characters and dashes (-). Each name is unique among all
		/// wl_output globals, but if a wl_output global is destroyed the same name
		/// may be reused later. The names will also remain consistent across
		/// sessions with the same hardware and software configuration.
		///
		/// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
		/// not assume that the name is a reflection of an underlying DRM
		/// connector, X11 connection, etc.
		///
		/// The name event is sent after creating an xdg_output (see
		/// xdg_output_manager.get_xdg_output). This event is only sent once per
		/// xdg_output, and the name does not change over the lifetime of the
		/// wl_output global.
		fn name(
			&self,
			proxy: &mut ZxdgOutputV1,
			name            : &str,
		);
		
		/// # human-readable description of this output
		///
		/// Many compositors can produce human-readable descriptions of their
		/// outputs.  The client may wish to know this description as well, to
		/// communicate the user for various purposes.
		///
		/// The description is a UTF-8 string with no convention defined for its
		/// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
		/// output via :1'.
		///
		/// The description event is sent after creating an xdg_output (see
		/// xdg_output_manager.get_xdg_output) and whenever the description
		/// changes. The description is optional, and may not be sent at all.
		///
		/// For objects of version 2 and lower, this event is only sent once per
		/// xdg_output, and the description does not change over the lifetime of
		/// the wl_output global.
		fn description(
			&self,
			proxy: &mut ZxdgOutputV1,
			description     : &str,
		);
	}
}
pub use xdg_decoration_unstable_v1::*;
mod xdg_decoration_unstable_v1 {
	use crate::*;
	
	// Copyright © 2018 Simon Ser
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZXDG_DECORATION_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_decoration_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_toplevel_decoration\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZXDG_TOPLEVEL_DECORATION_V1_INTERFACE as _, &XDG_TOPLEVEL_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # window decoration manager
	///
	/// This interface allows a compositor to announce support for server-side
	/// decorations.
	///
	/// A window decoration is a set of window controls as deemed appropriate by
	/// the party managing them, such as user interface components used to move,
	/// resize and change a window's state.
	///
	/// A client can use this protocol to request being decorated by a supporting
	/// compositor.
	///
	/// If compositor and client do not negotiate the use of a server-side
	/// decoration using this protocol, clients continue to self-decorate as they
	/// see fit.
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZxdgDecorationManagerV1(WlProxy);
	
	impl std::ops::Deref for ZxdgDecorationManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgDecorationManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgDecorationManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgDecorationManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgDecorationManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the decoration manager object
		///
		/// Destroy the decoration manager. This doesn't destroy objects created
		/// with the manager.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a new toplevel decoration object
		///
		/// Create a new decoration object associated with the given toplevel.
		///
		/// Creating an xdg_toplevel_decoration from an xdg_toplevel which has a
		/// buffer attached or committed is a client error, and any attempts by a
		/// client to attach or manipulate a buffer prior to the first
		/// xdg_toplevel_decoration.configure event must also be treated as
		/// errors.
		pub fn get_toplevel_decoration(
			&self,
			toplevel        : &XdgToplevel
		) -> Result<Box<ZxdgToplevelDecorationV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_TOPLEVEL_DECORATION_V1_INTERFACE, std::ptr::null::<u8>(), toplevel) as *mut ZxdgToplevelDecorationV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZXDG_TOPLEVEL_DECORATION_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_toplevel_decoration_v1\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_mode\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "unset_mode\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # decoration object for a toplevel surface
	///
	/// The decoration object allows the compositor to toggle server-side window
	/// decorations for a toplevel surface. The client can request to switch to
	/// another mode.
	///
	/// The xdg_toplevel_decoration object must be destroyed before its
	/// xdg_toplevel.
	pub struct ZxdgToplevelDecorationV1(WlProxy);
	
	impl std::ops::Deref for ZxdgToplevelDecorationV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgToplevelDecorationV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgToplevelDecorationV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgToplevelDecorationV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgToplevelDecorationV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgToplevelDecorationV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgToplevelDecorationV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.configure((proxy as *mut ZxdgToplevelDecorationV1).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: mode: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgToplevelDecorationV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the decoration object
		///
		/// Switch back to a mode without any server-side decorations at the next
		/// commit.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the decoration mode
		///
		/// Set the toplevel surface decoration mode. This informs the compositor
		/// that the client prefers the provided decoration mode.
		///
		/// After requesting a decoration mode, the compositor will respond by
		/// emitting a xdg_surface.configure event. The client should then update
		/// its content, drawing it without decorations if the received mode is
		/// server-side decorations. The client must also acknowledge the configure
		/// when committing the new content (see xdg_surface.ack_configure).
		///
		/// The compositor can decide not to use the client's mode and enforce a
		/// different mode instead.
		///
		/// Clients whose decoration mode depend on the xdg_toplevel state may send
		/// a set_mode request in response to a xdg_surface.configure event and wait
		/// for the next xdg_surface.configure event to prevent unwanted state.
		/// Such clients are responsible for preventing configure loops and must
		/// make sure not to send multiple successive set_mode requests with the
		/// same decoration mode.
		pub fn set_mode(
			&self,
			mode            : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, mode); }
		}
		
		/// # unset the decoration mode
		///
		/// Unset the toplevel surface decoration mode. This informs the compositor
		/// that the client doesn't prefer a particular decoration mode.
		///
		/// This request has the same semantics as set_mode.
		pub fn unset_mode(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2); }
		}
	}
	
	
	pub trait ZxdgToplevelDecorationV1Listener: std::any::Any {
		
		/// # suggest a surface change
		///
		/// The configure event asks the client to change its decoration mode. The
		/// configured state should not be applied immediately. Clients must send an
		/// ack_configure in response to this event. See xdg_surface.configure and
		/// xdg_surface.ack_configure for details.
		///
		/// A configure event can be sent at any time. The specified mode must be
		/// obeyed by the client.
		fn configure(
			&self,
			proxy: &mut ZxdgToplevelDecorationV1,
			mode            : u32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgToplevelDecorationV1Error {
		/// xdg_toplevel has a buffer attached before configure
		UnconfiguredBuffer = 0,
		/// xdg_toplevel already has a decoration object
		AlreadyConstructed = 1,
		/// xdg_toplevel destroyed before the decoration object
		Orphaned = 2,
	}
	
	/// # window decoration modes
	///
	/// These values describe window decoration modes.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZxdgToplevelDecorationV1Mode {
		/// no server-side window decoration
		ClientSide = 1,
		/// server-side window decoration
		ServerSide = 2,
	}
}

/// # Protocol for exporting xdg surface handles
///
/// This protocol specifies a way for making it possible to reference a surface
/// of a different client. With such a reference, a client can, by using the
/// interfaces provided by this protocol, manipulate the relationship between
/// its own surfaces and the surface of some other client. For example, stack
/// some of its own surface above the other clients surface.
///
/// In order for a client A to get a reference of a surface of client B, client
/// B must first export its surface using xdg_exporter.export_toplevel. Upon
/// doing this, client B will receive a handle (a unique string) that it may
/// share with client A in some way (for example D-Bus). After client A has
/// received the handle from client B, it may use xdg_importer.import_toplevel
/// to create a reference to the surface client B just exported. See the
/// corresponding requests for details.
///
/// A possible use case for this is out-of-process dialogs. For example when a
/// sandboxed client without file system access needs the user to select a file
/// on the file system, given sandbox environment support, it can export its
/// surface, passing the exported surface handle to an unsandboxed process that
/// can show a file browser dialog and stack it above the sandboxed client's
/// surface.
///
/// Warning! The protocol described in this file is experimental and backward
/// incompatible changes may be made. Backward compatible changes may be added
/// together with the corresponding interface version bump. Backward
/// incompatible changes are done by bumping the version number in the protocol
/// and interface names and resetting the interface version. Once the protocol
/// is to be declared stable, the 'z' prefix and the version number in the
/// protocol and interface names are removed and the interface version number is
/// reset.
pub use xdg_foreign_unstable_v2::*;
mod xdg_foreign_unstable_v2 {
	use crate::*;
	
	// Copyright © 2015-2016 Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZXDG_EXPORTER_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_exporter_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "export_toplevel\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZXDG_EXPORTED_V2_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # interface for exporting surfaces
	///
	/// A global interface used for exporting surfaces that can later be imported
	/// using xdg_importer.
	pub struct ZxdgExporterV2(WlProxy);
	
	impl std::ops::Deref for ZxdgExporterV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgExporterV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgExporterV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgExporterV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgExporterV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the xdg_exporter object
		///
		/// Notify the compositor that the xdg_exporter object will no longer be
		/// used.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # export a toplevel surface
		///
		/// The export_toplevel request exports the passed surface so that it can later be
		/// imported via xdg_importer. When called, a new xdg_exported object will
		/// be created and xdg_exported.handle will be sent immediately. See the
		/// corresponding interface and event for details.
		///
		/// A surface may be exported multiple times, and each exported handle may
		/// be used to create a xdg_imported multiple times. Only xdg_toplevel
		/// equivalent surfaces may be exported.
		pub fn export_toplevel(
			&self,
			surface         : &WlSurface
		) -> Result<Box<ZxdgExportedV2, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_EXPORTED_V2_INTERFACE, std::ptr::null::<u8>(), surface) as *mut ZxdgExportedV2 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZXDG_IMPORTER_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_importer_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "import_toplevel\0".as_ptr(),
				signature: "ns\0".as_ptr(),
				types:     [&ZXDG_IMPORTED_V2_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # interface for importing surfaces
	///
	/// A global interface used for importing surfaces exported by xdg_exporter.
	/// With this interface, a client can create a reference to a surface of
	/// another client.
	pub struct ZxdgImporterV2(WlProxy);
	
	impl std::ops::Deref for ZxdgImporterV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgImporterV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgImporterV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgImporterV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgImporterV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the xdg_importer object
		///
		/// Notify the compositor that the xdg_importer object will no longer be
		/// used.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # import a toplevel surface
		///
		/// The import_toplevel request imports a surface from any client given a handle
		/// retrieved by exporting said surface using xdg_exporter.export_toplevel.
		/// When called, a new xdg_imported object will be created. This new object
		/// represents the imported surface, and the importing client can
		/// manipulate its relationship using it. See xdg_imported for details.
		pub fn import_toplevel(
			&self,
			handle          : &str
		) -> Result<Box<ZxdgImportedV2, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_IMPORTED_V2_INTERFACE, std::ptr::null::<u8>(), handle.as_ptr()) as *mut ZxdgImportedV2 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZXDG_EXPORTED_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_exported_v2\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "handle\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # an exported surface handle
	///
	/// A xdg_exported object represents an exported reference to a surface. The
	/// exported surface may be referenced as long as the xdg_exported object not
	/// destroyed. Destroying the xdg_exported invalidates any relationship the
	/// importer may have established using xdg_imported.
	pub struct ZxdgExportedV2(WlProxy);
	
	impl std::ops::Deref for ZxdgExportedV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgExportedV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgExportedV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgExportedV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgExportedV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgExportedV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgExportedV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.handle((proxy as *mut ZxdgExportedV2).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `handle` ARGS: handle: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgExportedV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # unexport the exported surface
		///
		/// Revoke the previously exported surface. This invalidates any
		/// relationship the importer may have set up using the xdg_imported created
		/// given the handle sent via xdg_exported.handle.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZxdgExportedV2Listener: std::any::Any {
		
		/// # the exported surface handle
		///
		/// The handle event contains the unique handle of this exported surface
		/// reference. It may be shared with any client, which then can use it to
		/// import the surface by calling xdg_importer.import_toplevel. A handle
		/// may be used to import the surface multiple times.
		fn handle(
			&self,
			proxy: &mut ZxdgExportedV2,
			handle          : &str,
		);
	}
	
	pub static ZXDG_IMPORTED_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_imported_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_parent_of\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "destroyed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # an imported surface handle
	///
	/// A xdg_imported object represents an imported reference to surface exported
	/// by some client. A client can use this interface to manipulate
	/// relationships between its own surfaces and the imported surface.
	pub struct ZxdgImportedV2(WlProxy);
	
	impl std::ops::Deref for ZxdgImportedV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgImportedV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgImportedV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgImportedV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgImportedV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgImportedV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgImportedV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.destroyed((proxy as *mut ZxdgImportedV2).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `destroyed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgImportedV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the xdg_imported object
		///
		/// Notify the compositor that it will no longer use the xdg_imported
		/// object. Any relationship that may have been set up will at this point
		/// be invalidated.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set as the parent of some surface
		///
		/// Set the imported surface as the parent of some surface of the client.
		/// The passed surface must be a xdg_toplevel equivalent. Calling this
		/// function sets up a surface to surface relation with the same stacking
		/// and positioning semantics as xdg_toplevel.set_parent.
		pub fn set_parent_of(
			&self,
			surface         : &WlSurface
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, surface); }
		}
	}
	
	
	pub trait ZxdgImportedV2Listener: std::any::Any {
		
		/// # the imported surface handle has been destroyed
		///
		/// The imported surface handle has been destroyed and any relationship set
		/// up has been invalidated. This may happen for various reasons, for
		/// example if the exported surface or the exported surface handle has been
		/// destroyed, if the handle used for importing was invalid.
		fn destroyed(
			&self,
			proxy: &mut ZxdgImportedV2,
		);
	}
}

/// # Protocol for exporting xdg surface handles
///
/// This protocol specifies a way for making it possible to reference a surface
/// of a different client. With such a reference, a client can, by using the
/// interfaces provided by this protocol, manipulate the relationship between
/// its own surfaces and the surface of some other client. For example, stack
/// some of its own surface above the other clients surface.
///
/// In order for a client A to get a reference of a surface of client B, client
/// B must first export its surface using xdg_exporter.export. Upon doing this,
/// client B will receive a handle (a unique string) that it may share with
/// client A in some way (for example D-Bus). After client A has received the
/// handle from client B, it may use xdg_importer.import to create a reference
/// to the surface client B just exported. See the corresponding requests for
/// details.
///
/// A possible use case for this is out-of-process dialogs. For example when a
/// sandboxed client without file system access needs the user to select a file
/// on the file system, given sandbox environment support, it can export its
/// surface, passing the exported surface handle to an unsandboxed process that
/// can show a file browser dialog and stack it above the sandboxed client's
/// surface.
///
/// Warning! The protocol described in this file is experimental and backward
/// incompatible changes may be made. Backward compatible changes may be added
/// together with the corresponding interface version bump. Backward
/// incompatible changes are done by bumping the version number in the protocol
/// and interface names and resetting the interface version. Once the protocol
/// is to be declared stable, the 'z' prefix and the version number in the
/// protocol and interface names are removed and the interface version number is
/// reset.
pub use xdg_foreign_unstable_v1::*;
mod xdg_foreign_unstable_v1 {
	use crate::*;
	
	// Copyright © 2015-2016 Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZXDG_EXPORTER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_exporter_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "export\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZXDG_EXPORTED_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # interface for exporting surfaces
	///
	/// A global interface used for exporting surfaces that can later be imported
	/// using xdg_importer.
	pub struct ZxdgExporterV1(WlProxy);
	
	impl std::ops::Deref for ZxdgExporterV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgExporterV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgExporterV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgExporterV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgExporterV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the xdg_exporter object
		///
		/// Notify the compositor that the xdg_exporter object will no longer be
		/// used.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # export a surface
		///
		/// The export request exports the passed surface so that it can later be
		/// imported via xdg_importer. When called, a new xdg_exported object will
		/// be created and xdg_exported.handle will be sent immediately. See the
		/// corresponding interface and event for details.
		///
		/// A surface may be exported multiple times, and each exported handle may
		/// be used to create a xdg_imported multiple times. Only xdg_surface
		/// surfaces may be exported.
		pub fn export(
			&self,
			surface         : &WlSurface
		) -> Result<Box<ZxdgExportedV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_EXPORTED_V1_INTERFACE, std::ptr::null::<u8>(), surface) as *mut ZxdgExportedV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZXDG_IMPORTER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_importer_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "import\0".as_ptr(),
				signature: "ns\0".as_ptr(),
				types:     [&ZXDG_IMPORTED_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # interface for importing surfaces
	///
	/// A global interface used for importing surfaces exported by xdg_exporter.
	/// With this interface, a client can create a reference to a surface of
	/// another client.
	pub struct ZxdgImporterV1(WlProxy);
	
	impl std::ops::Deref for ZxdgImporterV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgImporterV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgImporterV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgImporterV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgImporterV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the xdg_importer object
		///
		/// Notify the compositor that the xdg_importer object will no longer be
		/// used.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # import a surface
		///
		/// The import request imports a surface from any client given a handle
		/// retrieved by exporting said surface using xdg_exporter.export. When
		/// called, a new xdg_imported object will be created. This new object
		/// represents the imported surface, and the importing client can
		/// manipulate its relationship using it. See xdg_imported for details.
		pub fn import(
			&self,
			handle          : &str
		) -> Result<Box<ZxdgImportedV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZXDG_IMPORTED_V1_INTERFACE, std::ptr::null::<u8>(), handle.as_ptr()) as *mut ZxdgImportedV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZXDG_EXPORTED_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_exported_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "handle\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # an exported surface handle
	///
	/// A xdg_exported object represents an exported reference to a surface. The
	/// exported surface may be referenced as long as the xdg_exported object not
	/// destroyed. Destroying the xdg_exported invalidates any relationship the
	/// importer may have established using xdg_imported.
	pub struct ZxdgExportedV1(WlProxy);
	
	impl std::ops::Deref for ZxdgExportedV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgExportedV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgExportedV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgExportedV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgExportedV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgExportedV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgExportedV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.handle((proxy as *mut ZxdgExportedV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `handle` ARGS: handle: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgExportedV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # unexport the exported surface
		///
		/// Revoke the previously exported surface. This invalidates any
		/// relationship the importer may have set up using the xdg_imported created
		/// given the handle sent via xdg_exported.handle.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZxdgExportedV1Listener: std::any::Any {
		
		/// # the exported surface handle
		///
		/// The handle event contains the unique handle of this exported surface
		/// reference. It may be shared with any client, which then can use it to
		/// import the surface by calling xdg_importer.import. A handle may be
		/// used to import the surface multiple times.
		fn handle(
			&self,
			proxy: &mut ZxdgExportedV1,
			handle          : &str,
		);
	}
	
	pub static ZXDG_IMPORTED_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zxdg_imported_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_parent_of\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "destroyed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # an imported surface handle
	///
	/// A xdg_imported object represents an imported reference to surface exported
	/// by some client. A client can use this interface to manipulate
	/// relationships between its own surfaces and the imported surface.
	pub struct ZxdgImportedV1(WlProxy);
	
	impl std::ops::Deref for ZxdgImportedV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZxdgImportedV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZxdgImportedV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZxdgImportedV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZxdgImportedV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZxdgImportedV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZxdgImportedV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.destroyed((proxy as *mut ZxdgImportedV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `destroyed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZxdgImportedV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the xdg_imported object
		///
		/// Notify the compositor that it will no longer use the xdg_imported
		/// object. Any relationship that may have been set up will at this point
		/// be invalidated.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set as the parent of some surface
		///
		/// Set the imported surface as the parent of some surface of the client.
		/// The passed surface must be a toplevel xdg_surface. Calling this function
		/// sets up a surface to surface relation with the same stacking and positioning
		/// semantics as xdg_surface.set_parent.
		pub fn set_parent_of(
			&self,
			surface         : &WlSurface
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, surface); }
		}
	}
	
	
	pub trait ZxdgImportedV1Listener: std::any::Any {
		
		/// # the imported surface handle has been destroyed
		///
		/// The imported surface handle has been destroyed and any relationship set
		/// up has been invalidated. This may happen for various reasons, for
		/// example if the exported surface or the exported surface handle has been
		/// destroyed, if the handle used for importing was invalid.
		fn destroyed(
			&self,
			proxy: &mut ZxdgImportedV1,
		);
	}
}

/// # Protocol for composing text
///
/// This protocol allows compositors to act as input methods and to send text
/// to applications. A text input object is used to manage state of what are
/// typically text entry fields in the application.
///
/// This document adheres to the RFC 2119 when using words like "must",
/// "should", "may", etc.
///
/// Warning! The protocol described in this file is experimental and
/// backward incompatible changes may be made. Backward compatible changes
/// may be added together with the corresponding interface version bump.
/// Backward incompatible changes are done by bumping the version number in
/// the protocol and interface names and resetting the interface version.
/// Once the protocol is to be declared stable, the 'z' prefix and the
/// version number in the protocol and interface names are removed and the
/// interface version number is reset.
pub use text_input_unstable_v3::*;
mod text_input_unstable_v3 {
	use crate::*;
	
	// Copyright © 2012, 2013 Intel Corporation
	// Copyright © 2015, 2016 Jan Arne Petersen
	// Copyright © 2017, 2018 Red Hat, Inc.
	// Copyright © 2018       Purism SPC
	//
	// Permission to use, copy, modify, distribute, and sell this
	// software and its documentation for any purpose is hereby granted
	// without fee, provided that the above copyright notice appear in
	// all copies and that both that copyright notice and this permission
	// notice appear in supporting documentation, and that the name of
	// the copyright holders not be used in advertising or publicity
	// pertaining to distribution of the software without specific,
	// written prior permission.  The copyright holders make no
	// representations about the suitability of this software for any
	// purpose.  It is provided "as is" without express or implied
	// warranty.
	//
	// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS
	// SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
	// FITNESS, IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY
	// SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
	// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
	// AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
	// ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
	// THIS SOFTWARE.
	
	pub static ZWP_TEXT_INPUT_V3_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_text_input_v3\0".as_ptr(),
		version:      1,
		method_count: 8,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "enable\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "disable\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_surrounding_text\0".as_ptr(),
				signature: "sii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_text_change_cause\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_content_type\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_cursor_rectangle\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "commit\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  6,
		events:       [
			WlMessage {
				name:      "enter\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "leave\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "preedit_string\0".as_ptr(),
				signature: "?sii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "commit_string\0".as_ptr(),
				signature: "?s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "delete_surrounding_text\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # text input
	///
	/// The zwp_text_input_v3 interface represents text input and input methods
	/// associated with a seat. It provides enter/leave events to follow the
	/// text input focus for a seat.
	///
	/// Requests are used to enable/disable the text-input object and set
	/// state information like surrounding and selected text or the content type.
	/// The information about the entered text is sent to the text-input object
	/// via the preedit_string and commit_string events.
	///
	/// Text is valid UTF-8 encoded, indices and lengths are in bytes. Indices
	/// must not point to middle bytes inside a code point: they must either
	/// point to the first byte of a code point or to the end of the buffer.
	/// Lengths must be measured between two valid indices.
	///
	/// Focus moving throughout surfaces will result in the emission of
	/// zwp_text_input_v3.enter and zwp_text_input_v3.leave events. The focused
	/// surface must commit zwp_text_input_v3.enable and
	/// zwp_text_input_v3.disable requests as the keyboard focus moves across
	/// editable and non-editable elements of the UI. Those two requests are not
	/// expected to be paired with each other, the compositor must be able to
	/// handle consecutive series of the same request.
	///
	/// State is sent by the state requests (set_surrounding_text,
	/// set_content_type and set_cursor_rectangle) and a commit request. After an
	/// enter event or disable request all state information is invalidated and
	/// needs to be resent by the client.
	pub struct ZwpTextInputV3(WlProxy);
	
	impl std::ops::Deref for ZwpTextInputV3 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTextInputV3 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTextInputV3 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTextInputV3")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTextInputV3 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTextInputV3Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTextInputV3Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.enter((proxy as *mut ZwpTextInputV3).as_mut().unwrap(), ((*args.add(0)).o as *mut WlSurface).as_mut(), ),
						1 => listener.leave((proxy as *mut ZwpTextInputV3).as_mut().unwrap(), ((*args.add(0)).o as *mut WlSurface).as_mut(), ),
						2 => listener.preedit_string((proxy as *mut ZwpTextInputV3).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).i, (*args.add(2)).i, ),
						3 => listener.commit_string((proxy as *mut ZwpTextInputV3).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						4 => listener.delete_surrounding_text((proxy as *mut ZwpTextInputV3).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						5 => listener.done((proxy as *mut ZwpTextInputV3).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `enter` ARGS: surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlSurface).as_mut()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `leave` ARGS: surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlSurface).as_mut()),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `preedit_string` ARGS: text: {:?}, cursor_begin: {:?}, cursor_end: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).i, (*args.add(2)).i),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `commit_string` ARGS: text: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `delete_surrounding_text` ARGS: before_length: {:?}, after_length: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTextInputV3Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # Destroy the wp_text_input
		///
		/// Destroy the wp_text_input object. Also disables all surfaces enabled
		/// through this wp_text_input object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # Request text input to be enabled
		///
		/// Requests text input on the surface previously obtained from the enter
		/// event.
		///
		/// This request must be issued every time the active text input changes
		/// to a new one, including within the current surface. Use
		/// zwp_text_input_v3.disable when there is no longer any input focus on
		/// the current surface.
		///
		/// This request resets all state associated with previous enable, disable,
		/// set_surrounding_text, set_text_change_cause, set_content_type, and
		/// set_cursor_rectangle requests, as well as the state associated with
		/// preedit_string, commit_string, and delete_surrounding_text events.
		///
		/// The set_surrounding_text, set_content_type and set_cursor_rectangle
		/// requests must follow if the text input supports the necessary
		/// functionality.
		///
		/// State set with this request is double-buffered. It will get applied on
		/// the next zwp_text_input_v3.commit request, and stay valid until the
		/// next committed enable or disable request.
		///
		/// The changes must be applied by the compositor after issuing a
		/// zwp_text_input_v3.commit request.
		pub fn enable(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
		}
		
		/// # Disable text input on a surface
		///
		/// Explicitly disable text input on the current surface (typically when
		/// there is no focus on any text entry inside the surface).
		///
		/// State set with this request is double-buffered. It will get applied on
		/// the next zwp_text_input_v3.commit request.
		pub fn disable(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2); }
		}
		
		/// # sets the surrounding text
		///
		/// Sets the surrounding plain text around the input, excluding the preedit
		/// text.
		///
		/// The client should notify the compositor of any changes in any of the
		/// values carried with this request, including changes caused by handling
		/// incoming text-input events as well as changes caused by other
		/// mechanisms like keyboard typing.
		///
		/// If the client is unaware of the text around the cursor, it should not
		/// issue this request, to signify lack of support to the compositor.
		///
		/// Text is UTF-8 encoded, and should include the cursor position, the
		/// complete selection and additional characters before and after them.
		/// There is a maximum length of wayland messages, so text can not be
		/// longer than 4000 bytes.
		///
		/// Cursor is the byte offset of the cursor within text buffer.
		///
		/// Anchor is the byte offset of the selection anchor within text buffer.
		/// If there is no selected text, anchor is the same as cursor.
		///
		/// If any preedit text is present, it is replaced with a cursor for the
		/// purpose of this event.
		///
		/// Values set with this request are double-buffered. They will get applied
		/// on the next zwp_text_input_v3.commit request, and stay valid until the
		/// next committed enable or disable request.
		///
		/// The initial state for affected fields is empty, meaning that the text
		/// input does not support sending surrounding text. If the empty values
		/// get applied, subsequent attempts to change them may have no effect.
		pub fn set_surrounding_text(
			&self,
			text            : &str,
			cursor          : i32,
			anchor          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, text.as_ptr(), cursor, anchor); }
		}
		
		/// # indicates the cause of surrounding text change
		///
		/// Tells the compositor why the text surrounding the cursor changed.
		///
		/// Whenever the client detects an external change in text, cursor, or
		/// anchor posision, it must issue this request to the compositor. This
		/// request is intended to give the input method a chance to update the
		/// preedit text in an appropriate way, e.g. by removing it when the user
		/// starts typing with a keyboard.
		///
		/// cause describes the source of the change.
		///
		/// The value set with this request is double-buffered. It must be applied
		/// and reset to initial at the next zwp_text_input_v3.commit request.
		///
		/// The initial value of cause is input_method.
		pub fn set_text_change_cause(
			&self,
			cause           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, cause); }
		}
		
		/// # set content purpose and hint
		///
		/// Sets the content purpose and content hint. While the purpose is the
		/// basic purpose of an input field, the hint flags allow to modify some of
		/// the behavior.
		///
		/// Values set with this request are double-buffered. They will get applied
		/// on the next zwp_text_input_v3.commit request.
		/// Subsequent attempts to update them may have no effect. The values
		/// remain valid until the next committed enable or disable request.
		///
		/// The initial value for hint is none, and the initial value for purpose
		/// is normal.
		pub fn set_content_type(
			&self,
			hint            : u32,
			purpose         : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, hint, purpose); }
		}
		
		/// # set cursor position
		///
		/// Marks an area around the cursor as a x, y, width, height rectangle in
		/// surface local coordinates.
		///
		/// Allows the compositor to put a window with word suggestions near the
		/// cursor, without obstructing the text being input.
		///
		/// If the client is unaware of the position of edited text, it should not
		/// issue this request, to signify lack of support to the compositor.
		///
		/// Values set with this request are double-buffered. They will get applied
		/// on the next zwp_text_input_v3.commit request, and stay valid until the
		/// next committed enable or disable request.
		///
		/// The initial values describing a cursor rectangle are empty. That means
		/// the text input does not support describing the cursor area. If the
		/// empty values get applied, subsequent attempts to change them may have
		/// no effect.
		pub fn set_cursor_rectangle(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, x, y, width, height); }
		}
		
		/// # commit state
		///
		/// Atomically applies state changes recently sent to the compositor.
		///
		/// The commit request establishes and updates the state of the client, and
		/// must be issued after any changes to apply them.
		///
		/// Text input state (enabled status, content purpose, content hint,
		/// surrounding text and change cause, cursor rectangle) is conceptually
		/// double-buffered within the context of a text input, i.e. between a
		/// committed enable request and the following committed enable or disable
		/// request.
		///
		/// Protocol requests modify the pending state, as opposed to the current
		/// state in use by the input method. A commit request atomically applies
		/// all pending state, replacing the current state. After commit, the new
		/// pending state is as documented for each related request.
		///
		/// Requests are applied in the order of arrival.
		///
		/// Neither current nor pending state are modified unless noted otherwise.
		///
		/// The compositor must count the number of commit requests coming from
		/// each zwp_text_input_v3 object and use the count as the serial in done
		/// events.
		pub fn commit(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7); }
		}
	}
	
	
	pub trait ZwpTextInputV3Listener: std::any::Any {
		
		/// # enter event
		///
		/// Notification that this seat's text-input focus is on a certain surface.
		///
		/// When the seat has the keyboard capability the text-input focus follows
		/// the keyboard focus. This event sets the current surface for the
		/// text-input object.
		fn enter(
			&self,
			proxy: &mut ZwpTextInputV3,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # leave event
		///
		/// Notification that this seat's text-input focus is no longer on a
		/// certain surface. The client should reset any preedit string previously
		/// set.
		///
		/// The leave notification clears the current surface. It is sent before
		/// the enter notification for the new focus.
		///
		/// When the seat has the keyboard capability the text-input focus follows
		/// the keyboard focus.
		fn leave(
			&self,
			proxy: &mut ZwpTextInputV3,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # pre-edit
		///
		/// Notify when a new composing text (pre-edit) should be set at the
		/// current cursor position. Any previously set composing text must be
		/// removed. Any previously existing selected text must be removed.
		///
		/// The argument text contains the pre-edit string buffer.
		///
		/// The parameters cursor_begin and cursor_end are counted in bytes
		/// relative to the beginning of the submitted text buffer. Cursor should
		/// be hidden when both are equal to -1.
		///
		/// They could be represented by the client as a line if both values are
		/// the same, or as a text highlight otherwise.
		///
		/// Values set with this event are double-buffered. They must be applied
		/// and reset to initial on the next zwp_text_input_v3.done event.
		///
		/// The initial value of text is an empty string, and cursor_begin,
		/// cursor_end and cursor_hidden are all 0.
		fn preedit_string(
			&self,
			proxy: &mut ZwpTextInputV3,
			text            : &str,
			cursor_begin    : i32,
			cursor_end      : i32,
		);
		
		/// # text commit
		///
		/// Notify when text should be inserted into the editor widget. The text to
		/// commit could be either just a single character after a key press or the
		/// result of some composing (pre-edit).
		///
		/// Values set with this event are double-buffered. They must be applied
		/// and reset to initial on the next zwp_text_input_v3.done event.
		///
		/// The initial value of text is an empty string.
		fn commit_string(
			&self,
			proxy: &mut ZwpTextInputV3,
			text            : &str,
		);
		
		/// # delete surrounding text
		///
		/// Notify when the text around the current cursor position should be
		/// deleted.
		///
		/// Before_length and after_length are the number of bytes before and after
		/// the current cursor index (excluding the selection) to delete.
		///
		/// If a preedit text is present, in effect before_length is counted from
		/// the beginning of it, and after_length from its end (see done event
		/// sequence).
		///
		/// Values set with this event are double-buffered. They must be applied
		/// and reset to initial on the next zwp_text_input_v3.done event.
		///
		/// The initial values of both before_length and after_length are 0.
		fn delete_surrounding_text(
			&self,
			proxy: &mut ZwpTextInputV3,
			before_length   : u32,
			after_length    : u32,
		);
		
		/// # apply changes
		///
		/// Instruct the application to apply changes to state requested by the
		/// preedit_string, commit_string and delete_surrounding_text events. The
		/// state relating to these events is double-buffered, and each one
		/// modifies the pending state. This event replaces the current state with
		/// the pending state.
		///
		/// The application must proceed by evaluating the changes in the following
		/// order:
		///
		/// 1. Replace existing preedit string with the cursor.
		/// 2. Delete requested surrounding text.
		/// 3. Insert commit string with the cursor at its end.
		/// 4. Calculate surrounding text to send.
		/// 5. Insert new preedit text in cursor position.
		/// 6. Place cursor inside preedit text.
		///
		/// The serial number reflects the last state of the zwp_text_input_v3
		/// object known to the compositor. The value of the serial argument must
		/// be equal to the number of commit requests already issued on that object.
		/// When the client receives a done event with a serial different than the
		/// number of past commit requests, it must proceed as normal, except it
		/// should not change the current state of the zwp_text_input_v3 object.
		fn done(
			&self,
			proxy: &mut ZwpTextInputV3,
			serial          : u32,
		);
	}
	
	/// # text change reason
	///
	/// Reason for the change of surrounding text or cursor posision.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTextInputV3ChangeCause {
		/// input method caused the change
		InputMethod = 0,
		/// something else than the input method caused the change
		Other = 1,
	}
	
	/// # content hint
	///
	/// Content hint is a bitmask to allow to modify the behavior of the text
	/// input.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTextInputV3ContentHint {
		/// no special behavior
		None = 0x0,
		/// suggest word completions
		Completion = 0x1,
		/// suggest word corrections
		Spellcheck = 0x2,
		/// switch to uppercase letters at the start of a sentence
		AutoCapitalization = 0x4,
		/// prefer lowercase letters
		Lowercase = 0x8,
		/// prefer uppercase letters
		Uppercase = 0x10,
		/// prefer casing for titles and headings (can be language dependent)
		Titlecase = 0x20,
		/// characters should be hidden
		HiddenText = 0x40,
		/// typed text should not be stored
		SensitiveData = 0x80,
		/// just Latin characters should be entered
		Latin = 0x100,
		/// the text input is multiline
		Multiline = 0x200,
	}
	
	/// # content purpose
	///
	/// The content purpose allows to specify the primary purpose of a text
	/// input.
	///
	/// This allows an input method to show special purpose input panels with
	/// extra characters or to disallow some characters.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTextInputV3ContentPurpose {
		/// default input, allowing all characters
		Normal = 0,
		/// allow only alphabetic characters
		Alpha = 1,
		/// allow only digits
		Digits = 2,
		/// input a number (including decimal separator and sign)
		Number = 3,
		/// input a phone number
		Phone = 4,
		/// input an URL
		Url = 5,
		/// input an email address
		Email = 6,
		/// input a name of a person
		Name = 7,
		/// input a password (combine with sensitive_data hint)
		Password = 8,
		/// input is a numeric password (combine with sensitive_data hint)
		Pin = 9,
		/// input a date
		Date = 10,
		/// input a time
		Time = 11,
		/// input a date and time
		Datetime = 12,
		/// input for a terminal
		Terminal = 13,
	}
	
	pub static ZWP_TEXT_INPUT_MANAGER_V3_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_text_input_manager_v3\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_text_input\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_TEXT_INPUT_V3_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # text input manager
	///
	/// A factory for text-input objects. This object is a global singleton.
	pub struct ZwpTextInputManagerV3(WlProxy);
	
	impl std::ops::Deref for ZwpTextInputManagerV3 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTextInputManagerV3 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTextInputManagerV3 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTextInputManagerV3")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTextInputManagerV3 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # Destroy the wp_text_input_manager
		///
		/// Destroy the wp_text_input_manager object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a new text input object
		///
		/// Creates a new text-input object for a given seat.
		pub fn get_text_input(
			&self,
			seat            : &WlSeat
		) -> Result<Box<ZwpTextInputV3, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_TEXT_INPUT_V3_INTERFACE, std::ptr::null::<u8>(), seat) as *mut ZwpTextInputV3 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
}
pub use viewporter::*;
mod viewporter {
	use crate::*;
	
	// Copyright © 2013-2016 Collabora, Ltd.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static WP_VIEWPORTER_INTERFACE: WlInterface = WlInterface {
		name:         "wp_viewporter\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_viewport\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&WP_VIEWPORT_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # surface cropping and scaling
	///
	/// The global interface exposing surface cropping and scaling
	/// capabilities is used to instantiate an interface extension for a
	/// wl_surface object. This extended interface will then allow
	/// cropping and scaling the surface contents, effectively
	/// disconnecting the direct relationship between the buffer and the
	/// surface size.
	pub struct WpViewporter(WlProxy);
	
	impl std::ops::Deref for WpViewporter {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WpViewporter {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WpViewporter {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WpViewporter")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WpViewporter {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # unbind from the cropping and scaling interface
		///
		/// Informs the server that the client will not be using this
		/// protocol object anymore. This does not affect any other objects,
		/// wp_viewport objects included.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # extend surface interface for crop and scale
		///
		/// Instantiate an interface extension for the given wl_surface to
		/// crop and scale its content. If the given wl_surface already has
		/// a wp_viewport object associated, the viewport_exists
		/// protocol error is raised.
		pub fn get_viewport(
			&self,
			surface         : &WlSurface
		) -> Result<Box<WpViewport, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &WP_VIEWPORT_INTERFACE, std::ptr::null::<u8>(), surface) as *mut WpViewport };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WpViewporterError {
		/// the surface already has a viewport object associated
		ViewportExists = 0,
	}
	
	pub static WP_VIEWPORT_INTERFACE: WlInterface = WlInterface {
		name:         "wp_viewport\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_source\0".as_ptr(),
				signature: "ffff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_destination\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # crop and scale interface to a wl_surface
	///
	/// An additional interface to a wl_surface object, which allows the
	/// client to specify the cropping and scaling of the surface
	/// contents.
	///
	/// This interface works with two concepts: the source rectangle (src_x,
	/// src_y, src_width, src_height), and the destination size (dst_width,
	/// dst_height). The contents of the source rectangle are scaled to the
	/// destination size, and content outside the source rectangle is ignored.
	/// This state is double-buffered, and is applied on the next
	/// wl_surface.commit.
	///
	/// The two parts of crop and scale state are independent: the source
	/// rectangle, and the destination size. Initially both are unset, that
	/// is, no scaling is applied. The whole of the current wl_buffer is
	/// used as the source, and the surface size is as defined in
	/// wl_surface.attach.
	///
	/// If the destination size is set, it causes the surface size to become
	/// dst_width, dst_height. The source (rectangle) is scaled to exactly
	/// this size. This overrides whatever the attached wl_buffer size is,
	/// unless the wl_buffer is NULL. If the wl_buffer is NULL, the surface
	/// has no content and therefore no size. Otherwise, the size is always
	/// at least 1x1 in surface local coordinates.
	///
	/// If the source rectangle is set, it defines what area of the wl_buffer is
	/// taken as the source. If the source rectangle is set and the destination
	/// size is not set, then src_width and src_height must be integers, and the
	/// surface size becomes the source rectangle size. This results in cropping
	/// without scaling. If src_width or src_height are not integers and
	/// destination size is not set, the bad_size protocol error is raised when
	/// the surface state is applied.
	///
	/// The coordinate transformations from buffer pixel coordinates up to
	/// the surface-local coordinates happen in the following order:
	/// 1. buffer_transform (wl_surface.set_buffer_transform)
	/// 2. buffer_scale (wl_surface.set_buffer_scale)
	/// 3. crop and scale (wp_viewport.set*)
	/// This means, that the source rectangle coordinates of crop and scale
	/// are given in the coordinates after the buffer transform and scale,
	/// i.e. in the coordinates that would be the surface-local coordinates
	/// if the crop and scale was not applied.
	///
	/// If src_x or src_y are negative, the bad_value protocol error is raised.
	/// Otherwise, if the source rectangle is partially or completely outside of
	/// the non-NULL wl_buffer, then the out_of_buffer protocol error is raised
	/// when the surface state is applied. A NULL wl_buffer does not raise the
	/// out_of_buffer error.
	///
	/// The x, y arguments of wl_surface.attach are applied as normal to
	/// the surface. They indicate how many pixels to remove from the
	/// surface size from the left and the top. In other words, they are
	/// still in the surface-local coordinate system, just like dst_width
	/// and dst_height are.
	///
	/// If the wl_surface associated with the wp_viewport is destroyed,
	/// all wp_viewport requests except 'destroy' raise the protocol error
	/// no_surface.
	///
	/// If the wp_viewport object is destroyed, the crop and scale
	/// state is removed from the wl_surface. The change will be applied
	/// on the next wl_surface.commit.
	pub struct WpViewport(WlProxy);
	
	impl std::ops::Deref for WpViewport {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WpViewport {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WpViewport {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WpViewport")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WpViewport {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # remove scaling and cropping from the surface
		///
		/// The associated wl_surface's crop and scale state is removed.
		/// The change is applied on the next wl_surface.commit.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the source rectangle for cropping
		///
		/// Set the source rectangle of the associated wl_surface. See
		/// wp_viewport for the description, and relation to the wl_buffer
		/// size.
		///
		/// If all of x, y, width and height are -1.0, the source rectangle is
		/// unset instead. Any other set of values where width or height are zero
		/// or negative, or x or y are negative, raise the bad_value protocol
		/// error.
		///
		/// The crop and scale state is double-buffered state, and will be
		/// applied on the next wl_surface.commit.
		pub fn set_source(
			&self,
			x               : WlFixed,
			y               : WlFixed,
			width           : WlFixed,
			height          : WlFixed
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, x, y, width, height); }
		}
		
		/// # set the surface size for scaling
		///
		/// Set the destination size of the associated wl_surface. See
		/// wp_viewport for the description, and relation to the wl_buffer
		/// size.
		///
		/// If width is -1 and height is -1, the destination size is unset
		/// instead. Any other pair of values for width and height that
		/// contains zero or negative values raises the bad_value protocol
		/// error.
		///
		/// The crop and scale state is double-buffered state, and will be
		/// applied on the next wl_surface.commit.
		pub fn set_destination(
			&self,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, width, height); }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WpViewportError {
		/// negative or zero values in width or height
		BadValue = 0,
		/// destination size is not integer
		BadSize = 1,
		/// source rectangle extends outside of the content area
		OutOfBuffer = 2,
		/// the wl_surface was destroyed
		NoSurface = 3,
	}
}

/// # Wayland protocol for graphics tablets
///
/// This description provides a high-level overview of the interplay between
/// the interfaces defined this protocol. For details, see the protocol
/// specification.
///
/// More than one tablet may exist, and device-specifics matter. Tablets are
/// not represented by a single virtual device like wl_pointer. A client
/// binds to the tablet manager object which is just a proxy object. From
/// that, the client requests wp_tablet_manager.get_tablet_seat(wl_seat)
/// and that returns the actual interface that has all the tablets. With
/// this indirection, we can avoid merging wp_tablet into the actual Wayland
/// protocol, a long-term benefit.
///
/// The wp_tablet_seat sends a "tablet added" event for each tablet
/// connected. That event is followed by descriptive events about the
/// hardware; currently that includes events for name, vid/pid and
/// a wp_tablet.path event that describes a local path. This path can be
/// used to uniquely identify a tablet or get more information through
/// libwacom. Emulated or nested tablets can skip any of those, e.g. a
/// virtual tablet may not have a vid/pid. The sequence of descriptive
/// events is terminated by a wp_tablet.done event to signal that a client
/// may now finalize any initialization for that tablet.
///
/// Events from tablets require a tool in proximity. Tools are also managed
/// by the tablet seat; a "tool added" event is sent whenever a tool is new
/// to the compositor. That event is followed by a number of descriptive
/// events about the hardware; currently that includes capabilities,
/// hardware id and serial number, and tool type. Similar to the tablet
/// interface, a wp_tablet_tool.done event is sent to terminate that initial
/// sequence.
///
/// Any event from a tool happens on the wp_tablet_tool interface. When the
/// tool gets into proximity of the tablet, a proximity_in event is sent on
/// the wp_tablet_tool interface, listing the tablet and the surface. That
/// event is followed by a motion event with the coordinates. After that,
/// it's the usual motion, axis, button, etc. events. The protocol's
/// serialisation means events are grouped by wp_tablet_tool.frame events.
///
/// Two special events (that don't exist in X) are down and up. They signal
/// "tip touching the surface". For tablets without real proximity
/// detection, the sequence is: proximity_in, motion, down, frame.
///
/// When the tool leaves proximity, a proximity_out event is sent. If any
/// button is still down, a button release event is sent before this
/// proximity event. These button events are sent in the same frame as the
/// proximity event to signal to the client that the buttons were held when
/// the tool left proximity.
///
/// If the tool moves out of the surface but stays in proximity (i.e.
/// between windows), compositor-specific grab policies apply. This usually
/// means that the proximity-out is delayed until all buttons are released.
///
/// Moving a tool physically from one tablet to the other has no real effect
/// on the protocol, since we already have the tool object from the "tool
/// added" event. All the information is already there and the proximity
/// events on both tablets are all a client needs to reconstruct what
/// happened.
///
/// Some extra axes are normalized, i.e. the client knows the range as
/// specified in the protocol (e.g. [0, 65535]), the granularity however is
/// unknown. The current normalized axes are pressure, distance, and slider.
///
/// Other extra axes are in physical units as specified in the protocol.
/// The current extra axes with physical units are tilt, rotation and
/// wheel rotation.
///
/// Since tablets work independently of the pointer controlled by the mouse,
/// the focus handling is independent too and controlled by proximity.
/// The wp_tablet_tool.set_cursor request sets a tool-specific cursor.
/// This cursor surface may be the same as the mouse cursor, and it may be
/// the same across tools but it is possible to be more fine-grained. For
/// example, a client may set different cursors for the pen and eraser.
///
/// Tools are generally independent of tablets and it is
/// compositor-specific policy when a tool can be removed. Common approaches
/// will likely include some form of removing a tool when all tablets the
/// tool was used on are removed.
///
/// Warning! The protocol described in this file is experimental and
/// backward incompatible changes may be made. Backward compatible changes
/// may be added together with the corresponding interface version bump.
/// Backward incompatible changes are done by bumping the version number in
/// the protocol and interface names and resetting the interface version.
/// Once the protocol is to be declared stable, the 'z' prefix and the
/// version number in the protocol and interface names are removed and the
/// interface version number is reset.
pub use tablet_unstable_v2::*;
mod tablet_unstable_v2 {
	use crate::*;
	
	// Copyright 2014 © Stephen "Lyude" Chandler Paul
	// Copyright 2015-2016 © Red Hat, Inc.
	//
	// Permission is hereby granted, free of charge, to any person
	// obtaining a copy of this software and associated documentation files
	// (the "Software"), to deal in the Software without restriction,
	// including without limitation the rights to use, copy, modify, merge,
	// publish, distribute, sublicense, and/or sell copies of the Software,
	// and to permit persons to whom the Software is furnished to do so,
	// subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the
	// next paragraph) shall be included in all copies or substantial
	// portions of the Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
	// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
	// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
	// NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
	// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
	// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
	// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
	// SOFTWARE.
	
	pub static ZWP_TABLET_MANAGER_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_manager_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "get_tablet_seat\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_TABLET_SEAT_V2_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # controller object for graphic tablet devices
	///
	/// An object that provides access to the graphics tablets available on this
	/// system. All tablets are associated with a seat, to get access to the
	/// actual tablets, use wp_tablet_manager.get_tablet_seat.
	pub struct ZwpTabletManagerV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletManagerV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletManagerV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletManagerV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletManagerV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletManagerV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # get the tablet seat
		///
		/// Get the wp_tablet_seat object for the given seat. This object
		/// provides access to all graphics tablets in this seat.
		pub fn get_tablet_seat(
			&self,
			seat            : &WlSeat
		) -> Result<Box<ZwpTabletSeatV2, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &ZWP_TABLET_SEAT_V2_INTERFACE, std::ptr::null::<u8>(), seat) as *mut ZwpTabletSeatV2 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # release the memory for the tablet manager object
		///
		/// Destroy the wp_tablet_manager object. Objects created from this
		/// object are unaffected and should be destroyed separately.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub static ZWP_TABLET_SEAT_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_seat_v2\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  3,
		events:       [
			WlMessage {
				name:      "tablet_added\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_V2_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "tool_added\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_TOOL_V2_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "pad_added\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_PAD_V2_INTERFACE as _].as_ptr()
			},
		].as_ptr()
	};
	
	/// # controller object for graphic tablet devices of a seat
	///
	/// An object that provides access to the graphics tablets available on this
	/// seat. After binding to this interface, the compositor sends a set of
	/// wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events.
	pub struct ZwpTabletSeatV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletSeatV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletSeatV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletSeatV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletSeatV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletSeatV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletSeatV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletSeatV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.tablet_added((proxy as *mut ZwpTabletSeatV2).as_mut().unwrap(), (*args.add(0)).n, ),
						1 => listener.tool_added((proxy as *mut ZwpTabletSeatV2).as_mut().unwrap(), (*args.add(0)).n, ),
						2 => listener.pad_added((proxy as *mut ZwpTabletSeatV2).as_mut().unwrap(), (*args.add(0)).n, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `tablet_added` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `tool_added` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `pad_added` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletSeatV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # release the memory for the tablet seat object
		///
		/// Destroy the wp_tablet_seat object. Objects created from this
		/// object are unaffected and should be destroyed separately.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletSeatV2Listener: std::any::Any {
		
		/// # new device notification
		///
		/// This event is sent whenever a new tablet becomes available on this
		/// seat. This event only provides the object id of the tablet, any
		/// static information about the tablet (device name, vid/pid, etc.) is
		/// sent through the wp_tablet interface.
		fn tablet_added(
			&self,
			proxy: &mut ZwpTabletSeatV2,
			id              : u32,
		);
		
		/// # a new tool has been used with a tablet
		///
		/// This event is sent whenever a tool that has not previously been used
		/// with a tablet comes into use. This event only provides the object id
		/// of the tool; any static information about the tool (capabilities,
		/// type, etc.) is sent through the wp_tablet_tool interface.
		fn tool_added(
			&self,
			proxy: &mut ZwpTabletSeatV2,
			id              : u32,
		);
		
		/// # new pad notification
		///
		/// This event is sent whenever a new pad is known to the system. Typically,
		/// pads are physically attached to tablets and a pad_added event is
		/// sent immediately after the wp_tablet_seat.tablet_added.
		/// However, some standalone pad devices logically attach to tablets at
		/// runtime, and the client must wait for wp_tablet_pad.enter to know
		/// the tablet a pad is attached to.
		///
		/// This event only provides the object id of the pad. All further
		/// features (buttons, strips, rings) are sent through the wp_tablet_pad
		/// interface.
		fn pad_added(
			&self,
			proxy: &mut ZwpTabletSeatV2,
			id              : u32,
		);
	}
	
	pub static ZWP_TABLET_TOOL_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_tool_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_cursor\0".as_ptr(),
				signature: "u?oii\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  19,
		events:       [
			WlMessage {
				name:      "type\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "hardware_serial\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "hardware_id_wacom\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "capability\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "removed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "proximity_in\0".as_ptr(),
				signature: "uoo\0".as_ptr(),
				types:     [&ZWP_TABLET_V2_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "proximity_out\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "down\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "up\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "motion\0".as_ptr(),
				signature: "ff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "pressure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "distance\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "tilt\0".as_ptr(),
				signature: "ff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "rotation\0".as_ptr(),
				signature: "f\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "slider\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "wheel\0".as_ptr(),
				signature: "fi\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "button\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "frame\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # a physical tablet tool
	///
	/// An object that represents a physical tool that has been, or is
	/// currently in use with a tablet in this seat. Each wp_tablet_tool
	/// object stays valid until the client destroys it; the compositor
	/// reuses the wp_tablet_tool object to indicate that the object's
	/// respective physical tool has come into proximity of a tablet again.
	///
	/// A wp_tablet_tool object's relation to a physical tool depends on the
	/// tablet's ability to report serial numbers. If the tablet supports
	/// this capability, then the object represents a specific physical tool
	/// and can be identified even when used on multiple tablets.
	///
	/// A tablet tool has a number of static characteristics, e.g. tool type,
	/// hardware_serial and capabilities. These capabilities are sent in an
	/// event sequence after the wp_tablet_seat.tool_added event before any
	/// actual events from this tool. This initial event sequence is
	/// terminated by a wp_tablet_tool.done event.
	///
	/// Tablet tool events are grouped by wp_tablet_tool.frame events.
	/// Any events received before a wp_tablet_tool.frame event should be
	/// considered part of the same hardware state change.
	pub struct ZwpTabletToolV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletToolV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletToolV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletToolV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletToolV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletToolV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletToolV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletToolV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.r#type((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, ),
						1 => listener.hardware_serial((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						2 => listener.hardware_id_wacom((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						3 => listener.capability((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, ),
						4 => listener.done((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), ),
						5 => listener.removed((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), ),
						6 => listener.proximity_in((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut ZwpTabletV2).as_mut(), ((*args.add(2)).o as *mut WlSurface).as_mut(), ),
						7 => listener.proximity_out((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), ),
						8 => listener.down((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, ),
						9 => listener.up((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), ),
						10 => listener.motion((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).f, (*args.add(1)).f, ),
						11 => listener.pressure((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, ),
						12 => listener.distance((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, ),
						13 => listener.tilt((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).f, (*args.add(1)).f, ),
						14 => listener.rotation((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).f, ),
						15 => listener.slider((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).i, ),
						16 => listener.wheel((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).f, (*args.add(1)).i, ),
						17 => listener.button((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, ),
						18 => listener.frame((proxy as *mut ZwpTabletToolV2).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `type` ARGS: tool_type: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `hardware_serial` ARGS: hardware_serial_hi: {:?}, hardware_serial_lo: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `hardware_id_wacom` ARGS: hardware_id_hi: {:?}, hardware_id_lo: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `capability` ARGS: capability: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `removed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						6 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `proximity_in` ARGS: serial: {:?}, tablet: {:?}, surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut ZwpTabletV2).as_mut(), ((*args.add(2)).o as *mut WlSurface).as_mut()),
						7 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `proximity_out` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						8 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `down` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						9 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `up` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						10 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `motion` ARGS: x: {:?}, y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).f, (*args.add(1)).f),
						11 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `pressure` ARGS: pressure: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						12 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `distance` ARGS: distance: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						13 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `tilt` ARGS: tilt_x: {:?}, tilt_y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).f, (*args.add(1)).f),
						14 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `rotation` ARGS: degrees: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).f),
						15 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `slider` ARGS: position: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i),
						16 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `wheel` ARGS: degrees: {:?}, clicks: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).f, (*args.add(1)).i),
						17 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `button` ARGS: serial: {:?}, button: {:?}, state: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u),
						18 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `frame` ARGS: time: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletToolV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # set the tablet tool's surface
		///
		/// Sets the surface of the cursor used for this tool on the given
		/// tablet. This request only takes effect if the tool is in proximity
		/// of one of the requesting client's surfaces or the surface parameter
		/// is the current pointer surface. If there was a previous surface set
		/// with this request it is replaced. If surface is NULL, the cursor
		/// image is hidden.
		///
		/// The parameters hotspot_x and hotspot_y define the position of the
		/// pointer surface relative to the pointer location. Its top-left corner
		/// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
		/// coordinates of the pointer location, in surface-local coordinates.
		///
		/// On surface.attach requests to the pointer surface, hotspot_x and
		/// hotspot_y are decremented by the x and y parameters passed to the
		/// request. Attach must be confirmed by wl_surface.commit as usual.
		///
		/// The hotspot can also be updated by passing the currently set pointer
		/// surface to this request with new values for hotspot_x and hotspot_y.
		///
		/// The current and pending input regions of the wl_surface are cleared,
		/// and wl_surface.set_input_region is ignored until the wl_surface is no
		/// longer used as the cursor. When the use as a cursor ends, the current
		/// and pending input regions become undefined, and the wl_surface is
		/// unmapped.
		///
		/// This request gives the surface the role of a wp_tablet_tool cursor. A
		/// surface may only ever be used as the cursor surface for one
		/// wp_tablet_tool. If the surface already has another role or has
		/// previously been used as cursor surface for a different tool, a
		/// protocol error is raised.
		pub fn set_cursor(
			&self,
			serial          : u32,
			surface         : Option<&WlSurface>,
			hotspot_x       : i32,
			hotspot_y       : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, serial, surface.map_or(std::ptr::null_mut(), |r| r as *const WlSurface as *mut WlSurface), hotspot_x, hotspot_y); }
		}
		
		/// # destroy the tool object
		///
		/// This destroys the client's resource for this tool object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletToolV2Listener: std::any::Any {
		
		/// # tool type
		///
		/// The tool type is the high-level type of the tool and usually decides
		/// the interaction expected from this tool.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn r#type(
			&self,
			proxy: &mut ZwpTabletToolV2,
			tool_type       : u32,
		);
		
		/// # unique hardware serial number of the tool
		///
		/// If the physical tool can be identified by a unique 64-bit serial
		/// number, this event notifies the client of this serial number.
		///
		/// If multiple tablets are available in the same seat and the tool is
		/// uniquely identifiable by the serial number, that tool may move
		/// between tablets.
		///
		/// Otherwise, if the tool has no serial number and this event is
		/// missing, the tool is tied to the tablet it first comes into
		/// proximity with. Even if the physical tool is used on multiple
		/// tablets, separate wp_tablet_tool objects will be created, one per
		/// tablet.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn hardware_serial(
			&self,
			proxy: &mut ZwpTabletToolV2,
			hardware_serial_hi: u32,
			hardware_serial_lo: u32,
		);
		
		/// # hardware id notification in Wacom's format
		///
		/// This event notifies the client of a hardware id available on this tool.
		///
		/// The hardware id is a device-specific 64-bit id that provides extra
		/// information about the tool in use, beyond the wl_tool.type
		/// enumeration. The format of the id is specific to tablets made by
		/// Wacom Inc. For example, the hardware id of a Wacom Grip
		/// Pen (a stylus) is 0x802.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn hardware_id_wacom(
			&self,
			proxy: &mut ZwpTabletToolV2,
			hardware_id_hi  : u32,
			hardware_id_lo  : u32,
		);
		
		/// # tool capability notification
		///
		/// This event notifies the client of any capabilities of this tool,
		/// beyond the main set of x/y axes and tip up/down detection.
		///
		/// One event is sent for each extra capability available on this tool.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn capability(
			&self,
			proxy: &mut ZwpTabletToolV2,
			capability      : u32,
		);
		
		/// # tool description events sequence complete
		///
		/// This event signals the end of the initial burst of descriptive
		/// events. A client may consider the static description of the tool to
		/// be complete and finalize initialization of the tool.
		fn done(
			&self,
			proxy: &mut ZwpTabletToolV2,
		);
		
		/// # tool removed
		///
		/// This event is sent when the tool is removed from the system and will
		/// send no further events. Should the physical tool come back into
		/// proximity later, a new wp_tablet_tool object will be created.
		///
		/// It is compositor-dependent when a tool is removed. A compositor may
		/// remove a tool on proximity out, tablet removal or any other reason.
		/// A compositor may also keep a tool alive until shutdown.
		///
		/// If the tool is currently in proximity, a proximity_out event will be
		/// sent before the removed event. See wp_tablet_tool.proximity_out for
		/// the handling of any buttons logically down.
		///
		/// When this event is received, the client must wp_tablet_tool.destroy
		/// the object.
		fn removed(
			&self,
			proxy: &mut ZwpTabletToolV2,
		);
		
		/// # proximity in event
		///
		/// Notification that this tool is focused on a certain surface.
		///
		/// This event can be received when the tool has moved from one surface to
		/// another, or when the tool has come back into proximity above the
		/// surface.
		///
		/// If any button is logically down when the tool comes into proximity,
		/// the respective button event is sent after the proximity_in event but
		/// within the same frame as the proximity_in event.
		fn proximity_in(
			&self,
			proxy: &mut ZwpTabletToolV2,
			serial          : u32,
			tablet          : Option<&mut ZwpTabletV2>,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # proximity out event
		///
		/// Notification that this tool has either left proximity, or is no
		/// longer focused on a certain surface.
		///
		/// When the tablet tool leaves proximity of the tablet, button release
		/// events are sent for each button that was held down at the time of
		/// leaving proximity. These events are sent before the proximity_out
		/// event but within the same wp_tablet.frame.
		///
		/// If the tool stays within proximity of the tablet, but the focus
		/// changes from one surface to another, a button release event may not
		/// be sent until the button is actually released or the tool leaves the
		/// proximity of the tablet.
		fn proximity_out(
			&self,
			proxy: &mut ZwpTabletToolV2,
		);
		
		/// # tablet tool is making contact
		///
		/// Sent whenever the tablet tool comes in contact with the surface of the
		/// tablet.
		///
		/// If the tool is already in contact with the tablet when entering the
		/// input region, the client owning said region will receive a
		/// wp_tablet.proximity_in event, followed by a wp_tablet.down
		/// event and a wp_tablet.frame event.
		///
		/// Note that this event describes logical contact, not physical
		/// contact. On some devices, a compositor may not consider a tool in
		/// logical contact until a minimum physical pressure threshold is
		/// exceeded.
		fn down(
			&self,
			proxy: &mut ZwpTabletToolV2,
			serial          : u32,
		);
		
		/// # tablet tool is no longer making contact
		///
		/// Sent whenever the tablet tool stops making contact with the surface of
		/// the tablet, or when the tablet tool moves out of the input region
		/// and the compositor grab (if any) is dismissed.
		///
		/// If the tablet tool moves out of the input region while in contact
		/// with the surface of the tablet and the compositor does not have an
		/// ongoing grab on the surface, the client owning said region will
		/// receive a wp_tablet.up event, followed by a wp_tablet.proximity_out
		/// event and a wp_tablet.frame event. If the compositor has an ongoing
		/// grab on this device, this event sequence is sent whenever the grab
		/// is dismissed in the future.
		///
		/// Note that this event describes logical contact, not physical
		/// contact. On some devices, a compositor may not consider a tool out
		/// of logical contact until physical pressure falls below a specific
		/// threshold.
		fn up(
			&self,
			proxy: &mut ZwpTabletToolV2,
		);
		
		/// # motion event
		///
		/// Sent whenever a tablet tool moves.
		fn motion(
			&self,
			proxy: &mut ZwpTabletToolV2,
			x               : WlFixed,
			y               : WlFixed,
		);
		
		/// # pressure change event
		///
		/// Sent whenever the pressure axis on a tool changes. The value of this
		/// event is normalized to a value between 0 and 65535.
		///
		/// Note that pressure may be nonzero even when a tool is not in logical
		/// contact. See the down and up events for more details.
		fn pressure(
			&self,
			proxy: &mut ZwpTabletToolV2,
			pressure        : u32,
		);
		
		/// # distance change event
		///
		/// Sent whenever the distance axis on a tool changes. The value of this
		/// event is normalized to a value between 0 and 65535.
		///
		/// Note that distance may be nonzero even when a tool is not in logical
		/// contact. See the down and up events for more details.
		fn distance(
			&self,
			proxy: &mut ZwpTabletToolV2,
			distance        : u32,
		);
		
		/// # tilt change event
		///
		/// Sent whenever one or both of the tilt axes on a tool change. Each tilt
		/// value is in degrees, relative to the z-axis of the tablet.
		/// The angle is positive when the top of a tool tilts along the
		/// positive x or y axis.
		fn tilt(
			&self,
			proxy: &mut ZwpTabletToolV2,
			tilt_x          : WlFixed,
			tilt_y          : WlFixed,
		);
		
		/// # z-rotation change event
		///
		/// Sent whenever the z-rotation axis on the tool changes. The
		/// rotation value is in degrees clockwise from the tool's
		/// logical neutral position.
		fn rotation(
			&self,
			proxy: &mut ZwpTabletToolV2,
			degrees         : WlFixed,
		);
		
		/// # Slider position change event
		///
		/// Sent whenever the slider position on the tool changes. The
		/// value is normalized between -65535 and 65535, with 0 as the logical
		/// neutral position of the slider.
		///
		/// The slider is available on e.g. the Wacom Airbrush tool.
		fn slider(
			&self,
			proxy: &mut ZwpTabletToolV2,
			position        : i32,
		);
		
		/// # Wheel delta event
		///
		/// Sent whenever the wheel on the tool emits an event. This event
		/// contains two values for the same axis change. The degrees value is
		/// in the same orientation as the wl_pointer.vertical_scroll axis. The
		/// clicks value is in discrete logical clicks of the mouse wheel. This
		/// value may be zero if the movement of the wheel was less
		/// than one logical click.
		///
		/// Clients should choose either value and avoid mixing degrees and
		/// clicks. The compositor may accumulate values smaller than a logical
		/// click and emulate click events when a certain threshold is met.
		/// Thus, wl_tablet_tool.wheel events with non-zero clicks values may
		/// have different degrees values.
		fn wheel(
			&self,
			proxy: &mut ZwpTabletToolV2,
			degrees         : WlFixed,
			clicks          : i32,
		);
		
		/// # button event
		///
		/// Sent whenever a button on the tool is pressed or released.
		///
		/// If a button is held down when the tool moves in or out of proximity,
		/// button events are generated by the compositor. See
		/// wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
		/// details.
		fn button(
			&self,
			proxy: &mut ZwpTabletToolV2,
			serial          : u32,
			button          : u32,
			state           : u32,
		);
		
		/// # frame event
		///
		/// Marks the end of a series of axis and/or button updates from the
		/// tablet. The Wayland protocol requires axis updates to be sent
		/// sequentially, however all events within a frame should be considered
		/// one hardware event.
		fn frame(
			&self,
			proxy: &mut ZwpTabletToolV2,
			time            : u32,
		);
	}
	
	/// # a physical tool type
	///
	/// Describes the physical type of a tool. The physical type of a tool
	/// generally defines its base usage.
	///
	/// The mouse tool represents a mouse-shaped tool that is not a relative
	/// device but bound to the tablet's surface, providing absolute
	/// coordinates.
	///
	/// The lens tool is a mouse-shaped tool with an attached lens to
	/// provide precision focus.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV2Type {
		/// Pen
		Pen = 0x140,
		/// Eraser
		Eraser = 0x141,
		/// Brush
		Brush = 0x142,
		/// Pencil
		Pencil = 0x143,
		/// Airbrush
		Airbrush = 0x144,
		/// Finger
		Finger = 0x145,
		/// Mouse
		Mouse = 0x146,
		/// Lens
		Lens = 0x147,
	}
	
	/// # capability flags for a tool
	///
	/// Describes extra capabilities on a tablet.
	///
	/// Any tool must provide x and y values, extra axes are
	/// device-specific.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV2Capability {
		/// Tilt axes
		Tilt = 1,
		/// Pressure axis
		Pressure = 2,
		/// Distance axis
		Distance = 3,
		/// Z-rotation axis
		Rotation = 4,
		/// Slider axis
		Slider = 5,
		/// Wheel axis
		Wheel = 6,
	}
	
	/// # physical button state
	///
	/// Describes the physical state of a button that produced the button event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV2ButtonState {
		/// button is not pressed
		Released = 0,
		/// button is pressed
		Pressed = 1,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV2Error {
		/// given wl_surface has another role
		Role = 0,
	}
	
	pub static ZWP_TABLET_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_v2\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  5,
		events:       [
			WlMessage {
				name:      "name\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "id\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "path\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "removed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # graphics tablet device
	///
	/// The wp_tablet interface represents one graphics tablet device. The
	/// tablet interface itself does not generate events; all events are
	/// generated by wp_tablet_tool objects when in proximity above a tablet.
	///
	/// A tablet has a number of static characteristics, e.g. device name and
	/// pid/vid. These capabilities are sent in an event sequence after the
	/// wp_tablet_seat.tablet_added event. This initial event sequence is
	/// terminated by a wp_tablet.done event.
	pub struct ZwpTabletV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.name((proxy as *mut ZwpTabletV2).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						1 => listener.id((proxy as *mut ZwpTabletV2).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						2 => listener.path((proxy as *mut ZwpTabletV2).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						3 => listener.done((proxy as *mut ZwpTabletV2).as_mut().unwrap(), ),
						4 => listener.removed((proxy as *mut ZwpTabletV2).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `name` ARGS: name: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `id` ARGS: vid: {:?}, pid: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `path` ARGS: path: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `removed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the tablet object
		///
		/// This destroys the client's resource for this tablet object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletV2Listener: std::any::Any {
		
		/// # tablet device name
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet.done event.
		fn name(
			&self,
			proxy: &mut ZwpTabletV2,
			name            : &str,
		);
		
		/// # tablet device USB vendor/product id
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet.done event.
		fn id(
			&self,
			proxy: &mut ZwpTabletV2,
			vid             : u32,
			pid             : u32,
		);
		
		/// # path to the device
		///
		/// A system-specific device path that indicates which device is behind
		/// this wp_tablet. This information may be used to gather additional
		/// information about the device, e.g. through libwacom.
		///
		/// A device may have more than one device path. If so, multiple
		/// wp_tablet.path events are sent. A device may be emulated and not
		/// have a device path, and in that case this event will not be sent.
		///
		/// The format of the path is unspecified, it may be a device node, a
		/// sysfs path, or some other identifier. It is up to the client to
		/// identify the string provided.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet.done event.
		fn path(
			&self,
			proxy: &mut ZwpTabletV2,
			path            : &str,
		);
		
		/// # tablet description events sequence complete
		///
		/// This event is sent immediately to signal the end of the initial
		/// burst of descriptive events. A client may consider the static
		/// description of the tablet to be complete and finalize initialization
		/// of the tablet.
		fn done(
			&self,
			proxy: &mut ZwpTabletV2,
		);
		
		/// # tablet removed event
		///
		/// Sent when the tablet has been removed from the system. When a tablet
		/// is removed, some tools may be removed.
		///
		/// When this event is received, the client must wp_tablet.destroy
		/// the object.
		fn removed(
			&self,
			proxy: &mut ZwpTabletV2,
		);
	}
	
	pub static ZWP_TABLET_PAD_RING_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_pad_ring_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_feedback\0".as_ptr(),
				signature: "su\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  4,
		events:       [
			WlMessage {
				name:      "source\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "angle\0".as_ptr(),
				signature: "f\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "stop\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "frame\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # pad ring
	///
	/// A circular interaction area, such as the touch ring on the Wacom Intuos
	/// Pro series tablets.
	///
	/// Events on a ring are logically grouped by the wl_tablet_pad_ring.frame
	/// event.
	pub struct ZwpTabletPadRingV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletPadRingV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletPadRingV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletPadRingV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletPadRingV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletPadRingV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletPadRingV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletPadRingV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.source((proxy as *mut ZwpTabletPadRingV2).as_mut().unwrap(), (*args.add(0)).u, ),
						1 => listener.angle((proxy as *mut ZwpTabletPadRingV2).as_mut().unwrap(), (*args.add(0)).f, ),
						2 => listener.stop((proxy as *mut ZwpTabletPadRingV2).as_mut().unwrap(), ),
						3 => listener.frame((proxy as *mut ZwpTabletPadRingV2).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `source` ARGS: source: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `angle` ARGS: degrees: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).f),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `stop` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `frame` ARGS: time: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletPadRingV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # set compositor feedback
		///
		/// Request that the compositor use the provided feedback string
		/// associated with this ring. This request should be issued immediately
		/// after a wp_tablet_pad_group.mode_switch event from the corresponding
		/// group is received, or whenever the ring is mapped to a different
		/// action. See wp_tablet_pad_group.mode_switch for more details.
		///
		/// Clients are encouraged to provide context-aware descriptions for
		/// the actions associated with the ring; compositors may use this
		/// information to offer visual feedback about the button layout
		/// (eg. on-screen displays).
		///
		/// The provided string 'description' is a UTF-8 encoded string to be
		/// associated with this ring, and is considered user-visible; general
		/// internationalization rules apply.
		///
		/// The serial argument will be that of the last
		/// wp_tablet_pad_group.mode_switch event received for the group of this
		/// ring. Requests providing other serials than the most recent one will be
		/// ignored.
		pub fn set_feedback(
			&self,
			description     : &str,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, description.as_ptr(), serial); }
		}
		
		/// # destroy the ring object
		///
		/// This destroys the client's resource for this ring object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletPadRingV2Listener: std::any::Any {
		
		/// # ring event source
		///
		/// Source information for ring events.
		///
		/// This event does not occur on its own. It is sent before a
		/// wp_tablet_pad_ring.frame event and carries the source information
		/// for all events within that frame.
		///
		/// The source specifies how this event was generated. If the source is
		/// wp_tablet_pad_ring.source.finger, a wp_tablet_pad_ring.stop event
		/// will be sent when the user lifts the finger off the device.
		///
		/// This event is optional. If the source is unknown for an interaction,
		/// no event is sent.
		fn source(
			&self,
			proxy: &mut ZwpTabletPadRingV2,
			source          : u32,
		);
		
		/// # angle changed
		///
		/// Sent whenever the angle on a ring changes.
		///
		/// The angle is provided in degrees clockwise from the logical
		/// north of the ring in the pad's current rotation.
		fn angle(
			&self,
			proxy: &mut ZwpTabletPadRingV2,
			degrees         : WlFixed,
		);
		
		/// # interaction stopped
		///
		/// Stop notification for ring events.
		///
		/// For some wp_tablet_pad_ring.source types, a wp_tablet_pad_ring.stop
		/// event is sent to notify a client that the interaction with the ring
		/// has terminated. This enables the client to implement kinetic scrolling.
		/// See the wp_tablet_pad_ring.source documentation for information on
		/// when this event may be generated.
		///
		/// Any wp_tablet_pad_ring.angle events with the same source after this
		/// event should be considered as the start of a new interaction.
		fn stop(
			&self,
			proxy: &mut ZwpTabletPadRingV2,
		);
		
		/// # end of a ring event sequence
		///
		/// Indicates the end of a set of ring events that logically belong
		/// together. A client is expected to accumulate the data in all events
		/// within the frame before proceeding.
		///
		/// All wp_tablet_pad_ring events before a wp_tablet_pad_ring.frame event belong
		/// logically together. For example, on termination of a finger interaction
		/// on a ring the compositor will send a wp_tablet_pad_ring.source event,
		/// a wp_tablet_pad_ring.stop event and a wp_tablet_pad_ring.frame event.
		///
		/// A wp_tablet_pad_ring.frame event is sent for every logical event
		/// group, even if the group only contains a single wp_tablet_pad_ring
		/// event. Specifically, a client may get a sequence: angle, frame,
		/// angle, frame, etc.
		fn frame(
			&self,
			proxy: &mut ZwpTabletPadRingV2,
			time            : u32,
		);
	}
	
	/// # ring axis source
	///
	/// Describes the source types for ring events. This indicates to the
	/// client how a ring event was physically generated; a client may
	/// adjust the user interface accordingly. For example, events
	/// from a "finger" source may trigger kinetic scrolling.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletPadRingV2Source {
		/// finger
		Finger = 1,
	}
	
	pub static ZWP_TABLET_PAD_STRIP_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_pad_strip_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_feedback\0".as_ptr(),
				signature: "su\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  4,
		events:       [
			WlMessage {
				name:      "source\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "position\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "stop\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "frame\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # pad strip
	///
	/// A linear interaction area, such as the strips found in Wacom Cintiq
	/// models.
	///
	/// Events on a strip are logically grouped by the wl_tablet_pad_strip.frame
	/// event.
	pub struct ZwpTabletPadStripV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletPadStripV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletPadStripV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletPadStripV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletPadStripV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletPadStripV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletPadStripV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletPadStripV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.source((proxy as *mut ZwpTabletPadStripV2).as_mut().unwrap(), (*args.add(0)).u, ),
						1 => listener.position((proxy as *mut ZwpTabletPadStripV2).as_mut().unwrap(), (*args.add(0)).u, ),
						2 => listener.stop((proxy as *mut ZwpTabletPadStripV2).as_mut().unwrap(), ),
						3 => listener.frame((proxy as *mut ZwpTabletPadStripV2).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `source` ARGS: source: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `position` ARGS: position: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `stop` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `frame` ARGS: time: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletPadStripV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # set compositor feedback
		///
		/// Requests the compositor to use the provided feedback string
		/// associated with this strip. This request should be issued immediately
		/// after a wp_tablet_pad_group.mode_switch event from the corresponding
		/// group is received, or whenever the strip is mapped to a different
		/// action. See wp_tablet_pad_group.mode_switch for more details.
		///
		/// Clients are encouraged to provide context-aware descriptions for
		/// the actions associated with the strip, and compositors may use this
		/// information to offer visual feedback about the button layout
		/// (eg. on-screen displays).
		///
		/// The provided string 'description' is a UTF-8 encoded string to be
		/// associated with this ring, and is considered user-visible; general
		/// internationalization rules apply.
		///
		/// The serial argument will be that of the last
		/// wp_tablet_pad_group.mode_switch event received for the group of this
		/// strip. Requests providing other serials than the most recent one will be
		/// ignored.
		pub fn set_feedback(
			&self,
			description     : &str,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, description.as_ptr(), serial); }
		}
		
		/// # destroy the strip object
		///
		/// This destroys the client's resource for this strip object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletPadStripV2Listener: std::any::Any {
		
		/// # strip event source
		///
		/// Source information for strip events.
		///
		/// This event does not occur on its own. It is sent before a
		/// wp_tablet_pad_strip.frame event and carries the source information
		/// for all events within that frame.
		///
		/// The source specifies how this event was generated. If the source is
		/// wp_tablet_pad_strip.source.finger, a wp_tablet_pad_strip.stop event
		/// will be sent when the user lifts their finger off the device.
		///
		/// This event is optional. If the source is unknown for an interaction,
		/// no event is sent.
		fn source(
			&self,
			proxy: &mut ZwpTabletPadStripV2,
			source          : u32,
		);
		
		/// # position changed
		///
		/// Sent whenever the position on a strip changes.
		///
		/// The position is normalized to a range of [0, 65535], the 0-value
		/// represents the top-most and/or left-most position of the strip in
		/// the pad's current rotation.
		fn position(
			&self,
			proxy: &mut ZwpTabletPadStripV2,
			position        : u32,
		);
		
		/// # interaction stopped
		///
		/// Stop notification for strip events.
		///
		/// For some wp_tablet_pad_strip.source types, a wp_tablet_pad_strip.stop
		/// event is sent to notify a client that the interaction with the strip
		/// has terminated. This enables the client to implement kinetic
		/// scrolling. See the wp_tablet_pad_strip.source documentation for
		/// information on when this event may be generated.
		///
		/// Any wp_tablet_pad_strip.position events with the same source after this
		/// event should be considered as the start of a new interaction.
		fn stop(
			&self,
			proxy: &mut ZwpTabletPadStripV2,
		);
		
		/// # end of a strip event sequence
		///
		/// Indicates the end of a set of events that represent one logical
		/// hardware strip event. A client is expected to accumulate the data
		/// in all events within the frame before proceeding.
		///
		/// All wp_tablet_pad_strip events before a wp_tablet_pad_strip.frame event belong
		/// logically together. For example, on termination of a finger interaction
		/// on a strip the compositor will send a wp_tablet_pad_strip.source event,
		/// a wp_tablet_pad_strip.stop event and a wp_tablet_pad_strip.frame
		/// event.
		///
		/// A wp_tablet_pad_strip.frame event is sent for every logical event
		/// group, even if the group only contains a single wp_tablet_pad_strip
		/// event. Specifically, a client may get a sequence: position, frame,
		/// position, frame, etc.
		fn frame(
			&self,
			proxy: &mut ZwpTabletPadStripV2,
			time            : u32,
		);
	}
	
	/// # strip axis source
	///
	/// Describes the source types for strip events. This indicates to the
	/// client how a strip event was physically generated; a client may
	/// adjust the user interface accordingly. For example, events
	/// from a "finger" source may trigger kinetic scrolling.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletPadStripV2Source {
		/// finger
		Finger = 1,
	}
	
	pub static ZWP_TABLET_PAD_GROUP_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_pad_group_v2\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  6,
		events:       [
			WlMessage {
				name:      "buttons\0".as_ptr(),
				signature: "a\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "ring\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_PAD_RING_V2_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "strip\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_PAD_STRIP_V2_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "modes\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "mode_switch\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # a set of buttons, rings and strips
	///
	/// A pad group describes a distinct (sub)set of buttons, rings and strips
	/// present in the tablet. The criteria of this grouping is usually positional,
	/// eg. if a tablet has buttons on the left and right side, 2 groups will be
	/// presented. The physical arrangement of groups is undisclosed and may
	/// change on the fly.
	///
	/// Pad groups will announce their features during pad initialization. Between
	/// the corresponding wp_tablet_pad.group event and wp_tablet_pad_group.done, the
	/// pad group will announce the buttons, rings and strips contained in it,
	/// plus the number of supported modes.
	///
	/// Modes are a mechanism to allow multiple groups of actions for every element
	/// in the pad group. The number of groups and available modes in each is
	/// persistent across device plugs. The current mode is user-switchable, it
	/// will be announced through the wp_tablet_pad_group.mode_switch event both
	/// whenever it is switched, and after wp_tablet_pad.enter.
	///
	/// The current mode logically applies to all elements in the pad group,
	/// although it is at clients' discretion whether to actually perform different
	/// actions, and/or issue the respective .set_feedback requests to notify the
	/// compositor. See the wp_tablet_pad_group.mode_switch event for more details.
	pub struct ZwpTabletPadGroupV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletPadGroupV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletPadGroupV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletPadGroupV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletPadGroupV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletPadGroupV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletPadGroupV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletPadGroupV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.buttons((proxy as *mut ZwpTabletPadGroupV2).as_mut().unwrap(), (*args.add(0)).a.as_ref().unwrap(), ),
						1 => listener.ring((proxy as *mut ZwpTabletPadGroupV2).as_mut().unwrap(), (*args.add(0)).n, ),
						2 => listener.strip((proxy as *mut ZwpTabletPadGroupV2).as_mut().unwrap(), (*args.add(0)).n, ),
						3 => listener.modes((proxy as *mut ZwpTabletPadGroupV2).as_mut().unwrap(), (*args.add(0)).u, ),
						4 => listener.done((proxy as *mut ZwpTabletPadGroupV2).as_mut().unwrap(), ),
						5 => listener.mode_switch((proxy as *mut ZwpTabletPadGroupV2).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `buttons` ARGS: buttons: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).a.as_ref().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `ring` ARGS: ring: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `strip` ARGS: strip: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `modes` ARGS: modes: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `mode_switch` ARGS: time: {:?}, serial: {:?}, mode: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletPadGroupV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the pad object
		///
		/// Destroy the wp_tablet_pad_group object. Objects created from this object
		/// are unaffected and should be destroyed separately.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletPadGroupV2Listener: std::any::Any {
		
		/// # buttons announced
		///
		/// Sent on wp_tablet_pad_group initialization to announce the available
		/// buttons in the group. Button indices start at 0, a button may only be
		/// in one group at a time.
		///
		/// This event is first sent in the initial burst of events before the
		/// wp_tablet_pad_group.done event.
		///
		/// Some buttons are reserved by the compositor. These buttons may not be
		/// assigned to any wp_tablet_pad_group. Compositors may broadcast this
		/// event in the case of changes to the mapping of these reserved buttons.
		/// If the compositor happens to reserve all buttons in a group, this event
		/// will be sent with an empty array.
		fn buttons(
			&self,
			proxy: &mut ZwpTabletPadGroupV2,
			buttons         : &WlArray,
		);
		
		/// # ring announced
		///
		/// Sent on wp_tablet_pad_group initialization to announce available rings.
		/// One event is sent for each ring available on this pad group.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_pad_group.done event.
		fn ring(
			&self,
			proxy: &mut ZwpTabletPadGroupV2,
			ring            : u32,
		);
		
		/// # strip announced
		///
		/// Sent on wp_tablet_pad initialization to announce available strips.
		/// One event is sent for each strip available on this pad group.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_pad_group.done event.
		fn strip(
			&self,
			proxy: &mut ZwpTabletPadGroupV2,
			strip           : u32,
		);
		
		/// # mode-switch ability announced
		///
		/// Sent on wp_tablet_pad_group initialization to announce that the pad
		/// group may switch between modes. A client may use a mode to store a
		/// specific configuration for buttons, rings and strips and use the
		/// wl_tablet_pad_group.mode_switch event to toggle between these
		/// configurations. Mode indices start at 0.
		///
		/// Switching modes is compositor-dependent. See the
		/// wp_tablet_pad_group.mode_switch event for more details.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_pad_group.done event. This event is only sent when more than
		/// more than one mode is available.
		fn modes(
			&self,
			proxy: &mut ZwpTabletPadGroupV2,
			modes           : u32,
		);
		
		/// # tablet group description events sequence complete
		///
		/// This event is sent immediately to signal the end of the initial
		/// burst of descriptive events. A client may consider the static
		/// description of the tablet to be complete and finalize initialization
		/// of the tablet group.
		fn done(
			&self,
			proxy: &mut ZwpTabletPadGroupV2,
		);
		
		/// # mode switch event
		///
		/// Notification that the mode was switched.
		///
		/// A mode applies to all buttons, rings and strips in a group
		/// simultaneously, but a client is not required to assign different actions
		/// for each mode. For example, a client may have mode-specific button
		/// mappings but map the ring to vertical scrolling in all modes. Mode
		/// indices start at 0.
		///
		/// Switching modes is compositor-dependent. The compositor may provide
		/// visual cues to the client about the mode, e.g. by toggling LEDs on
		/// the tablet device. Mode-switching may be software-controlled or
		/// controlled by one or more physical buttons. For example, on a Wacom
		/// Intuos Pro, the button inside the ring may be assigned to switch
		/// between modes.
		///
		/// The compositor will also send this event after wp_tablet_pad.enter on
		/// each group in order to notify of the current mode. Groups that only
		/// feature one mode will use mode=0 when emitting this event.
		///
		/// If a button action in the new mode differs from the action in the
		/// previous mode, the client should immediately issue a
		/// wp_tablet_pad.set_feedback request for each changed button.
		///
		/// If a ring or strip action in the new mode differs from the action
		/// in the previous mode, the client should immediately issue a
		/// wp_tablet_ring.set_feedback or wp_tablet_strip.set_feedback request
		/// for each changed ring or strip.
		fn mode_switch(
			&self,
			proxy: &mut ZwpTabletPadGroupV2,
			time            : u32,
			serial          : u32,
			mode            : u32,
		);
	}
	
	pub static ZWP_TABLET_PAD_V2_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_pad_v2\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_feedback\0".as_ptr(),
				signature: "usu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  8,
		events:       [
			WlMessage {
				name:      "group\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_PAD_GROUP_V2_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "path\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "buttons\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "button\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "enter\0".as_ptr(),
				signature: "uoo\0".as_ptr(),
				types:     [&ZWP_TABLET_V2_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "leave\0".as_ptr(),
				signature: "uo\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "removed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # a set of buttons, rings and strips
	///
	/// A pad device is a set of buttons, rings and strips
	/// usually physically present on the tablet device itself. Some
	/// exceptions exist where the pad device is physically detached, e.g. the
	/// Wacom ExpressKey Remote.
	///
	/// Pad devices have no axes that control the cursor and are generally
	/// auxiliary devices to the tool devices used on the tablet surface.
	///
	/// A pad device has a number of static characteristics, e.g. the number
	/// of rings. These capabilities are sent in an event sequence after the
	/// wp_tablet_seat.pad_added event before any actual events from this pad.
	/// This initial event sequence is terminated by a wp_tablet_pad.done
	/// event.
	///
	/// All pad features (buttons, rings and strips) are logically divided into
	/// groups and all pads have at least one group. The available groups are
	/// notified through the wp_tablet_pad.group event; the compositor will
	/// emit one event per group before emitting wp_tablet_pad.done.
	///
	/// Groups may have multiple modes. Modes allow clients to map multiple
	/// actions to a single pad feature. Only one mode can be active per group,
	/// although different groups may have different active modes.
	pub struct ZwpTabletPadV2(WlProxy);
	
	impl std::ops::Deref for ZwpTabletPadV2 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletPadV2 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletPadV2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletPadV2")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletPadV2 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletPadV2Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletPadV2Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.group((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), (*args.add(0)).n, ),
						1 => listener.path((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						2 => listener.buttons((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), (*args.add(0)).u, ),
						3 => listener.done((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), ),
						4 => listener.button((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, ),
						5 => listener.enter((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut ZwpTabletV2).as_mut(), ((*args.add(2)).o as *mut WlSurface).as_mut(), ),
						6 => listener.leave((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), ),
						7 => listener.removed((proxy as *mut ZwpTabletPadV2).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `group` ARGS: pad_group: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `path` ARGS: path: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `buttons` ARGS: buttons: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `button` ARGS: time: {:?}, button: {:?}, state: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `enter` ARGS: serial: {:?}, tablet: {:?}, surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut ZwpTabletV2).as_mut(), ((*args.add(2)).o as *mut WlSurface).as_mut()),
						6 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `leave` ARGS: serial: {:?}, surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut()),
						7 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `removed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletPadV2Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # set compositor feedback
		///
		/// Requests the compositor to use the provided feedback string
		/// associated with this button. This request should be issued immediately
		/// after a wp_tablet_pad_group.mode_switch event from the corresponding
		/// group is received, or whenever a button is mapped to a different
		/// action. See wp_tablet_pad_group.mode_switch for more details.
		///
		/// Clients are encouraged to provide context-aware descriptions for
		/// the actions associated with each button, and compositors may use
		/// this information to offer visual feedback on the button layout
		/// (e.g. on-screen displays).
		///
		/// Button indices start at 0. Setting the feedback string on a button
		/// that is reserved by the compositor (i.e. not belonging to any
		/// wp_tablet_pad_group) does not generate an error but the compositor
		/// is free to ignore the request.
		///
		/// The provided string 'description' is a UTF-8 encoded string to be
		/// associated with this ring, and is considered user-visible; general
		/// internationalization rules apply.
		///
		/// The serial argument will be that of the last
		/// wp_tablet_pad_group.mode_switch event received for the group of this
		/// button. Requests providing other serials than the most recent one will
		/// be ignored.
		pub fn set_feedback(
			&self,
			button          : u32,
			description     : &str,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, button, description.as_ptr(), serial); }
		}
		
		/// # destroy the pad object
		///
		/// Destroy the wp_tablet_pad object. Objects created from this object
		/// are unaffected and should be destroyed separately.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletPadV2Listener: std::any::Any {
		
		/// # group announced
		///
		/// Sent on wp_tablet_pad initialization to announce available groups.
		/// One event is sent for each pad group available.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_pad.done event. At least one group will be announced.
		fn group(
			&self,
			proxy: &mut ZwpTabletPadV2,
			pad_group       : u32,
		);
		
		/// # path to the device
		///
		/// A system-specific device path that indicates which device is behind
		/// this wp_tablet_pad. This information may be used to gather additional
		/// information about the device, e.g. through libwacom.
		///
		/// The format of the path is unspecified, it may be a device node, a
		/// sysfs path, or some other identifier. It is up to the client to
		/// identify the string provided.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_pad.done event.
		fn path(
			&self,
			proxy: &mut ZwpTabletPadV2,
			path            : &str,
		);
		
		/// # buttons announced
		///
		/// Sent on wp_tablet_pad initialization to announce the available
		/// buttons.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_pad.done event. This event is only sent when at least one
		/// button is available.
		fn buttons(
			&self,
			proxy: &mut ZwpTabletPadV2,
			buttons         : u32,
		);
		
		/// # pad description event sequence complete
		///
		/// This event signals the end of the initial burst of descriptive
		/// events. A client may consider the static description of the pad to
		/// be complete and finalize initialization of the pad.
		fn done(
			&self,
			proxy: &mut ZwpTabletPadV2,
		);
		
		/// # physical button state
		///
		/// Sent whenever the physical state of a button changes.
		fn button(
			&self,
			proxy: &mut ZwpTabletPadV2,
			time            : u32,
			button          : u32,
			state           : u32,
		);
		
		/// # enter event
		///
		/// Notification that this pad is focused on the specified surface.
		fn enter(
			&self,
			proxy: &mut ZwpTabletPadV2,
			serial          : u32,
			tablet          : Option<&mut ZwpTabletV2>,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # enter event
		///
		/// Notification that this pad is no longer focused on the specified
		/// surface.
		fn leave(
			&self,
			proxy: &mut ZwpTabletPadV2,
			serial          : u32,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # pad removed event
		///
		/// Sent when the pad has been removed from the system. When a tablet
		/// is removed its pad(s) will be removed too.
		///
		/// When this event is received, the client must destroy all rings, strips
		/// and groups that were offered by this pad, and issue wp_tablet_pad.destroy
		/// the pad itself.
		fn removed(
			&self,
			proxy: &mut ZwpTabletPadV2,
		);
	}
	
	/// # physical button state
	///
	/// Describes the physical state of a button that caused the button
	/// event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletPadV2ButtonState {
		/// the button is not pressed
		Released = 0,
		/// the button is pressed
		Pressed = 1,
	}
}
pub use wayland::*;
mod wayland {
	use crate::*;
	
	// Copyright © 2008-2011 Kristian Høgsberg
	// Copyright © 2010-2011 Intel Corporation
	// Copyright © 2012-2013 Collabora, Ltd.
	//
	// Permission is hereby granted, free of charge, to any person
	// obtaining a copy of this software and associated documentation files
	// (the "Software"), to deal in the Software without restriction,
	// including without limitation the rights to use, copy, modify, merge,
	// publish, distribute, sublicense, and/or sell copies of the Software,
	// and to permit persons to whom the Software is furnished to do so,
	// subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the
	// next paragraph) shall be included in all copies or substantial
	// portions of the Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
	// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
	// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
	// NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
	// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
	// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
	// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
	// SOFTWARE.
	
	pub static WL_DISPLAY_INTERFACE: WlInterface = WlInterface {
		name:         "wl_display\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "sync\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_CALLBACK_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_registry\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_REGISTRY_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "error\0".as_ptr(),
				signature: "ous\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "delete_id\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # core global object
	///
	/// The core global object.  This is a special singleton object.  It
	/// is used for internal Wayland protocol features.
	pub struct WlDisplay(WlProxy);
	
	impl std::ops::Deref for WlDisplay {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlDisplay {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlDisplay {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlDisplay")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlDisplay {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlDisplayListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlDisplayListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.error((proxy as *mut WlDisplay).as_mut().unwrap(), ((*args.add(0)).o as *mut WlProxy).as_mut(), (*args.add(1)).u, std::ffi::CStr::from_ptr((*args.add(2)).s as _).to_str().unwrap(), ),
						1 => listener.delete_id((proxy as *mut WlDisplay).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `error` ARGS: object_id: {:?}, code: {:?}, message: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlProxy).as_mut(), (*args.add(1)).u, std::ffi::CStr::from_ptr((*args.add(2)).s as _).to_str().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `delete_id` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlDisplayListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # asynchronous roundtrip
		///
		/// The sync request asks the server to emit the 'done' event
		/// on the returned wl_callback object.  Since requests are
		/// handled in-order and events are delivered in-order, this can
		/// be used as a barrier to ensure all previous requests and the
		/// resulting events have been handled.
		///
		/// The object returned by this request will be destroyed by the
		/// compositor after the callback is fired and as such the client must not
		/// attempt to use it after that point.
		///
		/// The callback_data passed in the callback is the event serial.
		pub fn sync(
			&self
		) -> Result<Box<WlCallback, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &WL_CALLBACK_INTERFACE, std::ptr::null::<u8>()) as *mut WlCallback };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # get global registry object
		///
		/// This request creates a registry object that allows the client
		/// to list and bind the global objects available from the
		/// compositor.
		///
		/// It should be noted that the server side resources consumed in
		/// response to a get_registry request can only be released when the
		/// client disconnects, not when the client side proxy is destroyed.
		/// Therefore, clients should invoke get_registry as infrequently as
		/// possible to avoid wasting memory.
		pub fn get_registry(
			&self
		) -> Result<Box<WlRegistry, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &WL_REGISTRY_INTERFACE, std::ptr::null::<u8>()) as *mut WlRegistry };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub trait WlDisplayListener: std::any::Any {
		
		/// # fatal error event
		///
		/// The error event is sent out when a fatal (non-recoverable)
		/// error has occurred.  The object_id argument is the object
		/// where the error occurred, most often in response to a request
		/// to that object.  The code identifies the error and is defined
		/// by the object interface.  As such, each interface defines its
		/// own set of error codes.  The message is a brief description
		/// of the error, for (debugging) convenience.
		fn error(
			&self,
			proxy: &mut WlDisplay,
			object_id       : Option<&mut WlProxy>,
			code            : u32,
			message         : &str,
		);
		
		/// # acknowledge object ID deletion
		///
		/// This event is used internally by the object ID management
		/// logic. When a client deletes an object that it had created,
		/// the server will send this event to acknowledge that it has
		/// seen the delete request. When the client receives this event,
		/// it will know that it can safely reuse the object ID.
		fn delete_id(
			&self,
			proxy: &mut WlDisplay,
			id              : u32,
		);
	}
	
	/// # global error values
	///
	/// These errors are global and can be emitted in response to any
	/// server request.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlDisplayError {
		/// server couldn't find object
		InvalidObject = 0,
		/// method doesn't exist on the specified interface or malformed request
		InvalidMethod = 1,
		/// server is out of memory
		NoMemory = 2,
		/// implementation error in compositor
		Implementation = 3,
	}
	
	pub static WL_REGISTRY_INTERFACE: WlInterface = WlInterface {
		name:         "wl_registry\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "bind\0".as_ptr(),
				signature: "usun\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "global\0".as_ptr(),
				signature: "usu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "global_remove\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # global registry object
	///
	/// The singleton global registry object.  The server has a number of
	/// global objects that are available to all clients.  These objects
	/// typically represent an actual object in the server (for example,
	/// an input device) or they are singleton objects that provide
	/// extension functionality.
	///
	/// When a client creates a registry object, the registry object
	/// will emit a global event for each global currently in the
	/// registry.  Globals come and go as a result of device or
	/// monitor hotplugs, reconfiguration or other events, and the
	/// registry will send out global and global_remove events to
	/// keep the client up to date with the changes.  To mark the end
	/// of the initial burst of events, the client can use the
	/// wl_display.sync request immediately after calling
	/// wl_display.get_registry.
	///
	/// A client can bind to a global object by using the bind
	/// request.  This creates a client-side handle that lets the object
	/// emit events to the client and lets the client invoke requests on
	/// the object.
	pub struct WlRegistry(WlProxy);
	
	impl std::ops::Deref for WlRegistry {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlRegistry {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlRegistry {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlRegistry")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlRegistry {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlRegistryListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlRegistryListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.global((proxy as *mut WlRegistry).as_mut().unwrap(), (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap(), (*args.add(2)).u, ),
						1 => listener.global_remove((proxy as *mut WlRegistry).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `global` ARGS: name: {:?}, interface: {:?}, version: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap(), (*args.add(2)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `global_remove` ARGS: name: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlRegistryListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # bind an object to the display
		///
		/// Binds a new, client-created object to the server using the
		/// specified name as the identifier.
		pub fn bind<T>(
			&self,
			name            : u32,
			interface       : *const WlInterface,
			version         : u32,
		) -> Result<Box<T, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor_versioned)(self as *const Self as _, 0, interface, version, name, (*interface).name, version, std::ptr::null::<u8>()) as *mut T };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub trait WlRegistryListener: std::any::Any {
		
		/// # announce global object
		///
		/// Notify the client of global objects.
		///
		/// The event notifies the client that a global object with
		/// the given name is now available, and it implements the
		/// given version of the given interface.
		fn global(
			&self,
			proxy: &mut WlRegistry,
			name            : u32,
			interface       : &str,
			version         : u32,
		);
		
		/// # announce removal of global object
		///
		/// Notify the client of removed global objects.
		///
		/// This event notifies the client that the global identified
		/// by name is no longer available.  If the client bound to
		/// the global using the bind request, the client should now
		/// destroy that object.
		///
		/// The object remains valid and requests to the object will be
		/// ignored until the client destroys it, to avoid races between
		/// the global going away and a client sending a request to it.
		fn global_remove(
			&self,
			proxy: &mut WlRegistry,
			name            : u32,
		);
	}
	
	pub static WL_CALLBACK_INTERFACE: WlInterface = WlInterface {
		name:         "wl_callback\0".as_ptr(),
		version:      1,
		method_count: 0,
		methods:      [
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # callback object
	///
	/// Clients can handle the 'done' event to get notified when
	/// the related request is done.
	pub struct WlCallback(WlProxy);
	
	impl std::ops::Deref for WlCallback {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlCallback {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlCallback {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlCallback")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlCallback {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlCallbackListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlCallbackListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.done((proxy as *mut WlCallback).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: callback_data: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlCallbackListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlCallbackListener: std::any::Any {
		
		/// # done event
		///
		/// Notify the client when the related request is done.
		fn done(
			&self,
			proxy: &mut WlCallback,
			callback_data   : u32,
		);
	}
	
	pub static WL_COMPOSITOR_INTERFACE: WlInterface = WlInterface {
		name:         "wl_compositor\0".as_ptr(),
		version:      4,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "create_surface\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "create_region\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_REGION_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # the compositor singleton
	///
	/// A compositor.  This object is a singleton global.  The
	/// compositor is in charge of combining the contents of multiple
	/// surfaces into one displayable output.
	pub struct WlCompositor(WlProxy);
	
	impl std::ops::Deref for WlCompositor {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlCompositor {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlCompositor {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlCompositor")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlCompositor {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create new surface
		///
		/// Ask the compositor to create a new surface.
		pub fn create_surface(
			&self
		) -> Result<Box<WlSurface, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &WL_SURFACE_INTERFACE, std::ptr::null::<u8>()) as *mut WlSurface };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # create new region
		///
		/// Ask the compositor to create a new region.
		pub fn create_region(
			&self
		) -> Result<Box<WlRegion, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &WL_REGION_INTERFACE, std::ptr::null::<u8>()) as *mut WlRegion };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static WL_SHM_POOL_INTERFACE: WlInterface = WlInterface {
		name:         "wl_shm_pool\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "create_buffer\0".as_ptr(),
				signature: "niiiiu\0".as_ptr(),
				types:     [&WL_BUFFER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "resize\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # a shared memory pool
	///
	/// The wl_shm_pool object encapsulates a piece of memory shared
	/// between the compositor and client.  Through the wl_shm_pool
	/// object, the client can allocate shared memory wl_buffer objects.
	/// All objects created through the same pool share the same
	/// underlying mapped memory. Reusing the mapped memory avoids the
	/// setup/teardown overhead and is useful when interactively resizing
	/// a surface or for many small buffers.
	pub struct WlShmPool(WlProxy);
	
	impl std::ops::Deref for WlShmPool {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlShmPool {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlShmPool {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlShmPool")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlShmPool {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # create a buffer from the pool
		///
		/// Create a wl_buffer object from the pool.
		///
		/// The buffer is created offset bytes into the pool and has
		/// width and height as specified.  The stride argument specifies
		/// the number of bytes from the beginning of one row to the beginning
		/// of the next.  The format is the pixel format of the buffer and
		/// must be one of those advertised through the wl_shm.format event.
		///
		/// A buffer will keep a reference to the pool it was created from
		/// so it is valid to destroy the pool immediately after creating
		/// a buffer from it.
		pub fn create_buffer(
			&self,
			offset          : i32,
			width           : i32,
			height          : i32,
			stride          : i32,
			format          : u32
		) -> Result<Box<WlBuffer, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &WL_BUFFER_INTERFACE, std::ptr::null::<u8>(), offset, width, height, stride, format) as *mut WlBuffer };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # destroy the pool
		///
		/// Destroy the shared memory pool.
		///
		/// The mmapped memory will be released when all
		/// buffers that have been created from this pool
		/// are gone.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # change the size of the pool mapping
		///
		/// This request will cause the server to remap the backing memory
		/// for the pool from the file descriptor passed when the pool was
		/// created, but using the new size.  This request can only be
		/// used to make the pool bigger.
		pub fn resize(
			&self,
			size            : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, size); }
		}
	}
	
	
	pub static WL_SHM_INTERFACE: WlInterface = WlInterface {
		name:         "wl_shm\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "create_pool\0".as_ptr(),
				signature: "nhi\0".as_ptr(),
				types:     [&WL_SHM_POOL_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "format\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # shared memory support
	///
	/// A singleton global object that provides support for shared
	/// memory.
	///
	/// Clients can create wl_shm_pool objects using the create_pool
	/// request.
	///
	/// At connection setup time, the wl_shm object emits one or more
	/// format events to inform clients about the valid pixel formats
	/// that can be used for buffers.
	pub struct WlShm(WlProxy);
	
	impl std::ops::Deref for WlShm {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlShm {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlShm {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlShm")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlShm {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlShmListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlShmListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.format((proxy as *mut WlShm).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `format` ARGS: format: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlShmListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a shm pool
		///
		/// Create a new wl_shm_pool object.
		///
		/// The pool can be used to create shared memory based buffer
		/// objects.  The server will mmap size bytes of the passed file
		/// descriptor, to use as backing memory for the pool.
		pub fn create_pool(
			&self,
			fd              : RawFd,
			size            : i32
		) -> Result<Box<WlShmPool, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &WL_SHM_POOL_INTERFACE, std::ptr::null::<u8>(), fd, size) as *mut WlShmPool };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub trait WlShmListener: std::any::Any {
		
		/// # pixel format description
		///
		/// Informs the client about a valid pixel format that
		/// can be used for buffers. Known formats include
		/// argb8888 and xrgb8888.
		fn format(
			&self,
			proxy: &mut WlShm,
			format          : u32,
		);
	}
	
	/// # wl_shm error values
	///
	/// These errors can be emitted in response to wl_shm requests.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlShmError {
		/// buffer format is not known
		InvalidFormat = 0,
		/// invalid size or stride during pool or buffer creation
		InvalidStride = 1,
		/// mmapping the file descriptor failed
		InvalidFd = 2,
	}
	
	/// # pixel formats
	///
	/// This describes the memory layout of an individual pixel.
	///
	/// All renderers should support argb8888 and xrgb8888 but any other
	/// formats are optional and may not be supported by the particular
	/// renderer in use.
	///
	/// The drm format codes match the macros defined in drm_fourcc.h, except
	/// argb8888 and xrgb8888. The formats actually supported by the compositor
	/// will be reported by the format event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlShmFormat {
		/// 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
		Argb8888 = 0,
		/// 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
		Xrgb8888 = 1,
		/// 8-bit color index format, [7:0] C
		C8 = 0x20203843,
		/// 8-bit RGB format, [7:0] R:G:B 3:3:2
		Rgb332 = 0x38424752,
		/// 8-bit BGR format, [7:0] B:G:R 2:3:3
		Bgr233 = 0x38524742,
		/// 16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian
		Xrgb4444 = 0x32315258,
		/// 16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian
		Xbgr4444 = 0x32314258,
		/// 16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian
		Rgbx4444 = 0x32315852,
		/// 16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian
		Bgrx4444 = 0x32315842,
		/// 16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian
		Argb4444 = 0x32315241,
		/// 16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian
		Abgr4444 = 0x32314241,
		/// 16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian
		Rgba4444 = 0x32314152,
		/// 16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian
		Bgra4444 = 0x32314142,
		/// 16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian
		Xrgb1555 = 0x35315258,
		/// 16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian
		Xbgr1555 = 0x35314258,
		/// 16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian
		Rgbx5551 = 0x35315852,
		/// 16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian
		Bgrx5551 = 0x35315842,
		/// 16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian
		Argb1555 = 0x35315241,
		/// 16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian
		Abgr1555 = 0x35314241,
		/// 16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian
		Rgba5551 = 0x35314152,
		/// 16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian
		Bgra5551 = 0x35314142,
		/// 16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian
		Rgb565 = 0x36314752,
		/// 16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian
		Bgr565 = 0x36314742,
		/// 24-bit RGB format, [23:0] R:G:B little endian
		Rgb888 = 0x34324752,
		/// 24-bit BGR format, [23:0] B:G:R little endian
		Bgr888 = 0x34324742,
		/// 32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian
		Xbgr8888 = 0x34324258,
		/// 32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian
		Rgbx8888 = 0x34325852,
		/// 32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian
		Bgrx8888 = 0x34325842,
		/// 32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian
		Abgr8888 = 0x34324241,
		/// 32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian
		Rgba8888 = 0x34324152,
		/// 32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian
		Bgra8888 = 0x34324142,
		/// 32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian
		Xrgb2101010 = 0x30335258,
		/// 32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian
		Xbgr2101010 = 0x30334258,
		/// 32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian
		Rgbx1010102 = 0x30335852,
		/// 32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian
		Bgrx1010102 = 0x30335842,
		/// 32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian
		Argb2101010 = 0x30335241,
		/// 32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian
		Abgr2101010 = 0x30334241,
		/// 32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian
		Rgba1010102 = 0x30334152,
		/// 32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian
		Bgra1010102 = 0x30334142,
		/// packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian
		Yuyv = 0x56595559,
		/// packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian
		Yvyu = 0x55595659,
		/// packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian
		Uyvy = 0x59565955,
		/// packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian
		Vyuy = 0x59555956,
		/// packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian
		Ayuv = 0x56555941,
		/// 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane
		Nv12 = 0x3231564e,
		/// 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane
		Nv21 = 0x3132564e,
		/// 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane
		Nv16 = 0x3631564e,
		/// 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane
		Nv61 = 0x3136564e,
		/// 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes
		Yuv410 = 0x39565559,
		/// 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes
		Yvu410 = 0x39555659,
		/// 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes
		Yuv411 = 0x31315559,
		/// 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes
		Yvu411 = 0x31315659,
		/// 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes
		Yuv420 = 0x32315559,
		/// 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes
		Yvu420 = 0x32315659,
		/// 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes
		Yuv422 = 0x36315559,
		/// 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes
		Yvu422 = 0x36315659,
		/// 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes
		Yuv444 = 0x34325559,
		/// 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes
		Yvu444 = 0x34325659,
		/// [7:0] R
		R8 = 0x20203852,
		/// [15:0] R little endian
		R16 = 0x20363152,
		/// [15:0] R:G 8:8 little endian
		Rg88 = 0x38384752,
		/// [15:0] G:R 8:8 little endian
		Gr88 = 0x38385247,
		/// [31:0] R:G 16:16 little endian
		Rg1616 = 0x32334752,
		/// [31:0] G:R 16:16 little endian
		Gr1616 = 0x32335247,
		/// [63:0] x:R:G:B 16:16:16:16 little endian
		Xrgb16161616f = 0x48345258,
		/// [63:0] x:B:G:R 16:16:16:16 little endian
		Xbgr16161616f = 0x48344258,
		/// [63:0] A:R:G:B 16:16:16:16 little endian
		Argb16161616f = 0x48345241,
		/// [63:0] A:B:G:R 16:16:16:16 little endian
		Abgr16161616f = 0x48344241,
		/// [31:0] X:Y:Cb:Cr 8:8:8:8 little endian
		Xyuv8888 = 0x56555958,
		/// [23:0] Cr:Cb:Y 8:8:8 little endian
		Vuy888 = 0x34325556,
		/// Y followed by U then V, 10:10:10. Non-linear modifier only
		Vuy101010 = 0x30335556,
		/// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels
		Y210 = 0x30313259,
		/// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels
		Y212 = 0x32313259,
		/// [63:0] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels
		Y216 = 0x36313259,
		/// [31:0] A:Cr:Y:Cb 2:10:10:10 little endian
		Y410 = 0x30313459,
		/// [63:0] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
		Y412 = 0x32313459,
		/// [63:0] A:Cr:Y:Cb 16:16:16:16 little endian
		Y416 = 0x36313459,
		/// [31:0] X:Cr:Y:Cb 2:10:10:10 little endian
		Xvyu2101010 = 0x30335658,
		/// [63:0] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
		Xvyu1216161616 = 0x36335658,
		/// [63:0] X:Cr:Y:Cb 16:16:16:16 little endian
		Xvyu16161616 = 0x38345658,
		/// [63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
		Y0l0 = 0x304c3059,
		/// [63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
		X0l0 = 0x304c3058,
		/// [63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
		Y0l2 = 0x324c3059,
		/// [63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
		X0l2 = 0x324c3058,
		///
		Yuv4208bit = 0x38305559,
		///
		Yuv42010bit = 0x30315559,
		///
		Xrgb8888A8 = 0x38415258,
		///
		Xbgr8888A8 = 0x38414258,
		///
		Rgbx8888A8 = 0x38415852,
		///
		Bgrx8888A8 = 0x38415842,
		///
		Rgb888A8 = 0x38413852,
		///
		Bgr888A8 = 0x38413842,
		///
		Rgb565A8 = 0x38413552,
		///
		Bgr565A8 = 0x38413542,
		/// non-subsampled Cr:Cb plane
		Nv24 = 0x3432564e,
		/// non-subsampled Cb:Cr plane
		Nv42 = 0x3234564e,
		/// 2x1 subsampled Cr:Cb plane, 10 bit per channel
		P210 = 0x30313250,
		/// 2x2 subsampled Cr:Cb plane 10 bits per channel
		P010 = 0x30313050,
		/// 2x2 subsampled Cr:Cb plane 12 bits per channel
		P012 = 0x32313050,
		/// 2x2 subsampled Cr:Cb plane 16 bits per channel
		P016 = 0x36313050,
	}
	
	pub static WL_BUFFER_INTERFACE: WlInterface = WlInterface {
		name:         "wl_buffer\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # content for a wl_surface
	///
	/// A buffer provides the content for a wl_surface. Buffers are
	/// created through factory interfaces such as wl_drm, wl_shm or
	/// similar. It has a width and a height and can be attached to a
	/// wl_surface, but the mechanism by which a client provides and
	/// updates the contents is defined by the buffer factory interface.
	pub struct WlBuffer(WlProxy);
	
	impl std::ops::Deref for WlBuffer {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlBuffer {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlBuffer {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlBuffer")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlBuffer {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlBufferListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlBufferListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.release((proxy as *mut WlBuffer).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `release` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlBufferListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy a buffer
		///
		/// Destroy a buffer. If and how you need to release the backing
		/// storage is defined by the buffer factory interface.
		///
		/// For possible side-effects to a surface, see wl_surface.attach.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlBufferListener: std::any::Any {
		
		/// # compositor releases buffer
		///
		/// Sent when this wl_buffer is no longer used by the compositor.
		/// The client is now free to reuse or destroy this buffer and its
		/// backing storage.
		///
		/// If a client receives a release event before the frame callback
		/// requested in the same wl_surface.commit that attaches this
		/// wl_buffer to a surface, then the client is immediately free to
		/// reuse the buffer and its backing storage, and does not need a
		/// second buffer for the next surface content update. Typically
		/// this is possible, when the compositor maintains a copy of the
		/// wl_surface contents, e.g. as a GL texture. This is an important
		/// optimization for GL(ES) compositors with wl_shm clients.
		fn release(
			&self,
			proxy: &mut WlBuffer,
		);
	}
	
	pub static WL_DATA_OFFER_INTERFACE: WlInterface = WlInterface {
		name:         "wl_data_offer\0".as_ptr(),
		version:      3,
		method_count: 5,
		methods:      [
			WlMessage {
				name:      "accept\0".as_ptr(),
				signature: "u?s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "receive\0".as_ptr(),
				signature: "sh\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "finish\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_actions\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  3,
		events:       [
			WlMessage {
				name:      "offer\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "source_actions\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "action\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # offer to transfer data
	///
	/// A wl_data_offer represents a piece of data offered for transfer
	/// by another client (the source client).  It is used by the
	/// copy-and-paste and drag-and-drop mechanisms.  The offer
	/// describes the different mime types that the data can be
	/// converted to and provides the mechanism for transferring the
	/// data directly from the source client.
	pub struct WlDataOffer(WlProxy);
	
	impl std::ops::Deref for WlDataOffer {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlDataOffer {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlDataOffer {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlDataOffer")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlDataOffer {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlDataOfferListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlDataOfferListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.offer((proxy as *mut WlDataOffer).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						1 => listener.source_actions((proxy as *mut WlDataOffer).as_mut().unwrap(), (*args.add(0)).u, ),
						2 => listener.action((proxy as *mut WlDataOffer).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `offer` ARGS: mime_type: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `source_actions` ARGS: source_actions: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `action` ARGS: dnd_action: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlDataOfferListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # accept one of the offered mime types
		///
		/// Indicate that the client can accept the given mime type, or
		/// NULL for not accepted.
		///
		/// For objects of version 2 or older, this request is used by the
		/// client to give feedback whether the client can receive the given
		/// mime type, or NULL if none is accepted; the feedback does not
		/// determine whether the drag-and-drop operation succeeds or not.
		///
		/// For objects of version 3 or newer, this request determines the
		/// final result of the drag-and-drop operation. If the end result
		/// is that no mime types were accepted, the drag-and-drop operation
		/// will be cancelled and the corresponding drag source will receive
		/// wl_data_source.cancelled. Clients may still use this event in
		/// conjunction with wl_data_source.action for feedback.
		pub fn accept(
			&self,
			serial          : u32,
			mime_type       : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, serial, mime_type.as_ptr()); }
		}
		
		/// # request that the data is transferred
		///
		/// To transfer the offered data, the client issues this request
		/// and indicates the mime type it wants to receive.  The transfer
		/// happens through the passed file descriptor (typically created
		/// with the pipe system call).  The source client writes the data
		/// in the mime type representation requested and then closes the
		/// file descriptor.
		///
		/// The receiving client reads from the read end of the pipe until
		/// EOF and then closes its end, at which point the transfer is
		/// complete.
		///
		/// This request may happen multiple times for different mime types,
		/// both before and after wl_data_device.drop. Drag-and-drop destination
		/// clients may preemptively fetch data or examine it more closely to
		/// determine acceptance.
		pub fn receive(
			&self,
			mime_type       : &str,
			fd              : RawFd
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, mime_type.as_ptr(), fd); }
		}
		
		/// # destroy data offer
		///
		/// Destroy the data offer.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # the offer will no longer be used
		///
		/// Notifies the compositor that the drag destination successfully
		/// finished the drag-and-drop operation.
		///
		/// Upon receiving this request, the compositor will emit
		/// wl_data_source.dnd_finished on the drag source client.
		///
		/// It is a client error to perform other requests than
		/// wl_data_offer.destroy after this one. It is also an error to perform
		/// this request after a NULL mime type has been set in
		/// wl_data_offer.accept or no action was received through
		/// wl_data_offer.action.
		///
		/// If wl_data_offer.finish request is received for a non drag and drop
		/// operation, the invalid_finish protocol error is raised.
		pub fn finish(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3); }
		}
		
		/// # set the available/preferred drag-and-drop actions
		///
		/// Sets the actions that the destination side client supports for
		/// this operation. This request may trigger the emission of
		/// wl_data_source.action and wl_data_offer.action events if the compositor
		/// needs to change the selected action.
		///
		/// This request can be called multiple times throughout the
		/// drag-and-drop operation, typically in response to wl_data_device.enter
		/// or wl_data_device.motion events.
		///
		/// This request determines the final result of the drag-and-drop
		/// operation. If the end result is that no action is accepted,
		/// the drag source will receive wl_data_source.cancelled.
		///
		/// The dnd_actions argument must contain only values expressed in the
		/// wl_data_device_manager.dnd_actions enum, and the preferred_action
		/// argument must only contain one of those values set, otherwise it
		/// will result in a protocol error.
		///
		/// While managing an "ask" action, the destination drag-and-drop client
		/// may perform further wl_data_offer.receive requests, and is expected
		/// to perform one last wl_data_offer.set_actions request with a preferred
		/// action other than "ask" (and optionally wl_data_offer.accept) before
		/// requesting wl_data_offer.finish, in order to convey the action selected
		/// by the user. If the preferred action is not in the
		/// wl_data_offer.source_actions mask, an error will be raised.
		///
		/// If the "ask" action is dismissed (e.g. user cancellation), the client
		/// is expected to perform wl_data_offer.destroy right away.
		///
		/// This request can only be made on drag-and-drop offers, a protocol error
		/// will be raised otherwise.
		pub fn set_actions(
			&self,
			dnd_actions     : u32,
			preferred_action: u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, dnd_actions, preferred_action); }
		}
	}
	
	
	pub trait WlDataOfferListener: std::any::Any {
		
		/// # advertise offered mime type
		///
		/// Sent immediately after creating the wl_data_offer object.  One
		/// event per offered mime type.
		fn offer(
			&self,
			proxy: &mut WlDataOffer,
			mime_type       : &str,
		);
		
		/// # notify the source-side available actions
		///
		/// This event indicates the actions offered by the data source. It
		/// will be sent right after wl_data_device.enter, or anytime the source
		/// side changes its offered actions through wl_data_source.set_actions.
		fn source_actions(
			&self,
			proxy: &mut WlDataOffer,
			source_actions  : u32,
		);
		
		/// # notify the selected action
		///
		/// This event indicates the action selected by the compositor after
		/// matching the source/destination side actions. Only one action (or
		/// none) will be offered here.
		///
		/// This event can be emitted multiple times during the drag-and-drop
		/// operation in response to destination side action changes through
		/// wl_data_offer.set_actions.
		///
		/// This event will no longer be emitted after wl_data_device.drop
		/// happened on the drag-and-drop destination, the client must
		/// honor the last action received, or the last preferred one set
		/// through wl_data_offer.set_actions when handling an "ask" action.
		///
		/// Compositors may also change the selected action on the fly, mainly
		/// in response to keyboard modifier changes during the drag-and-drop
		/// operation.
		///
		/// The most recent action received is always the valid one. Prior to
		/// receiving wl_data_device.drop, the chosen action may change (e.g.
		/// due to keyboard modifiers being pressed). At the time of receiving
		/// wl_data_device.drop the drag-and-drop destination must honor the
		/// last action received.
		///
		/// Action changes may still happen after wl_data_device.drop,
		/// especially on "ask" actions, where the drag-and-drop destination
		/// may choose another action afterwards. Action changes happening
		/// at this stage are always the result of inter-client negotiation, the
		/// compositor shall no longer be able to induce a different action.
		///
		/// Upon "ask" actions, it is expected that the drag-and-drop destination
		/// may potentially choose a different action and/or mime type,
		/// based on wl_data_offer.source_actions and finally chosen by the
		/// user (e.g. popping up a menu with the available options). The
		/// final wl_data_offer.set_actions and wl_data_offer.accept requests
		/// must happen before the call to wl_data_offer.finish.
		fn action(
			&self,
			proxy: &mut WlDataOffer,
			dnd_action      : u32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlDataOfferError {
		/// finish request was called untimely
		InvalidFinish = 0,
		/// action mask contains invalid values
		InvalidActionMask = 1,
		/// action argument has an invalid value
		InvalidAction = 2,
		/// offer doesn't accept this request
		InvalidOffer = 3,
	}
	
	pub static WL_DATA_SOURCE_INTERFACE: WlInterface = WlInterface {
		name:         "wl_data_source\0".as_ptr(),
		version:      3,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "offer\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_actions\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  6,
		events:       [
			WlMessage {
				name:      "target\0".as_ptr(),
				signature: "?s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "send\0".as_ptr(),
				signature: "sh\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "cancelled\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "dnd_drop_performed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "dnd_finished\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "action\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # offer to transfer data
	///
	/// The wl_data_source object is the source side of a wl_data_offer.
	/// It is created by the source client in a data transfer and
	/// provides a way to describe the offered data and a way to respond
	/// to requests to transfer the data.
	pub struct WlDataSource(WlProxy);
	
	impl std::ops::Deref for WlDataSource {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlDataSource {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlDataSource {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlDataSource")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlDataSource {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlDataSourceListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlDataSourceListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.target((proxy as *mut WlDataSource).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						1 => listener.send((proxy as *mut WlDataSource).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).h, ),
						2 => listener.cancelled((proxy as *mut WlDataSource).as_mut().unwrap(), ),
						3 => listener.dnd_drop_performed((proxy as *mut WlDataSource).as_mut().unwrap(), ),
						4 => listener.dnd_finished((proxy as *mut WlDataSource).as_mut().unwrap(), ),
						5 => listener.action((proxy as *mut WlDataSource).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `target` ARGS: mime_type: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `send` ARGS: mime_type: {:?}, fd: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).h),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `cancelled` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `dnd_drop_performed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `dnd_finished` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `action` ARGS: dnd_action: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlDataSourceListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # add an offered mime type
		///
		/// This request adds a mime type to the set of mime types
		/// advertised to targets.  Can be called several times to offer
		/// multiple types.
		pub fn offer(
			&self,
			mime_type       : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, mime_type.as_ptr()); }
		}
		
		/// # destroy the data source
		///
		/// Destroy the data source.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the available drag-and-drop actions
		///
		/// Sets the actions that the source side client supports for this
		/// operation. This request may trigger wl_data_source.action and
		/// wl_data_offer.action events if the compositor needs to change the
		/// selected action.
		///
		/// The dnd_actions argument must contain only values expressed in the
		/// wl_data_device_manager.dnd_actions enum, otherwise it will result
		/// in a protocol error.
		///
		/// This request must be made once only, and can only be made on sources
		/// used in drag-and-drop, so it must be performed before
		/// wl_data_device.start_drag. Attempting to use the source other than
		/// for drag-and-drop will raise a protocol error.
		pub fn set_actions(
			&self,
			dnd_actions     : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, dnd_actions); }
		}
	}
	
	
	pub trait WlDataSourceListener: std::any::Any {
		
		/// # a target accepts an offered mime type
		///
		/// Sent when a target accepts pointer_focus or motion events.  If
		/// a target does not accept any of the offered types, type is NULL.
		///
		/// Used for feedback during drag-and-drop.
		fn target(
			&self,
			proxy: &mut WlDataSource,
			mime_type       : &str,
		);
		
		/// # send the data
		///
		/// Request for data from the client.  Send the data as the
		/// specified mime type over the passed file descriptor, then
		/// close it.
		fn send(
			&self,
			proxy: &mut WlDataSource,
			mime_type       : &str,
			fd              : RawFd,
		);
		
		/// # selection was cancelled
		///
		/// This data source is no longer valid. There are several reasons why
		/// this could happen:
		///
		/// - The data source has been replaced by another data source.
		/// - The drag-and-drop operation was performed, but the drop destination
		/// did not accept any of the mime types offered through
		/// wl_data_source.target.
		/// - The drag-and-drop operation was performed, but the drop destination
		/// did not select any of the actions present in the mask offered through
		/// wl_data_source.action.
		/// - The drag-and-drop operation was performed but didn't happen over a
		/// surface.
		/// - The compositor cancelled the drag-and-drop operation (e.g. compositor
		/// dependent timeouts to avoid stale drag-and-drop transfers).
		///
		/// The client should clean up and destroy this data source.
		///
		/// For objects of version 2 or older, wl_data_source.cancelled will
		/// only be emitted if the data source was replaced by another data
		/// source.
		fn cancelled(
			&self,
			proxy: &mut WlDataSource,
		);
		
		/// # the drag-and-drop operation physically finished
		///
		/// The user performed the drop action. This event does not indicate
		/// acceptance, wl_data_source.cancelled may still be emitted afterwards
		/// if the drop destination does not accept any mime type.
		///
		/// However, this event might however not be received if the compositor
		/// cancelled the drag-and-drop operation before this event could happen.
		///
		/// Note that the data_source may still be used in the future and should
		/// not be destroyed here.
		fn dnd_drop_performed(
			&self,
			proxy: &mut WlDataSource,
		);
		
		/// # the drag-and-drop operation concluded
		///
		/// The drop destination finished interoperating with this data
		/// source, so the client is now free to destroy this data source and
		/// free all associated data.
		///
		/// If the action used to perform the operation was "move", the
		/// source can now delete the transferred data.
		fn dnd_finished(
			&self,
			proxy: &mut WlDataSource,
		);
		
		/// # notify the selected action
		///
		/// This event indicates the action selected by the compositor after
		/// matching the source/destination side actions. Only one action (or
		/// none) will be offered here.
		///
		/// This event can be emitted multiple times during the drag-and-drop
		/// operation, mainly in response to destination side changes through
		/// wl_data_offer.set_actions, and as the data device enters/leaves
		/// surfaces.
		///
		/// It is only possible to receive this event after
		/// wl_data_source.dnd_drop_performed if the drag-and-drop operation
		/// ended in an "ask" action, in which case the final wl_data_source.action
		/// event will happen immediately before wl_data_source.dnd_finished.
		///
		/// Compositors may also change the selected action on the fly, mainly
		/// in response to keyboard modifier changes during the drag-and-drop
		/// operation.
		///
		/// The most recent action received is always the valid one. The chosen
		/// action may change alongside negotiation (e.g. an "ask" action can turn
		/// into a "move" operation), so the effects of the final action must
		/// always be applied in wl_data_offer.dnd_finished.
		///
		/// Clients can trigger cursor surface changes from this point, so
		/// they reflect the current action.
		fn action(
			&self,
			proxy: &mut WlDataSource,
			dnd_action      : u32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlDataSourceError {
		/// action mask contains invalid values
		InvalidActionMask = 0,
		/// source doesn't accept this request
		InvalidSource = 1,
	}
	
	pub static WL_DATA_DEVICE_INTERFACE: WlInterface = WlInterface {
		name:         "wl_data_device\0".as_ptr(),
		version:      3,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "start_drag\0".as_ptr(),
				signature: "?oo?ou\0".as_ptr(),
				types:     [&WL_DATA_SOURCE_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_selection\0".as_ptr(),
				signature: "?ou\0".as_ptr(),
				types:     [&WL_DATA_SOURCE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  6,
		events:       [
			WlMessage {
				name:      "data_offer\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_DATA_OFFER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "enter\0".as_ptr(),
				signature: "uoff?o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _, &WL_DATA_OFFER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "leave\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "motion\0".as_ptr(),
				signature: "uff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "drop\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "selection\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_DATA_OFFER_INTERFACE as _].as_ptr()
			},
		].as_ptr()
	};
	
	/// # data transfer device
	///
	/// There is one wl_data_device per seat which can be obtained
	/// from the global wl_data_device_manager singleton.
	///
	/// A wl_data_device provides access to inter-client data transfer
	/// mechanisms such as copy-and-paste and drag-and-drop.
	pub struct WlDataDevice(WlProxy);
	
	impl std::ops::Deref for WlDataDevice {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlDataDevice {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlDataDevice {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlDataDevice")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlDataDevice {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlDataDeviceListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlDataDeviceListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.data_offer((proxy as *mut WlDataDevice).as_mut().unwrap(), (*args.add(0)).n, ),
						1 => listener.enter((proxy as *mut WlDataDevice).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), (*args.add(2)).f, (*args.add(3)).f, ((*args.add(4)).o as *mut WlDataOffer).as_mut(), ),
						2 => listener.leave((proxy as *mut WlDataDevice).as_mut().unwrap(), ),
						3 => listener.motion((proxy as *mut WlDataDevice).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f, ),
						4 => listener.drop((proxy as *mut WlDataDevice).as_mut().unwrap(), ),
						5 => listener.selection((proxy as *mut WlDataDevice).as_mut().unwrap(), ((*args.add(0)).o as *mut WlDataOffer).as_mut(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `data_offer` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `enter` ARGS: serial: {:?}, surface: {:?}, x: {:?}, y: {:?}, id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), (*args.add(2)).f, (*args.add(3)).f, ((*args.add(4)).o as *mut WlDataOffer).as_mut()),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `leave` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `motion` ARGS: time: {:?}, x: {:?}, y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `drop` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `selection` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlDataOffer).as_mut()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlDataDeviceListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # start drag-and-drop operation
		///
		/// This request asks the compositor to start a drag-and-drop
		/// operation on behalf of the client.
		///
		/// The source argument is the data source that provides the data
		/// for the eventual data transfer. If source is NULL, enter, leave
		/// and motion events are sent only to the client that initiated the
		/// drag and the client is expected to handle the data passing
		/// internally. If source is destroyed, the drag-and-drop session will be
		/// cancelled.
		///
		/// The origin surface is the surface where the drag originates and
		/// the client must have an active implicit grab that matches the
		/// serial.
		///
		/// The icon surface is an optional (can be NULL) surface that
		/// provides an icon to be moved around with the cursor.  Initially,
		/// the top-left corner of the icon surface is placed at the cursor
		/// hotspot, but subsequent wl_surface.attach request can move the
		/// relative position. Attach requests must be confirmed with
		/// wl_surface.commit as usual. The icon surface is given the role of
		/// a drag-and-drop icon. If the icon surface already has another role,
		/// it raises a protocol error.
		///
		/// The current and pending input regions of the icon wl_surface are
		/// cleared, and wl_surface.set_input_region is ignored until the
		/// wl_surface is no longer used as the icon surface. When the use
		/// as an icon ends, the current and pending input regions become
		/// undefined, and the wl_surface is unmapped.
		pub fn start_drag(
			&self,
			source          : Option<&WlDataSource>,
			origin          : &WlSurface,
			icon            : Option<&WlSurface>,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, source.map_or(std::ptr::null_mut(), |r| r as *const WlDataSource as *mut WlDataSource), origin, icon.map_or(std::ptr::null_mut(), |r| r as *const WlSurface as *mut WlSurface), serial); }
		}
		
		/// # copy data to the selection
		///
		/// This request asks the compositor to set the selection
		/// to the data from the source on behalf of the client.
		///
		/// To unset the selection, set the source to NULL.
		pub fn set_selection(
			&self,
			source          : Option<&WlDataSource>,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, source.map_or(std::ptr::null_mut(), |r| r as *const WlDataSource as *mut WlDataSource), serial); }
		}
		
		/// # destroy data device
		///
		/// This request destroys the data device.
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlDataDeviceListener: std::any::Any {
		
		/// # introduce a new wl_data_offer
		///
		/// The data_offer event introduces a new wl_data_offer object,
		/// which will subsequently be used in either the
		/// data_device.enter event (for drag-and-drop) or the
		/// data_device.selection event (for selections).  Immediately
		/// following the data_device_data_offer event, the new data_offer
		/// object will send out data_offer.offer events to describe the
		/// mime types it offers.
		fn data_offer(
			&self,
			proxy: &mut WlDataDevice,
			id              : u32,
		);
		
		/// # initiate drag-and-drop session
		///
		/// This event is sent when an active drag-and-drop pointer enters
		/// a surface owned by the client.  The position of the pointer at
		/// enter time is provided by the x and y arguments, in surface-local
		/// coordinates.
		fn enter(
			&self,
			proxy: &mut WlDataDevice,
			serial          : u32,
			surface         : Option<&mut WlSurface>,
			x               : WlFixed,
			y               : WlFixed,
			id              : Option<&mut WlDataOffer>,
		);
		
		/// # end drag-and-drop session
		///
		/// This event is sent when the drag-and-drop pointer leaves the
		/// surface and the session ends.  The client must destroy the
		/// wl_data_offer introduced at enter time at this point.
		fn leave(
			&self,
			proxy: &mut WlDataDevice,
		);
		
		/// # drag-and-drop session motion
		///
		/// This event is sent when the drag-and-drop pointer moves within
		/// the currently focused surface. The new position of the pointer
		/// is provided by the x and y arguments, in surface-local
		/// coordinates.
		fn motion(
			&self,
			proxy: &mut WlDataDevice,
			time            : u32,
			x               : WlFixed,
			y               : WlFixed,
		);
		
		/// # end drag-and-drop session successfully
		///
		/// The event is sent when a drag-and-drop operation is ended
		/// because the implicit grab is removed.
		///
		/// The drag-and-drop destination is expected to honor the last action
		/// received through wl_data_offer.action, if the resulting action is
		/// "copy" or "move", the destination can still perform
		/// wl_data_offer.receive requests, and is expected to end all
		/// transfers with a wl_data_offer.finish request.
		///
		/// If the resulting action is "ask", the action will not be considered
		/// final. The drag-and-drop destination is expected to perform one last
		/// wl_data_offer.set_actions request, or wl_data_offer.destroy in order
		/// to cancel the operation.
		fn drop(
			&self,
			proxy: &mut WlDataDevice,
		);
		
		/// # advertise new selection
		///
		/// The selection event is sent out to notify the client of a new
		/// wl_data_offer for the selection for this device.  The
		/// data_device.data_offer and the data_offer.offer events are
		/// sent out immediately before this event to introduce the data
		/// offer object.  The selection event is sent to a client
		/// immediately before receiving keyboard focus and when a new
		/// selection is set while the client has keyboard focus.  The
		/// data_offer is valid until a new data_offer or NULL is received
		/// or until the client loses keyboard focus.  The client must
		/// destroy the previous selection data_offer, if any, upon receiving
		/// this event.
		fn selection(
			&self,
			proxy: &mut WlDataDevice,
			id              : Option<&mut WlDataOffer>,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlDataDeviceError {
		/// given wl_surface has another role
		Role = 0,
	}
	
	pub static WL_DATA_DEVICE_MANAGER_INTERFACE: WlInterface = WlInterface {
		name:         "wl_data_device_manager\0".as_ptr(),
		version:      3,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "create_data_source\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_DATA_SOURCE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_data_device\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&WL_DATA_DEVICE_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # data transfer interface
	///
	/// The wl_data_device_manager is a singleton global object that
	/// provides access to inter-client data transfer mechanisms such as
	/// copy-and-paste and drag-and-drop.  These mechanisms are tied to
	/// a wl_seat and this interface lets a client get a wl_data_device
	/// corresponding to a wl_seat.
	///
	/// Depending on the version bound, the objects created from the bound
	/// wl_data_device_manager object will have different requirements for
	/// functioning properly. See wl_data_source.set_actions,
	/// wl_data_offer.accept and wl_data_offer.finish for details.
	pub struct WlDataDeviceManager(WlProxy);
	
	impl std::ops::Deref for WlDataDeviceManager {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlDataDeviceManager {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlDataDeviceManager {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlDataDeviceManager")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlDataDeviceManager {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a new data source
		///
		/// Create a new data source.
		pub fn create_data_source(
			&self
		) -> Result<Box<WlDataSource, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &WL_DATA_SOURCE_INTERFACE, std::ptr::null::<u8>()) as *mut WlDataSource };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # create a new data device
		///
		/// Create a new data device for a given seat.
		pub fn get_data_device(
			&self,
			seat            : &WlSeat
		) -> Result<Box<WlDataDevice, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &WL_DATA_DEVICE_INTERFACE, std::ptr::null::<u8>(), seat) as *mut WlDataDevice };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	/// # drag and drop actions
	///
	/// This is a bitmask of the available/preferred actions in a
	/// drag-and-drop operation.
	///
	/// In the compositor, the selected action is a result of matching the
	/// actions offered by the source and destination sides.  "action" events
	/// with a "none" action will be sent to both source and destination if
	/// there is no match. All further checks will effectively happen on
	/// (source actions ∩ destination actions).
	///
	/// In addition, compositors may also pick different actions in
	/// reaction to key modifiers being pressed. One common design that
	/// is used in major toolkits (and the behavior recommended for
	/// compositors) is:
	///
	/// - If no modifiers are pressed, the first match (in bit order)
	/// will be used.
	/// - Pressing Shift selects "move", if enabled in the mask.
	/// - Pressing Control selects "copy", if enabled in the mask.
	///
	/// Behavior beyond that is considered implementation-dependent.
	/// Compositors may for example bind other modifiers (like Alt/Meta)
	/// or drags initiated with other buttons than BTN_LEFT to specific
	/// actions (e.g. "ask").
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlDataDeviceManagerDndAction {
		/// no action
		None = 0,
		/// copy action
		Copy = 1,
		/// move action
		Move = 2,
		/// ask action
		Ask = 4,
	}
	
	pub static WL_SHELL_INTERFACE: WlInterface = WlInterface {
		name:         "wl_shell\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "get_shell_surface\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&WL_SHELL_SURFACE_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # create desktop-style surfaces
	///
	/// This interface is implemented by servers that provide
	/// desktop-style user interfaces.
	///
	/// It allows clients to associate a wl_shell_surface with
	/// a basic surface.
	///
	/// Note! This protocol is deprecated and not intended for production use.
	/// For desktop-style user interfaces, use xdg_shell.
	pub struct WlShell(WlProxy);
	
	impl std::ops::Deref for WlShell {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlShell {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlShell {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlShell")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlShell {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a shell surface from a surface
		///
		/// Create a shell surface for an existing surface. This gives
		/// the wl_surface the role of a shell surface. If the wl_surface
		/// already has another role, it raises a protocol error.
		///
		/// Only one shell surface can be associated with a given surface.
		pub fn get_shell_surface(
			&self,
			surface         : &WlSurface
		) -> Result<Box<WlShellSurface, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &WL_SHELL_SURFACE_INTERFACE, std::ptr::null::<u8>(), surface) as *mut WlShellSurface };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlShellError {
		/// given wl_surface has another role
		Role = 0,
	}
	
	pub static WL_SHELL_SURFACE_INTERFACE: WlInterface = WlInterface {
		name:         "wl_shell_surface\0".as_ptr(),
		version:      1,
		method_count: 10,
		methods:      [
			WlMessage {
				name:      "pong\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "move\0".as_ptr(),
				signature: "ou\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "resize\0".as_ptr(),
				signature: "ouu\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_toplevel\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_transient\0".as_ptr(),
				signature: "oiiu\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_fullscreen\0".as_ptr(),
				signature: "uu?o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_popup\0".as_ptr(),
				signature: "ouoiiu\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_maximized\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_title\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_class\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  3,
		events:       [
			WlMessage {
				name:      "ping\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "configure\0".as_ptr(),
				signature: "uii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "popup_done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # desktop-style metadata interface
	///
	/// An interface that may be implemented by a wl_surface, for
	/// implementations that provide a desktop-style user interface.
	///
	/// It provides requests to treat surfaces like toplevel, fullscreen
	/// or popup windows, move, resize or maximize them, associate
	/// metadata like title and class, etc.
	///
	/// On the server side the object is automatically destroyed when
	/// the related wl_surface is destroyed. On the client side,
	/// wl_shell_surface_destroy() must be called before destroying
	/// the wl_surface object.
	pub struct WlShellSurface(WlProxy);
	
	impl std::ops::Deref for WlShellSurface {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlShellSurface {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlShellSurface {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlShellSurface")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlShellSurface {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlShellSurfaceListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlShellSurfaceListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.ping((proxy as *mut WlShellSurface).as_mut().unwrap(), (*args.add(0)).u, ),
						1 => listener.configure((proxy as *mut WlShellSurface).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).i, (*args.add(2)).i, ),
						2 => listener.popup_done((proxy as *mut WlShellSurface).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `ping` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `configure` ARGS: edges: {:?}, width: {:?}, height: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).i, (*args.add(2)).i),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `popup_done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlShellSurfaceListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # respond to a ping event
		///
		/// A client must respond to a ping event with a pong request or
		/// the client may be deemed unresponsive.
		pub fn pong(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, serial); }
		}
		
		/// # start an interactive move
		///
		/// Start a pointer-driven move of the surface.
		///
		/// This request must be used in response to a button press event.
		/// The server may ignore move requests depending on the state of
		/// the surface (e.g. fullscreen or maximized).
		pub fn r#move(
			&self,
			seat            : &WlSeat,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, seat, serial); }
		}
		
		/// # start an interactive resize
		///
		/// Start a pointer-driven resizing of the surface.
		///
		/// This request must be used in response to a button press event.
		/// The server may ignore resize requests depending on the state of
		/// the surface (e.g. fullscreen or maximized).
		pub fn resize(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			edges           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, seat, serial, edges); }
		}
		
		/// # make the surface a toplevel surface
		///
		/// Map the surface as a toplevel surface.
		///
		/// A toplevel surface is not fullscreen, maximized or transient.
		pub fn set_toplevel(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3); }
		}
		
		/// # make the surface a transient surface
		///
		/// Map the surface relative to an existing surface.
		///
		/// The x and y arguments specify the location of the upper left
		/// corner of the surface relative to the upper left corner of the
		/// parent surface, in surface-local coordinates.
		///
		/// The flags argument controls details of the transient behaviour.
		pub fn set_transient(
			&self,
			parent          : &WlSurface,
			x               : i32,
			y               : i32,
			flags           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, parent, x, y, flags); }
		}
		
		/// # make the surface a fullscreen surface
		///
		/// Map the surface as a fullscreen surface.
		///
		/// If an output parameter is given then the surface will be made
		/// fullscreen on that output. If the client does not specify the
		/// output then the compositor will apply its policy - usually
		/// choosing the output on which the surface has the biggest surface
		/// area.
		///
		/// The client may specify a method to resolve a size conflict
		/// between the output size and the surface size - this is provided
		/// through the method parameter.
		///
		/// The framerate parameter is used only when the method is set
		/// to "driver", to indicate the preferred framerate. A value of 0
		/// indicates that the client does not care about framerate.  The
		/// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
		///
		/// A method of "scale" or "driver" implies a scaling operation of
		/// the surface, either via a direct scaling operation or a change of
		/// the output mode. This will override any kind of output scaling, so
		/// that mapping a surface with a buffer size equal to the mode can
		/// fill the screen independent of buffer_scale.
		///
		/// A method of "fill" means we don't scale up the buffer, however
		/// any output scale is applied. This means that you may run into
		/// an edge case where the application maps a buffer with the same
		/// size of the output mode but buffer_scale 1 (thus making a
		/// surface larger than the output). In this case it is allowed to
		/// downscale the results to fit the screen.
		///
		/// The compositor must reply to this request with a configure event
		/// with the dimensions for the output on which the surface will
		/// be made fullscreen.
		pub fn set_fullscreen(
			&self,
			method          : u32,
			framerate       : u32,
			output          : Option<&WlOutput>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, method, framerate, output.map_or(std::ptr::null_mut(), |r| r as *const WlOutput as *mut WlOutput)); }
		}
		
		/// # make the surface a popup surface
		///
		/// Map the surface as a popup.
		///
		/// A popup surface is a transient surface with an added pointer
		/// grab.
		///
		/// An existing implicit grab will be changed to owner-events mode,
		/// and the popup grab will continue after the implicit grab ends
		/// (i.e. releasing the mouse button does not cause the popup to
		/// be unmapped).
		///
		/// The popup grab continues until the window is destroyed or a
		/// mouse button is pressed in any other client's window. A click
		/// in any of the client's surfaces is reported as normal, however,
		/// clicks in other clients' surfaces will be discarded and trigger
		/// the callback.
		///
		/// The x and y arguments specify the location of the upper left
		/// corner of the surface relative to the upper left corner of the
		/// parent surface, in surface-local coordinates.
		pub fn set_popup(
			&self,
			seat            : &WlSeat,
			serial          : u32,
			parent          : &WlSurface,
			x               : i32,
			y               : i32,
			flags           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, seat, serial, parent, x, y, flags); }
		}
		
		/// # make the surface a maximized surface
		///
		/// Map the surface as a maximized surface.
		///
		/// If an output parameter is given then the surface will be
		/// maximized on that output. If the client does not specify the
		/// output then the compositor will apply its policy - usually
		/// choosing the output on which the surface has the biggest surface
		/// area.
		///
		/// The compositor will reply with a configure event telling
		/// the expected new surface size. The operation is completed
		/// on the next buffer attach to this surface.
		///
		/// A maximized surface typically fills the entire output it is
		/// bound to, except for desktop elements such as panels. This is
		/// the main difference between a maximized shell surface and a
		/// fullscreen shell surface.
		///
		/// The details depend on the compositor implementation.
		pub fn set_maximized(
			&self,
			output          : Option<&WlOutput>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7, output.map_or(std::ptr::null_mut(), |r| r as *const WlOutput as *mut WlOutput)); }
		}
		
		/// # set surface title
		///
		/// Set a short title for the surface.
		///
		/// This string may be used to identify the surface in a task bar,
		/// window list, or other user interface elements provided by the
		/// compositor.
		///
		/// The string must be encoded in UTF-8.
		pub fn set_title(
			&self,
			title           : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 8, title.as_ptr()); }
		}
		
		/// # set surface class
		///
		/// Set a class for the surface.
		///
		/// The surface class identifies the general class of applications
		/// to which the surface belongs. A common convention is to use the
		/// file name (or the full path if it is a non-standard location) of
		/// the application's .desktop file as the class.
		pub fn set_class(
			&self,
			class_          : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 9, class_.as_ptr()); }
		}
	}
	
	
	pub trait WlShellSurfaceListener: std::any::Any {
		
		/// # ping client
		///
		/// Ping a client to check if it is receiving events and sending
		/// requests. A client is expected to reply with a pong request.
		fn ping(
			&self,
			proxy: &mut WlShellSurface,
			serial          : u32,
		);
		
		/// # suggest resize
		///
		/// The configure event asks the client to resize its surface.
		///
		/// The size is a hint, in the sense that the client is free to
		/// ignore it if it doesn't resize, pick a smaller size (to
		/// satisfy aspect ratio or resize in steps of NxM pixels).
		///
		/// The edges parameter provides a hint about how the surface
		/// was resized. The client may use this information to decide
		/// how to adjust its content to the new size (e.g. a scrolling
		/// area might adjust its content position to leave the viewable
		/// content unmoved).
		///
		/// The client is free to dismiss all but the last configure
		/// event it received.
		///
		/// The width and height arguments specify the size of the window
		/// in surface-local coordinates.
		fn configure(
			&self,
			proxy: &mut WlShellSurface,
			edges           : u32,
			width           : i32,
			height          : i32,
		);
		
		/// # popup interaction is done
		///
		/// The popup_done event is sent out when a popup grab is broken,
		/// that is, when the user clicks a surface that doesn't belong
		/// to the client owning the popup surface.
		fn popup_done(
			&self,
			proxy: &mut WlShellSurface,
		);
	}
	
	/// # edge values for resizing
	///
	/// These values are used to indicate which edge of a surface
	/// is being dragged in a resize operation. The server may
	/// use this information to adapt its behavior, e.g. choose
	/// an appropriate cursor image.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlShellSurfaceResize {
		/// no edge
		None = 0,
		/// top edge
		Top = 1,
		/// bottom edge
		Bottom = 2,
		/// left edge
		Left = 4,
		/// top and left edges
		TopLeft = 5,
		/// bottom and left edges
		BottomLeft = 6,
		/// right edge
		Right = 8,
		/// top and right edges
		TopRight = 9,
		/// bottom and right edges
		BottomRight = 10,
	}
	
	/// # details of transient behaviour
	///
	/// These flags specify details of the expected behaviour
	/// of transient surfaces. Used in the set_transient request.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlShellSurfaceTransient {
		/// do not set keyboard focus
		Inactive = 0x1,
	}
	
	/// # different method to set the surface fullscreen
	///
	/// Hints to indicate to the compositor how to deal with a conflict
	/// between the dimensions of the surface and the dimensions of the
	/// output. The compositor is free to ignore this parameter.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlShellSurfaceFullscreenMethod {
		/// no preference, apply default policy
		Default = 0,
		/// scale, preserve the surface's aspect ratio and center on output
		Scale = 1,
		/// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
		Driver = 2,
		/// no upscaling, center on output and add black borders to compensate size mismatch
		Fill = 3,
	}
	
	pub static WL_SURFACE_INTERFACE: WlInterface = WlInterface {
		name:         "wl_surface\0".as_ptr(),
		version:      4,
		method_count: 10,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "attach\0".as_ptr(),
				signature: "?oii\0".as_ptr(),
				types:     [&WL_BUFFER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "damage\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "frame\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_CALLBACK_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_opaque_region\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_REGION_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_input_region\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_REGION_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "commit\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_buffer_transform\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_buffer_scale\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "damage_buffer\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "enter\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "leave\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
		].as_ptr()
	};
	
	/// # an onscreen surface
	///
	/// A surface is a rectangular area that may be displayed on zero
	/// or more outputs, and shown any number of times at the compositor's
	/// discretion. They can present wl_buffers, receive user input, and
	/// define a local coordinate system.
	///
	/// The size of a surface (and relative positions on it) is described
	/// in surface-local coordinates, which may differ from the buffer
	/// coordinates of the pixel content, in case a buffer_transform
	/// or a buffer_scale is used.
	///
	/// A surface without a "role" is fairly useless: a compositor does
	/// not know where, when or how to present it. The role is the
	/// purpose of a wl_surface. Examples of roles are a cursor for a
	/// pointer (as set by wl_pointer.set_cursor), a drag icon
	/// (wl_data_device.start_drag), a sub-surface
	/// (wl_subcompositor.get_subsurface), and a window as defined by a
	/// shell protocol (e.g. wl_shell.get_shell_surface).
	///
	/// A surface can have only one role at a time. Initially a
	/// wl_surface does not have a role. Once a wl_surface is given a
	/// role, it is set permanently for the whole lifetime of the
	/// wl_surface object. Giving the current role again is allowed,
	/// unless explicitly forbidden by the relevant interface
	/// specification.
	///
	/// Surface roles are given by requests in other interfaces such as
	/// wl_pointer.set_cursor. The request should explicitly mention
	/// that this request gives a role to a wl_surface. Often, this
	/// request also creates a new protocol object that represents the
	/// role and adds additional functionality to wl_surface. When a
	/// client wants to destroy a wl_surface, they must destroy this 'role
	/// object' before the wl_surface.
	///
	/// Destroying the role object does not remove the role from the
	/// wl_surface, but it may stop the wl_surface from "playing the role".
	/// For instance, if a wl_subsurface object is destroyed, the wl_surface
	/// it was created for will be unmapped and forget its position and
	/// z-order. It is allowed to create a wl_subsurface for the same
	/// wl_surface again, but it is not allowed to use the wl_surface as
	/// a cursor (cursor is a different role than sub-surface, and role
	/// switching is not allowed).
	pub struct WlSurface(WlProxy);
	
	impl std::ops::Deref for WlSurface {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlSurface {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlSurface {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlSurface")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlSurface {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlSurfaceListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlSurfaceListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.enter((proxy as *mut WlSurface).as_mut().unwrap(), ((*args.add(0)).o as *mut WlOutput).as_mut(), ),
						1 => listener.leave((proxy as *mut WlSurface).as_mut().unwrap(), ((*args.add(0)).o as *mut WlOutput).as_mut(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `enter` ARGS: output: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlOutput).as_mut()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `leave` ARGS: output: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlOutput).as_mut()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlSurfaceListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # delete surface
		///
		/// Deletes the surface and invalidates its object ID.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the surface contents
		///
		/// Set a buffer as the content of this surface.
		///
		/// The new size of the surface is calculated based on the buffer
		/// size transformed by the inverse buffer_transform and the
		/// inverse buffer_scale. This means that at commit time the supplied
		/// buffer size must be an integer multiple of the buffer_scale. If
		/// that's not the case, an invalid_size error is sent.
		///
		/// The x and y arguments specify the location of the new pending
		/// buffer's upper left corner, relative to the current buffer's upper
		/// left corner, in surface-local coordinates. In other words, the
		/// x and y, combined with the new surface size define in which
		/// directions the surface's size changes.
		///
		/// Surface contents are double-buffered state, see wl_surface.commit.
		///
		/// The initial surface contents are void; there is no content.
		/// wl_surface.attach assigns the given wl_buffer as the pending
		/// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
		/// surface contents, and the size of the surface becomes the size
		/// calculated from the wl_buffer, as described above. After commit,
		/// there is no pending buffer until the next attach.
		///
		/// Committing a pending wl_buffer allows the compositor to read the
		/// pixels in the wl_buffer. The compositor may access the pixels at
		/// any time after the wl_surface.commit request. When the compositor
		/// will not access the pixels anymore, it will send the
		/// wl_buffer.release event. Only after receiving wl_buffer.release,
		/// the client may reuse the wl_buffer. A wl_buffer that has been
		/// attached and then replaced by another attach instead of committed
		/// will not receive a release event, and is not used by the
		/// compositor.
		///
		/// If a pending wl_buffer has been committed to more than one wl_surface,
		/// the delivery of wl_buffer.release events becomes undefined. A well
		/// behaved client should not rely on wl_buffer.release events in this
		/// case. Alternatively, a client could create multiple wl_buffer objects
		/// from the same backing storage or use wp_linux_buffer_release.
		///
		/// Destroying the wl_buffer after wl_buffer.release does not change
		/// the surface contents. However, if the client destroys the
		/// wl_buffer before receiving the wl_buffer.release event, the surface
		/// contents become undefined immediately.
		///
		/// If wl_surface.attach is sent with a NULL wl_buffer, the
		/// following wl_surface.commit will remove the surface content.
		pub fn attach(
			&self,
			buffer          : Option<&WlBuffer>,
			x               : i32,
			y               : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, buffer.map_or(std::ptr::null_mut(), |r| r as *const WlBuffer as *mut WlBuffer), x, y); }
		}
		
		/// # mark part of the surface damaged
		///
		/// This request is used to describe the regions where the pending
		/// buffer is different from the current surface contents, and where
		/// the surface therefore needs to be repainted. The compositor
		/// ignores the parts of the damage that fall outside of the surface.
		///
		/// Damage is double-buffered state, see wl_surface.commit.
		///
		/// The damage rectangle is specified in surface-local coordinates,
		/// where x and y specify the upper left corner of the damage rectangle.
		///
		/// The initial value for pending damage is empty: no damage.
		/// wl_surface.damage adds pending damage: the new pending damage
		/// is the union of old pending damage and the given rectangle.
		///
		/// wl_surface.commit assigns pending damage as the current damage,
		/// and clears pending damage. The server will clear the current
		/// damage as it repaints the surface.
		///
		/// Note! New clients should not use this request. Instead damage can be
		/// posted with wl_surface.damage_buffer which uses buffer coordinates
		/// instead of surface coordinates.
		pub fn damage(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, x, y, width, height); }
		}
		
		/// # request a frame throttling hint
		///
		/// Request a notification when it is a good time to start drawing a new
		/// frame, by creating a frame callback. This is useful for throttling
		/// redrawing operations, and driving animations.
		///
		/// When a client is animating on a wl_surface, it can use the 'frame'
		/// request to get notified when it is a good time to draw and commit the
		/// next frame of animation. If the client commits an update earlier than
		/// that, it is likely that some updates will not make it to the display,
		/// and the client is wasting resources by drawing too often.
		///
		/// The frame request will take effect on the next wl_surface.commit.
		/// The notification will only be posted for one frame unless
		/// requested again. For a wl_surface, the notifications are posted in
		/// the order the frame requests were committed.
		///
		/// The server must send the notifications so that a client
		/// will not send excessive updates, while still allowing
		/// the highest possible update rate for clients that wait for the reply
		/// before drawing again. The server should give some time for the client
		/// to draw and commit after sending the frame callback events to let it
		/// hit the next output refresh.
		///
		/// A server should avoid signaling the frame callbacks if the
		/// surface is not visible in any way, e.g. the surface is off-screen,
		/// or completely obscured by other opaque surfaces.
		///
		/// The object returned by this request will be destroyed by the
		/// compositor after the callback is fired and as such the client must not
		/// attempt to use it after that point.
		///
		/// The callback_data passed in the callback is the current time, in
		/// milliseconds, with an undefined base.
		pub fn frame(
			&self
		) -> Result<Box<WlCallback, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 3, &WL_CALLBACK_INTERFACE, std::ptr::null::<u8>()) as *mut WlCallback };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # set opaque region
		///
		/// This request sets the region of the surface that contains
		/// opaque content.
		///
		/// The opaque region is an optimization hint for the compositor
		/// that lets it optimize the redrawing of content behind opaque
		/// regions.  Setting an opaque region is not required for correct
		/// behaviour, but marking transparent content as opaque will result
		/// in repaint artifacts.
		///
		/// The opaque region is specified in surface-local coordinates.
		///
		/// The compositor ignores the parts of the opaque region that fall
		/// outside of the surface.
		///
		/// Opaque region is double-buffered state, see wl_surface.commit.
		///
		/// wl_surface.set_opaque_region changes the pending opaque region.
		/// wl_surface.commit copies the pending region to the current region.
		/// Otherwise, the pending and current regions are never changed.
		///
		/// The initial value for an opaque region is empty. Setting the pending
		/// opaque region has copy semantics, and the wl_region object can be
		/// destroyed immediately. A NULL wl_region causes the pending opaque
		/// region to be set to empty.
		pub fn set_opaque_region(
			&self,
			region          : Option<&WlRegion>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, region.map_or(std::ptr::null_mut(), |r| r as *const WlRegion as *mut WlRegion)); }
		}
		
		/// # set input region
		///
		/// This request sets the region of the surface that can receive
		/// pointer and touch events.
		///
		/// Input events happening outside of this region will try the next
		/// surface in the server surface stack. The compositor ignores the
		/// parts of the input region that fall outside of the surface.
		///
		/// The input region is specified in surface-local coordinates.
		///
		/// Input region is double-buffered state, see wl_surface.commit.
		///
		/// wl_surface.set_input_region changes the pending input region.
		/// wl_surface.commit copies the pending region to the current region.
		/// Otherwise the pending and current regions are never changed,
		/// except cursor and icon surfaces are special cases, see
		/// wl_pointer.set_cursor and wl_data_device.start_drag.
		///
		/// The initial value for an input region is infinite. That means the
		/// whole surface will accept input. Setting the pending input region
		/// has copy semantics, and the wl_region object can be destroyed
		/// immediately. A NULL wl_region causes the input region to be set
		/// to infinite.
		pub fn set_input_region(
			&self,
			region          : Option<&WlRegion>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, region.map_or(std::ptr::null_mut(), |r| r as *const WlRegion as *mut WlRegion)); }
		}
		
		/// # commit pending surface state
		///
		/// Surface state (input, opaque, and damage regions, attached buffers,
		/// etc.) is double-buffered. Protocol requests modify the pending state,
		/// as opposed to the current state in use by the compositor. A commit
		/// request atomically applies all pending state, replacing the current
		/// state. After commit, the new pending state is as documented for each
		/// related request.
		///
		/// On commit, a pending wl_buffer is applied first, and all other state
		/// second. This means that all coordinates in double-buffered state are
		/// relative to the new wl_buffer coming into use, except for
		/// wl_surface.attach itself. If there is no pending wl_buffer, the
		/// coordinates are relative to the current surface contents.
		///
		/// All requests that need a commit to become effective are documented
		/// to affect double-buffered state.
		///
		/// Other interfaces may add further double-buffered surface state.
		pub fn commit(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6); }
		}
		
		/// # sets the buffer transformation
		///
		/// This request sets an optional transformation on how the compositor
		/// interprets the contents of the buffer attached to the surface. The
		/// accepted values for the transform parameter are the values for
		/// wl_output.transform.
		///
		/// Buffer transform is double-buffered state, see wl_surface.commit.
		///
		/// A newly created surface has its buffer transformation set to normal.
		///
		/// wl_surface.set_buffer_transform changes the pending buffer
		/// transformation. wl_surface.commit copies the pending buffer
		/// transformation to the current one. Otherwise, the pending and current
		/// values are never changed.
		///
		/// The purpose of this request is to allow clients to render content
		/// according to the output transform, thus permitting the compositor to
		/// use certain optimizations even if the display is rotated. Using
		/// hardware overlays and scanning out a client buffer for fullscreen
		/// surfaces are examples of such optimizations. Those optimizations are
		/// highly dependent on the compositor implementation, so the use of this
		/// request should be considered on a case-by-case basis.
		///
		/// Note that if the transform value includes 90 or 270 degree rotation,
		/// the width of the buffer will become the surface height and the height
		/// of the buffer will become the surface width.
		///
		/// If transform is not one of the values from the
		/// wl_output.transform enum the invalid_transform protocol error
		/// is raised.
		pub fn set_buffer_transform(
			&self,
			transform       : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7, transform); }
		}
		
		/// # sets the buffer scaling factor
		///
		/// This request sets an optional scaling factor on how the compositor
		/// interprets the contents of the buffer attached to the window.
		///
		/// Buffer scale is double-buffered state, see wl_surface.commit.
		///
		/// A newly created surface has its buffer scale set to 1.
		///
		/// wl_surface.set_buffer_scale changes the pending buffer scale.
		/// wl_surface.commit copies the pending buffer scale to the current one.
		/// Otherwise, the pending and current values are never changed.
		///
		/// The purpose of this request is to allow clients to supply higher
		/// resolution buffer data for use on high resolution outputs. It is
		/// intended that you pick the same buffer scale as the scale of the
		/// output that the surface is displayed on. This means the compositor
		/// can avoid scaling when rendering the surface on that output.
		///
		/// Note that if the scale is larger than 1, then you have to attach
		/// a buffer that is larger (by a factor of scale in each dimension)
		/// than the desired surface size.
		///
		/// If scale is not positive the invalid_scale protocol error is
		/// raised.
		pub fn set_buffer_scale(
			&self,
			scale           : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 8, scale); }
		}
		
		/// # mark part of the surface damaged using buffer coordinates
		///
		/// This request is used to describe the regions where the pending
		/// buffer is different from the current surface contents, and where
		/// the surface therefore needs to be repainted. The compositor
		/// ignores the parts of the damage that fall outside of the surface.
		///
		/// Damage is double-buffered state, see wl_surface.commit.
		///
		/// The damage rectangle is specified in buffer coordinates,
		/// where x and y specify the upper left corner of the damage rectangle.
		///
		/// The initial value for pending damage is empty: no damage.
		/// wl_surface.damage_buffer adds pending damage: the new pending
		/// damage is the union of old pending damage and the given rectangle.
		///
		/// wl_surface.commit assigns pending damage as the current damage,
		/// and clears pending damage. The server will clear the current
		/// damage as it repaints the surface.
		///
		/// This request differs from wl_surface.damage in only one way - it
		/// takes damage in buffer coordinates instead of surface-local
		/// coordinates. While this generally is more intuitive than surface
		/// coordinates, it is especially desirable when using wp_viewport
		/// or when a drawing library (like EGL) is unaware of buffer scale
		/// and buffer transform.
		///
		/// Note: Because buffer transformation changes and damage requests may
		/// be interleaved in the protocol stream, it is impossible to determine
		/// the actual mapping between surface and buffer damage until
		/// wl_surface.commit time. Therefore, compositors wishing to take both
		/// kinds of damage into account will have to accumulate damage from the
		/// two requests separately and only transform from one to the other
		/// after receiving the wl_surface.commit.
		pub fn damage_buffer(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 9, x, y, width, height); }
		}
	}
	
	
	pub trait WlSurfaceListener: std::any::Any {
		
		/// # surface enters an output
		///
		/// This is emitted whenever a surface's creation, movement, or resizing
		/// results in some part of it being within the scanout region of an
		/// output.
		///
		/// Note that a surface may be overlapping with zero or more outputs.
		fn enter(
			&self,
			proxy: &mut WlSurface,
			output          : Option<&mut WlOutput>,
		);
		
		/// # surface leaves an output
		///
		/// This is emitted whenever a surface's creation, movement, or resizing
		/// results in it no longer having any part of it within the scanout region
		/// of an output.
		///
		/// Clients should not use the number of outputs the surface is on for frame
		/// throttling purposes. The surface might be hidden even if no leave event
		/// has been sent, and the compositor might expect new surface content
		/// updates even if no enter event has been sent. The frame event should be
		/// used instead.
		fn leave(
			&self,
			proxy: &mut WlSurface,
			output          : Option<&mut WlOutput>,
		);
	}
	
	/// # wl_surface error values
	///
	/// These errors can be emitted in response to wl_surface requests.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlSurfaceError {
		/// buffer scale value is invalid
		InvalidScale = 0,
		/// buffer transform value is invalid
		InvalidTransform = 1,
		/// buffer size is invalid
		InvalidSize = 2,
	}
	
	pub static WL_SEAT_INTERFACE: WlInterface = WlInterface {
		name:         "wl_seat\0".as_ptr(),
		version:      7,
		method_count: 4,
		methods:      [
			WlMessage {
				name:      "get_pointer\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_POINTER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_keyboard\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_KEYBOARD_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_touch\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_TOUCH_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "capabilities\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "name\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # group of input devices
	///
	/// A seat is a group of keyboards, pointer and touch devices. This
	/// object is published as a global during start up, or when such a
	/// device is hot plugged.  A seat typically has a pointer and
	/// maintains a keyboard focus and a pointer focus.
	pub struct WlSeat(WlProxy);
	
	impl std::ops::Deref for WlSeat {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlSeat {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlSeat {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlSeat")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlSeat {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlSeatListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlSeatListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.capabilities((proxy as *mut WlSeat).as_mut().unwrap(), (*args.add(0)).u, ),
						1 => listener.name((proxy as *mut WlSeat).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `capabilities` ARGS: capabilities: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `name` ARGS: name: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlSeatListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # return pointer object
		///
		/// The ID provided will be initialized to the wl_pointer interface
		/// for this seat.
		///
		/// This request only takes effect if the seat has the pointer
		/// capability, or has had the pointer capability in the past.
		/// It is a protocol violation to issue this request on a seat that has
		/// never had the pointer capability. The missing_capability error will
		/// be sent in this case.
		pub fn get_pointer(
			&self
		) -> Result<Box<WlPointer, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &WL_POINTER_INTERFACE, std::ptr::null::<u8>()) as *mut WlPointer };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # return keyboard object
		///
		/// The ID provided will be initialized to the wl_keyboard interface
		/// for this seat.
		///
		/// This request only takes effect if the seat has the keyboard
		/// capability, or has had the keyboard capability in the past.
		/// It is a protocol violation to issue this request on a seat that has
		/// never had the keyboard capability. The missing_capability error will
		/// be sent in this case.
		pub fn get_keyboard(
			&self
		) -> Result<Box<WlKeyboard, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &WL_KEYBOARD_INTERFACE, std::ptr::null::<u8>()) as *mut WlKeyboard };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # return touch object
		///
		/// The ID provided will be initialized to the wl_touch interface
		/// for this seat.
		///
		/// This request only takes effect if the seat has the touch
		/// capability, or has had the touch capability in the past.
		/// It is a protocol violation to issue this request on a seat that has
		/// never had the touch capability. The missing_capability error will
		/// be sent in this case.
		pub fn get_touch(
			&self
		) -> Result<Box<WlTouch, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &WL_TOUCH_INTERFACE, std::ptr::null::<u8>()) as *mut WlTouch };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # release the seat object
		///
		/// Using this request a client can tell the server that it is not going to
		/// use the seat object anymore.
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlSeatListener: std::any::Any {
		
		/// # seat capabilities changed
		///
		/// This is emitted whenever a seat gains or loses the pointer,
		/// keyboard or touch capabilities.  The argument is a capability
		/// enum containing the complete set of capabilities this seat has.
		///
		/// When the pointer capability is added, a client may create a
		/// wl_pointer object using the wl_seat.get_pointer request. This object
		/// will receive pointer events until the capability is removed in the
		/// future.
		///
		/// When the pointer capability is removed, a client should destroy the
		/// wl_pointer objects associated with the seat where the capability was
		/// removed, using the wl_pointer.release request. No further pointer
		/// events will be received on these objects.
		///
		/// In some compositors, if a seat regains the pointer capability and a
		/// client has a previously obtained wl_pointer object of version 4 or
		/// less, that object may start sending pointer events again. This
		/// behavior is considered a misinterpretation of the intended behavior
		/// and must not be relied upon by the client. wl_pointer objects of
		/// version 5 or later must not send events if created before the most
		/// recent event notifying the client of an added pointer capability.
		///
		/// The above behavior also applies to wl_keyboard and wl_touch with the
		/// keyboard and touch capabilities, respectively.
		fn capabilities(
			&self,
			proxy: &mut WlSeat,
			capabilities    : u32,
		);
		
		/// # unique identifier for this seat
		///
		/// In a multiseat configuration this can be used by the client to help
		/// identify which physical devices the seat represents. Based on
		/// the seat configuration used by the compositor.
		fn name(
			&self,
			proxy: &mut WlSeat,
			name            : &str,
		);
	}
	
	/// # seat capability bitmask
	///
	/// This is a bitmask of capabilities this seat has; if a member is
	/// set, then it is present on the seat.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlSeatCapability {
		/// the seat has pointer devices
		Pointer = 1,
		/// the seat has one or more keyboards
		Keyboard = 2,
		/// the seat has touch devices
		Touch = 4,
	}
	
	/// # wl_seat error values
	///
	/// These errors can be emitted in response to wl_seat requests.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlSeatError {
		/// get_pointer, get_keyboard or get_touch called on seat without the matching capability
		MissingCapability = 0,
	}
	
	pub static WL_POINTER_INTERFACE: WlInterface = WlInterface {
		name:         "wl_pointer\0".as_ptr(),
		version:      7,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_cursor\0".as_ptr(),
				signature: "u?oii\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  9,
		events:       [
			WlMessage {
				name:      "enter\0".as_ptr(),
				signature: "uoff\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "leave\0".as_ptr(),
				signature: "uo\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "motion\0".as_ptr(),
				signature: "uff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "button\0".as_ptr(),
				signature: "uuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "axis\0".as_ptr(),
				signature: "uuf\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "frame\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "axis_source\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "axis_stop\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "axis_discrete\0".as_ptr(),
				signature: "ui\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # pointer input device
	///
	/// The wl_pointer interface represents one or more input devices,
	/// such as mice, which control the pointer location and pointer_focus
	/// of a seat.
	///
	/// The wl_pointer interface generates motion, enter and leave
	/// events for the surfaces that the pointer is located over,
	/// and button and axis events for button presses, button releases
	/// and scrolling.
	pub struct WlPointer(WlProxy);
	
	impl std::ops::Deref for WlPointer {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlPointer {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlPointer {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlPointer")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlPointer {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlPointerListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlPointerListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.enter((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), (*args.add(2)).f, (*args.add(3)).f, ),
						1 => listener.leave((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), ),
						2 => listener.motion((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f, ),
						3 => listener.button((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, ),
						4 => listener.axis((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).f, ),
						5 => listener.frame((proxy as *mut WlPointer).as_mut().unwrap(), ),
						6 => listener.axis_source((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, ),
						7 => listener.axis_stop((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						8 => listener.axis_discrete((proxy as *mut WlPointer).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).i, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `enter` ARGS: serial: {:?}, surface: {:?}, surface_x: {:?}, surface_y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), (*args.add(2)).f, (*args.add(3)).f),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `leave` ARGS: serial: {:?}, surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut()),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `motion` ARGS: time: {:?}, surface_x: {:?}, surface_y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `button` ARGS: serial: {:?}, time: {:?}, button: {:?}, state: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `axis` ARGS: time: {:?}, axis: {:?}, value: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).f),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `frame` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						6 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `axis_source` ARGS: axis_source: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						7 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `axis_stop` ARGS: time: {:?}, axis: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						8 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `axis_discrete` ARGS: axis: {:?}, discrete: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).i),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlPointerListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the pointer surface
		///
		/// Set the pointer surface, i.e., the surface that contains the
		/// pointer image (cursor). This request gives the surface the role
		/// of a cursor. If the surface already has another role, it raises
		/// a protocol error.
		///
		/// The cursor actually changes only if the pointer
		/// focus for this device is one of the requesting client's surfaces
		/// or the surface parameter is the current pointer surface. If
		/// there was a previous surface set with this request it is
		/// replaced. If surface is NULL, the pointer image is hidden.
		///
		/// The parameters hotspot_x and hotspot_y define the position of
		/// the pointer surface relative to the pointer location. Its
		/// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
		/// where (x, y) are the coordinates of the pointer location, in
		/// surface-local coordinates.
		///
		/// On surface.attach requests to the pointer surface, hotspot_x
		/// and hotspot_y are decremented by the x and y parameters
		/// passed to the request. Attach must be confirmed by
		/// wl_surface.commit as usual.
		///
		/// The hotspot can also be updated by passing the currently set
		/// pointer surface to this request with new values for hotspot_x
		/// and hotspot_y.
		///
		/// The current and pending input regions of the wl_surface are
		/// cleared, and wl_surface.set_input_region is ignored until the
		/// wl_surface is no longer used as the cursor. When the use as a
		/// cursor ends, the current and pending input regions become
		/// undefined, and the wl_surface is unmapped.
		pub fn set_cursor(
			&self,
			serial          : u32,
			surface         : Option<&WlSurface>,
			hotspot_x       : i32,
			hotspot_y       : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, serial, surface.map_or(std::ptr::null_mut(), |r| r as *const WlSurface as *mut WlSurface), hotspot_x, hotspot_y); }
		}
		
		/// # release the pointer object
		///
		/// Using this request a client can tell the server that it is not going to
		/// use the pointer object anymore.
		///
		/// This request destroys the pointer proxy object, so clients must not call
		/// wl_pointer_destroy() after using this request.
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlPointerListener: std::any::Any {
		
		/// # enter event
		///
		/// Notification that this seat's pointer is focused on a certain
		/// surface.
		///
		/// When a seat's focus enters a surface, the pointer image
		/// is undefined and a client should respond to this event by setting
		/// an appropriate pointer image with the set_cursor request.
		fn enter(
			&self,
			proxy: &mut WlPointer,
			serial          : u32,
			surface         : Option<&mut WlSurface>,
			surface_x       : WlFixed,
			surface_y       : WlFixed,
		);
		
		/// # leave event
		///
		/// Notification that this seat's pointer is no longer focused on
		/// a certain surface.
		///
		/// The leave notification is sent before the enter notification
		/// for the new focus.
		fn leave(
			&self,
			proxy: &mut WlPointer,
			serial          : u32,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # pointer motion event
		///
		/// Notification of pointer location change. The arguments
		/// surface_x and surface_y are the location relative to the
		/// focused surface.
		fn motion(
			&self,
			proxy: &mut WlPointer,
			time            : u32,
			surface_x       : WlFixed,
			surface_y       : WlFixed,
		);
		
		/// # pointer button event
		///
		/// Mouse button click and release notifications.
		///
		/// The location of the click is given by the last motion or
		/// enter event.
		/// The time argument is a timestamp with millisecond
		/// granularity, with an undefined base.
		///
		/// The button is a button code as defined in the Linux kernel's
		/// linux/input-event-codes.h header file, e.g. BTN_LEFT.
		///
		/// Any 16-bit button code value is reserved for future additions to the
		/// kernel's event code list. All other button codes above 0xFFFF are
		/// currently undefined but may be used in future versions of this
		/// protocol.
		fn button(
			&self,
			proxy: &mut WlPointer,
			serial          : u32,
			time            : u32,
			button          : u32,
			state           : u32,
		);
		
		/// # axis event
		///
		/// Scroll and other axis notifications.
		///
		/// For scroll events (vertical and horizontal scroll axes), the
		/// value parameter is the length of a vector along the specified
		/// axis in a coordinate space identical to those of motion events,
		/// representing a relative movement along the specified axis.
		///
		/// For devices that support movements non-parallel to axes multiple
		/// axis events will be emitted.
		///
		/// When applicable, for example for touch pads, the server can
		/// choose to emit scroll events where the motion vector is
		/// equivalent to a motion event vector.
		///
		/// When applicable, a client can transform its content relative to the
		/// scroll distance.
		fn axis(
			&self,
			proxy: &mut WlPointer,
			time            : u32,
			axis            : u32,
			value           : WlFixed,
		);
		
		/// # end of a pointer event sequence
		///
		/// Indicates the end of a set of events that logically belong together.
		/// A client is expected to accumulate the data in all events within the
		/// frame before proceeding.
		///
		/// All wl_pointer events before a wl_pointer.frame event belong
		/// logically together. For example, in a diagonal scroll motion the
		/// compositor will send an optional wl_pointer.axis_source event, two
		/// wl_pointer.axis events (horizontal and vertical) and finally a
		/// wl_pointer.frame event. The client may use this information to
		/// calculate a diagonal vector for scrolling.
		///
		/// When multiple wl_pointer.axis events occur within the same frame,
		/// the motion vector is the combined motion of all events.
		/// When a wl_pointer.axis and a wl_pointer.axis_stop event occur within
		/// the same frame, this indicates that axis movement in one axis has
		/// stopped but continues in the other axis.
		/// When multiple wl_pointer.axis_stop events occur within the same
		/// frame, this indicates that these axes stopped in the same instance.
		///
		/// A wl_pointer.frame event is sent for every logical event group,
		/// even if the group only contains a single wl_pointer event.
		/// Specifically, a client may get a sequence: motion, frame, button,
		/// frame, axis, frame, axis_stop, frame.
		///
		/// The wl_pointer.enter and wl_pointer.leave events are logical events
		/// generated by the compositor and not the hardware. These events are
		/// also grouped by a wl_pointer.frame. When a pointer moves from one
		/// surface to another, a compositor should group the
		/// wl_pointer.leave event within the same wl_pointer.frame.
		/// However, a client must not rely on wl_pointer.leave and
		/// wl_pointer.enter being in the same wl_pointer.frame.
		/// Compositor-specific policies may require the wl_pointer.leave and
		/// wl_pointer.enter event being split across multiple wl_pointer.frame
		/// groups.
		fn frame(
			&self,
			proxy: &mut WlPointer,
		);
		
		/// # axis source event
		///
		/// Source information for scroll and other axes.
		///
		/// This event does not occur on its own. It is sent before a
		/// wl_pointer.frame event and carries the source information for
		/// all events within that frame.
		///
		/// The source specifies how this event was generated. If the source is
		/// wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
		/// sent when the user lifts the finger off the device.
		///
		/// If the source is wl_pointer.axis_source.wheel,
		/// wl_pointer.axis_source.wheel_tilt or
		/// wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
		/// or may not be sent. Whether a compositor sends an axis_stop event
		/// for these sources is hardware-specific and implementation-dependent;
		/// clients must not rely on receiving an axis_stop event for these
		/// scroll sources and should treat scroll sequences from these scroll
		/// sources as unterminated by default.
		///
		/// This event is optional. If the source is unknown for a particular
		/// axis event sequence, no event is sent.
		/// Only one wl_pointer.axis_source event is permitted per frame.
		///
		/// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
		/// not guaranteed.
		fn axis_source(
			&self,
			proxy: &mut WlPointer,
			axis_source     : u32,
		);
		
		/// # axis stop event
		///
		/// Stop notification for scroll and other axes.
		///
		/// For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
		/// is sent to notify a client that the axis sequence has terminated.
		/// This enables the client to implement kinetic scrolling.
		/// See the wl_pointer.axis_source documentation for information on when
		/// this event may be generated.
		///
		/// Any wl_pointer.axis events with the same axis_source after this
		/// event should be considered as the start of a new axis motion.
		///
		/// The timestamp is to be interpreted identical to the timestamp in the
		/// wl_pointer.axis event. The timestamp value may be the same as a
		/// preceding wl_pointer.axis event.
		fn axis_stop(
			&self,
			proxy: &mut WlPointer,
			time            : u32,
			axis            : u32,
		);
		
		/// # axis click event
		///
		/// Discrete step information for scroll and other axes.
		///
		/// This event carries the axis value of the wl_pointer.axis event in
		/// discrete steps (e.g. mouse wheel clicks).
		///
		/// This event does not occur on its own, it is coupled with a
		/// wl_pointer.axis event that represents this axis value on a
		/// continuous scale. The protocol guarantees that each axis_discrete
		/// event is always followed by exactly one axis event with the same
		/// axis number within the same wl_pointer.frame. Note that the protocol
		/// allows for other events to occur between the axis_discrete and
		/// its coupled axis event, including other axis_discrete or axis
		/// events.
		///
		/// This event is optional; continuous scrolling devices
		/// like two-finger scrolling on touchpads do not have discrete
		/// steps and do not generate this event.
		///
		/// The discrete value carries the directional information. e.g. a value
		/// of -2 is two steps towards the negative direction of this axis.
		///
		/// The axis number is identical to the axis number in the associated
		/// axis event.
		///
		/// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
		/// not guaranteed.
		fn axis_discrete(
			&self,
			proxy: &mut WlPointer,
			axis            : u32,
			discrete        : i32,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlPointerError {
		/// given wl_surface has another role
		Role = 0,
	}
	
	/// # physical button state
	///
	/// Describes the physical state of a button that produced the button
	/// event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlPointerButtonState {
		/// the button is not pressed
		Released = 0,
		/// the button is pressed
		Pressed = 1,
	}
	
	/// # axis types
	///
	/// Describes the axis types of scroll events.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlPointerAxis {
		/// vertical axis
		VerticalScroll = 0,
		/// horizontal axis
		HorizontalScroll = 1,
	}
	
	/// # axis source types
	///
	/// Describes the source types for axis events. This indicates to the
	/// client how an axis event was physically generated; a client may
	/// adjust the user interface accordingly. For example, scroll events
	/// from a "finger" source may be in a smooth coordinate space with
	/// kinetic scrolling whereas a "wheel" source may be in discrete steps
	/// of a number of lines.
	///
	/// The "continuous" axis source is a device generating events in a
	/// continuous coordinate space, but using something other than a
	/// finger. One example for this source is button-based scrolling where
	/// the vertical motion of a device is converted to scroll events while
	/// a button is held down.
	///
	/// The "wheel tilt" axis source indicates that the actual device is a
	/// wheel but the scroll event is not caused by a rotation but a
	/// (usually sideways) tilt of the wheel.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlPointerAxisSource {
		/// a physical wheel rotation
		Wheel = 0,
		/// finger on a touch surface
		Finger = 1,
		/// continuous coordinate space
		Continuous = 2,
		/// a physical wheel tilt
		WheelTilt = 3,
	}
	
	pub static WL_KEYBOARD_INTERFACE: WlInterface = WlInterface {
		name:         "wl_keyboard\0".as_ptr(),
		version:      7,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  6,
		events:       [
			WlMessage {
				name:      "keymap\0".as_ptr(),
				signature: "uhu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "enter\0".as_ptr(),
				signature: "uoa\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "leave\0".as_ptr(),
				signature: "uo\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "key\0".as_ptr(),
				signature: "uuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "modifiers\0".as_ptr(),
				signature: "uuuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "repeat_info\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # keyboard input device
	///
	/// The wl_keyboard interface represents one or more keyboards
	/// associated with a seat.
	pub struct WlKeyboard(WlProxy);
	
	impl std::ops::Deref for WlKeyboard {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlKeyboard {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlKeyboard {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlKeyboard")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlKeyboard {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlKeyboardListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlKeyboardListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.keymap((proxy as *mut WlKeyboard).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).h, (*args.add(2)).u, ),
						1 => listener.enter((proxy as *mut WlKeyboard).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), (*args.add(2)).a.as_ref().unwrap(), ),
						2 => listener.leave((proxy as *mut WlKeyboard).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), ),
						3 => listener.key((proxy as *mut WlKeyboard).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, ),
						4 => listener.modifiers((proxy as *mut WlKeyboard).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, (*args.add(4)).u, ),
						5 => listener.repeat_info((proxy as *mut WlKeyboard).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `keymap` ARGS: format: {:?}, fd: {:?}, size: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).h, (*args.add(2)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `enter` ARGS: serial: {:?}, surface: {:?}, keys: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut(), (*args.add(2)).a.as_ref().unwrap()),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `leave` ARGS: serial: {:?}, surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut WlSurface).as_mut()),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `key` ARGS: serial: {:?}, time: {:?}, key: {:?}, state: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `modifiers` ARGS: serial: {:?}, mods_depressed: {:?}, mods_latched: {:?}, mods_locked: {:?}, group: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, (*args.add(4)).u),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `repeat_info` ARGS: rate: {:?}, delay: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlKeyboardListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # release the keyboard object
		///
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlKeyboardListener: std::any::Any {
		
		/// # keyboard mapping
		///
		/// This event provides a file descriptor to the client which can be
		/// memory-mapped to provide a keyboard mapping description.
		///
		/// From version 7 onwards, the fd must be mapped with MAP_PRIVATE by
		/// the recipient, as MAP_SHARED may fail.
		fn keymap(
			&self,
			proxy: &mut WlKeyboard,
			format          : u32,
			fd              : RawFd,
			size            : u32,
		);
		
		/// # enter event
		///
		/// Notification that this seat's keyboard focus is on a certain
		/// surface.
		///
		/// The compositor must send the wl_keyboard.modifiers event after this
		/// event.
		fn enter(
			&self,
			proxy: &mut WlKeyboard,
			serial          : u32,
			surface         : Option<&mut WlSurface>,
			keys            : &WlArray,
		);
		
		/// # leave event
		///
		/// Notification that this seat's keyboard focus is no longer on
		/// a certain surface.
		///
		/// The leave notification is sent before the enter notification
		/// for the new focus.
		///
		/// After this event client must assume that all keys, including modifiers,
		/// are lifted and also it must stop key repeating if there's some going on.
		fn leave(
			&self,
			proxy: &mut WlKeyboard,
			serial          : u32,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # key event
		///
		/// A key was pressed or released.
		/// The time argument is a timestamp with millisecond
		/// granularity, with an undefined base.
		///
		/// The key is a platform-specific key code that can be interpreted
		/// by feeding it to the keyboard mapping (see the keymap event).
		///
		/// If this event produces a change in modifiers, then the resulting
		/// wl_keyboard.modifiers event must be sent after this event.
		fn key(
			&self,
			proxy: &mut WlKeyboard,
			serial          : u32,
			time            : u32,
			key             : u32,
			state           : u32,
		);
		
		/// # modifier and group state
		///
		/// Notifies clients that the modifier and/or group state has
		/// changed, and it should update its local state.
		fn modifiers(
			&self,
			proxy: &mut WlKeyboard,
			serial          : u32,
			mods_depressed  : u32,
			mods_latched    : u32,
			mods_locked     : u32,
			group           : u32,
		);
		
		/// # repeat rate and delay
		///
		/// Informs the client about the keyboard's repeat rate and delay.
		///
		/// This event is sent as soon as the wl_keyboard object has been created,
		/// and is guaranteed to be received by the client before any key press
		/// event.
		///
		/// Negative values for either rate or delay are illegal. A rate of zero
		/// will disable any repeating (regardless of the value of delay).
		///
		/// This event can be sent later on as well with a new value if necessary,
		/// so clients should continue listening for the event past the creation
		/// of wl_keyboard.
		fn repeat_info(
			&self,
			proxy: &mut WlKeyboard,
			rate            : i32,
			delay           : i32,
		);
	}
	
	/// # keyboard mapping format
	///
	/// This specifies the format of the keymap provided to the
	/// client with the wl_keyboard.keymap event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlKeyboardKeymapFormat {
		/// no keymap; client must understand how to interpret the raw keycode
		NoKeymap = 0,
		/// libxkbcommon compatible; to determine the xkb keycode, clients must add 8 to the key event keycode
		XkbV1 = 1,
	}
	
	/// # physical key state
	///
	/// Describes the physical state of a key that produced the key event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlKeyboardKeyState {
		/// key is not pressed
		Released = 0,
		/// key is pressed
		Pressed = 1,
	}
	
	pub static WL_TOUCH_INTERFACE: WlInterface = WlInterface {
		name:         "wl_touch\0".as_ptr(),
		version:      7,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  7,
		events:       [
			WlMessage {
				name:      "down\0".as_ptr(),
				signature: "uuoiff\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "up\0".as_ptr(),
				signature: "uui\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "motion\0".as_ptr(),
				signature: "uiff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "frame\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "cancel\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "shape\0".as_ptr(),
				signature: "iff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "orientation\0".as_ptr(),
				signature: "if\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # touchscreen input device
	///
	/// The wl_touch interface represents a touchscreen
	/// associated with a seat.
	///
	/// Touch interactions can consist of one or more contacts.
	/// For each contact, a series of events is generated, starting
	/// with a down event, followed by zero or more motion events,
	/// and ending with an up event. Events relating to the same
	/// contact point can be identified by the ID of the sequence.
	pub struct WlTouch(WlProxy);
	
	impl std::ops::Deref for WlTouch {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlTouch {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlTouch {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlTouch")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlTouch {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlTouchListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlTouchListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.down((proxy as *mut WlTouch).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ((*args.add(2)).o as *mut WlSurface).as_mut(), (*args.add(3)).i, (*args.add(4)).f, (*args.add(5)).f, ),
						1 => listener.up((proxy as *mut WlTouch).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).i, ),
						2 => listener.motion((proxy as *mut WlTouch).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).i, (*args.add(2)).f, (*args.add(3)).f, ),
						3 => listener.frame((proxy as *mut WlTouch).as_mut().unwrap(), ),
						4 => listener.cancel((proxy as *mut WlTouch).as_mut().unwrap(), ),
						5 => listener.shape((proxy as *mut WlTouch).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).f, (*args.add(2)).f, ),
						6 => listener.orientation((proxy as *mut WlTouch).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).f, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `down` ARGS: serial: {:?}, time: {:?}, surface: {:?}, id: {:?}, x: {:?}, y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, ((*args.add(2)).o as *mut WlSurface).as_mut(), (*args.add(3)).i, (*args.add(4)).f, (*args.add(5)).f),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `up` ARGS: serial: {:?}, time: {:?}, id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).i),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `motion` ARGS: time: {:?}, id: {:?}, x: {:?}, y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).i, (*args.add(2)).f, (*args.add(3)).f),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `frame` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `cancel` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `shape` ARGS: id: {:?}, major: {:?}, minor: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).f, (*args.add(2)).f),
						6 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `orientation` ARGS: id: {:?}, orientation: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).f),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlTouchListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # release the touch object
		///
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlTouchListener: std::any::Any {
		
		/// # touch down event and beginning of a touch sequence
		///
		/// A new touch point has appeared on the surface. This touch point is
		/// assigned a unique ID. Future events from this touch point reference
		/// this ID. The ID ceases to be valid after a touch up event and may be
		/// reused in the future.
		fn down(
			&self,
			proxy: &mut WlTouch,
			serial          : u32,
			time            : u32,
			surface         : Option<&mut WlSurface>,
			id              : i32,
			x               : WlFixed,
			y               : WlFixed,
		);
		
		/// # end of a touch event sequence
		///
		/// The touch point has disappeared. No further events will be sent for
		/// this touch point and the touch point's ID is released and may be
		/// reused in a future touch down event.
		fn up(
			&self,
			proxy: &mut WlTouch,
			serial          : u32,
			time            : u32,
			id              : i32,
		);
		
		/// # update of touch point coordinates
		///
		/// A touch point has changed coordinates.
		fn motion(
			&self,
			proxy: &mut WlTouch,
			time            : u32,
			id              : i32,
			x               : WlFixed,
			y               : WlFixed,
		);
		
		/// # end of touch frame event
		///
		/// Indicates the end of a set of events that logically belong together.
		/// A client is expected to accumulate the data in all events within the
		/// frame before proceeding.
		///
		/// A wl_touch.frame terminates at least one event but otherwise no
		/// guarantee is provided about the set of events within a frame. A client
		/// must assume that any state not updated in a frame is unchanged from the
		/// previously known state.
		fn frame(
			&self,
			proxy: &mut WlTouch,
		);
		
		/// # touch session cancelled
		///
		/// Sent if the compositor decides the touch stream is a global
		/// gesture. No further events are sent to the clients from that
		/// particular gesture. Touch cancellation applies to all touch points
		/// currently active on this client's surface. The client is
		/// responsible for finalizing the touch points, future touch points on
		/// this surface may reuse the touch point ID.
		fn cancel(
			&self,
			proxy: &mut WlTouch,
		);
		
		/// # update shape of touch point
		///
		/// Sent when a touchpoint has changed its shape.
		///
		/// This event does not occur on its own. It is sent before a
		/// wl_touch.frame event and carries the new shape information for
		/// any previously reported, or new touch points of that frame.
		///
		/// Other events describing the touch point such as wl_touch.down,
		/// wl_touch.motion or wl_touch.orientation may be sent within the
		/// same wl_touch.frame. A client should treat these events as a single
		/// logical touch point update. The order of wl_touch.shape,
		/// wl_touch.orientation and wl_touch.motion is not guaranteed.
		/// A wl_touch.down event is guaranteed to occur before the first
		/// wl_touch.shape event for this touch ID but both events may occur within
		/// the same wl_touch.frame.
		///
		/// A touchpoint shape is approximated by an ellipse through the major and
		/// minor axis length. The major axis length describes the longer diameter
		/// of the ellipse, while the minor axis length describes the shorter
		/// diameter. Major and minor are orthogonal and both are specified in
		/// surface-local coordinates. The center of the ellipse is always at the
		/// touchpoint location as reported by wl_touch.down or wl_touch.move.
		///
		/// This event is only sent by the compositor if the touch device supports
		/// shape reports. The client has to make reasonable assumptions about the
		/// shape if it did not receive this event.
		fn shape(
			&self,
			proxy: &mut WlTouch,
			id              : i32,
			major           : WlFixed,
			minor           : WlFixed,
		);
		
		/// # update orientation of touch point
		///
		/// Sent when a touchpoint has changed its orientation.
		///
		/// This event does not occur on its own. It is sent before a
		/// wl_touch.frame event and carries the new shape information for
		/// any previously reported, or new touch points of that frame.
		///
		/// Other events describing the touch point such as wl_touch.down,
		/// wl_touch.motion or wl_touch.shape may be sent within the
		/// same wl_touch.frame. A client should treat these events as a single
		/// logical touch point update. The order of wl_touch.shape,
		/// wl_touch.orientation and wl_touch.motion is not guaranteed.
		/// A wl_touch.down event is guaranteed to occur before the first
		/// wl_touch.orientation event for this touch ID but both events may occur
		/// within the same wl_touch.frame.
		///
		/// The orientation describes the clockwise angle of a touchpoint's major
		/// axis to the positive surface y-axis and is normalized to the -180 to
		/// +180 degree range. The granularity of orientation depends on the touch
		/// device, some devices only support binary rotation values between 0 and
		/// 90 degrees.
		///
		/// This event is only sent by the compositor if the touch device supports
		/// orientation reports.
		fn orientation(
			&self,
			proxy: &mut WlTouch,
			id              : i32,
			orientation     : WlFixed,
		);
	}
	
	pub static WL_OUTPUT_INTERFACE: WlInterface = WlInterface {
		name:         "wl_output\0".as_ptr(),
		version:      3,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  4,
		events:       [
			WlMessage {
				name:      "geometry\0".as_ptr(),
				signature: "iiiiissi\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "mode\0".as_ptr(),
				signature: "uiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "scale\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # compositor output region
	///
	/// An output describes part of the compositor geometry.  The
	/// compositor works in the 'compositor coordinate system' and an
	/// output corresponds to a rectangular area in that space that is
	/// actually visible.  This typically corresponds to a monitor that
	/// displays part of the compositor space.  This object is published
	/// as global during start up, or when a monitor is hotplugged.
	pub struct WlOutput(WlProxy);
	
	impl std::ops::Deref for WlOutput {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlOutput {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlOutput {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlOutput")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlOutput {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WlOutputListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WlOutputListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.geometry((proxy as *mut WlOutput).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i, (*args.add(4)).i, std::ffi::CStr::from_ptr((*args.add(5)).s as _).to_str().unwrap(), std::ffi::CStr::from_ptr((*args.add(6)).s as _).to_str().unwrap(), (*args.add(7)).i, ),
						1 => listener.mode((proxy as *mut WlOutput).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i, ),
						2 => listener.done((proxy as *mut WlOutput).as_mut().unwrap(), ),
						3 => listener.scale((proxy as *mut WlOutput).as_mut().unwrap(), (*args.add(0)).i, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `geometry` ARGS: x: {:?}, y: {:?}, physical_width: {:?}, physical_height: {:?}, subpixel: {:?}, make: {:?}, model: {:?}, transform: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i, (*args.add(4)).i, std::ffi::CStr::from_ptr((*args.add(5)).s as _).to_str().unwrap(), std::ffi::CStr::from_ptr((*args.add(6)).s as _).to_str().unwrap(), (*args.add(7)).i),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `mode` ARGS: flags: {:?}, width: {:?}, height: {:?}, refresh: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).i, (*args.add(2)).i, (*args.add(3)).i),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `scale` ARGS: factor: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WlOutputListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # release the output object
		///
		/// Using this request a client can tell the server that it is not going to
		/// use the output object anymore.
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WlOutputListener: std::any::Any {
		
		/// # properties of the output
		///
		/// The geometry event describes geometric properties of the output.
		/// The event is sent when binding to the output object and whenever
		/// any of the properties change.
		///
		/// The physical size can be set to zero if it doesn't make sense for this
		/// output (e.g. for projectors or virtual outputs).
		///
		/// Note: wl_output only advertises partial information about the output
		/// position and identification. Some compositors, for instance those not
		/// implementing a desktop-style output layout or those exposing virtual
		/// outputs, might fake this information. Instead of using x and y, clients
		/// should use xdg_output.logical_position. Instead of using make and model,
		/// clients should use xdg_output.name and xdg_output.description.
		fn geometry(
			&self,
			proxy: &mut WlOutput,
			x               : i32,
			y               : i32,
			physical_width  : i32,
			physical_height : i32,
			subpixel        : i32,
			make            : &str,
			model           : &str,
			transform       : i32,
		);
		
		/// # advertise available modes for the output
		///
		/// The mode event describes an available mode for the output.
		///
		/// The event is sent when binding to the output object and there
		/// will always be one mode, the current mode.  The event is sent
		/// again if an output changes mode, for the mode that is now
		/// current.  In other words, the current mode is always the last
		/// mode that was received with the current flag set.
		///
		/// Non-current modes are deprecated. A compositor can decide to only
		/// advertise the current mode and never send other modes. Clients
		/// should not rely on non-current modes.
		///
		/// The size of a mode is given in physical hardware units of
		/// the output device. This is not necessarily the same as
		/// the output size in the global compositor space. For instance,
		/// the output may be scaled, as described in wl_output.scale,
		/// or transformed, as described in wl_output.transform. Clients
		/// willing to retrieve the output size in the global compositor
		/// space should use xdg_output.logical_size instead.
		///
		/// The vertical refresh rate can be set to zero if it doesn't make
		/// sense for this output (e.g. for virtual outputs).
		///
		/// Clients should not use the refresh rate to schedule frames. Instead,
		/// they should use the wl_surface.frame event or the presentation-time
		/// protocol.
		///
		/// Note: this information is not always meaningful for all outputs. Some
		/// compositors, such as those exposing virtual outputs, might fake the
		/// refresh rate or the size.
		fn mode(
			&self,
			proxy: &mut WlOutput,
			flags           : u32,
			width           : i32,
			height          : i32,
			refresh         : i32,
		);
		
		/// # sent all information about output
		///
		/// This event is sent after all other properties have been
		/// sent after binding to the output object and after any
		/// other property changes done after that. This allows
		/// changes to the output properties to be seen as
		/// atomic, even if they happen via multiple events.
		fn done(
			&self,
			proxy: &mut WlOutput,
		);
		
		/// # output scaling properties
		///
		/// This event contains scaling geometry information
		/// that is not in the geometry event. It may be sent after
		/// binding the output object or if the output scale changes
		/// later. If it is not sent, the client should assume a
		/// scale of 1.
		///
		/// A scale larger than 1 means that the compositor will
		/// automatically scale surface buffers by this amount
		/// when rendering. This is used for very high resolution
		/// displays where applications rendering at the native
		/// resolution would be too small to be legible.
		///
		/// It is intended that scaling aware clients track the
		/// current output of a surface, and if it is on a scaled
		/// output it should use wl_surface.set_buffer_scale with
		/// the scale of the output. That way the compositor can
		/// avoid scaling the surface, and the client can supply
		/// a higher detail image.
		fn scale(
			&self,
			proxy: &mut WlOutput,
			factor          : i32,
		);
	}
	
	/// # subpixel geometry information
	///
	/// This enumeration describes how the physical
	/// pixels on an output are laid out.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlOutputSubpixel {
		/// unknown geometry
		Unknown = 0,
		/// no geometry
		None = 1,
		/// horizontal RGB
		HorizontalRgb = 2,
		/// horizontal BGR
		HorizontalBgr = 3,
		/// vertical RGB
		VerticalRgb = 4,
		/// vertical BGR
		VerticalBgr = 5,
	}
	
	/// # transform from framebuffer to output
	///
	/// This describes the transform that a compositor will apply to a
	/// surface to compensate for the rotation or mirroring of an
	/// output device.
	///
	/// The flipped values correspond to an initial flip around a
	/// vertical axis followed by rotation.
	///
	/// The purpose is mainly to allow clients to render accordingly and
	/// tell the compositor, so that for fullscreen surfaces, the
	/// compositor will still be able to scan out directly from client
	/// surfaces.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlOutputTransform {
		/// no transform
		Normal = 0,
		/// 90 degrees counter-clockwise
		_90 = 1,
		/// 180 degrees counter-clockwise
		_180 = 2,
		/// 270 degrees counter-clockwise
		_270 = 3,
		/// 180 degree flip around a vertical axis
		Flipped = 4,
		/// flip and rotate 90 degrees counter-clockwise
		Flipped90 = 5,
		/// flip and rotate 180 degrees counter-clockwise
		Flipped180 = 6,
		/// flip and rotate 270 degrees counter-clockwise
		Flipped270 = 7,
	}
	
	/// # mode information
	///
	/// These flags describe properties of an output mode.
	/// They are used in the flags bitfield of the mode event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlOutputMode {
		/// indicates this is the current mode
		Current = 0x1,
		/// indicates this is the preferred mode
		Preferred = 0x2,
	}
	
	pub static WL_REGION_INTERFACE: WlInterface = WlInterface {
		name:         "wl_region\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "add\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "subtract\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # region interface
	///
	/// A region object describes an area.
	///
	/// Region objects are used to describe the opaque and input
	/// regions of a surface.
	pub struct WlRegion(WlProxy);
	
	impl std::ops::Deref for WlRegion {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlRegion {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlRegion {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlRegion")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlRegion {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy region
		///
		/// Destroy the region.  This will invalidate the object ID.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # add rectangle to region
		///
		/// Add the specified rectangle to the region.
		pub fn add(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, x, y, width, height); }
		}
		
		/// # subtract rectangle from region
		///
		/// Subtract the specified rectangle from the region.
		pub fn subtract(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, x, y, width, height); }
		}
	}
	
	
	pub static WL_SUBCOMPOSITOR_INTERFACE: WlInterface = WlInterface {
		name:         "wl_subcompositor\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_subsurface\0".as_ptr(),
				signature: "noo\0".as_ptr(),
				types:     [&WL_SUBSURFACE_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # sub-surface compositing
	///
	/// The global interface exposing sub-surface compositing capabilities.
	/// A wl_surface, that has sub-surfaces associated, is called the
	/// parent surface. Sub-surfaces can be arbitrarily nested and create
	/// a tree of sub-surfaces.
	///
	/// The root surface in a tree of sub-surfaces is the main
	/// surface. The main surface cannot be a sub-surface, because
	/// sub-surfaces must always have a parent.
	///
	/// A main surface with its sub-surfaces forms a (compound) window.
	/// For window management purposes, this set of wl_surface objects is
	/// to be considered as a single window, and it should also behave as
	/// such.
	///
	/// The aim of sub-surfaces is to offload some of the compositing work
	/// within a window from clients to the compositor. A prime example is
	/// a video player with decorations and video in separate wl_surface
	/// objects. This should allow the compositor to pass YUV video buffer
	/// processing to dedicated overlay hardware when possible.
	pub struct WlSubcompositor(WlProxy);
	
	impl std::ops::Deref for WlSubcompositor {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlSubcompositor {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlSubcompositor {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlSubcompositor")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlSubcompositor {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # unbind from the subcompositor interface
		///
		/// Informs the server that the client will not be using this
		/// protocol object anymore. This does not affect any other
		/// objects, wl_subsurface objects included.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # give a surface the role sub-surface
		///
		/// Create a sub-surface interface for the given surface, and
		/// associate it with the given parent surface. This turns a
		/// plain wl_surface into a sub-surface.
		///
		/// The to-be sub-surface must not already have another role, and it
		/// must not have an existing wl_subsurface object. Otherwise a protocol
		/// error is raised.
		///
		/// Adding sub-surfaces to a parent is a double-buffered operation on the
		/// parent (see wl_surface.commit). The effect of adding a sub-surface
		/// becomes visible on the next time the state of the parent surface is
		/// applied.
		///
		/// This request modifies the behaviour of wl_surface.commit request on
		/// the sub-surface, see the documentation on wl_subsurface interface.
		pub fn get_subsurface(
			&self,
			surface         : &WlSurface,
			parent          : &WlSurface
		) -> Result<Box<WlSubsurface, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &WL_SUBSURFACE_INTERFACE, std::ptr::null::<u8>(), surface, parent) as *mut WlSubsurface };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlSubcompositorError {
		/// the to-be sub-surface is invalid
		BadSurface = 0,
	}
	
	pub static WL_SUBSURFACE_INTERFACE: WlInterface = WlInterface {
		name:         "wl_subsurface\0".as_ptr(),
		version:      1,
		method_count: 6,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_position\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "place_above\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "place_below\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_sync\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_desync\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # sub-surface interface to a wl_surface
	///
	/// An additional interface to a wl_surface object, which has been
	/// made a sub-surface. A sub-surface has one parent surface. A
	/// sub-surface's size and position are not limited to that of the parent.
	/// Particularly, a sub-surface is not automatically clipped to its
	/// parent's area.
	///
	/// A sub-surface becomes mapped, when a non-NULL wl_buffer is applied
	/// and the parent surface is mapped. The order of which one happens
	/// first is irrelevant. A sub-surface is hidden if the parent becomes
	/// hidden, or if a NULL wl_buffer is applied. These rules apply
	/// recursively through the tree of surfaces.
	///
	/// The behaviour of a wl_surface.commit request on a sub-surface
	/// depends on the sub-surface's mode. The possible modes are
	/// synchronized and desynchronized, see methods
	/// wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized
	/// mode caches the wl_surface state to be applied when the parent's
	/// state gets applied, and desynchronized mode applies the pending
	/// wl_surface state directly. A sub-surface is initially in the
	/// synchronized mode.
	///
	/// Sub-surfaces also have another kind of state, which is managed by
	/// wl_subsurface requests, as opposed to wl_surface requests. This
	/// state includes the sub-surface position relative to the parent
	/// surface (wl_subsurface.set_position), and the stacking order of
	/// the parent and its sub-surfaces (wl_subsurface.place_above and
	/// .place_below). This state is applied when the parent surface's
	/// wl_surface state is applied, regardless of the sub-surface's mode.
	/// As the exception, set_sync and set_desync are effective immediately.
	///
	/// The main surface can be thought to be always in desynchronized mode,
	/// since it does not have a parent in the sub-surfaces sense.
	///
	/// Even if a sub-surface is in desynchronized mode, it will behave as
	/// in synchronized mode, if its parent surface behaves as in
	/// synchronized mode. This rule is applied recursively throughout the
	/// tree of surfaces. This means, that one can set a sub-surface into
	/// synchronized mode, and then assume that all its child and grand-child
	/// sub-surfaces are synchronized, too, without explicitly setting them.
	///
	/// If the wl_surface associated with the wl_subsurface is destroyed, the
	/// wl_subsurface object becomes inert. Note, that destroying either object
	/// takes effect immediately. If you need to synchronize the removal
	/// of a sub-surface to the parent surface update, unmap the sub-surface
	/// first by attaching a NULL wl_buffer, update parent, and then destroy
	/// the sub-surface.
	///
	/// If the parent wl_surface object is destroyed, the sub-surface is
	/// unmapped.
	pub struct WlSubsurface(WlProxy);
	
	impl std::ops::Deref for WlSubsurface {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WlSubsurface {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WlSubsurface {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WlSubsurface")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WlSubsurface {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # remove sub-surface interface
		///
		/// The sub-surface interface is removed from the wl_surface object
		/// that was turned into a sub-surface with a
		/// wl_subcompositor.get_subsurface request. The wl_surface's association
		/// to the parent is deleted, and the wl_surface loses its role as
		/// a sub-surface. The wl_surface is unmapped immediately.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # reposition the sub-surface
		///
		/// This schedules a sub-surface position change.
		/// The sub-surface will be moved so that its origin (top left
		/// corner pixel) will be at the location x, y of the parent surface
		/// coordinate system. The coordinates are not restricted to the parent
		/// surface area. Negative values are allowed.
		///
		/// The scheduled coordinates will take effect whenever the state of the
		/// parent surface is applied. When this happens depends on whether the
		/// parent surface is in synchronized mode or not. See
		/// wl_subsurface.set_sync and wl_subsurface.set_desync for details.
		///
		/// If more than one set_position request is invoked by the client before
		/// the commit of the parent surface, the position of a new request always
		/// replaces the scheduled position from any previous request.
		///
		/// The initial position is 0, 0.
		pub fn set_position(
			&self,
			x               : i32,
			y               : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, x, y); }
		}
		
		/// # restack the sub-surface
		///
		/// This sub-surface is taken from the stack, and put back just
		/// above the reference surface, changing the z-order of the sub-surfaces.
		/// The reference surface must be one of the sibling surfaces, or the
		/// parent surface. Using any other surface, including this sub-surface,
		/// will cause a protocol error.
		///
		/// The z-order is double-buffered. Requests are handled in order and
		/// applied immediately to a pending state. The final pending state is
		/// copied to the active state the next time the state of the parent
		/// surface is applied. When this happens depends on whether the parent
		/// surface is in synchronized mode or not. See wl_subsurface.set_sync and
		/// wl_subsurface.set_desync for details.
		///
		/// A new sub-surface is initially added as the top-most in the stack
		/// of its siblings and parent.
		pub fn place_above(
			&self,
			sibling         : &WlSurface
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, sibling); }
		}
		
		/// # restack the sub-surface
		///
		/// The sub-surface is placed just below the reference surface.
		/// See wl_subsurface.place_above.
		pub fn place_below(
			&self,
			sibling         : &WlSurface
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, sibling); }
		}
		
		/// # set sub-surface to synchronized mode
		///
		/// Change the commit behaviour of the sub-surface to synchronized
		/// mode, also described as the parent dependent mode.
		///
		/// In synchronized mode, wl_surface.commit on a sub-surface will
		/// accumulate the committed state in a cache, but the state will
		/// not be applied and hence will not change the compositor output.
		/// The cached state is applied to the sub-surface immediately after
		/// the parent surface's state is applied. This ensures atomic
		/// updates of the parent and all its synchronized sub-surfaces.
		/// Applying the cached state will invalidate the cache, so further
		/// parent surface commits do not (re-)apply old state.
		///
		/// See wl_subsurface for the recursive effect of this mode.
		pub fn set_sync(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4); }
		}
		
		/// # set sub-surface to desynchronized mode
		///
		/// Change the commit behaviour of the sub-surface to desynchronized
		/// mode, also described as independent or freely running mode.
		///
		/// In desynchronized mode, wl_surface.commit on a sub-surface will
		/// apply the pending state directly, without caching, as happens
		/// normally with a wl_surface. Calling wl_surface.commit on the
		/// parent surface has no effect on the sub-surface's wl_surface
		/// state. This mode allows a sub-surface to be updated on its own.
		///
		/// If cached state exists when wl_surface.commit is called in
		/// desynchronized mode, the pending state is added to the cached
		/// state, and applied as a whole. This invalidates the cache.
		///
		/// Note: even if a sub-surface is set to desynchronized, a parent
		/// sub-surface may override it to behave as synchronized. For details,
		/// see wl_subsurface.
		///
		/// If a surface's parent surface behaves as desynchronized, then
		/// the cached state is applied on set_desync.
		pub fn set_desync(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5); }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WlSubsurfaceError {
		/// wl_surface is not a sibling or the parent
		BadSurface = 0,
	}
}

/// # protocol for relative pointer motion events
///
/// This protocol specifies a set of interfaces used for making clients able to
/// receive relative pointer events not obstructed by barriers (such as the
/// monitor edge or other pointer barriers).
///
/// To start receiving relative pointer events, a client must first bind the
/// global interface "wp_relative_pointer_manager" which, if a compositor
/// supports relative pointer motion events, is exposed by the registry. After
/// having created the relative pointer manager proxy object, the client uses
/// it to create the actual relative pointer object using the
/// "get_relative_pointer" request given a wl_pointer. The relative pointer
/// motion events will then, when applicable, be transmitted via the proxy of
/// the newly created relative pointer object. See the documentation of the
/// relative pointer interface for more details.
///
/// Warning! The protocol described in this file is experimental and backward
/// incompatible changes may be made. Backward compatible changes may be added
/// together with the corresponding interface version bump. Backward
/// incompatible changes are done by bumping the version number in the protocol
/// and interface names and resetting the interface version. Once the protocol
/// is to be declared stable, the 'z' prefix and the version number in the
/// protocol and interface names are removed and the interface version number is
/// reset.
pub use relative_pointer_unstable_v1::*;
mod relative_pointer_unstable_v1 {
	use crate::*;
	
	// Copyright © 2014      Jonas Ådahl
	// Copyright © 2015      Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_RELATIVE_POINTER_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_relative_pointer_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_relative_pointer\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_RELATIVE_POINTER_V1_INTERFACE as _, &WL_POINTER_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # get relative pointer objects
	///
	/// A global interface used for getting the relative pointer object for a
	/// given pointer.
	pub struct ZwpRelativePointerManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpRelativePointerManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpRelativePointerManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpRelativePointerManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpRelativePointerManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpRelativePointerManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the relative pointer manager object
		///
		/// Used by the client to notify the server that it will no longer use this
		/// relative pointer manager object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # get a relative pointer object
		///
		/// Create a relative pointer interface given a wl_pointer object. See the
		/// wp_relative_pointer interface for more details.
		pub fn get_relative_pointer(
			&self,
			pointer         : &WlPointer
		) -> Result<Box<ZwpRelativePointerV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_RELATIVE_POINTER_V1_INTERFACE, std::ptr::null::<u8>(), pointer) as *mut ZwpRelativePointerV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZWP_RELATIVE_POINTER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_relative_pointer_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "relative_motion\0".as_ptr(),
				signature: "uuffff\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # relative pointer object
	///
	/// A wp_relative_pointer object is an extension to the wl_pointer interface
	/// used for emitting relative pointer events. It shares the same focus as
	/// wl_pointer objects of the same seat and will only emit events when it has
	/// focus.
	pub struct ZwpRelativePointerV1(WlProxy);
	
	impl std::ops::Deref for ZwpRelativePointerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpRelativePointerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpRelativePointerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpRelativePointerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpRelativePointerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpRelativePointerV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpRelativePointerV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.relative_motion((proxy as *mut ZwpRelativePointerV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).f, (*args.add(3)).f, (*args.add(4)).f, (*args.add(5)).f, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `relative_motion` ARGS: utime_hi: {:?}, utime_lo: {:?}, dx: {:?}, dy: {:?}, dx_unaccel: {:?}, dy_unaccel: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).f, (*args.add(3)).f, (*args.add(4)).f, (*args.add(5)).f),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpRelativePointerV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # release the relative pointer object
		///
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpRelativePointerV1Listener: std::any::Any {
		
		/// # relative pointer motion
		///
		/// Relative x/y pointer motion from the pointer of the seat associated with
		/// this object.
		///
		/// A relative motion is in the same dimension as regular wl_pointer motion
		/// events, except they do not represent an absolute position. For example,
		/// moving a pointer from (x, y) to (x', y') would have the equivalent
		/// relative motion (x' - x, y' - y). If a pointer motion caused the
		/// absolute pointer position to be clipped by for example the edge of the
		/// monitor, the relative motion is unaffected by the clipping and will
		/// represent the unclipped motion.
		///
		/// This event also contains non-accelerated motion deltas. The
		/// non-accelerated delta is, when applicable, the regular pointer motion
		/// delta as it was before having applied motion acceleration and other
		/// transformations such as normalization.
		///
		/// Note that the non-accelerated delta does not represent 'raw' events as
		/// they were read from some device. Pointer motion acceleration is device-
		/// and configuration-specific and non-accelerated deltas and accelerated
		/// deltas may have the same value on some devices.
		///
		/// Relative motions are not coupled to wl_pointer.motion events, and can be
		/// sent in combination with such events, but also independently. There may
		/// also be scenarios where wl_pointer.motion is sent, but there is no
		/// relative motion. The order of an absolute and relative motion event
		/// originating from the same physical motion is not guaranteed.
		///
		/// If the client needs button events or focus state, it can receive them
		/// from a wl_pointer object of the same seat that the wp_relative_pointer
		/// object is associated with.
		fn relative_motion(
			&self,
			proxy: &mut ZwpRelativePointerV1,
			utime_hi        : u32,
			utime_lo        : u32,
			dx              : WlFixed,
			dy              : WlFixed,
			dx_unaccel      : WlFixed,
			dy_unaccel      : WlFixed,
		);
	}
}
pub use text_input_unstable_v1::*;
mod text_input_unstable_v1 {
	use crate::*;
	
	// Copyright © 2012, 2013 Intel Corporation
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_TEXT_INPUT_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_text_input_v1\0".as_ptr(),
		version:      1,
		method_count: 11,
		methods:      [
			WlMessage {
				name:      "activate\0".as_ptr(),
				signature: "oo\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "deactivate\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "show_input_panel\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "hide_input_panel\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "reset\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_surrounding_text\0".as_ptr(),
				signature: "suu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_content_type\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_cursor_rectangle\0".as_ptr(),
				signature: "iiii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_preferred_language\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "commit_state\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "invoke_action\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  13,
		events:       [
			WlMessage {
				name:      "enter\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "leave\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "modifiers_map\0".as_ptr(),
				signature: "a\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "input_panel_state\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "preedit_string\0".as_ptr(),
				signature: "uss\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "preedit_styling\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "preedit_cursor\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "commit_string\0".as_ptr(),
				signature: "us\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "cursor_position\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "delete_surrounding_text\0".as_ptr(),
				signature: "iu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "keysym\0".as_ptr(),
				signature: "uuuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "language\0".as_ptr(),
				signature: "us\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "text_direction\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # text input
	///
	/// An object used for text input. Adds support for text input and input
	/// methods to applications. A text_input object is created from a
	/// wl_text_input_manager and corresponds typically to a text entry in an
	/// application.
	///
	/// Requests are used to activate/deactivate the text_input object and set
	/// state information like surrounding and selected text or the content type.
	/// The information about entered text is sent to the text_input object via
	/// the pre-edit and commit events. Using this interface removes the need
	/// for applications to directly process hardware key events and compose text
	/// out of them.
	///
	/// Text is generally UTF-8 encoded, indices and lengths are in bytes.
	///
	/// Serials are used to synchronize the state between the text input and
	/// an input method. New serials are sent by the text input in the
	/// commit_state request and are used by the input method to indicate
	/// the known text input state in events like preedit_string, commit_string,
	/// and keysym. The text input can then ignore events from the input method
	/// which are based on an outdated state (for example after a reset).
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZwpTextInputV1(WlProxy);
	
	impl std::ops::Deref for ZwpTextInputV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTextInputV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTextInputV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTextInputV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTextInputV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTextInputV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTextInputV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.enter((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), ((*args.add(0)).o as *mut WlSurface).as_mut(), ),
						1 => listener.leave((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), ),
						2 => listener.modifiers_map((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).a.as_ref().unwrap(), ),
						3 => listener.input_panel_state((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).u, ),
						4 => listener.preedit_string((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap(), std::ffi::CStr::from_ptr((*args.add(2)).s as _).to_str().unwrap(), ),
						5 => listener.preedit_styling((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, ),
						6 => listener.preedit_cursor((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).i, ),
						7 => listener.commit_string((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap(), ),
						8 => listener.cursor_position((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, ),
						9 => listener.delete_surrounding_text((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).u, ),
						10 => listener.keysym((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, (*args.add(4)).u, ),
						11 => listener.language((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap(), ),
						12 => listener.text_direction((proxy as *mut ZwpTextInputV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `enter` ARGS: surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlSurface).as_mut()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `leave` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `modifiers_map` ARGS: map: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).a.as_ref().unwrap()),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `input_panel_state` ARGS: state: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `preedit_string` ARGS: serial: {:?}, text: {:?}, commit: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap(), std::ffi::CStr::from_ptr((*args.add(2)).s as _).to_str().unwrap()),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `preedit_styling` ARGS: index: {:?}, length: {:?}, style: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u),
						6 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `preedit_cursor` ARGS: index: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i),
						7 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `commit_string` ARGS: serial: {:?}, text: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap()),
						8 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `cursor_position` ARGS: index: {:?}, anchor: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i),
						9 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `delete_surrounding_text` ARGS: index: {:?}, length: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).u),
						10 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `keysym` ARGS: serial: {:?}, time: {:?}, sym: {:?}, state: {:?}, modifiers: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, (*args.add(4)).u),
						11 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `language` ARGS: serial: {:?}, language: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, std::ffi::CStr::from_ptr((*args.add(1)).s as _).to_str().unwrap()),
						12 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `text_direction` ARGS: serial: {:?}, direction: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTextInputV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # request activation
		///
		/// Requests the text_input object to be activated (typically when the
		/// text entry gets focus).
		///
		/// The seat argument is a wl_seat which maintains the focus for this
		/// activation. The surface argument is a wl_surface assigned to the
		/// text_input object and tracked for focus lost. The enter event
		/// is emitted on successful activation.
		pub fn activate(
			&self,
			seat            : &WlSeat,
			surface         : &WlSurface
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, seat, surface); }
		}
		
		/// # request deactivation
		///
		/// Requests the text_input object to be deactivated (typically when the
		/// text entry lost focus). The seat argument is a wl_seat which was used
		/// for activation.
		pub fn deactivate(
			&self,
			seat            : &WlSeat
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, seat); }
		}
		
		/// # show input panels
		///
		/// Requests input panels (virtual keyboard) to show.
		pub fn show_input_panel(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2); }
		}
		
		/// # hide input panels
		///
		/// Requests input panels (virtual keyboard) to hide.
		pub fn hide_input_panel(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3); }
		}
		
		/// # reset
		///
		/// Should be called by an editor widget when the input state should be
		/// reset, for example after the text was changed outside of the normal
		/// input method flow.
		pub fn reset(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4); }
		}
		
		/// # sets the surrounding text
		///
		/// Sets the plain surrounding text around the input position. Text is
		/// UTF-8 encoded. Cursor is the byte offset within the
		/// surrounding text. Anchor is the byte offset of the
		/// selection anchor within the surrounding text. If there is no selected
		/// text anchor, then it is the same as cursor.
		pub fn set_surrounding_text(
			&self,
			text            : &str,
			cursor          : u32,
			anchor          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, text.as_ptr(), cursor, anchor); }
		}
		
		/// # set content purpose and hint
		///
		/// Sets the content purpose and content hint. While the purpose is the
		/// basic purpose of an input field, the hint flags allow to modify some
		/// of the behavior.
		///
		/// When no content type is explicitly set, a normal content purpose with
		/// default hints (auto completion, auto correction, auto capitalization)
		/// should be assumed.
		pub fn set_content_type(
			&self,
			hint            : u32,
			purpose         : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, hint, purpose); }
		}
		pub fn set_cursor_rectangle(
			&self,
			x               : i32,
			y               : i32,
			width           : i32,
			height          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7, x, y, width, height); }
		}
		
		/// # sets preferred language
		///
		/// Sets a specific language. This allows for example a virtual keyboard to
		/// show a language specific layout. The "language" argument is an RFC-3066
		/// format language tag.
		///
		/// It could be used for example in a word processor to indicate the
		/// language of the currently edited document or in an instant message
		/// application which tracks languages of contacts.
		pub fn set_preferred_language(
			&self,
			language        : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 8, language.as_ptr()); }
		}
		pub fn commit_state(
			&self,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 9, serial); }
		}
		pub fn invoke_action(
			&self,
			button          : u32,
			index           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 10, button, index); }
		}
	}
	
	
	pub trait ZwpTextInputV1Listener: std::any::Any {
		
		/// # enter event
		///
		/// Notify the text_input object when it received focus. Typically in
		/// response to an activate request.
		fn enter(
			&self,
			proxy: &mut ZwpTextInputV1,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # leave event
		///
		/// Notify the text_input object when it lost focus. Either in response
		/// to a deactivate request or when the assigned surface lost focus or was
		/// destroyed.
		fn leave(
			&self,
			proxy: &mut ZwpTextInputV1,
		);
		
		/// # modifiers map
		///
		/// Transfer an array of 0-terminated modifier names. The position in
		/// the array is the index of the modifier as used in the modifiers
		/// bitmask in the keysym event.
		fn modifiers_map(
			&self,
			proxy: &mut ZwpTextInputV1,
			map             : &WlArray,
		);
		
		/// # state of the input panel
		///
		/// Notify when the visibility state of the input panel changed.
		fn input_panel_state(
			&self,
			proxy: &mut ZwpTextInputV1,
			state           : u32,
		);
		
		/// # pre-edit
		///
		/// Notify when a new composing text (pre-edit) should be set around the
		/// current cursor position. Any previously set composing text should
		/// be removed.
		///
		/// The commit text can be used to replace the preedit text on reset
		/// (for example on unfocus).
		///
		/// The text input should also handle all preedit_style and preedit_cursor
		/// events occurring directly before preedit_string.
		fn preedit_string(
			&self,
			proxy: &mut ZwpTextInputV1,
			serial          : u32,
			text            : &str,
			commit          : &str,
		);
		
		/// # pre-edit styling
		///
		/// Sets styling information on composing text. The style is applied for
		/// length bytes from index relative to the beginning of the composing
		/// text (as byte offset). Multiple styles can
		/// be applied to a composing text by sending multiple preedit_styling
		/// events.
		///
		/// This event is handled as part of a following preedit_string event.
		fn preedit_styling(
			&self,
			proxy: &mut ZwpTextInputV1,
			index           : u32,
			length          : u32,
			style           : u32,
		);
		
		/// # pre-edit cursor
		///
		/// Sets the cursor position inside the composing text (as byte
		/// offset) relative to the start of the composing text. When index is a
		/// negative number no cursor is shown.
		///
		/// This event is handled as part of a following preedit_string event.
		fn preedit_cursor(
			&self,
			proxy: &mut ZwpTextInputV1,
			index           : i32,
		);
		
		/// # commit
		///
		/// Notify when text should be inserted into the editor widget. The text to
		/// commit could be either just a single character after a key press or the
		/// result of some composing (pre-edit). It could also be an empty text
		/// when some text should be removed (see delete_surrounding_text) or when
		/// the input cursor should be moved (see cursor_position).
		///
		/// Any previously set composing text should be removed.
		fn commit_string(
			&self,
			proxy: &mut ZwpTextInputV1,
			serial          : u32,
			text            : &str,
		);
		
		/// # set cursor to new position
		///
		/// Notify when the cursor or anchor position should be modified.
		///
		/// This event should be handled as part of a following commit_string
		/// event.
		fn cursor_position(
			&self,
			proxy: &mut ZwpTextInputV1,
			index           : i32,
			anchor          : i32,
		);
		
		/// # delete surrounding text
		///
		/// Notify when the text around the current cursor position should be
		/// deleted.
		///
		/// Index is relative to the current cursor (in bytes).
		/// Length is the length of deleted text (in bytes).
		///
		/// This event should be handled as part of a following commit_string
		/// event.
		fn delete_surrounding_text(
			&self,
			proxy: &mut ZwpTextInputV1,
			index           : i32,
			length          : u32,
		);
		
		/// # keysym
		///
		/// Notify when a key event was sent. Key events should not be used
		/// for normal text input operations, which should be done with
		/// commit_string, delete_surrounding_text, etc. The key event follows
		/// the wl_keyboard key event convention. Sym is an XKB keysym, state a
		/// wl_keyboard key_state. Modifiers are a mask for effective modifiers
		/// (where the modifier indices are set by the modifiers_map event)
		fn keysym(
			&self,
			proxy: &mut ZwpTextInputV1,
			serial          : u32,
			time            : u32,
			sym             : u32,
			state           : u32,
			modifiers       : u32,
		);
		
		/// # language
		///
		/// Sets the language of the input text. The "language" argument is an
		/// RFC-3066 format language tag.
		fn language(
			&self,
			proxy: &mut ZwpTextInputV1,
			serial          : u32,
			language        : &str,
		);
		
		/// # text direction
		///
		/// Sets the text direction of input text.
		///
		/// It is mainly needed for showing an input cursor on the correct side of
		/// the editor when there is no input done yet and making sure neutral
		/// direction text is laid out properly.
		fn text_direction(
			&self,
			proxy: &mut ZwpTextInputV1,
			serial          : u32,
			direction       : u32,
		);
	}
	
	/// # content hint
	///
	/// Content hint is a bitmask to allow to modify the behavior of the text
	/// input.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTextInputV1ContentHint {
		/// no special behaviour
		None = 0x0,
		/// auto completion, correction and capitalization
		Default = 0x7,
		/// hidden and sensitive text
		Password = 0xc0,
		/// suggest word completions
		AutoCompletion = 0x1,
		/// suggest word corrections
		AutoCorrection = 0x2,
		/// switch to uppercase letters at the start of a sentence
		AutoCapitalization = 0x4,
		/// prefer lowercase letters
		Lowercase = 0x8,
		/// prefer uppercase letters
		Uppercase = 0x10,
		/// prefer casing for titles and headings (can be language dependent)
		Titlecase = 0x20,
		/// characters should be hidden
		HiddenText = 0x40,
		/// typed text should not be stored
		SensitiveData = 0x80,
		/// just latin characters should be entered
		Latin = 0x100,
		/// the text input is multiline
		Multiline = 0x200,
	}
	
	/// # content purpose
	///
	/// The content purpose allows to specify the primary purpose of a text
	/// input.
	///
	/// This allows an input method to show special purpose input panels with
	/// extra characters or to disallow some characters.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTextInputV1ContentPurpose {
		/// default input, allowing all characters
		Normal = 0,
		/// allow only alphabetic characters
		Alpha = 1,
		/// allow only digits
		Digits = 2,
		/// input a number (including decimal separator and sign)
		Number = 3,
		/// input a phone number
		Phone = 4,
		/// input an URL
		Url = 5,
		/// input an email address
		Email = 6,
		/// input a name of a person
		Name = 7,
		/// input a password (combine with password or sensitive_data hint)
		Password = 8,
		/// input a date
		Date = 9,
		/// input a time
		Time = 10,
		/// input a date and time
		Datetime = 11,
		/// input for a terminal
		Terminal = 12,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTextInputV1PreeditStyle {
		/// default style for composing text
		Default = 0,
		/// style should be the same as in non-composing text
		None = 1,
		///
		Active = 2,
		///
		Inactive = 3,
		///
		Highlight = 4,
		///
		Underline = 5,
		///
		Selection = 6,
		///
		Incorrect = 7,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTextInputV1TextDirection {
		/// automatic text direction based on text and language
		Auto = 0,
		/// left-to-right
		Ltr = 1,
		/// right-to-left
		Rtl = 2,
	}
	
	pub static ZWP_TEXT_INPUT_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_text_input_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "create_text_input\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TEXT_INPUT_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # text input manager
	///
	/// A factory for text_input objects. This object is a global singleton.
	pub struct ZwpTextInputManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpTextInputManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTextInputManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTextInputManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTextInputManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTextInputManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create text input
		///
		/// Creates a new text_input object.
		pub fn create_text_input(
			&self
		) -> Result<Box<ZwpTextInputV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &ZWP_TEXT_INPUT_V1_INTERFACE, std::ptr::null::<u8>()) as *mut ZwpTextInputV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
}

/// # Wayland protocol for graphics tablets
///
/// This description provides a high-level overview of the interplay between
/// the interfaces defined this protocol. For details, see the protocol
/// specification.
///
/// More than one tablet may exist, and device-specifics matter. Tablets are
/// not represented by a single virtual device like wl_pointer. A client
/// binds to the tablet manager object which is just a proxy object. From
/// that, the client requests wp_tablet_manager.get_tablet_seat(wl_seat)
/// and that returns the actual interface that has all the tablets. With
/// this indirection, we can avoid merging wp_tablet into the actual Wayland
/// protocol, a long-term benefit.
///
/// The wp_tablet_seat sends a "tablet added" event for each tablet
/// connected. That event is followed by descriptive events about the
/// hardware; currently that includes events for name, vid/pid and
/// a wp_tablet.path event that describes a local path. This path can be
/// used to uniquely identify a tablet or get more information through
/// libwacom. Emulated or nested tablets can skip any of those, e.g. a
/// virtual tablet may not have a vid/pid. The sequence of descriptive
/// events is terminated by a wp_tablet.done event to signal that a client
/// may now finalize any initialization for that tablet.
///
/// Events from tablets require a tool in proximity. Tools are also managed
/// by the tablet seat; a "tool added" event is sent whenever a tool is new
/// to the compositor. That event is followed by a number of descriptive
/// events about the hardware; currently that includes capabilities,
/// hardware id and serial number, and tool type. Similar to the tablet
/// interface, a wp_tablet_tool.done event is sent to terminate that initial
/// sequence.
///
/// Any event from a tool happens on the wp_tablet_tool interface. When the
/// tool gets into proximity of the tablet, a proximity_in event is sent on
/// the wp_tablet_tool interface, listing the tablet and the surface. That
/// event is followed by a motion event with the coordinates. After that,
/// it's the usual motion, axis, button, etc. events. The protocol's
/// serialisation means events are grouped by wp_tablet_tool.frame events.
///
/// Two special events (that don't exist in X) are down and up. They signal
/// "tip touching the surface". For tablets without real proximity
/// detection, the sequence is: proximity_in, motion, down, frame.
///
/// When the tool leaves proximity, a proximity_out event is sent. If any
/// button is still down, a button release event is sent before this
/// proximity event. These button events are sent in the same frame as the
/// proximity event to signal to the client that the buttons were held when
/// the tool left proximity.
///
/// If the tool moves out of the surface but stays in proximity (i.e.
/// between windows), compositor-specific grab policies apply. This usually
/// means that the proximity-out is delayed until all buttons are released.
///
/// Moving a tool physically from one tablet to the other has no real effect
/// on the protocol, since we already have the tool object from the "tool
/// added" event. All the information is already there and the proximity
/// events on both tablets are all a client needs to reconstruct what
/// happened.
///
/// Some extra axes are normalized, i.e. the client knows the range as
/// specified in the protocol (e.g. [0, 65535]), the granularity however is
/// unknown. The current normalized axes are pressure, distance, and slider.
///
/// Other extra axes are in physical units as specified in the protocol.
/// The current extra axes with physical units are tilt, rotation and
/// wheel rotation.
///
/// Since tablets work independently of the pointer controlled by the mouse,
/// the focus handling is independent too and controlled by proximity.
/// The wp_tablet_tool.set_cursor request sets a tool-specific cursor.
/// This cursor surface may be the same as the mouse cursor, and it may be
/// the same across tools but it is possible to be more fine-grained. For
/// example, a client may set different cursors for the pen and eraser.
///
/// Tools are generally independent of tablets and it is
/// compositor-specific policy when a tool can be removed. Common approaches
/// will likely include some form of removing a tool when all tablets the
/// tool was used on are removed.
///
/// Warning! The protocol described in this file is experimental and
/// backward incompatible changes may be made. Backward compatible changes
/// may be added together with the corresponding interface version bump.
/// Backward incompatible changes are done by bumping the version number in
/// the protocol and interface names and resetting the interface version.
/// Once the protocol is to be declared stable, the 'z' prefix and the
/// version number in the protocol and interface names are removed and the
/// interface version number is reset.
pub use tablet_unstable_v1::*;
mod tablet_unstable_v1 {
	use crate::*;
	
	// Copyright 2014 © Stephen "Lyude" Chandler Paul
	// Copyright 2015-2016 © Red Hat, Inc.
	//
	// Permission is hereby granted, free of charge, to any person
	// obtaining a copy of this software and associated documentation files
	// (the "Software"), to deal in the Software without restriction,
	// including without limitation the rights to use, copy, modify, merge,
	// publish, distribute, sublicense, and/or sell copies of the Software,
	// and to permit persons to whom the Software is furnished to do so,
	// subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the
	// next paragraph) shall be included in all copies or substantial
	// portions of the Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
	// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
	// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
	// NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
	// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
	// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
	// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
	// SOFTWARE.
	
	pub static ZWP_TABLET_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "get_tablet_seat\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_TABLET_SEAT_V1_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # controller object for graphic tablet devices
	///
	/// An object that provides access to the graphics tablets available on this
	/// system. All tablets are associated with a seat, to get access to the
	/// actual tablets, use wp_tablet_manager.get_tablet_seat.
	pub struct ZwpTabletManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpTabletManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # get the tablet seat
		///
		/// Get the wp_tablet_seat object for the given seat. This object
		/// provides access to all graphics tablets in this seat.
		pub fn get_tablet_seat(
			&self,
			seat            : &WlSeat
		) -> Result<Box<ZwpTabletSeatV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &ZWP_TABLET_SEAT_V1_INTERFACE, std::ptr::null::<u8>(), seat) as *mut ZwpTabletSeatV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # release the memory for the tablet manager object
		///
		/// Destroy the wp_tablet_manager object. Objects created from this
		/// object are unaffected and should be destroyed separately.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub static ZWP_TABLET_SEAT_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_seat_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "tablet_added\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_V1_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "tool_added\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_TABLET_TOOL_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr()
	};
	
	/// # controller object for graphic tablet devices of a seat
	///
	/// An object that provides access to the graphics tablets available on this
	/// seat. After binding to this interface, the compositor sends a set of
	/// wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events.
	pub struct ZwpTabletSeatV1(WlProxy);
	
	impl std::ops::Deref for ZwpTabletSeatV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletSeatV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletSeatV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletSeatV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletSeatV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletSeatV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletSeatV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.tablet_added((proxy as *mut ZwpTabletSeatV1).as_mut().unwrap(), (*args.add(0)).n, ),
						1 => listener.tool_added((proxy as *mut ZwpTabletSeatV1).as_mut().unwrap(), (*args.add(0)).n, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `tablet_added` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `tool_added` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletSeatV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # release the memory for the tablet seat object
		///
		/// Destroy the wp_tablet_seat object. Objects created from this
		/// object are unaffected and should be destroyed separately.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletSeatV1Listener: std::any::Any {
		
		/// # new device notification
		///
		/// This event is sent whenever a new tablet becomes available on this
		/// seat. This event only provides the object id of the tablet, any
		/// static information about the tablet (device name, vid/pid, etc.) is
		/// sent through the wp_tablet interface.
		fn tablet_added(
			&self,
			proxy: &mut ZwpTabletSeatV1,
			id              : u32,
		);
		
		/// # a new tool has been used with a tablet
		///
		/// This event is sent whenever a tool that has not previously been used
		/// with a tablet comes into use. This event only provides the object id
		/// of the tool; any static information about the tool (capabilities,
		/// type, etc.) is sent through the wp_tablet_tool interface.
		fn tool_added(
			&self,
			proxy: &mut ZwpTabletSeatV1,
			id              : u32,
		);
	}
	
	pub static ZWP_TABLET_TOOL_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_tool_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_cursor\0".as_ptr(),
				signature: "u?oii\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  19,
		events:       [
			WlMessage {
				name:      "type\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "hardware_serial\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "hardware_id_wacom\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "capability\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "removed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "proximity_in\0".as_ptr(),
				signature: "uoo\0".as_ptr(),
				types:     [&ZWP_TABLET_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "proximity_out\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "down\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "up\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "motion\0".as_ptr(),
				signature: "ff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "pressure\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "distance\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "tilt\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "rotation\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "slider\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "wheel\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "button\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "frame\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # a physical tablet tool
	///
	/// An object that represents a physical tool that has been, or is
	/// currently in use with a tablet in this seat. Each wp_tablet_tool
	/// object stays valid until the client destroys it; the compositor
	/// reuses the wp_tablet_tool object to indicate that the object's
	/// respective physical tool has come into proximity of a tablet again.
	///
	/// A wp_tablet_tool object's relation to a physical tool depends on the
	/// tablet's ability to report serial numbers. If the tablet supports
	/// this capability, then the object represents a specific physical tool
	/// and can be identified even when used on multiple tablets.
	///
	/// A tablet tool has a number of static characteristics, e.g. tool type,
	/// hardware_serial and capabilities. These capabilities are sent in an
	/// event sequence after the wp_tablet_seat.tool_added event before any
	/// actual events from this tool. This initial event sequence is
	/// terminated by a wp_tablet_tool.done event.
	///
	/// Tablet tool events are grouped by wp_tablet_tool.frame events.
	/// Any events received before a wp_tablet_tool.frame event should be
	/// considered part of the same hardware state change.
	pub struct ZwpTabletToolV1(WlProxy);
	
	impl std::ops::Deref for ZwpTabletToolV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletToolV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletToolV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletToolV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletToolV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletToolV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletToolV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.r#type((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, ),
						1 => listener.hardware_serial((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						2 => listener.hardware_id_wacom((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						3 => listener.capability((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, ),
						4 => listener.done((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), ),
						5 => listener.removed((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), ),
						6 => listener.proximity_in((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, ((*args.add(1)).o as *mut ZwpTabletV1).as_mut(), ((*args.add(2)).o as *mut WlSurface).as_mut(), ),
						7 => listener.proximity_out((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), ),
						8 => listener.down((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, ),
						9 => listener.up((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), ),
						10 => listener.motion((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).f, (*args.add(1)).f, ),
						11 => listener.pressure((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, ),
						12 => listener.distance((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, ),
						13 => listener.tilt((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, ),
						14 => listener.rotation((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).i, ),
						15 => listener.slider((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).i, ),
						16 => listener.wheel((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).i, (*args.add(1)).i, ),
						17 => listener.button((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, ),
						18 => listener.frame((proxy as *mut ZwpTabletToolV1).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `type` ARGS: tool_type: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `hardware_serial` ARGS: hardware_serial_hi: {:?}, hardware_serial_lo: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `hardware_id_wacom` ARGS: hardware_id_hi: {:?}, hardware_id_lo: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `capability` ARGS: capability: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `removed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						6 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `proximity_in` ARGS: serial: {:?}, tablet: {:?}, surface: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, ((*args.add(1)).o as *mut ZwpTabletV1).as_mut(), ((*args.add(2)).o as *mut WlSurface).as_mut()),
						7 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `proximity_out` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						8 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `down` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						9 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `up` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						10 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `motion` ARGS: x: {:?}, y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).f, (*args.add(1)).f),
						11 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `pressure` ARGS: pressure: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						12 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `distance` ARGS: distance: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						13 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `tilt` ARGS: tilt_x: {:?}, tilt_y: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i),
						14 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `rotation` ARGS: degrees: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i),
						15 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `slider` ARGS: position: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i),
						16 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `wheel` ARGS: degrees: {:?}, clicks: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).i, (*args.add(1)).i),
						17 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `button` ARGS: serial: {:?}, button: {:?}, state: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u),
						18 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `frame` ARGS: time: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletToolV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # set the tablet tool's surface
		///
		/// Sets the surface of the cursor used for this tool on the given
		/// tablet. This request only takes effect if the tool is in proximity
		/// of one of the requesting client's surfaces or the surface parameter
		/// is the current pointer surface. If there was a previous surface set
		/// with this request it is replaced. If surface is NULL, the cursor
		/// image is hidden.
		///
		/// The parameters hotspot_x and hotspot_y define the position of the
		/// pointer surface relative to the pointer location. Its top-left corner
		/// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
		/// coordinates of the pointer location, in surface-local coordinates.
		///
		/// On surface.attach requests to the pointer surface, hotspot_x and
		/// hotspot_y are decremented by the x and y parameters passed to the
		/// request. Attach must be confirmed by wl_surface.commit as usual.
		///
		/// The hotspot can also be updated by passing the currently set pointer
		/// surface to this request with new values for hotspot_x and hotspot_y.
		///
		/// The current and pending input regions of the wl_surface are cleared,
		/// and wl_surface.set_input_region is ignored until the wl_surface is no
		/// longer used as the cursor. When the use as a cursor ends, the current
		/// and pending input regions become undefined, and the wl_surface is
		/// unmapped.
		///
		/// This request gives the surface the role of a cursor. The role
		/// assigned by this request is the same as assigned by
		/// wl_pointer.set_cursor meaning the same surface can be
		/// used both as a wl_pointer cursor and a wp_tablet cursor. If the
		/// surface already has another role, it raises a protocol error.
		/// The surface may be used on multiple tablets and across multiple
		/// seats.
		pub fn set_cursor(
			&self,
			serial          : u32,
			surface         : Option<&WlSurface>,
			hotspot_x       : i32,
			hotspot_y       : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, serial, surface.map_or(std::ptr::null_mut(), |r| r as *const WlSurface as *mut WlSurface), hotspot_x, hotspot_y); }
		}
		
		/// # destroy the tool object
		///
		/// This destroys the client's resource for this tool object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletToolV1Listener: std::any::Any {
		
		/// # tool type
		///
		/// The tool type is the high-level type of the tool and usually decides
		/// the interaction expected from this tool.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn r#type(
			&self,
			proxy: &mut ZwpTabletToolV1,
			tool_type       : u32,
		);
		
		/// # unique hardware serial number of the tool
		///
		/// If the physical tool can be identified by a unique 64-bit serial
		/// number, this event notifies the client of this serial number.
		///
		/// If multiple tablets are available in the same seat and the tool is
		/// uniquely identifiable by the serial number, that tool may move
		/// between tablets.
		///
		/// Otherwise, if the tool has no serial number and this event is
		/// missing, the tool is tied to the tablet it first comes into
		/// proximity with. Even if the physical tool is used on multiple
		/// tablets, separate wp_tablet_tool objects will be created, one per
		/// tablet.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn hardware_serial(
			&self,
			proxy: &mut ZwpTabletToolV1,
			hardware_serial_hi: u32,
			hardware_serial_lo: u32,
		);
		
		/// # hardware id notification in Wacom's format
		///
		/// This event notifies the client of a hardware id available on this tool.
		///
		/// The hardware id is a device-specific 64-bit id that provides extra
		/// information about the tool in use, beyond the wl_tool.type
		/// enumeration. The format of the id is specific to tablets made by
		/// Wacom Inc. For example, the hardware id of a Wacom Grip
		/// Pen (a stylus) is 0x802.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn hardware_id_wacom(
			&self,
			proxy: &mut ZwpTabletToolV1,
			hardware_id_hi  : u32,
			hardware_id_lo  : u32,
		);
		
		/// # tool capability notification
		///
		/// This event notifies the client of any capabilities of this tool,
		/// beyond the main set of x/y axes and tip up/down detection.
		///
		/// One event is sent for each extra capability available on this tool.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet_tool.done event.
		fn capability(
			&self,
			proxy: &mut ZwpTabletToolV1,
			capability      : u32,
		);
		
		/// # tool description events sequence complete
		///
		/// This event signals the end of the initial burst of descriptive
		/// events. A client may consider the static description of the tool to
		/// be complete and finalize initialization of the tool.
		fn done(
			&self,
			proxy: &mut ZwpTabletToolV1,
		);
		
		/// # tool removed
		///
		/// This event is sent when the tool is removed from the system and will
		/// send no further events. Should the physical tool come back into
		/// proximity later, a new wp_tablet_tool object will be created.
		///
		/// It is compositor-dependent when a tool is removed. A compositor may
		/// remove a tool on proximity out, tablet removal or any other reason.
		/// A compositor may also keep a tool alive until shutdown.
		///
		/// If the tool is currently in proximity, a proximity_out event will be
		/// sent before the removed event. See wp_tablet_tool.proximity_out for
		/// the handling of any buttons logically down.
		///
		/// When this event is received, the client must wp_tablet_tool.destroy
		/// the object.
		fn removed(
			&self,
			proxy: &mut ZwpTabletToolV1,
		);
		
		/// # proximity in event
		///
		/// Notification that this tool is focused on a certain surface.
		///
		/// This event can be received when the tool has moved from one surface to
		/// another, or when the tool has come back into proximity above the
		/// surface.
		///
		/// If any button is logically down when the tool comes into proximity,
		/// the respective button event is sent after the proximity_in event but
		/// within the same frame as the proximity_in event.
		fn proximity_in(
			&self,
			proxy: &mut ZwpTabletToolV1,
			serial          : u32,
			tablet          : Option<&mut ZwpTabletV1>,
			surface         : Option<&mut WlSurface>,
		);
		
		/// # proximity out event
		///
		/// Notification that this tool has either left proximity, or is no
		/// longer focused on a certain surface.
		///
		/// When the tablet tool leaves proximity of the tablet, button release
		/// events are sent for each button that was held down at the time of
		/// leaving proximity. These events are sent before the proximity_out
		/// event but within the same wp_tablet.frame.
		///
		/// If the tool stays within proximity of the tablet, but the focus
		/// changes from one surface to another, a button release event may not
		/// be sent until the button is actually released or the tool leaves the
		/// proximity of the tablet.
		fn proximity_out(
			&self,
			proxy: &mut ZwpTabletToolV1,
		);
		
		/// # tablet tool is making contact
		///
		/// Sent whenever the tablet tool comes in contact with the surface of the
		/// tablet.
		///
		/// If the tool is already in contact with the tablet when entering the
		/// input region, the client owning said region will receive a
		/// wp_tablet.proximity_in event, followed by a wp_tablet.down
		/// event and a wp_tablet.frame event.
		///
		/// Note that this event describes logical contact, not physical
		/// contact. On some devices, a compositor may not consider a tool in
		/// logical contact until a minimum physical pressure threshold is
		/// exceeded.
		fn down(
			&self,
			proxy: &mut ZwpTabletToolV1,
			serial          : u32,
		);
		
		/// # tablet tool is no longer making contact
		///
		/// Sent whenever the tablet tool stops making contact with the surface of
		/// the tablet, or when the tablet tool moves out of the input region
		/// and the compositor grab (if any) is dismissed.
		///
		/// If the tablet tool moves out of the input region while in contact
		/// with the surface of the tablet and the compositor does not have an
		/// ongoing grab on the surface, the client owning said region will
		/// receive a wp_tablet.up event, followed by a wp_tablet.proximity_out
		/// event and a wp_tablet.frame event. If the compositor has an ongoing
		/// grab on this device, this event sequence is sent whenever the grab
		/// is dismissed in the future.
		///
		/// Note that this event describes logical contact, not physical
		/// contact. On some devices, a compositor may not consider a tool out
		/// of logical contact until physical pressure falls below a specific
		/// threshold.
		fn up(
			&self,
			proxy: &mut ZwpTabletToolV1,
		);
		
		/// # motion event
		///
		/// Sent whenever a tablet tool moves.
		fn motion(
			&self,
			proxy: &mut ZwpTabletToolV1,
			x               : WlFixed,
			y               : WlFixed,
		);
		
		/// # pressure change event
		///
		/// Sent whenever the pressure axis on a tool changes. The value of this
		/// event is normalized to a value between 0 and 65535.
		///
		/// Note that pressure may be nonzero even when a tool is not in logical
		/// contact. See the down and up events for more details.
		fn pressure(
			&self,
			proxy: &mut ZwpTabletToolV1,
			pressure        : u32,
		);
		
		/// # distance change event
		///
		/// Sent whenever the distance axis on a tool changes. The value of this
		/// event is normalized to a value between 0 and 65535.
		///
		/// Note that distance may be nonzero even when a tool is not in logical
		/// contact. See the down and up events for more details.
		fn distance(
			&self,
			proxy: &mut ZwpTabletToolV1,
			distance        : u32,
		);
		
		/// # tilt change event
		///
		/// Sent whenever one or both of the tilt axes on a tool change. Each tilt
		/// value is in 0.01 of a degree, relative to the z-axis of the tablet.
		/// The angle is positive when the top of a tool tilts along the
		/// positive x or y axis.
		fn tilt(
			&self,
			proxy: &mut ZwpTabletToolV1,
			tilt_x          : i32,
			tilt_y          : i32,
		);
		
		/// # z-rotation change event
		///
		/// Sent whenever the z-rotation axis on the tool changes. The
		/// rotation value is in 0.01 of a degree clockwise from the tool's
		/// logical neutral position.
		fn rotation(
			&self,
			proxy: &mut ZwpTabletToolV1,
			degrees         : i32,
		);
		
		/// # Slider position change event
		///
		/// Sent whenever the slider position on the tool changes. The
		/// value is normalized between -65535 and 65535, with 0 as the logical
		/// neutral position of the slider.
		///
		/// The slider is available on e.g. the Wacom Airbrush tool.
		fn slider(
			&self,
			proxy: &mut ZwpTabletToolV1,
			position        : i32,
		);
		
		/// # Wheel delta event
		///
		/// Sent whenever the wheel on the tool emits an event. This event
		/// contains two values for the same axis change. The degrees value is
		/// in 0.01 of a degree in the same orientation as the
		/// wl_pointer.vertical_scroll axis. The clicks value is in discrete
		/// logical clicks of the mouse wheel. This value may be zero if the
		/// movement of the wheel was less than one logical click.
		///
		/// Clients should choose either value and avoid mixing degrees and
		/// clicks. The compositor may accumulate values smaller than a logical
		/// click and emulate click events when a certain threshold is met.
		/// Thus, wl_tablet_tool.wheel events with non-zero clicks values may
		/// have different degrees values.
		fn wheel(
			&self,
			proxy: &mut ZwpTabletToolV1,
			degrees         : i32,
			clicks          : i32,
		);
		
		/// # button event
		///
		/// Sent whenever a button on the tool is pressed or released.
		///
		/// If a button is held down when the tool moves in or out of proximity,
		/// button events are generated by the compositor. See
		/// wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
		/// details.
		fn button(
			&self,
			proxy: &mut ZwpTabletToolV1,
			serial          : u32,
			button          : u32,
			state           : u32,
		);
		
		/// # frame event
		///
		/// Marks the end of a series of axis and/or button updates from the
		/// tablet. The Wayland protocol requires axis updates to be sent
		/// sequentially, however all events within a frame should be considered
		/// one hardware event.
		fn frame(
			&self,
			proxy: &mut ZwpTabletToolV1,
			time            : u32,
		);
	}
	
	/// # a physical tool type
	///
	/// Describes the physical type of a tool. The physical type of a tool
	/// generally defines its base usage.
	///
	/// The mouse tool represents a mouse-shaped tool that is not a relative
	/// device but bound to the tablet's surface, providing absolute
	/// coordinates.
	///
	/// The lens tool is a mouse-shaped tool with an attached lens to
	/// provide precision focus.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV1Type {
		/// Pen
		Pen = 0x140,
		/// Eraser
		Eraser = 0x141,
		/// Brush
		Brush = 0x142,
		/// Pencil
		Pencil = 0x143,
		/// Airbrush
		Airbrush = 0x144,
		/// Finger
		Finger = 0x145,
		/// Mouse
		Mouse = 0x146,
		/// Lens
		Lens = 0x147,
	}
	
	/// # capability flags for a tool
	///
	/// Describes extra capabilities on a tablet.
	///
	/// Any tool must provide x and y values, extra axes are
	/// device-specific.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV1Capability {
		/// Tilt axes
		Tilt = 1,
		/// Pressure axis
		Pressure = 2,
		/// Distance axis
		Distance = 3,
		/// Z-rotation axis
		Rotation = 4,
		/// Slider axis
		Slider = 5,
		/// Wheel axis
		Wheel = 6,
	}
	
	/// # physical button state
	///
	/// Describes the physical state of a button that produced the button event.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV1ButtonState {
		/// button is not pressed
		Released = 0,
		/// button is pressed
		Pressed = 1,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpTabletToolV1Error {
		/// given wl_surface has another role
		Role = 0,
	}
	
	pub static ZWP_TABLET_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_tablet_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  5,
		events:       [
			WlMessage {
				name:      "name\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "id\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "path\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "done\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "removed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # graphics tablet device
	///
	/// The wp_tablet interface represents one graphics tablet device. The
	/// tablet interface itself does not generate events; all events are
	/// generated by wp_tablet_tool objects when in proximity above a tablet.
	///
	/// A tablet has a number of static characteristics, e.g. device name and
	/// pid/vid. These capabilities are sent in an event sequence after the
	/// wp_tablet_seat.tablet_added event. This initial event sequence is
	/// terminated by a wp_tablet.done event.
	pub struct ZwpTabletV1(WlProxy);
	
	impl std::ops::Deref for ZwpTabletV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpTabletV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpTabletV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpTabletV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpTabletV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpTabletV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpTabletV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.name((proxy as *mut ZwpTabletV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						1 => listener.id((proxy as *mut ZwpTabletV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						2 => listener.path((proxy as *mut ZwpTabletV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						3 => listener.done((proxy as *mut ZwpTabletV1).as_mut().unwrap(), ),
						4 => listener.removed((proxy as *mut ZwpTabletV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `name` ARGS: name: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `id` ARGS: vid: {:?}, pid: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `path` ARGS: path: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `done` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `removed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpTabletV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the tablet object
		///
		/// This destroys the client's resource for this tablet object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpTabletV1Listener: std::any::Any {
		
		/// # tablet device name
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet.done event.
		fn name(
			&self,
			proxy: &mut ZwpTabletV1,
			name            : &str,
		);
		
		/// # tablet device USB vendor/product id
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet.done event.
		fn id(
			&self,
			proxy: &mut ZwpTabletV1,
			vid             : u32,
			pid             : u32,
		);
		
		/// # path to the device
		///
		/// A system-specific device path that indicates which device is behind
		/// this wp_tablet. This information may be used to gather additional
		/// information about the device, e.g. through libwacom.
		///
		/// A device may have more than one device path. If so, multiple
		/// wp_tablet.path events are sent. A device may be emulated and not
		/// have a device path, and in that case this event will not be sent.
		///
		/// The format of the path is unspecified, it may be a device node, a
		/// sysfs path, or some other identifier. It is up to the client to
		/// identify the string provided.
		///
		/// This event is sent in the initial burst of events before the
		/// wp_tablet.done event.
		fn path(
			&self,
			proxy: &mut ZwpTabletV1,
			path            : &str,
		);
		
		/// # tablet description events sequence complete
		///
		/// This event is sent immediately to signal the end of the initial
		/// burst of descriptive events. A client may consider the static
		/// description of the tablet to be complete and finalize initialization
		/// of the tablet.
		fn done(
			&self,
			proxy: &mut ZwpTabletV1,
		);
		
		/// # tablet removed event
		///
		/// Sent when the tablet has been removed from the system. When a tablet
		/// is removed, some tools may be removed.
		///
		/// When this event is received, the client must wp_tablet.destroy
		/// the object.
		fn removed(
			&self,
			proxy: &mut ZwpTabletV1,
		);
	}
}

/// # Primary selection protocol
///
/// This protocol provides the ability to have a primary selection device to
/// match that of the X server. This primary selection is a shortcut to the
/// common clipboard selection, where text just needs to be selected in order
/// to allow copying it elsewhere. The de facto way to perform this action
/// is the middle mouse button, although it is not limited to this one.
///
/// Clients wishing to honor primary selection should create a primary
/// selection source and set it as the selection through
/// wp_primary_selection_device.set_selection whenever the text selection
/// changes. In order to minimize calls in pointer-driven text selection,
/// it should happen only once after the operation finished. Similarly,
/// a NULL source should be set when text is unselected.
///
/// wp_primary_selection_offer objects are first announced through the
/// wp_primary_selection_device.data_offer event. Immediately after this event,
/// the primary data offer will emit wp_primary_selection_offer.offer events
/// to let know of the mime types being offered.
///
/// When the primary selection changes, the client with the keyboard focus
/// will receive wp_primary_selection_device.selection events. Only the client
/// with the keyboard focus will receive such events with a non-NULL
/// wp_primary_selection_offer. Across keyboard focus changes, previously
/// focused clients will receive wp_primary_selection_device.events with a
/// NULL wp_primary_selection_offer.
///
/// In order to request the primary selection data, the client must pass
/// a recent serial pertaining to the press event that is triggering the
/// operation, if the compositor deems the serial valid and recent, the
/// wp_primary_selection_source.send event will happen in the other end
/// to let the transfer begin. The client owning the primary selection
/// should write the requested data, and close the file descriptor
/// immediately.
///
/// If the primary selection owner client disappeared during the transfer,
/// the client reading the data will receive a
/// wp_primary_selection_device.selection event with a NULL
/// wp_primary_selection_offer, the client should take this as a hint
/// to finish the reads related to the no longer existing offer.
///
/// The primary selection owner should be checking for errors during
/// writes, merely cancelling the ongoing transfer if any happened.
pub use wp_primary_selection_unstable_v1::*;
mod wp_primary_selection_unstable_v1 {
	use crate::*;
	
	// Copyright © 2015, 2016 Red Hat
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_PRIMARY_SELECTION_DEVICE_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_primary_selection_device_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "create_source\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_PRIMARY_SELECTION_SOURCE_V1_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_device\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_PRIMARY_SELECTION_DEVICE_V1_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # X primary selection emulation
	///
	/// The primary selection device manager is a singleton global object that
	/// provides access to the primary selection. It allows to create
	/// wp_primary_selection_source objects, as well as retrieving the per-seat
	/// wp_primary_selection_device objects.
	pub struct ZwpPrimarySelectionDeviceManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpPrimarySelectionDeviceManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPrimarySelectionDeviceManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPrimarySelectionDeviceManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPrimarySelectionDeviceManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPrimarySelectionDeviceManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # create a new primary selection source
		///
		/// Create a new primary selection source.
		pub fn create_source(
			&self
		) -> Result<Box<ZwpPrimarySelectionSourceV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &ZWP_PRIMARY_SELECTION_SOURCE_V1_INTERFACE, std::ptr::null::<u8>()) as *mut ZwpPrimarySelectionSourceV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # create a new primary selection device
		///
		/// Create a new data device for a given seat.
		pub fn get_device(
			&self,
			seat            : &WlSeat
		) -> Result<Box<ZwpPrimarySelectionDeviceV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_PRIMARY_SELECTION_DEVICE_V1_INTERFACE, std::ptr::null::<u8>(), seat) as *mut ZwpPrimarySelectionDeviceV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # destroy the primary selection device manager
		///
		/// Destroy the primary selection device manager.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub static ZWP_PRIMARY_SELECTION_DEVICE_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_primary_selection_device_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_selection\0".as_ptr(),
				signature: "?ou\0".as_ptr(),
				types:     [&ZWP_PRIMARY_SELECTION_SOURCE_V1_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "data_offer\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_PRIMARY_SELECTION_OFFER_V1_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "selection\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&ZWP_PRIMARY_SELECTION_OFFER_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr()
	};
	pub struct ZwpPrimarySelectionDeviceV1(WlProxy);
	
	impl std::ops::Deref for ZwpPrimarySelectionDeviceV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPrimarySelectionDeviceV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPrimarySelectionDeviceV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPrimarySelectionDeviceV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPrimarySelectionDeviceV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpPrimarySelectionDeviceV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpPrimarySelectionDeviceV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.data_offer((proxy as *mut ZwpPrimarySelectionDeviceV1).as_mut().unwrap(), (*args.add(0)).n, ),
						1 => listener.selection((proxy as *mut ZwpPrimarySelectionDeviceV1).as_mut().unwrap(), ((*args.add(0)).o as *mut ZwpPrimarySelectionOfferV1).as_mut(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `data_offer` ARGS: offer: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `selection` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut ZwpPrimarySelectionOfferV1).as_mut()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpPrimarySelectionDeviceV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # set the primary selection
		///
		/// Replaces the current selection. The previous owner of the primary
		/// selection will receive a wp_primary_selection_source.cancelled event.
		///
		/// To unset the selection, set the source to NULL.
		pub fn set_selection(
			&self,
			source          : Option<&ZwpPrimarySelectionSourceV1>,
			serial          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, source.map_or(std::ptr::null_mut(), |r| r as *const ZwpPrimarySelectionSourceV1 as *mut ZwpPrimarySelectionSourceV1), serial); }
		}
		
		/// # destroy the primary selection device
		///
		/// Destroy the primary selection device.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpPrimarySelectionDeviceV1Listener: std::any::Any {
		
		/// # introduce a new wp_primary_selection_offer
		///
		/// Introduces a new wp_primary_selection_offer object that may be used
		/// to receive the current primary selection. Immediately following this
		/// event, the new wp_primary_selection_offer object will send
		/// wp_primary_selection_offer.offer events to describe the offered mime
		/// types.
		fn data_offer(
			&self,
			proxy: &mut ZwpPrimarySelectionDeviceV1,
			offer           : u32,
		);
		
		/// # advertise a new primary selection
		///
		/// The wp_primary_selection_device.selection event is sent to notify the
		/// client of a new primary selection. This event is sent after the
		/// wp_primary_selection.data_offer event introducing this object, and after
		/// the offer has announced its mimetypes through
		/// wp_primary_selection_offer.offer.
		///
		/// The data_offer is valid until a new offer or NULL is received
		/// or until the client loses keyboard focus. The client must destroy the
		/// previous selection data_offer, if any, upon receiving this event.
		fn selection(
			&self,
			proxy: &mut ZwpPrimarySelectionDeviceV1,
			id              : Option<&mut ZwpPrimarySelectionOfferV1>,
		);
	}
	
	pub static ZWP_PRIMARY_SELECTION_OFFER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_primary_selection_offer_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "receive\0".as_ptr(),
				signature: "sh\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "offer\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # offer to transfer primary selection contents
	///
	/// A wp_primary_selection_offer represents an offer to transfer the contents
	/// of the primary selection clipboard to the client. Similar to
	/// wl_data_offer, the offer also describes the mime types that the data can
	/// be converted to and provides the mechanisms for transferring the data
	/// directly to the client.
	pub struct ZwpPrimarySelectionOfferV1(WlProxy);
	
	impl std::ops::Deref for ZwpPrimarySelectionOfferV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPrimarySelectionOfferV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPrimarySelectionOfferV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPrimarySelectionOfferV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPrimarySelectionOfferV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpPrimarySelectionOfferV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpPrimarySelectionOfferV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.offer((proxy as *mut ZwpPrimarySelectionOfferV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `offer` ARGS: mime_type: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpPrimarySelectionOfferV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # request that the data is transferred
		///
		/// To transfer the contents of the primary selection clipboard, the client
		/// issues this request and indicates the mime type that it wants to
		/// receive. The transfer happens through the passed file descriptor
		/// (typically created with the pipe system call). The source client writes
		/// the data in the mime type representation requested and then closes the
		/// file descriptor.
		///
		/// The receiving client reads from the read end of the pipe until EOF and
		/// closes its end, at which point the transfer is complete.
		pub fn receive(
			&self,
			mime_type       : &str,
			fd              : RawFd
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, mime_type.as_ptr(), fd); }
		}
		
		/// # destroy the primary selection offer
		///
		/// Destroy the primary selection offer.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpPrimarySelectionOfferV1Listener: std::any::Any {
		
		/// # advertise offered mime type
		///
		/// Sent immediately after creating announcing the
		/// wp_primary_selection_offer through
		/// wp_primary_selection_device.data_offer. One event is sent per offered
		/// mime type.
		fn offer(
			&self,
			proxy: &mut ZwpPrimarySelectionOfferV1,
			mime_type       : &str,
		);
	}
	
	pub static ZWP_PRIMARY_SELECTION_SOURCE_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_primary_selection_source_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "offer\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "send\0".as_ptr(),
				signature: "sh\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "cancelled\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # offer to replace the contents of the primary selection
	///
	/// The source side of a wp_primary_selection_offer, it provides a way to
	/// describe the offered data and respond to requests to transfer the
	/// requested contents of the primary selection clipboard.
	pub struct ZwpPrimarySelectionSourceV1(WlProxy);
	
	impl std::ops::Deref for ZwpPrimarySelectionSourceV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPrimarySelectionSourceV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPrimarySelectionSourceV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPrimarySelectionSourceV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPrimarySelectionSourceV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpPrimarySelectionSourceV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpPrimarySelectionSourceV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.send((proxy as *mut ZwpPrimarySelectionSourceV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).h, ),
						1 => listener.cancelled((proxy as *mut ZwpPrimarySelectionSourceV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `send` ARGS: mime_type: {:?}, fd: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).h),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `cancelled` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpPrimarySelectionSourceV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # add an offered mime type
		///
		/// This request adds a mime type to the set of mime types advertised to
		/// targets. Can be called several times to offer multiple types.
		pub fn offer(
			&self,
			mime_type       : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, mime_type.as_ptr()); }
		}
		
		/// # destroy the primary selection source
		///
		/// Destroy the primary selection source.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpPrimarySelectionSourceV1Listener: std::any::Any {
		
		/// # send the primary selection contents
		///
		/// Request for the current primary selection contents from the client.
		/// Send the specified mime type over the passed file descriptor, then
		/// close it.
		fn send(
			&self,
			proxy: &mut ZwpPrimarySelectionSourceV1,
			mime_type       : &str,
			fd              : RawFd,
		);
		
		/// # request for primary selection contents was canceled
		///
		/// This primary selection source is no longer valid. The client should
		/// clean up and destroy this primary selection source.
		fn cancelled(
			&self,
			proxy: &mut ZwpPrimarySelectionSourceV1,
		);
	}
}
pub use presentation_time::*;
mod presentation_time {
	use crate::*;
	
	// Copyright © 2013-2014 Collabora, Ltd.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static WP_PRESENTATION_INTERFACE: WlInterface = WlInterface {
		name:         "wp_presentation\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "feedback\0".as_ptr(),
				signature: "on\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _, &WP_PRESENTATION_FEEDBACK_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "clock_id\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # timed presentation related wl_surface requests
	///
	/// The main feature of this interface is accurate presentation
	/// timing feedback to ensure smooth video playback while maintaining
	/// audio/video synchronization. Some features use the concept of a
	/// presentation clock, which is defined in the
	/// presentation.clock_id event.
	///
	/// A content update for a wl_surface is submitted by a
	/// wl_surface.commit request. Request 'feedback' associates with
	/// the wl_surface.commit and provides feedback on the content
	/// update, particularly the final realized presentation time.
	///
	/// When the final realized presentation time is available, e.g.
	/// after a framebuffer flip completes, the requested
	/// presentation_feedback.presented events are sent. The final
	/// presentation time can differ from the compositor's predicted
	/// display update time and the update's target time, especially
	/// when the compositor misses its target vertical blanking period.
	pub struct WpPresentation(WlProxy);
	
	impl std::ops::Deref for WpPresentation {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WpPresentation {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WpPresentation {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WpPresentation")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WpPresentation {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WpPresentationListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WpPresentationListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.clock_id((proxy as *mut WpPresentation).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `clock_id` ARGS: clk_id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WpPresentationListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # unbind from the presentation interface
		///
		/// Informs the server that the client will no longer be using
		/// this protocol object. Existing objects created by this object
		/// are not affected.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # request presentation feedback information
		///
		/// Request presentation feedback for the current content submission
		/// on the given surface. This creates a new presentation_feedback
		/// object, which will deliver the feedback information once. If
		/// multiple presentation_feedback objects are created for the same
		/// submission, they will all deliver the same information.
		///
		/// For details on what information is returned, see the
		/// presentation_feedback interface.
		pub fn feedback(
			&self,
			surface         : &WlSurface
		) -> Result<Box<WpPresentationFeedback, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &WP_PRESENTATION_FEEDBACK_INTERFACE, surface, std::ptr::null::<u8>()) as *mut WpPresentationFeedback };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub trait WpPresentationListener: std::any::Any {
		
		/// # clock ID for timestamps
		///
		/// This event tells the client in which clock domain the
		/// compositor interprets the timestamps used by the presentation
		/// extension. This clock is called the presentation clock.
		///
		/// The compositor sends this event when the client binds to the
		/// presentation interface. The presentation clock does not change
		/// during the lifetime of the client connection.
		///
		/// The clock identifier is platform dependent. On Linux/glibc,
		/// the identifier value is one of the clockid_t values accepted
		/// by clock_gettime(). clock_gettime() is defined by
		/// POSIX.1-2001.
		///
		/// Timestamps in this clock domain are expressed as tv_sec_hi,
		/// tv_sec_lo, tv_nsec triples, each component being an unsigned
		/// 32-bit value. Whole seconds are in tv_sec which is a 64-bit
		/// value combined from tv_sec_hi and tv_sec_lo, and the
		/// additional fractional part in tv_nsec as nanoseconds. Hence,
		/// for valid timestamps tv_nsec must be in [0, 999999999].
		///
		/// Note that clock_id applies only to the presentation clock,
		/// and implies nothing about e.g. the timestamps used in the
		/// Wayland core protocol input events.
		///
		/// Compositors should prefer a clock which does not jump and is
		/// not slewed e.g. by NTP. The absolute value of the clock is
		/// irrelevant. Precision of one millisecond or better is
		/// recommended. Clients must be able to query the current clock
		/// value directly, not by asking the compositor.
		fn clock_id(
			&self,
			proxy: &mut WpPresentation,
			clk_id          : u32,
		);
	}
	
	/// # fatal presentation errors
	///
	/// These fatal protocol errors may be emitted in response to
	/// illegal presentation requests.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WpPresentationError {
		/// invalid value in tv_nsec
		InvalidTimestamp = 0,
		/// invalid flag
		InvalidFlag = 1,
	}
	
	pub static WP_PRESENTATION_FEEDBACK_INTERFACE: WlInterface = WlInterface {
		name:         "wp_presentation_feedback\0".as_ptr(),
		version:      1,
		method_count: 0,
		methods:      [
		].as_ptr(),
		event_count:  3,
		events:       [
			WlMessage {
				name:      "sync_output\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "presented\0".as_ptr(),
				signature: "uuuuuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "discarded\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # presentation time feedback event
	///
	/// A presentation_feedback object returns an indication that a
	/// wl_surface content update has become visible to the user.
	/// One object corresponds to one content update submission
	/// (wl_surface.commit). There are two possible outcomes: the
	/// content update is presented to the user, and a presentation
	/// timestamp delivered; or, the user did not see the content
	/// update because it was superseded or its surface destroyed,
	/// and the content update is discarded.
	///
	/// Once a presentation_feedback object has delivered a 'presented'
	/// or 'discarded' event it is automatically destroyed.
	pub struct WpPresentationFeedback(WlProxy);
	
	impl std::ops::Deref for WpPresentationFeedback {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for WpPresentationFeedback {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for WpPresentationFeedback {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("WpPresentationFeedback")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl WpPresentationFeedback {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl WpPresentationFeedbackListener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn WpPresentationFeedbackListener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.sync_output((proxy as *mut WpPresentationFeedback).as_mut().unwrap(), ((*args.add(0)).o as *mut WlOutput).as_mut(), ),
						1 => listener.presented((proxy as *mut WpPresentationFeedback).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, (*args.add(4)).u, (*args.add(5)).u, (*args.add(6)).u, ),
						2 => listener.discarded((proxy as *mut WpPresentationFeedback).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `sync_output` ARGS: output: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut WlOutput).as_mut()),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `presented` ARGS: tv_sec_hi: {:?}, tv_sec_lo: {:?}, tv_nsec: {:?}, refresh: {:?}, seq_hi: {:?}, seq_lo: {:?}, flags: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, (*args.add(3)).u, (*args.add(4)).u, (*args.add(5)).u, (*args.add(6)).u),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `discarded` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn WpPresentationFeedbackListener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait WpPresentationFeedbackListener: std::any::Any {
		
		/// # presentation synchronized to this output
		///
		/// As presentation can be synchronized to only one output at a
		/// time, this event tells which output it was. This event is only
		/// sent prior to the presented event.
		///
		/// As clients may bind to the same global wl_output multiple
		/// times, this event is sent for each bound instance that matches
		/// the synchronized output. If a client has not bound to the
		/// right wl_output global at all, this event is not sent.
		fn sync_output(
			&self,
			proxy: &mut WpPresentationFeedback,
			output          : Option<&mut WlOutput>,
		);
		
		/// # the content update was displayed
		///
		/// The associated content update was displayed to the user at the
		/// indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of
		/// the timestamp, see presentation.clock_id event.
		///
		/// The timestamp corresponds to the time when the content update
		/// turned into light the first time on the surface's main output.
		/// Compositors may approximate this from the framebuffer flip
		/// completion events from the system, and the latency of the
		/// physical display path if known.
		///
		/// This event is preceded by all related sync_output events
		/// telling which output's refresh cycle the feedback corresponds
		/// to, i.e. the main output for the surface. Compositors are
		/// recommended to choose the output containing the largest part
		/// of the wl_surface, or keeping the output they previously
		/// chose. Having a stable presentation output association helps
		/// clients predict future output refreshes (vblank).
		///
		/// The 'refresh' argument gives the compositor's prediction of how
		/// many nanoseconds after tv_sec, tv_nsec the very next output
		/// refresh may occur. This is to further aid clients in
		/// predicting future refreshes, i.e., estimating the timestamps
		/// targeting the next few vblanks. If such prediction cannot
		/// usefully be done, the argument is zero.
		///
		/// If the output does not have a constant refresh rate, explicit
		/// video mode switches excluded, then the refresh argument must
		/// be zero.
		///
		/// The 64-bit value combined from seq_hi and seq_lo is the value
		/// of the output's vertical retrace counter when the content
		/// update was first scanned out to the display. This value must
		/// be compatible with the definition of MSC in
		/// GLX_OML_sync_control specification. Note, that if the display
		/// path has a non-zero latency, the time instant specified by
		/// this counter may differ from the timestamp's.
		///
		/// If the output does not have a concept of vertical retrace or a
		/// refresh cycle, or the output device is self-refreshing without
		/// a way to query the refresh count, then the arguments seq_hi
		/// and seq_lo must be zero.
		fn presented(
			&self,
			proxy: &mut WpPresentationFeedback,
			tv_sec_hi       : u32,
			tv_sec_lo       : u32,
			tv_nsec         : u32,
			refresh         : u32,
			seq_hi          : u32,
			seq_lo          : u32,
			flags           : u32,
		);
		
		/// # the content update was not displayed
		///
		/// The content update was never displayed to the user.
		fn discarded(
			&self,
			proxy: &mut WpPresentationFeedback,
		);
	}
	
	/// # bitmask of flags in presented event
	///
	/// These flags provide information about how the presentation of
	/// the related content update was done. The intent is to help
	/// clients assess the reliability of the feedback and the visual
	/// quality with respect to possible tearing and timings. The
	/// flags are:
	///
	/// VSYNC:
	/// The presentation was synchronized to the "vertical retrace" by
	/// the display hardware such that tearing does not happen.
	/// Relying on user space scheduling is not acceptable for this
	/// flag. If presentation is done by a copy to the active
	/// frontbuffer, then it must guarantee that tearing cannot
	/// happen.
	///
	/// HW_CLOCK:
	/// The display hardware provided measurements that the hardware
	/// driver converted into a presentation timestamp. Sampling a
	/// clock in user space is not acceptable for this flag.
	///
	/// HW_COMPLETION:
	/// The display hardware signalled that it started using the new
	/// image content. The opposite of this is e.g. a timer being used
	/// to guess when the display hardware has switched to the new
	/// image content.
	///
	/// ZERO_COPY:
	/// The presentation of this update was done zero-copy. This means
	/// the buffer from the client was given to display hardware as
	/// is, without copying it. Compositing with OpenGL counts as
	/// copying, even if textured directly from the client buffer.
	/// Possible zero-copy cases include direct scanout of a
	/// fullscreen surface and a surface on a hardware overlay.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum WpPresentationFeedbackKind {
		/// presentation was vsync'd
		Vsync = 0x1,
		/// hardware provided the presentation timestamp
		HwClock = 0x2,
		/// hardware signalled the start of the presentation
		HwCompletion = 0x4,
		/// presentation was done zero-copy
		ZeroCopy = 0x8,
	}
}
pub use pointer_gestures_unstable_v1::*;
mod pointer_gestures_unstable_v1 {
	use crate::*;
	
	
	pub static ZWP_POINTER_GESTURES_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_pointer_gestures_v1\0".as_ptr(),
		version:      2,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "get_swipe_gesture\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_POINTER_GESTURE_SWIPE_V1_INTERFACE as _, &WL_POINTER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_pinch_gesture\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_POINTER_GESTURE_PINCH_V1_INTERFACE as _, &WL_POINTER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # touchpad gestures
	///
	/// A global interface to provide semantic touchpad gestures for a given
	/// pointer.
	///
	/// Two gestures are currently supported: swipe and zoom/rotate.
	/// All gestures follow a three-stage cycle: begin, update, end and
	/// are identified by a unique id.
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZwpPointerGesturesV1(WlProxy);
	
	impl std::ops::Deref for ZwpPointerGesturesV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPointerGesturesV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPointerGesturesV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPointerGesturesV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPointerGesturesV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # get swipe gesture
		///
		/// Create a swipe gesture object. See the
		/// wl_pointer_gesture_swipe interface for details.
		pub fn get_swipe_gesture(
			&self,
			pointer         : &WlPointer
		) -> Result<Box<ZwpPointerGestureSwipeV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &ZWP_POINTER_GESTURE_SWIPE_V1_INTERFACE, std::ptr::null::<u8>(), pointer) as *mut ZwpPointerGestureSwipeV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # get pinch gesture
		///
		/// Create a pinch gesture object. See the
		/// wl_pointer_gesture_pinch interface for details.
		pub fn get_pinch_gesture(
			&self,
			pointer         : &WlPointer
		) -> Result<Box<ZwpPointerGesturePinchV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_POINTER_GESTURE_PINCH_V1_INTERFACE, std::ptr::null::<u8>(), pointer) as *mut ZwpPointerGesturePinchV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # destroy the pointer gesture object
		///
		/// Destroy the pointer gesture object. Swipe and pinch objects created via this
		/// gesture object remain valid.
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub static ZWP_POINTER_GESTURE_SWIPE_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_pointer_gesture_swipe_v1\0".as_ptr(),
		version:      2,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  3,
		events:       [
			WlMessage {
				name:      "begin\0".as_ptr(),
				signature: "uuou\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "update\0".as_ptr(),
				signature: "uff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "end\0".as_ptr(),
				signature: "uui\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # a swipe gesture object
	///
	/// A swipe gesture object notifies a client about a multi-finger swipe
	/// gesture detected on an indirect input device such as a touchpad.
	/// The gesture is usually initiated by multiple fingers moving in the
	/// same direction but once initiated the direction may change.
	/// The precise conditions of when such a gesture is detected are
	/// implementation-dependent.
	///
	/// A gesture consists of three stages: begin, update (optional) and end.
	/// There cannot be multiple simultaneous pinch or swipe gestures on a
	/// same pointer/seat, how compositors prevent these situations is
	/// implementation-dependent.
	///
	/// A gesture may be cancelled by the compositor or the hardware.
	/// Clients should not consider performing permanent or irreversible
	/// actions until the end of a gesture has been received.
	pub struct ZwpPointerGestureSwipeV1(WlProxy);
	
	impl std::ops::Deref for ZwpPointerGestureSwipeV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPointerGestureSwipeV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPointerGestureSwipeV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPointerGestureSwipeV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPointerGestureSwipeV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpPointerGestureSwipeV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpPointerGestureSwipeV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.begin((proxy as *mut ZwpPointerGestureSwipeV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ((*args.add(2)).o as *mut WlSurface).as_mut(), (*args.add(3)).u, ),
						1 => listener.update((proxy as *mut ZwpPointerGestureSwipeV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f, ),
						2 => listener.end((proxy as *mut ZwpPointerGestureSwipeV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).i, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `begin` ARGS: serial: {:?}, time: {:?}, surface: {:?}, fingers: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, ((*args.add(2)).o as *mut WlSurface).as_mut(), (*args.add(3)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `update` ARGS: time: {:?}, dx: {:?}, dy: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `end` ARGS: serial: {:?}, time: {:?}, cancelled: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).i),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpPointerGestureSwipeV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the pointer swipe gesture object
		///
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpPointerGestureSwipeV1Listener: std::any::Any {
		
		/// # multi-finger swipe begin
		///
		/// This event is sent when a multi-finger swipe gesture is detected
		/// on the device.
		fn begin(
			&self,
			proxy: &mut ZwpPointerGestureSwipeV1,
			serial          : u32,
			time            : u32,
			surface         : Option<&mut WlSurface>,
			fingers         : u32,
		);
		
		/// # multi-finger swipe motion
		///
		/// This event is sent when a multi-finger swipe gesture changes the
		/// position of the logical center.
		///
		/// The dx and dy coordinates are relative coordinates of the logical
		/// center of the gesture compared to the previous event.
		fn update(
			&self,
			proxy: &mut ZwpPointerGestureSwipeV1,
			time            : u32,
			dx              : WlFixed,
			dy              : WlFixed,
		);
		
		/// # multi-finger swipe end
		///
		/// This event is sent when a multi-finger swipe gesture ceases to
		/// be valid. This may happen when one or more fingers are lifted or
		/// the gesture is cancelled.
		///
		/// When a gesture is cancelled, the client should undo state changes
		/// caused by this gesture. What causes a gesture to be cancelled is
		/// implementation-dependent.
		fn end(
			&self,
			proxy: &mut ZwpPointerGestureSwipeV1,
			serial          : u32,
			time            : u32,
			cancelled       : i32,
		);
	}
	
	pub static ZWP_POINTER_GESTURE_PINCH_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_pointer_gesture_pinch_v1\0".as_ptr(),
		version:      2,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  3,
		events:       [
			WlMessage {
				name:      "begin\0".as_ptr(),
				signature: "uuou\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "update\0".as_ptr(),
				signature: "uffff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "end\0".as_ptr(),
				signature: "uui\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # a pinch gesture object
	///
	/// A pinch gesture object notifies a client about a multi-finger pinch
	/// gesture detected on an indirect input device such as a touchpad.
	/// The gesture is usually initiated by multiple fingers moving towards
	/// each other or away from each other, or by two or more fingers rotating
	/// around a logical center of gravity. The precise conditions of when
	/// such a gesture is detected are implementation-dependent.
	///
	/// A gesture consists of three stages: begin, update (optional) and end.
	/// There cannot be multiple simultaneous pinch or swipe gestures on a
	/// same pointer/seat, how compositors prevent these situations is
	/// implementation-dependent.
	///
	/// A gesture may be cancelled by the compositor or the hardware.
	/// Clients should not consider performing permanent or irreversible
	/// actions until the end of a gesture has been received.
	pub struct ZwpPointerGesturePinchV1(WlProxy);
	
	impl std::ops::Deref for ZwpPointerGesturePinchV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPointerGesturePinchV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPointerGesturePinchV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPointerGesturePinchV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPointerGesturePinchV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpPointerGesturePinchV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpPointerGesturePinchV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.begin((proxy as *mut ZwpPointerGesturePinchV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ((*args.add(2)).o as *mut WlSurface).as_mut(), (*args.add(3)).u, ),
						1 => listener.update((proxy as *mut ZwpPointerGesturePinchV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f, (*args.add(3)).f, (*args.add(4)).f, ),
						2 => listener.end((proxy as *mut ZwpPointerGesturePinchV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).i, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `begin` ARGS: serial: {:?}, time: {:?}, surface: {:?}, fingers: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, ((*args.add(2)).o as *mut WlSurface).as_mut(), (*args.add(3)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `update` ARGS: time: {:?}, dx: {:?}, dy: {:?}, scale: {:?}, rotation: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).f, (*args.add(2)).f, (*args.add(3)).f, (*args.add(4)).f),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `end` ARGS: serial: {:?}, time: {:?}, cancelled: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).i),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpPointerGesturePinchV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the pinch gesture object
		///
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpPointerGesturePinchV1Listener: std::any::Any {
		
		/// # multi-finger pinch begin
		///
		/// This event is sent when a multi-finger pinch gesture is detected
		/// on the device.
		fn begin(
			&self,
			proxy: &mut ZwpPointerGesturePinchV1,
			serial          : u32,
			time            : u32,
			surface         : Option<&mut WlSurface>,
			fingers         : u32,
		);
		
		/// # multi-finger pinch motion
		///
		/// This event is sent when a multi-finger pinch gesture changes the
		/// position of the logical center, the rotation or the relative scale.
		///
		/// The dx and dy coordinates are relative coordinates in the
		/// surface coordinate space of the logical center of the gesture.
		///
		/// The scale factor is an absolute scale compared to the
		/// pointer_gesture_pinch.begin event, e.g. a scale of 2 means the fingers
		/// are now twice as far apart as on pointer_gesture_pinch.begin.
		///
		/// The rotation is the relative angle in degrees clockwise compared to the previous
		/// pointer_gesture_pinch.begin or pointer_gesture_pinch.update event.
		fn update(
			&self,
			proxy: &mut ZwpPointerGesturePinchV1,
			time            : u32,
			dx              : WlFixed,
			dy              : WlFixed,
			scale           : WlFixed,
			rotation        : WlFixed,
		);
		
		/// # multi-finger pinch end
		///
		/// This event is sent when a multi-finger pinch gesture ceases to
		/// be valid. This may happen when one or more fingers are lifted or
		/// the gesture is cancelled.
		///
		/// When a gesture is cancelled, the client should undo state changes
		/// caused by this gesture. What causes a gesture to be cancelled is
		/// implementation-dependent.
		fn end(
			&self,
			proxy: &mut ZwpPointerGesturePinchV1,
			serial          : u32,
			time            : u32,
			cancelled       : i32,
		);
	}
}

/// # protocol for constraining pointer motions
///
/// This protocol specifies a set of interfaces used for adding constraints to
/// the motion of a pointer. Possible constraints include confining pointer
/// motions to a given region, or locking it to its current position.
///
/// In order to constrain the pointer, a client must first bind the global
/// interface "wp_pointer_constraints" which, if a compositor supports pointer
/// constraints, is exposed by the registry. Using the bound global object, the
/// client uses the request that corresponds to the type of constraint it wants
/// to make. See wp_pointer_constraints for more details.
///
/// Warning! The protocol described in this file is experimental and backward
/// incompatible changes may be made. Backward compatible changes may be added
/// together with the corresponding interface version bump. Backward
/// incompatible changes are done by bumping the version number in the protocol
/// and interface names and resetting the interface version. Once the protocol
/// is to be declared stable, the 'z' prefix and the version number in the
/// protocol and interface names are removed and the interface version number is
/// reset.
pub use pointer_constraints_unstable_v1::*;
mod pointer_constraints_unstable_v1 {
	use crate::*;
	
	// Copyright © 2014      Jonas Ådahl
	// Copyright © 2015      Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_POINTER_CONSTRAINTS_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_pointer_constraints_v1\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "lock_pointer\0".as_ptr(),
				signature: "noo?ou\0".as_ptr(),
				types:     [&ZWP_LOCKED_POINTER_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_POINTER_INTERFACE as _, &WL_REGION_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "confine_pointer\0".as_ptr(),
				signature: "noo?ou\0".as_ptr(),
				types:     [&ZWP_CONFINED_POINTER_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_POINTER_INTERFACE as _, &WL_REGION_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # constrain the movement of a pointer
	///
	/// The global interface exposing pointer constraining functionality. It
	/// exposes two requests: lock_pointer for locking the pointer to its
	/// position, and confine_pointer for locking the pointer to a region.
	///
	/// The lock_pointer and confine_pointer requests create the objects
	/// wp_locked_pointer and wp_confined_pointer respectively, and the client can
	/// use these objects to interact with the lock.
	///
	/// For any surface, only one lock or confinement may be active across all
	/// wl_pointer objects of the same seat. If a lock or confinement is requested
	/// when another lock or confinement is active or requested on the same surface
	/// and with any of the wl_pointer objects of the same seat, an
	/// 'already_constrained' error will be raised.
	pub struct ZwpPointerConstraintsV1(WlProxy);
	
	impl std::ops::Deref for ZwpPointerConstraintsV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpPointerConstraintsV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpPointerConstraintsV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpPointerConstraintsV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpPointerConstraintsV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the pointer constraints manager object
		///
		/// Used by the client to notify the server that it will no longer use this
		/// pointer constraints object.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # lock pointer to a position
		///
		/// The lock_pointer request lets the client request to disable movements of
		/// the virtual pointer (i.e. the cursor), effectively locking the pointer
		/// to a position. This request may not take effect immediately; in the
		/// future, when the compositor deems implementation-specific constraints
		/// are satisfied, the pointer lock will be activated and the compositor
		/// sends a locked event.
		///
		/// The protocol provides no guarantee that the constraints are ever
		/// satisfied, and does not require the compositor to send an error if the
		/// constraints cannot ever be satisfied. It is thus possible to request a
		/// lock that will never activate.
		///
		/// There may not be another pointer constraint of any kind requested or
		/// active on the surface for any of the wl_pointer objects of the seat of
		/// the passed pointer when requesting a lock. If there is, an error will be
		/// raised. See general pointer lock documentation for more details.
		///
		/// The intersection of the region passed with this request and the input
		/// region of the surface is used to determine where the pointer must be
		/// in order for the lock to activate. It is up to the compositor whether to
		/// warp the pointer or require some kind of user interaction for the lock
		/// to activate. If the region is null the surface input region is used.
		///
		/// A surface may receive pointer focus without the lock being activated.
		///
		/// The request creates a new object wp_locked_pointer which is used to
		/// interact with the lock as well as receive updates about its state. See
		/// the the description of wp_locked_pointer for further information.
		///
		/// Note that while a pointer is locked, the wl_pointer objects of the
		/// corresponding seat will not emit any wl_pointer.motion events, but
		/// relative motion events will still be emitted via wp_relative_pointer
		/// objects of the same seat. wl_pointer.axis and wl_pointer.button events
		/// are unaffected.
		pub fn lock_pointer(
			&self,
			surface         : &WlSurface,
			pointer         : &WlPointer,
			region          : Option<&WlRegion>,
			lifetime        : u32
		) -> Result<Box<ZwpLockedPointerV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_LOCKED_POINTER_V1_INTERFACE, std::ptr::null::<u8>(), surface, pointer, region.map_or(std::ptr::null_mut(), |r| r as *const WlRegion as *mut WlRegion), lifetime) as *mut ZwpLockedPointerV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # confine pointer to a region
		///
		/// The confine_pointer request lets the client request to confine the
		/// pointer cursor to a given region. This request may not take effect
		/// immediately; in the future, when the compositor deems implementation-
		/// specific constraints are satisfied, the pointer confinement will be
		/// activated and the compositor sends a confined event.
		///
		/// The intersection of the region passed with this request and the input
		/// region of the surface is used to determine where the pointer must be
		/// in order for the confinement to activate. It is up to the compositor
		/// whether to warp the pointer or require some kind of user interaction for
		/// the confinement to activate. If the region is null the surface input
		/// region is used.
		///
		/// The request will create a new object wp_confined_pointer which is used
		/// to interact with the confinement as well as receive updates about its
		/// state. See the the description of wp_confined_pointer for further
		/// information.
		pub fn confine_pointer(
			&self,
			surface         : &WlSurface,
			pointer         : &WlPointer,
			region          : Option<&WlRegion>,
			lifetime        : u32
		) -> Result<Box<ZwpConfinedPointerV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &ZWP_CONFINED_POINTER_V1_INTERFACE, std::ptr::null::<u8>(), surface, pointer, region.map_or(std::ptr::null_mut(), |r| r as *const WlRegion as *mut WlRegion), lifetime) as *mut ZwpConfinedPointerV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	/// # wp_pointer_constraints error values
	///
	/// These errors can be emitted in response to wp_pointer_constraints
	/// requests.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpPointerConstraintsV1Error {
		/// pointer constraint already requested on that surface
		AlreadyConstrained = 1,
	}
	
	/// # constraint lifetime
	///
	/// These values represent different lifetime semantics. They are passed
	/// as arguments to the factory requests to specify how the constraint
	/// lifetimes should be managed.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpPointerConstraintsV1Lifetime {
		///
		Oneshot = 1,
		///
		Persistent = 2,
	}
	
	pub static ZWP_LOCKED_POINTER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_locked_pointer_v1\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_cursor_position_hint\0".as_ptr(),
				signature: "ff\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_region\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_REGION_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "locked\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "unlocked\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # receive relative pointer motion events
	///
	/// The wp_locked_pointer interface represents a locked pointer state.
	///
	/// While the lock of this object is active, the wl_pointer objects of the
	/// associated seat will not emit any wl_pointer.motion events.
	///
	/// This object will send the event 'locked' when the lock is activated.
	/// Whenever the lock is activated, it is guaranteed that the locked surface
	/// will already have received pointer focus and that the pointer will be
	/// within the region passed to the request creating this object.
	///
	/// To unlock the pointer, send the destroy request. This will also destroy
	/// the wp_locked_pointer object.
	///
	/// If the compositor decides to unlock the pointer the unlocked event is
	/// sent. See wp_locked_pointer.unlock for details.
	///
	/// When unlocking, the compositor may warp the cursor position to the set
	/// cursor position hint. If it does, it will not result in any relative
	/// motion events emitted via wp_relative_pointer.
	///
	/// If the surface the lock was requested on is destroyed and the lock is not
	/// yet activated, the wp_locked_pointer object is now defunct and must be
	/// destroyed.
	pub struct ZwpLockedPointerV1(WlProxy);
	
	impl std::ops::Deref for ZwpLockedPointerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpLockedPointerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpLockedPointerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpLockedPointerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpLockedPointerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpLockedPointerV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpLockedPointerV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.locked((proxy as *mut ZwpLockedPointerV1).as_mut().unwrap(), ),
						1 => listener.unlocked((proxy as *mut ZwpLockedPointerV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `locked` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `unlocked` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpLockedPointerV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the locked pointer object
		///
		/// Destroy the locked pointer object. If applicable, the compositor will
		/// unlock the pointer.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the pointer cursor position hint
		///
		/// Set the cursor position hint relative to the top left corner of the
		/// surface.
		///
		/// If the client is drawing its own cursor, it should update the position
		/// hint to the position of its own cursor. A compositor may use this
		/// information to warp the pointer upon unlock in order to avoid pointer
		/// jumps.
		///
		/// The cursor position hint is double buffered. The new hint will only take
		/// effect when the associated surface gets it pending state applied. See
		/// wl_surface.commit for details.
		pub fn set_cursor_position_hint(
			&self,
			surface_x       : WlFixed,
			surface_y       : WlFixed
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, surface_x, surface_y); }
		}
		
		/// # set a new lock region
		///
		/// Set a new region used to lock the pointer.
		///
		/// The new lock region is double-buffered. The new lock region will
		/// only take effect when the associated surface gets its pending state
		/// applied. See wl_surface.commit for details.
		///
		/// For details about the lock region, see wp_locked_pointer.
		pub fn set_region(
			&self,
			region          : Option<&WlRegion>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, region.map_or(std::ptr::null_mut(), |r| r as *const WlRegion as *mut WlRegion)); }
		}
	}
	
	
	pub trait ZwpLockedPointerV1Listener: std::any::Any {
		
		/// # lock activation event
		///
		/// Notification that the pointer lock of the seat's pointer is activated.
		fn locked(
			&self,
			proxy: &mut ZwpLockedPointerV1,
		);
		
		/// # lock deactivation event
		///
		/// Notification that the pointer lock of the seat's pointer is no longer
		/// active. If this is a oneshot pointer lock (see
		/// wp_pointer_constraints.lifetime) this object is now defunct and should
		/// be destroyed. If this is a persistent pointer lock (see
		/// wp_pointer_constraints.lifetime) this pointer lock may again
		/// reactivate in the future.
		fn unlocked(
			&self,
			proxy: &mut ZwpLockedPointerV1,
		);
	}
	
	pub static ZWP_CONFINED_POINTER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_confined_pointer_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_region\0".as_ptr(),
				signature: "?o\0".as_ptr(),
				types:     [&WL_REGION_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "confined\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "unconfined\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # confined pointer object
	///
	/// The wp_confined_pointer interface represents a confined pointer state.
	///
	/// This object will send the event 'confined' when the confinement is
	/// activated. Whenever the confinement is activated, it is guaranteed that
	/// the surface the pointer is confined to will already have received pointer
	/// focus and that the pointer will be within the region passed to the request
	/// creating this object. It is up to the compositor to decide whether this
	/// requires some user interaction and if the pointer will warp to within the
	/// passed region if outside.
	///
	/// To unconfine the pointer, send the destroy request. This will also destroy
	/// the wp_confined_pointer object.
	///
	/// If the compositor decides to unconfine the pointer the unconfined event is
	/// sent. The wp_confined_pointer object is at this point defunct and should
	/// be destroyed.
	pub struct ZwpConfinedPointerV1(WlProxy);
	
	impl std::ops::Deref for ZwpConfinedPointerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpConfinedPointerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpConfinedPointerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpConfinedPointerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpConfinedPointerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpConfinedPointerV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpConfinedPointerV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.confined((proxy as *mut ZwpConfinedPointerV1).as_mut().unwrap(), ),
						1 => listener.unconfined((proxy as *mut ZwpConfinedPointerV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `confined` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `unconfined` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpConfinedPointerV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the confined pointer object
		///
		/// Destroy the confined pointer object. If applicable, the compositor will
		/// unconfine the pointer.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set a new confine region
		///
		/// Set a new region used to confine the pointer.
		///
		/// The new confine region is double-buffered. The new confine region will
		/// only take effect when the associated surface gets its pending state
		/// applied. See wl_surface.commit for details.
		///
		/// If the confinement is active when the new confinement region is applied
		/// and the pointer ends up outside of newly applied region, the pointer may
		/// warped to a position within the new confinement region. If warped, a
		/// wl_pointer.motion event will be emitted, but no
		/// wp_relative_pointer.relative_motion event.
		///
		/// The compositor may also, instead of using the new region, unconfine the
		/// pointer.
		///
		/// For details about the confine region, see wp_confined_pointer.
		pub fn set_region(
			&self,
			region          : Option<&WlRegion>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, region.map_or(std::ptr::null_mut(), |r| r as *const WlRegion as *mut WlRegion)); }
		}
	}
	
	
	pub trait ZwpConfinedPointerV1Listener: std::any::Any {
		
		/// # pointer confined
		///
		/// Notification that the pointer confinement of the seat's pointer is
		/// activated.
		fn confined(
			&self,
			proxy: &mut ZwpConfinedPointerV1,
		);
		
		/// # pointer unconfined
		///
		/// Notification that the pointer confinement of the seat's pointer is no
		/// longer active. If this is a oneshot pointer confinement (see
		/// wp_pointer_constraints.lifetime) this object is now defunct and should
		/// be destroyed. If this is a persistent pointer confinement (see
		/// wp_pointer_constraints.lifetime) this pointer confinement may again
		/// reactivate in the future.
		fn unconfined(
			&self,
			proxy: &mut ZwpConfinedPointerV1,
		);
	}
}
pub use zwp_linux_explicit_synchronization_unstable_v1::*;
mod zwp_linux_explicit_synchronization_unstable_v1 {
	use crate::*;
	
	// Copyright 2016 The Chromium Authors.
	// Copyright 2017 Intel Corporation
	// Copyright 2018 Collabora, Ltd
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_LINUX_EXPLICIT_SYNCHRONIZATION_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_linux_explicit_synchronization_v1\0".as_ptr(),
		version:      2,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_synchronization\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_LINUX_SURFACE_SYNCHRONIZATION_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # protocol for providing explicit synchronization
	///
	/// This global is a factory interface, allowing clients to request
	/// explicit synchronization for buffers on a per-surface basis.
	///
	/// See zwp_linux_surface_synchronization_v1 for more information.
	///
	/// This interface is derived from Chromium's
	/// zcr_linux_explicit_synchronization_v1.
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZwpLinuxExplicitSynchronizationV1(WlProxy);
	
	impl std::ops::Deref for ZwpLinuxExplicitSynchronizationV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpLinuxExplicitSynchronizationV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpLinuxExplicitSynchronizationV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpLinuxExplicitSynchronizationV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpLinuxExplicitSynchronizationV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy explicit synchronization factory object
		///
		/// Destroy this explicit synchronization factory object. Other objects,
		/// including zwp_linux_surface_synchronization_v1 objects created by this
		/// factory, shall not be affected by this request.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # extend surface interface for explicit synchronization
		///
		/// Instantiate an interface extension for the given wl_surface to provide
		/// explicit synchronization.
		///
		/// If the given wl_surface already has an explicit synchronization object
		/// associated, the synchronization_exists protocol error is raised.
		///
		/// Graphics APIs, like EGL or Vulkan, that manage the buffer queue and
		/// commits of a wl_surface themselves, are likely to be using this
		/// extension internally. If a client is using such an API for a
		/// wl_surface, it should not directly use this extension on that surface,
		/// to avoid raising a synchronization_exists protocol error.
		pub fn get_synchronization(
			&self,
			surface         : &WlSurface
		) -> Result<Box<ZwpLinuxSurfaceSynchronizationV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_LINUX_SURFACE_SYNCHRONIZATION_V1_INTERFACE, std::ptr::null::<u8>(), surface) as *mut ZwpLinuxSurfaceSynchronizationV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpLinuxExplicitSynchronizationV1Error {
		/// the surface already has a synchronization object associated
		SynchronizationExists = 0,
	}
	
	pub static ZWP_LINUX_SURFACE_SYNCHRONIZATION_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_linux_surface_synchronization_v1\0".as_ptr(),
		version:      2,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "set_acquire_fence\0".as_ptr(),
				signature: "h\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_release\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_LINUX_BUFFER_RELEASE_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # per-surface explicit synchronization support
	///
	/// This object implements per-surface explicit synchronization.
	///
	/// Synchronization refers to co-ordination of pipelined operations performed
	/// on buffers. Most GPU clients will schedule an asynchronous operation to
	/// render to the buffer, then immediately send the buffer to the compositor
	/// to be attached to a surface.
	///
	/// In implicit synchronization, ensuring that the rendering operation is
	/// complete before the compositor displays the buffer is an implementation
	/// detail handled by either the kernel or userspace graphics driver.
	///
	/// By contrast, in explicit synchronization, dma_fence objects mark when the
	/// asynchronous operations are complete. When submitting a buffer, the
	/// client provides an acquire fence which will be waited on before the
	/// compositor accesses the buffer. The Wayland server, through a
	/// zwp_linux_buffer_release_v1 object, will inform the client with an event
	/// which may be accompanied by a release fence, when the compositor will no
	/// longer access the buffer contents due to the specific commit that
	/// requested the release event.
	///
	/// Each surface can be associated with only one object of this interface at
	/// any time.
	///
	/// In version 1 of this interface, explicit synchronization is only
	/// guaranteed to be supported for buffers created with any version of the
	/// wp_linux_dmabuf buffer factory. Version 2 additionally guarantees
	/// explicit synchronization support for opaque EGL buffers, which is a type
	/// of platform specific buffers described in the EGL_WL_bind_wayland_display
	/// extension. Compositors are free to support explicit synchronization for
	/// additional buffer types.
	pub struct ZwpLinuxSurfaceSynchronizationV1(WlProxy);
	
	impl std::ops::Deref for ZwpLinuxSurfaceSynchronizationV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpLinuxSurfaceSynchronizationV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpLinuxSurfaceSynchronizationV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpLinuxSurfaceSynchronizationV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpLinuxSurfaceSynchronizationV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy synchronization object
		///
		/// Destroy this explicit synchronization object.
		///
		/// Any fence set by this object with set_acquire_fence since the last
		/// commit will be discarded by the server. Any fences set by this object
		/// before the last commit are not affected.
		///
		/// zwp_linux_buffer_release_v1 objects created by this object are not
		/// affected by this request.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the acquire fence
		///
		/// Set the acquire fence that must be signaled before the compositor
		/// may sample from the buffer attached with wl_surface.attach. The fence
		/// is a dma_fence kernel object.
		///
		/// The acquire fence is double-buffered state, and will be applied on the
		/// next wl_surface.commit request for the associated surface. Thus, it
		/// applies only to the buffer that is attached to the surface at commit
		/// time.
		///
		/// If the provided fd is not a valid dma_fence fd, then an INVALID_FENCE
		/// error is raised.
		///
		/// If a fence has already been attached during the same commit cycle, a
		/// DUPLICATE_FENCE error is raised.
		///
		/// If the associated wl_surface was destroyed, a NO_SURFACE error is
		/// raised.
		///
		/// If at surface commit time the attached buffer does not support explicit
		/// synchronization, an UNSUPPORTED_BUFFER error is raised.
		///
		/// If at surface commit time there is no buffer attached, a NO_BUFFER
		/// error is raised.
		pub fn set_acquire_fence(
			&self,
			fd              : RawFd
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, fd); }
		}
		
		/// # release fence for last-attached buffer
		///
		/// Create a listener for the release of the buffer attached by the
		/// client with wl_surface.attach. See zwp_linux_buffer_release_v1
		/// documentation for more information.
		///
		/// The release object is double-buffered state, and will be associated
		/// with the buffer that is attached to the surface at wl_surface.commit
		/// time.
		///
		/// If a zwp_linux_buffer_release_v1 object has already been requested for
		/// the surface in the same commit cycle, a DUPLICATE_RELEASE error is
		/// raised.
		///
		/// If the associated wl_surface was destroyed, a NO_SURFACE error
		/// is raised.
		///
		/// If at surface commit time there is no buffer attached, a NO_BUFFER
		/// error is raised.
		pub fn get_release(
			&self
		) -> Result<Box<ZwpLinuxBufferReleaseV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &ZWP_LINUX_BUFFER_RELEASE_V1_INTERFACE, std::ptr::null::<u8>()) as *mut ZwpLinuxBufferReleaseV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpLinuxSurfaceSynchronizationV1Error {
		/// the fence specified by the client could not be imported
		InvalidFence = 0,
		/// multiple fences added for a single surface commit
		DuplicateFence = 1,
		/// multiple releases added for a single surface commit
		DuplicateRelease = 2,
		/// the associated wl_surface was destroyed
		NoSurface = 3,
		/// the buffer does not support explicit synchronization
		UnsupportedBuffer = 4,
		/// no buffer was attached
		NoBuffer = 5,
	}
	
	pub static ZWP_LINUX_BUFFER_RELEASE_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_linux_buffer_release_v1\0".as_ptr(),
		version:      1,
		method_count: 0,
		methods:      [
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "fenced_release\0".as_ptr(),
				signature: "h\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "immediate_release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # buffer release explicit synchronization
	///
	/// This object is instantiated in response to a
	/// zwp_linux_surface_synchronization_v1.get_release request.
	///
	/// It provides an alternative to wl_buffer.release events, providing a
	/// unique release from a single wl_surface.commit request. The release event
	/// also supports explicit synchronization, providing a fence FD for the
	/// client to synchronize against.
	///
	/// Exactly one event, either a fenced_release or an immediate_release, will
	/// be emitted for the wl_surface.commit request. The compositor can choose
	/// release by release which event it uses.
	///
	/// This event does not replace wl_buffer.release events; servers are still
	/// required to send those events.
	///
	/// Once a buffer release object has delivered a 'fenced_release' or an
	/// 'immediate_release' event it is automatically destroyed.
	pub struct ZwpLinuxBufferReleaseV1(WlProxy);
	
	impl std::ops::Deref for ZwpLinuxBufferReleaseV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpLinuxBufferReleaseV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpLinuxBufferReleaseV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpLinuxBufferReleaseV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpLinuxBufferReleaseV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpLinuxBufferReleaseV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpLinuxBufferReleaseV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.fenced_release((proxy as *mut ZwpLinuxBufferReleaseV1).as_mut().unwrap(), (*args.add(0)).h, ),
						1 => listener.immediate_release((proxy as *mut ZwpLinuxBufferReleaseV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `fenced_release` ARGS: fence: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).h),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `immediate_release` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpLinuxBufferReleaseV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpLinuxBufferReleaseV1Listener: std::any::Any {
		
		/// # release buffer with fence
		///
		/// Sent when the compositor has finalised its usage of the associated
		/// buffer for the relevant commit, providing a dma_fence which will be
		/// signaled when all operations by the compositor on that buffer for that
		/// commit have finished.
		///
		/// Once the fence has signaled, and assuming the associated buffer is not
		/// pending release from other wl_surface.commit requests, no additional
		/// explicit or implicit synchronization is required to safely reuse or
		/// destroy the buffer.
		///
		/// This event destroys the zwp_linux_buffer_release_v1 object.
		fn fenced_release(
			&self,
			proxy: &mut ZwpLinuxBufferReleaseV1,
			fence           : RawFd,
		);
		
		/// # release buffer immediately
		///
		/// Sent when the compositor has finalised its usage of the associated
		/// buffer for the relevant commit, and either performed no operations
		/// using it, or has a guarantee that all its operations on that buffer for
		/// that commit have finished.
		///
		/// Once this event is received, and assuming the associated buffer is not
		/// pending release from other wl_surface.commit requests, no additional
		/// explicit or implicit synchronization is required to safely reuse or
		/// destroy the buffer.
		///
		/// This event destroys the zwp_linux_buffer_release_v1 object.
		fn immediate_release(
			&self,
			proxy: &mut ZwpLinuxBufferReleaseV1,
		);
	}
}
pub use linux_dmabuf_unstable_v1::*;
mod linux_dmabuf_unstable_v1 {
	use crate::*;
	
	// Copyright © 2014, 2015 Collabora, Ltd.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_LINUX_DMABUF_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_linux_dmabuf_v1\0".as_ptr(),
		version:      3,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "create_params\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_LINUX_BUFFER_PARAMS_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "format\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "modifier\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # factory for creating dmabuf-based wl_buffers
	///
	/// Following the interfaces from:
	/// https://www.khronos.org/registry/egl/extensions/EXT/EGL_EXT_image_dma_buf_import.txt
	/// https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt
	/// and the Linux DRM sub-system's AddFb2 ioctl.
	///
	/// This interface offers ways to create generic dmabuf-based
	/// wl_buffers. Immediately after a client binds to this interface,
	/// the set of supported formats and format modifiers is sent with
	/// 'format' and 'modifier' events.
	///
	/// The following are required from clients:
	///
	/// - Clients must ensure that either all data in the dma-buf is
	/// coherent for all subsequent read access or that coherency is
	/// correctly handled by the underlying kernel-side dma-buf
	/// implementation.
	///
	/// - Don't make any more attachments after sending the buffer to the
	/// compositor. Making more attachments later increases the risk of
	/// the compositor not being able to use (re-import) an existing
	/// dmabuf-based wl_buffer.
	///
	/// The underlying graphics stack must ensure the following:
	///
	/// - The dmabuf file descriptors relayed to the server will stay valid
	/// for the whole lifetime of the wl_buffer. This means the server may
	/// at any time use those fds to import the dmabuf into any kernel
	/// sub-system that might accept it.
	///
	/// To create a wl_buffer from one or more dmabufs, a client creates a
	/// zwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params
	/// request. All planes required by the intended format are added with
	/// the 'add' request. Finally, a 'create' or 'create_immed' request is
	/// issued, which has the following outcome depending on the import success.
	///
	/// The 'create' request,
	/// - on success, triggers a 'created' event which provides the final
	/// wl_buffer to the client.
	/// - on failure, triggers a 'failed' event to convey that the server
	/// cannot use the dmabufs received from the client.
	///
	/// For the 'create_immed' request,
	/// - on success, the server immediately imports the added dmabufs to
	/// create a wl_buffer. No event is sent from the server in this case.
	/// - on failure, the server can choose to either:
	/// - terminate the client by raising a fatal error.
	/// - mark the wl_buffer as failed, and send a 'failed' event to the
	/// client. If the client uses a failed wl_buffer as an argument to any
	/// request, the behaviour is compositor implementation-defined.
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZwpLinuxDmabufV1(WlProxy);
	
	impl std::ops::Deref for ZwpLinuxDmabufV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpLinuxDmabufV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpLinuxDmabufV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpLinuxDmabufV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpLinuxDmabufV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpLinuxDmabufV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpLinuxDmabufV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.format((proxy as *mut ZwpLinuxDmabufV1).as_mut().unwrap(), (*args.add(0)).u, ),
						1 => listener.modifier((proxy as *mut ZwpLinuxDmabufV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `format` ARGS: format: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `modifier` ARGS: format: {:?}, modifier_hi: {:?}, modifier_lo: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpLinuxDmabufV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # unbind the factory
		///
		/// Objects created through this interface, especially wl_buffers, will
		/// remain valid.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a temporary object for buffer parameters
		///
		/// This temporary object is used to collect multiple dmabuf handles into
		/// a single batch to create a wl_buffer. It can only be used once and
		/// should be destroyed after a 'created' or 'failed' event has been
		/// received.
		pub fn create_params(
			&self
		) -> Result<Box<ZwpLinuxBufferParamsV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_LINUX_BUFFER_PARAMS_V1_INTERFACE, std::ptr::null::<u8>()) as *mut ZwpLinuxBufferParamsV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub trait ZwpLinuxDmabufV1Listener: std::any::Any {
		
		/// # supported buffer format
		///
		/// This event advertises one buffer format that the server supports.
		/// All the supported formats are advertised once when the client
		/// binds to this interface. A roundtrip after binding guarantees
		/// that the client has received all supported formats.
		///
		/// For the definition of the format codes, see the
		/// zwp_linux_buffer_params_v1::create request.
		///
		/// Warning: the 'format' event is likely to be deprecated and replaced
		/// with the 'modifier' event introduced in zwp_linux_dmabuf_v1
		/// version 3, described below. Please refrain from using the information
		/// received from this event.
		fn format(
			&self,
			proxy: &mut ZwpLinuxDmabufV1,
			format          : u32,
		);
		
		/// # supported buffer format modifier
		///
		/// This event advertises the formats that the server supports, along with
		/// the modifiers supported for each format. All the supported modifiers
		/// for all the supported formats are advertised once when the client
		/// binds to this interface. A roundtrip after binding guarantees that
		/// the client has received all supported format-modifier pairs.
		///
		/// For legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi ==
		/// 0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event.
		/// It indicates that the server can support the format with an implicit
		/// modifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it
		/// is as if no explicit modifier is specified. The effective modifier
		/// will be derived from the dmabuf.
		///
		/// For the definition of the format and modifier codes, see the
		/// zwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add
		/// requests.
		fn modifier(
			&self,
			proxy: &mut ZwpLinuxDmabufV1,
			format          : u32,
			modifier_hi     : u32,
			modifier_lo     : u32,
		);
	}
	
	pub static ZWP_LINUX_BUFFER_PARAMS_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_linux_buffer_params_v1\0".as_ptr(),
		version:      3,
		method_count: 4,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "add\0".as_ptr(),
				signature: "huuuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "create\0".as_ptr(),
				signature: "iiuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "create_immed\0".as_ptr(),
				signature: "niiuu\0".as_ptr(),
				types:     [&WL_BUFFER_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "created\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_BUFFER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "failed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # parameters for creating a dmabuf-based wl_buffer
	///
	/// This temporary object is a collection of dmabufs and other
	/// parameters that together form a single logical buffer. The temporary
	/// object may eventually create one wl_buffer unless cancelled by
	/// destroying it before requesting 'create'.
	///
	/// Single-planar formats only require one dmabuf, however
	/// multi-planar formats may require more than one dmabuf. For all
	/// formats, an 'add' request must be called once per plane (even if the
	/// underlying dmabuf fd is identical).
	///
	/// You must use consecutive plane indices ('plane_idx' argument for 'add')
	/// from zero to the number of planes used by the drm_fourcc format code.
	/// All planes required by the format must be given exactly once, but can
	/// be given in any order. Each plane index can be set only once.
	pub struct ZwpLinuxBufferParamsV1(WlProxy);
	
	impl std::ops::Deref for ZwpLinuxBufferParamsV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpLinuxBufferParamsV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpLinuxBufferParamsV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpLinuxBufferParamsV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpLinuxBufferParamsV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpLinuxBufferParamsV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpLinuxBufferParamsV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.created((proxy as *mut ZwpLinuxBufferParamsV1).as_mut().unwrap(), (*args.add(0)).n, ),
						1 => listener.failed((proxy as *mut ZwpLinuxBufferParamsV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `created` ARGS: buffer: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `failed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpLinuxBufferParamsV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # delete this object, used or not
		///
		/// Cleans up the temporary data sent to the server for dmabuf-based
		/// wl_buffer creation.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # add a dmabuf to the temporary set
		///
		/// This request adds one dmabuf to the set in this
		/// zwp_linux_buffer_params_v1.
		///
		/// The 64-bit unsigned value combined from modifier_hi and modifier_lo
		/// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
		/// fb modifier, which is defined in drm_mode.h of Linux UAPI.
		/// This is an opaque token. Drivers use this token to express tiling,
		/// compression, etc. driver-specific modifications to the base format
		/// defined by the DRM fourcc code.
		///
		/// Warning: It should be an error if the format/modifier pair was not
		/// advertised with the modifier event. This is not enforced yet because
		/// some implementations always accept DRM_FORMAT_MOD_INVALID. Also
		/// version 2 of this protocol does not have the modifier event.
		///
		/// This request raises the PLANE_IDX error if plane_idx is too large.
		/// The error PLANE_SET is raised if attempting to set a plane that
		/// was already set.
		pub fn add(
			&self,
			fd              : RawFd,
			plane_idx       : u32,
			offset          : u32,
			stride          : u32,
			modifier_hi     : u32,
			modifier_lo     : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, fd, plane_idx, offset, stride, modifier_hi, modifier_lo); }
		}
		
		/// # create a wl_buffer from the given dmabufs
		///
		/// This asks for creation of a wl_buffer from the added dmabuf
		/// buffers. The wl_buffer is not created immediately but returned via
		/// the 'created' event if the dmabuf sharing succeeds. The sharing
		/// may fail at runtime for reasons a client cannot predict, in
		/// which case the 'failed' event is triggered.
		///
		/// The 'format' argument is a DRM_FORMAT code, as defined by the
		/// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
		/// authoritative source on how the format codes should work.
		///
		/// The 'flags' is a bitfield of the flags defined in enum "flags".
		/// 'y_invert' means the that the image needs to be y-flipped.
		///
		/// Flag 'interlaced' means that the frame in the buffer is not
		/// progressive as usual, but interlaced. An interlaced buffer as
		/// supported here must always contain both top and bottom fields.
		/// The top field always begins on the first pixel row. The temporal
		/// ordering between the two fields is top field first, unless
		/// 'bottom_first' is specified. It is undefined whether 'bottom_first'
		/// is ignored if 'interlaced' is not set.
		///
		/// This protocol does not convey any information about field rate,
		/// duration, or timing, other than the relative ordering between the
		/// two fields in one buffer. A compositor may have to estimate the
		/// intended field rate from the incoming buffer rate. It is undefined
		/// whether the time of receiving wl_surface.commit with a new buffer
		/// attached, applying the wl_surface state, wl_surface.frame callback
		/// trigger, presentation, or any other point in the compositor cycle
		/// is used to measure the frame or field times. There is no support
		/// for detecting missed or late frames/fields/buffers either, and
		/// there is no support whatsoever for cooperating with interlaced
		/// compositor output.
		///
		/// The composited image quality resulting from the use of interlaced
		/// buffers is explicitly undefined. A compositor may use elaborate
		/// hardware features or software to deinterlace and create progressive
		/// output frames from a sequence of interlaced input buffers, or it
		/// may produce substandard image quality. However, compositors that
		/// cannot guarantee reasonable image quality in all cases are recommended
		/// to just reject all interlaced buffers.
		///
		/// Any argument errors, including non-positive width or height,
		/// mismatch between the number of planes and the format, bad
		/// format, bad offset or stride, may be indicated by fatal protocol
		/// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
		/// OUT_OF_BOUNDS.
		///
		/// Dmabuf import errors in the server that are not obvious client
		/// bugs are returned via the 'failed' event as non-fatal. This
		/// allows attempting dmabuf sharing and falling back in the client
		/// if it fails.
		///
		/// This request can be sent only once in the object's lifetime, after
		/// which the only legal request is destroy. This object should be
		/// destroyed after issuing a 'create' request. Attempting to use this
		/// object after issuing 'create' raises ALREADY_USED protocol error.
		///
		/// It is not mandatory to issue 'create'. If a client wants to
		/// cancel the buffer creation, it can just destroy this object.
		pub fn create(
			&self,
			width           : i32,
			height          : i32,
			format          : u32,
			flags           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, width, height, format, flags); }
		}
		
		/// # immediately create a wl_buffer from the givendmabufs
		///
		/// This asks for immediate creation of a wl_buffer by importing the
		/// added dmabufs.
		///
		/// In case of import success, no event is sent from the server, and the
		/// wl_buffer is ready to be used by the client.
		///
		/// Upon import failure, either of the following may happen, as seen fit
		/// by the implementation:
		/// - the client is terminated with one of the following fatal protocol
		/// errors:
		/// - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
		/// in case of argument errors such as mismatch between the number
		/// of planes and the format, bad format, non-positive width or
		/// height, or bad offset or stride.
		/// - INVALID_WL_BUFFER, in case the cause for failure is unknown or
		/// plaform specific.
		/// - the server creates an invalid wl_buffer, marks it as failed and
		/// sends a 'failed' event to the client. The result of using this
		/// invalid wl_buffer as an argument in any request by the client is
		/// defined by the compositor implementation.
		///
		/// This takes the same arguments as a 'create' request, and obeys the
		/// same restrictions.
		pub fn create_immed(
			&self,
			width           : i32,
			height          : i32,
			format          : u32,
			flags           : u32
		) -> Result<Box<WlBuffer, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 3, &WL_BUFFER_INTERFACE, std::ptr::null::<u8>(), width, height, format, flags) as *mut WlBuffer };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub trait ZwpLinuxBufferParamsV1Listener: std::any::Any {
		
		/// # buffer creation succeeded
		///
		/// This event indicates that the attempted buffer creation was
		/// successful. It provides the new wl_buffer referencing the dmabuf(s).
		///
		/// Upon receiving this event, the client should destroy the
		/// zlinux_dmabuf_params object.
		fn created(
			&self,
			proxy: &mut ZwpLinuxBufferParamsV1,
			buffer          : u32,
		);
		
		/// # buffer creation failed
		///
		/// This event indicates that the attempted buffer creation has
		/// failed. It usually means that one of the dmabuf constraints
		/// has not been fulfilled.
		///
		/// Upon receiving this event, the client should destroy the
		/// zlinux_buffer_params object.
		fn failed(
			&self,
			proxy: &mut ZwpLinuxBufferParamsV1,
		);
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpLinuxBufferParamsV1Error {
		/// the dmabuf_batch object has already been used to create a wl_buffer
		AlreadyUsed = 0,
		/// plane index out of bounds
		PlaneIdx = 1,
		/// the plane index was already set
		PlaneSet = 2,
		/// missing or too many planes to create a buffer
		Incomplete = 3,
		/// format not supported
		InvalidFormat = 4,
		/// invalid width or height
		InvalidDimensions = 5,
		/// offset + stride * height goes out of dmabuf bounds
		OutOfBounds = 6,
		/// invalid wl_buffer resulted from importing dmabufs viathe create_immed request on given buffer_params
		InvalidWlBuffer = 7,
	}
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpLinuxBufferParamsV1Flags {
		/// contents are y-inverted
		YInvert = 1,
		/// content is interlaced
		Interlaced = 2,
		/// bottom field first
		BottomFirst = 4,
	}
}

/// # Protocol for inhibiting the compositor keyboard shortcuts
///
/// This protocol specifies a way for a client to request the compositor
/// to ignore its own keyboard shortcuts for a given seat, so that all
/// key events from that seat get forwarded to a surface.
///
/// Warning! The protocol described in this file is experimental and
/// backward incompatible changes may be made. Backward compatible
/// changes may be added together with the corresponding interface
/// version bump.
/// Backward incompatible changes are done by bumping the version
/// number in the protocol and interface names and resetting the
/// interface version. Once the protocol is to be declared stable,
/// the 'z' prefix and the version number in the protocol and
/// interface names are removed and the interface version number is
/// reset.
pub use keyboard_shortcuts_inhibit_unstable_v1::*;
mod keyboard_shortcuts_inhibit_unstable_v1 {
	use crate::*;
	
	// Copyright © 2017 Red Hat Inc.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_KEYBOARD_SHORTCUTS_INHIBIT_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_keyboard_shortcuts_inhibit_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "inhibit_shortcuts\0".as_ptr(),
				signature: "noo\0".as_ptr(),
				types:     [&ZWP_KEYBOARD_SHORTCUTS_INHIBITOR_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _, &WL_SEAT_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # context object for keyboard grab_manager
	///
	/// A global interface used for inhibiting the compositor keyboard shortcuts.
	pub struct ZwpKeyboardShortcutsInhibitManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpKeyboardShortcutsInhibitManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpKeyboardShortcutsInhibitManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpKeyboardShortcutsInhibitManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpKeyboardShortcutsInhibitManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpKeyboardShortcutsInhibitManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the keyboard shortcuts inhibitor object
		///
		/// Destroy the keyboard shortcuts inhibitor manager.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a new keyboard shortcuts inhibitor object
		///
		/// Create a new keyboard shortcuts inhibitor object associated with
		/// the given surface for the given seat.
		///
		/// If shortcuts are already inhibited for the specified seat and surface,
		/// a protocol error "already_inhibited" is raised by the compositor.
		pub fn inhibit_shortcuts(
			&self,
			surface         : &WlSurface,
			seat            : &WlSeat
		) -> Result<Box<ZwpKeyboardShortcutsInhibitorV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_KEYBOARD_SHORTCUTS_INHIBITOR_V1_INTERFACE, std::ptr::null::<u8>(), surface, seat) as *mut ZwpKeyboardShortcutsInhibitorV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpKeyboardShortcutsInhibitManagerV1Error {
		/// the shortcuts are already inhibited for this surface
		AlreadyInhibited = 0,
	}
	
	pub static ZWP_KEYBOARD_SHORTCUTS_INHIBITOR_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_keyboard_shortcuts_inhibitor_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "active\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "inactive\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # context object for keyboard shortcuts inhibitor
	///
	/// A keyboard shortcuts inhibitor instructs the compositor to ignore
	/// its own keyboard shortcuts when the associated surface has keyboard
	/// focus. As a result, when the surface has keyboard focus on the given
	/// seat, it will receive all key events originating from the specified
	/// seat, even those which would normally be caught by the compositor for
	/// its own shortcuts.
	///
	/// The Wayland compositor is however under no obligation to disable
	/// all of its shortcuts, and may keep some special key combo for its own
	/// use, including but not limited to one allowing the user to forcibly
	/// restore normal keyboard events routing in the case of an unwilling
	/// client. The compositor may also use the same key combo to reactivate
	/// an existing shortcut inhibitor that was previously deactivated on
	/// user request.
	///
	/// When the compositor restores its own keyboard shortcuts, an
	/// "inactive" event is emitted to notify the client that the keyboard
	/// shortcuts inhibitor is not effectively active for the surface and
	/// seat any more, and the client should not expect to receive all
	/// keyboard events.
	///
	/// When the keyboard shortcuts inhibitor is inactive, the client has
	/// no way to forcibly reactivate the keyboard shortcuts inhibitor.
	///
	/// The user can chose to re-enable a previously deactivated keyboard
	/// shortcuts inhibitor using any mechanism the compositor may offer,
	/// in which case the compositor will send an "active" event to notify
	/// the client.
	///
	/// If the surface is destroyed, unmapped, or loses the seat's keyboard
	/// focus, the keyboard shortcuts inhibitor becomes irrelevant and the
	/// compositor will restore its own keyboard shortcuts but no "inactive"
	/// event is emitted in this case.
	pub struct ZwpKeyboardShortcutsInhibitorV1(WlProxy);
	
	impl std::ops::Deref for ZwpKeyboardShortcutsInhibitorV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpKeyboardShortcutsInhibitorV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpKeyboardShortcutsInhibitorV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpKeyboardShortcutsInhibitorV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpKeyboardShortcutsInhibitorV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpKeyboardShortcutsInhibitorV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpKeyboardShortcutsInhibitorV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.active((proxy as *mut ZwpKeyboardShortcutsInhibitorV1).as_mut().unwrap(), ),
						1 => listener.inactive((proxy as *mut ZwpKeyboardShortcutsInhibitorV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `active` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `inactive` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpKeyboardShortcutsInhibitorV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the keyboard shortcuts inhibitor object
		///
		/// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpKeyboardShortcutsInhibitorV1Listener: std::any::Any {
		
		/// # shortcuts are inhibited
		///
		/// This event indicates that the shortcut inhibitor is active.
		///
		/// The compositor sends this event every time compositor shortcuts
		/// are inhibited on behalf of the surface. When active, the client
		/// may receive input events normally reserved by the compositor
		/// (see zwp_keyboard_shortcuts_inhibitor_v1).
		///
		/// This occurs typically when the initial request "inhibit_shortcuts"
		/// first becomes active or when the user instructs the compositor to
		/// re-enable and existing shortcuts inhibitor using any mechanism
		/// offered by the compositor.
		fn active(
			&self,
			proxy: &mut ZwpKeyboardShortcutsInhibitorV1,
		);
		
		/// # shortcuts are restored
		///
		/// This event indicates that the shortcuts inhibitor is inactive,
		/// normal shortcuts processing is restored by the compositor.
		fn inactive(
			&self,
			proxy: &mut ZwpKeyboardShortcutsInhibitorV1,
		);
	}
}
pub use input_method_unstable_v1::*;
mod input_method_unstable_v1 {
	use crate::*;
	
	// Copyright © 2012, 2013 Intel Corporation
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_INPUT_METHOD_CONTEXT_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_input_method_context_v1\0".as_ptr(),
		version:      1,
		method_count: 14,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "commit_string\0".as_ptr(),
				signature: "us\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "preedit_string\0".as_ptr(),
				signature: "uss\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "preedit_styling\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "preedit_cursor\0".as_ptr(),
				signature: "i\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "delete_surrounding_text\0".as_ptr(),
				signature: "iu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "cursor_position\0".as_ptr(),
				signature: "ii\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "modifiers_map\0".as_ptr(),
				signature: "a\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "keysym\0".as_ptr(),
				signature: "uuuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "grab_keyboard\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&WL_KEYBOARD_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "key\0".as_ptr(),
				signature: "uuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "modifiers\0".as_ptr(),
				signature: "uuuuu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "language\0".as_ptr(),
				signature: "us\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "text_direction\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  6,
		events:       [
			WlMessage {
				name:      "surrounding_text\0".as_ptr(),
				signature: "suu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "reset\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "content_type\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "invoke_action\0".as_ptr(),
				signature: "uu\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "commit_state\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "preferred_language\0".as_ptr(),
				signature: "s\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # input method context
	///
	/// Corresponds to a text input on the input method side. An input method context
	/// is created on text input activation on the input method side. It allows
	/// receiving information about the text input from the application via events.
	/// Input method contexts do not keep state after deactivation and should be
	/// destroyed after deactivation is handled.
	///
	/// Text is generally UTF-8 encoded, indices and lengths are in bytes.
	///
	/// Serials are used to synchronize the state between the text input and
	/// an input method. New serials are sent by the text input in the
	/// commit_state request and are used by the input method to indicate
	/// the known text input state in events like preedit_string, commit_string,
	/// and keysym. The text input can then ignore events from the input method
	/// which are based on an outdated state (for example after a reset).
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZwpInputMethodContextV1(WlProxy);
	
	impl std::ops::Deref for ZwpInputMethodContextV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpInputMethodContextV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpInputMethodContextV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpInputMethodContextV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpInputMethodContextV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpInputMethodContextV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpInputMethodContextV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.surrounding_text((proxy as *mut ZwpInputMethodContextV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).u, (*args.add(2)).u, ),
						1 => listener.reset((proxy as *mut ZwpInputMethodContextV1).as_mut().unwrap(), ),
						2 => listener.content_type((proxy as *mut ZwpInputMethodContextV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						3 => listener.invoke_action((proxy as *mut ZwpInputMethodContextV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, ),
						4 => listener.commit_state((proxy as *mut ZwpInputMethodContextV1).as_mut().unwrap(), (*args.add(0)).u, ),
						5 => listener.preferred_language((proxy as *mut ZwpInputMethodContextV1).as_mut().unwrap(), std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `surrounding_text` ARGS: text: {:?}, cursor: {:?}, anchor: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap(), (*args.add(1)).u, (*args.add(2)).u),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `reset` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `content_type` ARGS: hint: {:?}, purpose: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						3 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `invoke_action` ARGS: button: {:?}, index: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u),
						4 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `commit_state` ARGS: serial: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						5 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `preferred_language` ARGS: language: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, std::ffi::CStr::from_ptr((*args.add(0)).s as _).to_str().unwrap()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpInputMethodContextV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # commit string
		///
		/// Send the commit string text for insertion to the application.
		///
		/// The text to commit could be either just a single character after a key
		/// press or the result of some composing (pre-edit). It could be also an
		/// empty text when some text should be removed (see
		/// delete_surrounding_text) or when the input cursor should be moved (see
		/// cursor_position).
		///
		/// Any previously set composing text will be removed.
		pub fn commit_string(
			&self,
			serial          : u32,
			text            : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, serial, text.as_ptr()); }
		}
		
		/// # pre-edit string
		///
		/// Send the pre-edit string text to the application text input.
		///
		/// The commit text can be used to replace the pre-edit text on reset (for
		/// example on unfocus).
		///
		/// Previously sent preedit_style and preedit_cursor requests are also
		/// processed by the text_input.
		pub fn preedit_string(
			&self,
			serial          : u32,
			text            : &str,
			commit          : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 2, serial, text.as_ptr(), commit.as_ptr()); }
		}
		
		/// # pre-edit styling
		///
		/// Set the styling information on composing text. The style is applied for
		/// length in bytes from index relative to the beginning of
		/// the composing text (as byte offset). Multiple styles can
		/// be applied to a composing text.
		///
		/// This request should be sent before sending a preedit_string request.
		pub fn preedit_styling(
			&self,
			index           : u32,
			length          : u32,
			style           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 3, index, length, style); }
		}
		
		/// # pre-edit cursor
		///
		/// Set the cursor position inside the composing text (as byte offset)
		/// relative to the start of the composing text.
		///
		/// When index is negative no cursor should be displayed.
		///
		/// This request should be sent before sending a preedit_string request.
		pub fn preedit_cursor(
			&self,
			index           : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 4, index); }
		}
		
		/// # delete text
		///
		/// Remove the surrounding text.
		///
		/// This request will be handled on the text_input side directly following
		/// a commit_string request.
		pub fn delete_surrounding_text(
			&self,
			index           : i32,
			length          : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 5, index, length); }
		}
		
		/// # set cursor to a new position
		///
		/// Set the cursor and anchor to a new position. Index is the new cursor
		/// position in bytes (when >= 0 this is relative to the end of the inserted text,
		/// otherwise it is relative to the beginning of the inserted text). Anchor is
		/// the new anchor position in bytes (when >= 0 this is relative to the end of the
		/// inserted text, otherwise it is relative to the beginning of the inserted
		/// text). When there should be no selected text, anchor should be the same
		/// as index.
		///
		/// This request will be handled on the text_input side directly following
		/// a commit_string request.
		pub fn cursor_position(
			&self,
			index           : i32,
			anchor          : i32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 6, index, anchor); }
		}
		pub fn modifiers_map(
			&self,
			map             : &WlArray
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 7, map); }
		}
		
		/// # keysym
		///
		/// Notify when a key event was sent. Key events should not be used for
		/// normal text input operations, which should be done with commit_string,
		/// delete_surrounding_text, etc. The key event follows the wl_keyboard key
		/// event convention. Sym is an XKB keysym, state is a wl_keyboard key_state.
		pub fn keysym(
			&self,
			serial          : u32,
			time            : u32,
			sym             : u32,
			state           : u32,
			modifiers       : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 8, serial, time, sym, state, modifiers); }
		}
		
		/// # grab hardware keyboard
		///
		/// Allow an input method to receive hardware keyboard input and process
		/// key events to generate text events (with pre-edit) over the wire. This
		/// allows input methods which compose multiple key events for inputting
		/// text like it is done for CJK languages.
		pub fn grab_keyboard(
			&self
		) -> Result<Box<WlKeyboard, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 9, &WL_KEYBOARD_INTERFACE, std::ptr::null::<u8>()) as *mut WlKeyboard };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # forward key event
		///
		/// Forward a wl_keyboard::key event to the client that was not processed
		/// by the input method itself. Should be used when filtering key events
		/// with grab_keyboard.  The arguments should be the ones from the
		/// wl_keyboard::key event.
		///
		/// For generating custom key events use the keysym request instead.
		pub fn key(
			&self,
			serial          : u32,
			time            : u32,
			key             : u32,
			state           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 10, serial, time, key, state); }
		}
		
		/// # forward modifiers event
		///
		/// Forward a wl_keyboard::modifiers event to the client that was not
		/// processed by the input method itself.  Should be used when filtering
		/// key events with grab_keyboard. The arguments should be the ones
		/// from the wl_keyboard::modifiers event.
		pub fn modifiers(
			&self,
			serial          : u32,
			mods_depressed  : u32,
			mods_latched    : u32,
			mods_locked     : u32,
			group           : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 11, serial, mods_depressed, mods_latched, mods_locked, group); }
		}
		pub fn language(
			&self,
			serial          : u32,
			language        : &str
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 12, serial, language.as_ptr()); }
		}
		pub fn text_direction(
			&self,
			serial          : u32,
			direction       : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 13, serial, direction); }
		}
	}
	
	
	pub trait ZwpInputMethodContextV1Listener: std::any::Any {
		
		/// # surrounding text event
		///
		/// The plain surrounding text around the input position. Cursor is the
		/// position in bytes within the surrounding text relative to the beginning
		/// of the text. Anchor is the position in bytes of the selection anchor
		/// within the surrounding text relative to the beginning of the text. If
		/// there is no selected text then anchor is the same as cursor.
		fn surrounding_text(
			&self,
			proxy: &mut ZwpInputMethodContextV1,
			text            : &str,
			cursor          : u32,
			anchor          : u32,
		);
		fn reset(
			&self,
			proxy: &mut ZwpInputMethodContextV1,
		);
		fn content_type(
			&self,
			proxy: &mut ZwpInputMethodContextV1,
			hint            : u32,
			purpose         : u32,
		);
		fn invoke_action(
			&self,
			proxy: &mut ZwpInputMethodContextV1,
			button          : u32,
			index           : u32,
		);
		fn commit_state(
			&self,
			proxy: &mut ZwpInputMethodContextV1,
			serial          : u32,
		);
		fn preferred_language(
			&self,
			proxy: &mut ZwpInputMethodContextV1,
			language        : &str,
		);
	}
	
	pub static ZWP_INPUT_METHOD_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_input_method_v1\0".as_ptr(),
		version:      1,
		method_count: 0,
		methods:      [
		].as_ptr(),
		event_count:  2,
		events:       [
			WlMessage {
				name:      "activate\0".as_ptr(),
				signature: "n\0".as_ptr(),
				types:     [&ZWP_INPUT_METHOD_CONTEXT_V1_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "deactivate\0".as_ptr(),
				signature: "o\0".as_ptr(),
				types:     [&ZWP_INPUT_METHOD_CONTEXT_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr()
	};
	
	/// # input method
	///
	/// An input method object is responsible for composing text in response to
	/// input from hardware or virtual keyboards. There is one input method
	/// object per seat. On activate there is a new input method context object
	/// created which allows the input method to communicate with the text input.
	pub struct ZwpInputMethodV1(WlProxy);
	
	impl std::ops::Deref for ZwpInputMethodV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpInputMethodV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpInputMethodV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpInputMethodV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpInputMethodV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpInputMethodV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpInputMethodV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.activate((proxy as *mut ZwpInputMethodV1).as_mut().unwrap(), (*args.add(0)).n, ),
						1 => listener.deactivate((proxy as *mut ZwpInputMethodV1).as_mut().unwrap(), ((*args.add(0)).o as *mut ZwpInputMethodContextV1).as_mut(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `activate` ARGS: id: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).n),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `deactivate` ARGS: context: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, ((*args.add(0)).o as *mut ZwpInputMethodContextV1).as_mut()),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpInputMethodV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpInputMethodV1Listener: std::any::Any {
		
		/// # activate event
		///
		/// A text input was activated. Creates an input method context object
		/// which allows communication with the text input.
		fn activate(
			&self,
			proxy: &mut ZwpInputMethodV1,
			id              : u32,
		);
		
		/// # deactivate event
		///
		/// The text input corresponding to the context argument was deactivated.
		/// The input method context should be destroyed after deactivation is
		/// handled.
		fn deactivate(
			&self,
			proxy: &mut ZwpInputMethodV1,
			context         : Option<&mut ZwpInputMethodContextV1>,
		);
	}
	
	pub static ZWP_INPUT_PANEL_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_input_panel_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "get_input_panel_surface\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_INPUT_PANEL_SURFACE_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # interface for implementing keyboards
	///
	/// Only one client can bind this interface at a time.
	pub struct ZwpInputPanelV1(WlProxy);
	
	impl std::ops::Deref for ZwpInputPanelV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpInputPanelV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpInputPanelV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpInputPanelV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpInputPanelV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		pub fn get_input_panel_surface(
			&self,
			surface         : &WlSurface
		) -> Result<Box<ZwpInputPanelSurfaceV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 0, &ZWP_INPUT_PANEL_SURFACE_V1_INTERFACE, std::ptr::null::<u8>(), surface) as *mut ZwpInputPanelSurfaceV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZWP_INPUT_PANEL_SURFACE_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_input_panel_surface_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "set_toplevel\0".as_ptr(),
				signature: "ou\0".as_ptr(),
				types:     [&WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "set_overlay_panel\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	pub struct ZwpInputPanelSurfaceV1(WlProxy);
	
	impl std::ops::Deref for ZwpInputPanelSurfaceV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpInputPanelSurfaceV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpInputPanelSurfaceV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpInputPanelSurfaceV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpInputPanelSurfaceV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # set the surface type as a keyboard
		///
		/// Set the input_panel_surface type to keyboard.
		///
		/// A keyboard surface is only shown when a text input is active.
		pub fn set_toplevel(
			&self,
			output          : &WlOutput,
			position        : u32
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0, output, position); }
		}
		
		/// # set the surface type as an overlay panel
		///
		/// Set the input_panel_surface to be an overlay panel.
		///
		/// This is shown near the input cursor above the application window when
		/// a text input is active.
		pub fn set_overlay_panel(
			&self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1); }
		}
	}
	
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpInputPanelSurfaceV1Position {
		///
		CenterBottom = 0,
	}
}

/// # High-resolution timestamps for input events
///
/// This protocol specifies a way for a client to request and receive
/// high-resolution timestamps for input events.
///
/// Warning! The protocol described in this file is experimental and
/// backward incompatible changes may be made. Backward compatible changes
/// may be added together with the corresponding interface version bump.
/// Backward incompatible changes are done by bumping the version number in
/// the protocol and interface names and resetting the interface version.
/// Once the protocol is to be declared stable, the 'z' prefix and the
/// version number in the protocol and interface names are removed and the
/// interface version number is reset.
pub use input_timestamps_unstable_v1::*;
mod input_timestamps_unstable_v1 {
	use crate::*;
	
	// Copyright © 2017 Collabora, Ltd.
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_INPUT_TIMESTAMPS_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_input_timestamps_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 4,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "get_keyboard_timestamps\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_INPUT_TIMESTAMPS_V1_INTERFACE as _, &WL_KEYBOARD_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_pointer_timestamps\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_INPUT_TIMESTAMPS_V1_INTERFACE as _, &WL_POINTER_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "get_touch_timestamps\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_INPUT_TIMESTAMPS_V1_INTERFACE as _, &WL_TOUCH_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # context object for high-resolution input timestamps
	///
	/// A global interface used for requesting high-resolution timestamps
	/// for input events.
	pub struct ZwpInputTimestampsManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpInputTimestampsManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpInputTimestampsManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpInputTimestampsManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpInputTimestampsManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpInputTimestampsManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the input timestamps manager object
		///
		/// Informs the server that the client will no longer be using this
		/// protocol object. Existing objects created by this object are not
		/// affected.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # subscribe to high-resolution keyboard timestamp events
		///
		/// Creates a new input timestamps object that represents a subscription
		/// to high-resolution timestamp events for all wl_keyboard events that
		/// carry a timestamp.
		///
		/// If the associated wl_keyboard object is invalidated, either through
		/// client action (e.g. release) or server-side changes, the input
		/// timestamps object becomes inert and the client should destroy it
		/// by calling zwp_input_timestamps_v1.destroy.
		pub fn get_keyboard_timestamps(
			&self,
			keyboard        : &WlKeyboard
		) -> Result<Box<ZwpInputTimestampsV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_INPUT_TIMESTAMPS_V1_INTERFACE, std::ptr::null::<u8>(), keyboard) as *mut ZwpInputTimestampsV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # subscribe to high-resolution pointer timestamp events
		///
		/// Creates a new input timestamps object that represents a subscription
		/// to high-resolution timestamp events for all wl_pointer events that
		/// carry a timestamp.
		///
		/// If the associated wl_pointer object is invalidated, either through
		/// client action (e.g. release) or server-side changes, the input
		/// timestamps object becomes inert and the client should destroy it
		/// by calling zwp_input_timestamps_v1.destroy.
		pub fn get_pointer_timestamps(
			&self,
			pointer         : &WlPointer
		) -> Result<Box<ZwpInputTimestampsV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &ZWP_INPUT_TIMESTAMPS_V1_INTERFACE, std::ptr::null::<u8>(), pointer) as *mut ZwpInputTimestampsV1 };
			unsafe { box_from_raw(proxy) }
		}
		
		/// # subscribe to high-resolution touch timestamp events
		///
		/// Creates a new input timestamps object that represents a subscription
		/// to high-resolution timestamp events for all wl_touch events that
		/// carry a timestamp.
		///
		/// If the associated wl_touch object becomes invalid, either through
		/// client action (e.g. release) or server-side changes, the input
		/// timestamps object becomes inert and the client should destroy it
		/// by calling zwp_input_timestamps_v1.destroy.
		pub fn get_touch_timestamps(
			&self,
			touch           : &WlTouch
		) -> Result<Box<ZwpInputTimestampsV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 3, &ZWP_INPUT_TIMESTAMPS_V1_INTERFACE, std::ptr::null::<u8>(), touch) as *mut ZwpInputTimestampsV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZWP_INPUT_TIMESTAMPS_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_input_timestamps_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "timestamp\0".as_ptr(),
				signature: "uuu\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # context object for input timestamps
	///
	/// Provides high-resolution timestamp events for a set of subscribed input
	/// events. The set of subscribed input events is determined by the
	/// zwp_input_timestamps_manager_v1 request used to create this object.
	pub struct ZwpInputTimestampsV1(WlProxy);
	
	impl std::ops::Deref for ZwpInputTimestampsV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpInputTimestampsV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpInputTimestampsV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpInputTimestampsV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpInputTimestampsV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpInputTimestampsV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpInputTimestampsV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.timestamp((proxy as *mut ZwpInputTimestampsV1).as_mut().unwrap(), (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `timestamp` ARGS: tv_sec_hi: {:?}, tv_sec_lo: {:?}, tv_nsec: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u, (*args.add(1)).u, (*args.add(2)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpInputTimestampsV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		/// # destroy the input timestamps object
		///
		/// Informs the server that the client will no longer be using this
		/// protocol object. After the server processes the request, no more
		/// timestamp events will be emitted.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpInputTimestampsV1Listener: std::any::Any {
		
		/// # high-resolution timestamp event
		///
		/// The timestamp event is associated with the first subsequent input event
		/// carrying a timestamp which belongs to the set of input events this
		/// object is subscribed to.
		///
		/// The timestamp provided by this event is a high-resolution version of
		/// the timestamp argument of the associated input event. The provided
		/// timestamp is in the same clock domain and is at least as accurate as
		/// the associated input event timestamp.
		///
		/// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
		/// each component being an unsigned 32-bit value. Whole seconds are in
		/// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
		/// and the additional fractional part in tv_nsec as nanoseconds. Hence,
		/// for valid timestamps tv_nsec must be in [0, 999999999].
		fn timestamp(
			&self,
			proxy: &mut ZwpInputTimestampsV1,
			tv_sec_hi       : u32,
			tv_sec_lo       : u32,
			tv_nsec         : u32,
		);
	}
}
pub use idle_inhibit_unstable_v1::*;
mod idle_inhibit_unstable_v1 {
	use crate::*;
	
	// Copyright © 2015 Samsung Electronics Co., Ltd
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_IDLE_INHIBIT_MANAGER_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_idle_inhibit_manager_v1\0".as_ptr(),
		version:      1,
		method_count: 2,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "create_inhibitor\0".as_ptr(),
				signature: "no\0".as_ptr(),
				types:     [&ZWP_IDLE_INHIBITOR_V1_INTERFACE as _, &WL_SURFACE_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # control behavior when display idles
	///
	/// This interface permits inhibiting the idle behavior such as screen
	/// blanking, locking, and screensaving.  The client binds the idle manager
	/// globally, then creates idle-inhibitor objects for each surface.
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZwpIdleInhibitManagerV1(WlProxy);
	
	impl std::ops::Deref for ZwpIdleInhibitManagerV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpIdleInhibitManagerV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpIdleInhibitManagerV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpIdleInhibitManagerV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpIdleInhibitManagerV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the idle inhibitor object
		///
		/// Destroy the inhibit manager.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # create a new inhibitor object
		///
		/// Create a new inhibitor object associated with the given surface.
		pub fn create_inhibitor(
			&self,
			surface         : &WlSurface
		) -> Result<Box<ZwpIdleInhibitorV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 1, &ZWP_IDLE_INHIBITOR_V1_INTERFACE, std::ptr::null::<u8>(), surface) as *mut ZwpIdleInhibitorV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub static ZWP_IDLE_INHIBITOR_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_idle_inhibitor_v1\0".as_ptr(),
		version:      1,
		method_count: 1,
		methods:      [
			WlMessage {
				name:      "destroy\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr(),
		event_count:  0,
		events:       [
		].as_ptr()
	};
	
	/// # context object for inhibiting idle behavior
	///
	/// An idle inhibitor prevents the output that the associated surface is
	/// visible on from being set to a state where it is not visually usable due
	/// to lack of user interaction (e.g. blanked, dimmed, locked, set to power
	/// save, etc.)  Any screensaver processes are also blocked from displaying.
	///
	/// If the surface is destroyed, unmapped, becomes occluded, loses
	/// visibility, or otherwise becomes not visually relevant for the user, the
	/// idle inhibitor will not be honored by the compositor; if the surface
	/// subsequently regains visibility the inhibitor takes effect once again.
	/// Likewise, the inhibitor isn't honored if the system was already idled at
	/// the time the inhibitor was established, although if the system later
	/// de-idles and re-idles the inhibitor will take effect.
	pub struct ZwpIdleInhibitorV1(WlProxy);
	
	impl std::ops::Deref for ZwpIdleInhibitorV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpIdleInhibitorV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpIdleInhibitorV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpIdleInhibitorV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpIdleInhibitorV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		/// # destroy the idle inhibitor object
		///
		/// Remove the inhibitor effect from the associated wl_surface.
		pub fn destroy(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
}
pub use fullscreen_shell_unstable_v1::*;
mod fullscreen_shell_unstable_v1 {
	use crate::*;
	
	// Copyright © 2016 Yong Bakos
	// Copyright © 2015 Jason Ekstrand
	// Copyright © 2015 Jonas Ådahl
	//
	// Permission is hereby granted, free of charge, to any person obtaining a
	// copy of this software and associated documentation files (the "Software"),
	// to deal in the Software without restriction, including without limitation
	// the rights to use, copy, modify, merge, publish, distribute, sublicense,
	// and/or sell copies of the Software, and to permit persons to whom the
	// Software is furnished to do so, subject to the following conditions:
	//
	// The above copyright notice and this permission notice (including the next
	// paragraph) shall be included in all copies or substantial portions of the
	// Software.
	//
	// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
	// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
	// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
	// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
	// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
	// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
	// DEALINGS IN THE SOFTWARE.
	
	pub static ZWP_FULLSCREEN_SHELL_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_fullscreen_shell_v1\0".as_ptr(),
		version:      1,
		method_count: 3,
		methods:      [
			WlMessage {
				name:      "release\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "present_surface\0".as_ptr(),
				signature: "?ou?o\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _, &WL_OUTPUT_INTERFACE as _].as_ptr()
			},
			WlMessage {
				name:      "present_surface_for_mode\0".as_ptr(),
				signature: "ooin\0".as_ptr(),
				types:     [&WL_SURFACE_INTERFACE as _, &WL_OUTPUT_INTERFACE as _, &ZWP_FULLSCREEN_SHELL_MODE_FEEDBACK_V1_INTERFACE as _].as_ptr()
			},
		].as_ptr(),
		event_count:  1,
		events:       [
			WlMessage {
				name:      "capability\0".as_ptr(),
				signature: "u\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	
	/// # displays a single surface per output
	///
	/// Displays a single surface per output.
	///
	/// This interface provides a mechanism for a single client to display
	/// simple full-screen surfaces.  While there technically may be multiple
	/// clients bound to this interface, only one of those clients should be
	/// shown at a time.
	///
	/// To present a surface, the client uses either the present_surface or
	/// present_surface_for_mode requests.  Presenting a surface takes effect
	/// on the next wl_surface.commit.  See the individual requests for
	/// details about scaling and mode switches.
	///
	/// The client can have at most one surface per output at any time.
	/// Requesting a surface to be presented on an output that already has a
	/// surface replaces the previously presented surface.  Presenting a null
	/// surface removes its content and effectively disables the output.
	/// Exactly what happens when an output is "disabled" is
	/// compositor-specific.  The same surface may be presented on multiple
	/// outputs simultaneously.
	///
	/// Once a surface is presented on an output, it stays on that output
	/// until either the client removes it or the compositor destroys the
	/// output.  This way, the client can update the output's contents by
	/// simply attaching a new buffer.
	///
	/// Warning! The protocol described in this file is experimental and
	/// backward incompatible changes may be made. Backward compatible changes
	/// may be added together with the corresponding interface version bump.
	/// Backward incompatible changes are done by bumping the version number in
	/// the protocol and interface names and resetting the interface version.
	/// Once the protocol is to be declared stable, the 'z' prefix and the
	/// version number in the protocol and interface names are removed and the
	/// interface version number is reset.
	pub struct ZwpFullscreenShellV1(WlProxy);
	
	impl std::ops::Deref for ZwpFullscreenShellV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpFullscreenShellV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpFullscreenShellV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpFullscreenShellV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpFullscreenShellV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpFullscreenShellV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpFullscreenShellV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.capability((proxy as *mut ZwpFullscreenShellV1).as_mut().unwrap(), (*args.add(0)).u, ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `capability` ARGS: capability: {:?}, ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode, (*args.add(0)).u),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpFullscreenShellV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # release the wl_fullscreen_shell interface
		///
		/// Release the binding from the wl_fullscreen_shell interface.
		///
		/// This destroys the server-side object and frees this binding.  If
		/// the client binds to wl_fullscreen_shell multiple times, it may wish
		/// to free some of those bindings.
		pub fn release(
			&mut self
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 0); }
			unsafe { WlProxy::destroy(&mut self.0); }
		}
		
		/// # present surface for display
		///
		/// Present a surface on the given output.
		///
		/// If the output is null, the compositor will present the surface on
		/// whatever display (or displays) it thinks best.  In particular, this
		/// may replace any or all surfaces currently presented so it should
		/// not be used in combination with placing surfaces on specific
		/// outputs.
		///
		/// The method parameter is a hint to the compositor for how the surface
		/// is to be presented.  In particular, it tells the compositor how to
		/// handle a size mismatch between the presented surface and the
		/// output.  The compositor is free to ignore this parameter.
		///
		/// The "zoom", "zoom_crop", and "stretch" methods imply a scaling
		/// operation on the surface.  This will override any kind of output
		/// scaling, so the buffer_scale property of the surface is effectively
		/// ignored.
		pub fn present_surface(
			&self,
			surface         : Option<&WlSurface>,
			method          : u32,
			output          : Option<&WlOutput>
		) {
			unsafe { (LIB_WAYLAND.wl_proxy_marshal)(self as *const Self as _, 1, surface.map_or(std::ptr::null_mut(), |r| r as *const WlSurface as *mut WlSurface), method, output.map_or(std::ptr::null_mut(), |r| r as *const WlOutput as *mut WlOutput)); }
		}
		
		/// # present surface for display at a particular mode
		///
		/// Presents a surface on the given output for a particular mode.
		///
		/// If the current size of the output differs from that of the surface,
		/// the compositor will attempt to change the size of the output to
		/// match the surface.  The result of the mode-switch operation will be
		/// returned via the provided wl_fullscreen_shell_mode_feedback object.
		///
		/// If the current output mode matches the one requested or if the
		/// compositor successfully switches the mode to match the surface,
		/// then the mode_successful event will be sent and the output will
		/// contain the contents of the given surface.  If the compositor
		/// cannot match the output size to the surface size, the mode_failed
		/// will be sent and the output will contain the contents of the
		/// previously presented surface (if any).  If another surface is
		/// presented on the given output before either of these has a chance
		/// to happen, the present_cancelled event will be sent.
		///
		/// Due to race conditions and other issues unknown to the client, no
		/// mode-switch operation is guaranteed to succeed.  However, if the
		/// mode is one advertised by wl_output.mode or if the compositor
		/// advertises the ARBITRARY_MODES capability, then the client should
		/// expect that the mode-switch operation will usually succeed.
		///
		/// If the size of the presented surface changes, the resulting output
		/// is undefined.  The compositor may attempt to change the output mode
		/// to compensate.  However, there is no guarantee that a suitable mode
		/// will be found and the client has no way to be notified of success
		/// or failure.
		///
		/// The framerate parameter specifies the desired framerate for the
		/// output in mHz.  The compositor is free to ignore this parameter.  A
		/// value of 0 indicates that the client has no preference.
		///
		/// If the value of wl_output.scale differs from wl_surface.buffer_scale,
		/// then the compositor may choose a mode that matches either the buffer
		/// size or the surface size.  In either case, the surface will fill the
		/// output.
		pub fn present_surface_for_mode(
			&self,
			surface         : &WlSurface,
			output          : &WlOutput,
			framerate       : i32
		) -> Result<Box<ZwpFullscreenShellModeFeedbackV1, WlAlloc>> {
			let proxy = unsafe { (LIB_WAYLAND.wl_proxy_marshal_constructor)(self as *const Self as _, 2, &ZWP_FULLSCREEN_SHELL_MODE_FEEDBACK_V1_INTERFACE, surface, output, framerate, std::ptr::null::<u8>()) as *mut ZwpFullscreenShellModeFeedbackV1 };
			unsafe { box_from_raw(proxy) }
		}
	}
	
	
	pub trait ZwpFullscreenShellV1Listener: std::any::Any {
		
		/// # advertises a capability of the compositor
		///
		/// Advertises a single capability of the compositor.
		///
		/// When the wl_fullscreen_shell interface is bound, this event is emitted
		/// once for each capability advertised.  Valid capabilities are given by
		/// the wl_fullscreen_shell.capability enum.  If clients want to take
		/// advantage of any of these capabilities, they should use a
		/// wl_display.sync request immediately after binding to ensure that they
		/// receive all the capability events.
		fn capability(
			&self,
			proxy: &mut ZwpFullscreenShellV1,
			capability      : u32,
		);
	}
	
	/// # capabilities advertised by the compositor
	///
	/// Various capabilities that can be advertised by the compositor.  They
	/// are advertised one-at-a-time when the wl_fullscreen_shell interface is
	/// bound.  See the wl_fullscreen_shell.capability event for more details.
	///
	/// ARBITRARY_MODES:
	/// This is a hint to the client that indicates that the compositor is
	/// capable of setting practically any mode on its outputs.  If this
	/// capability is provided, wl_fullscreen_shell.present_surface_for_mode
	/// will almost never fail and clients should feel free to set whatever
	/// mode they like.  If the compositor does not advertise this, it may
	/// still support some modes that are not advertised through wl_global.mode
	/// but it is less likely.
	///
	/// CURSOR_PLANE:
	/// This is a hint to the client that indicates that the compositor can
	/// handle a cursor surface from the client without actually compositing.
	/// This may be because of a hardware cursor plane or some other mechanism.
	/// If the compositor does not advertise this capability then setting
	/// wl_pointer.cursor may degrade performance or be ignored entirely.  If
	/// CURSOR_PLANE is not advertised, it is recommended that the client draw
	/// its own cursor and set wl_pointer.cursor(NULL).
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpFullscreenShellV1Capability {
		/// compositor is capable of almost any output mode
		ArbitraryModes = 1,
		/// compositor has a separate cursor plane
		CursorPlane = 2,
	}
	
	/// # different method to set the surface fullscreen
	///
	/// Hints to indicate to the compositor how to deal with a conflict
	/// between the dimensions of the surface and the dimensions of the
	/// output. The compositor is free to ignore this parameter.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpFullscreenShellV1PresentMethod {
		/// no preference, apply default policy
		Default = 0,
		/// center the surface on the output
		Center = 1,
		/// scale the surface, preserving aspect ratio, to the largest size that will fit on the output
		Zoom = 2,
		/// scale the surface, preserving aspect ratio, to fully fill the output cropping if needed
		ZoomCrop = 3,
		/// scale the surface to the size of the output ignoring aspect ratio
		Stretch = 4,
	}
	
	/// # wl_fullscreen_shell error values
	///
	/// These errors can be emitted in response to wl_fullscreen_shell requests.
	#[repr(u32)]
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	pub enum ZwpFullscreenShellV1Error {
		/// present_method is not known
		InvalidMethod = 0,
	}
	
	pub static ZWP_FULLSCREEN_SHELL_MODE_FEEDBACK_V1_INTERFACE: WlInterface = WlInterface {
		name:         "zwp_fullscreen_shell_mode_feedback_v1\0".as_ptr(),
		version:      1,
		method_count: 0,
		methods:      [
		].as_ptr(),
		event_count:  3,
		events:       [
			WlMessage {
				name:      "mode_successful\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "mode_failed\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
			WlMessage {
				name:      "present_cancelled\0".as_ptr(),
				signature: "\0".as_ptr(),
				types:     [].as_ptr()
			},
		].as_ptr()
	};
	pub struct ZwpFullscreenShellModeFeedbackV1(WlProxy);
	
	impl std::ops::Deref for ZwpFullscreenShellModeFeedbackV1 {
		type Target = WlProxy;
		
		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}
	
	impl std::ops::DerefMut for ZwpFullscreenShellModeFeedbackV1 {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}
	
	impl std::fmt::Debug for ZwpFullscreenShellModeFeedbackV1 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("ZwpFullscreenShellModeFeedbackV1")
				.field("class", &self.get_class())
				.field("version", &self.get_version())
				.field("id", &self.get_id())
				.finish()
		}
	}
	
	impl ZwpFullscreenShellModeFeedbackV1 {
		pub fn get_version(&self) -> u32 {
			unsafe { WlProxy::get_version(&self.0) }
		}
		
		pub fn get_class(&self) -> &'static std::ffi::CStr {
			unsafe { std::ffi::CStr::from_ptr(WlProxy::get_class(&self.0) as _) }
		}
		
		pub fn get_id(&self) -> u32 {
			unsafe { WlProxy::get_id(&self.0) }
		}
		pub fn set_listener(&mut self, listener: impl ZwpFullscreenShellModeFeedbackV1Listener) -> Result<()> {
			extern fn dispatch(
				implementation: *const u8,
				proxy:          *mut WlProxy,
				opcode:         u32,
				message:        *const WlMessage,
				args:           *mut WlArgument
			) {
				unsafe {
					let listener = std::mem::transmute::<_, &dyn ZwpFullscreenShellModeFeedbackV1Listener>(TraitObject {
						vtable: implementation as _,
						data:    WlProxy::get_user_data(proxy) as _
					});
					
					match opcode {
						0 => listener.mode_successful((proxy as *mut ZwpFullscreenShellModeFeedbackV1).as_mut().unwrap(), ),
						1 => listener.mode_failed((proxy as *mut ZwpFullscreenShellModeFeedbackV1).as_mut().unwrap(), ),
						2 => listener.present_cancelled((proxy as *mut ZwpFullscreenShellModeFeedbackV1).as_mut().unwrap(), ),
						_ => ()
					}
					
					match opcode {
						
						
						0 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `mode_successful` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						1 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `mode_failed` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						2 => log::trace!("[WAYLAND] ID `{}` CLASS `{}` VERSION `{}` OPCODE `{}` EVENT `present_cancelled` ARGS: ", WlProxy::get_id(proxy), std::ffi::CStr::from_ptr(WlProxy::get_class(proxy) as _).to_string_lossy().as_ref(), WlProxy::get_version(proxy), opcode),
						_ => ()
					}
				}
			}
			
			let listener: Box<dyn ZwpFullscreenShellModeFeedbackV1Listener> = Box::new(listener);
			unsafe {
				let obj: TraitObject = std::mem::transmute(Box::leak(listener));
				match WlProxy::add_dispatcher(&mut self.0, dispatch, obj.vtable as _, obj.data as _) {
					0 => Ok(()),
					_ => Err(())
				}
			}
		}
		
		pub fn destroy(&mut self) {
			unsafe { WlProxy::destroy(&mut self.0); }
		}
	}
	
	
	pub trait ZwpFullscreenShellModeFeedbackV1Listener: std::any::Any {
		
		/// # mode switch succeeded
		///
		/// This event indicates that the attempted mode switch operation was
		/// successful.  A surface of the size requested in the mode switch
		/// will fill the output without scaling.
		///
		/// Upon receiving this event, the client should destroy the
		/// wl_fullscreen_shell_mode_feedback object.
		fn mode_successful(
			&self,
			proxy: &mut ZwpFullscreenShellModeFeedbackV1,
		);
		
		/// # mode switch failed
		///
		/// This event indicates that the attempted mode switch operation
		/// failed.  This may be because the requested output mode is not
		/// possible or it may mean that the compositor does not want to allow it.
		///
		/// Upon receiving this event, the client should destroy the
		/// wl_fullscreen_shell_mode_feedback object.
		fn mode_failed(
			&self,
			proxy: &mut ZwpFullscreenShellModeFeedbackV1,
		);
		
		/// # mode switch cancelled
		///
		/// This event indicates that the attempted mode switch operation was
		/// cancelled.  Most likely this is because the client requested a
		/// second mode switch before the first one completed.
		///
		/// Upon receiving this event, the client should destroy the
		/// wl_fullscreen_shell_mode_feedback object.
		fn present_cancelled(
			&self,
			proxy: &mut ZwpFullscreenShellModeFeedbackV1,
		);
	}
}
