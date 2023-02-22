use std::{
	ptr::{null, null_mut},
	thread::{self, JoinHandle},
	sync::{Arc, Mutex},
	collections::{HashMap, hash_map::Entry},
};
use winapi::{
	shared::{minwindef::*, windef::*, windowsx::*},
	um::{winuser::*, libloaderapi::*, errhandlingapi::GetLastError, minwinbase::PSECURITY_ATTRIBUTES},
};
use crate::utils::LateInit;
use super::*;

static DEFAULT_WINDOW_CLASS_NAME: *const i8 = b"Arcutos WSI Win32 Default Window Class\0".as_ptr() as *const i8;

pub type PlatformEvent = MSG;
pub type PlatformError = DWORD;

pub struct WindowSystemImpl {
	hinstance: HINSTANCE,
	event_queues: Arc<Mutex<HashMap<HWND, Vec<MSG>>>>,
	event_thread: JoinHandle<()>
}

impl WindowSystemImpl {
	pub(super) fn init() -> Result<Box<dyn WindowSystem>> {
		let event_queues = Arc::new(Mutex::new(HashMap::new()));
		Ok(Box::new(Self {
			hinstance: {
				let hinstance = unsafe { GetModuleHandleA(null()) } as HINSTANCE;
				let mut class = WNDCLASSA {
					hInstance:     hinstance,
					lpfnWndProc:   Some(window_proc),
					lpszClassName: DEFAULT_WINDOW_CLASS_NAME,
					..WNDCLASSA::default()
				};
				unsafe { RegisterClassA(&class); }
				hinstance
			},
			event_queues: event_queues.clone(),
			event_thread: thread::Builder::new()
				.name("win32-event-handler".to_string())
				.spawn(move || {
					let mut msg = unsafe { std::mem::zeroed::<MSG>() };
					loop {
						unsafe {
							if GetMessageA(&mut msg, null(), 0, 0) == FALSE {
								return;
							}
							event_queues.lock()
								.unwrap()
								.entry(msg.hwnd)
								.or_insert_with(|| Vec::new())
								.push(MSG);
						}
					}
				})
				.expect("failed to spawn thread")
		}))
	}
}

impl WindowSystem for WindowSystemImpl {
	fn get_vk_instance_extensions(&self) -> &'static [*const u8] {
		&[VK_KHR_SURFACE_EXTENSION_NAME, VK_KHR_WIN32_SURFACE_EXTENSION_NAME]
	}
	
	fn create_window(&mut self, create_info: &WindowCreateInfo) -> Result<Box<dyn Window>> {
		let name = create_info.title.map_or(null::<i8>(), |str| {
			let mut s = String::from(str);
			s.push('\0');
			s.as_ptr() as *const i8
		});
		let window = unsafe { CreateWindowExA(
			0,
			DEFAULT_WINDOW_CLASS_NAME,
			name,
			WS_OVERLAPPEDWINDOW,
			create_info.x as i32,
			create_info.y as i32,
			create_info.width as i32,
			create_info.height as i32,
			create_info.parent.map_or(null(), |w| w.as_any()
				.downcast_ref::<WindowImpl>()
				.unwrap()
				.1),
			null(),
			self.hinstance,
			null()
		) };
		
		if window == null_mut() {
			return Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
		
		let window = WindowImpl(window);
		self.event_queues.lock().unwrap().insert(window.0, Vec::new());
		
		if create_info.flags & WindowCreateFlagBits::MappedBit as u32 != 0
			&& unsafe { ShowWindow(window.0, SW_SHOWNORMAL) } == FALSE {
			return Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
		
		Ok(Box::new(window))
	}
	
	
	fn get_features(&self) -> WindowSystemFeatures {
		WindowSystemFeatures {
			set_state: true,
			set_rect:  true
		}
	}
}

/// A window created with winuser.h.
pub struct WindowImpl(HWND);

unsafe impl Send for WindowImpl {}
unsafe impl Sync for WindowImpl {}

impl Target for WindowImpl {
	fn get_extent(&mut self) -> VkExtent2D {
		self.get_rect().extent
	}
	
	#[allow(non_snake_case)]
	fn create_surface(&self, instance: &VkInstanceImpl, allocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult {
		let surface_info = VkWin32SurfaceCreateInfoKHR {
			sType:     VkStructureType::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
			pNext:     null(),
			flags:     0,
			hinstance: unsafe { HINSTANCE },
			hwnd:      self.0
		};
		instance.createWin32SurfaceKHR(&surface_info, allocator, &mut surface)
	}
}

impl Window for WindowImpl {
	fn as_any(&self) -> &dyn Any {
		self
	}
	
	fn poll_event(&self) -> Option<Event> {
		unsafe {
			match EVENT_QUEUES.lock().unwrap().get_mut(&self.0) {
				Some(e) => e.pop().map(|msg| match LOWORD(msg.uMsg) {
					WM_CREATE                    => Event::Create { time: msg.time },
					WM_DESTROY                   => Event::Destroy { time: msg.time },
					WM_MOVE                      => Event::Move {
						time: msg.time,
						x: GET_X_LPARAM(msg.lParam),
						y: GET_Y_LPARAM(msg.lParam)
					},
					WM_SIZE                      => Event::Resize {
						time: msg.time,
						width: GET_X_LPARAM(msg.lParam) as u32,
						height: GET_Y_LPARAM(msg.lParam) as u32
					},
					WM_QUIT                      => Event::Quit { time: msg.time },
					WM_WINDOWPOSCHANGING         => Event::PlatformWin32(msg),
					WM_KEYDOWN                   => Event::KeyPress {
						time:  msg.time,
						key:   msg.wParam as u32,
						state: 0,
						x:     msg.pt.x as i32,
						y:     msg.pt.y as i32
					},
					WM_KEYUP                     => Event::KeyRelease {
						time:  msg.time,
						key:   msg.wParam as u32,
						state: 0,
						x:     msg.pt.x as i32,
						y:     msg.pt.y as i32
					},
					WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => Event::PointerButtonPress {
						time:  msg.time,
						but:   match LOWORD(msg.message) {
							WM_LBUTTONDOWN => Button::Left as u32,
							WM_MBUTTONDOWN => Button::Middle as u32,
							WM_RBUTTONDOWN => Button::Right as u32,
							WM_XBUTTONDOWN => match GET_XBUTTON_WPARAM(wParam) {
								XBUTTON1 => Button::Extra1 as u32,
								XBUTTON2 => Button::Extra2 as u32,
								_ => Button::Other as u32
							},
							_ => Button::Other as u32
						},
						state: msg.wParam as u32,
						x:     GET_X_LPARAM(msg.lParam),
						y:     GET_Y_LPARAM(msg.lParam)
					},
					WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => Event::PointerButtonRelease {
						time:  msg.time,
						but:   match LOWORD(msg.message) {
							WM_LBUTTONUP => Button::Left as u32,
							WM_MBUTTONUP => Button::Middle as u32,
							WM_RBUTTONUP => Button::Right as u32,
							WM_XBUTTONUP => match GET_XBUTTON_WPARAM(wParam) {
								XBUTTON1 => Button::Extra1 as u32,
								XBUTTON2 => Button::Extra2 as u32,
								button   => eprintln!("unknown extra button: {}", button)
							},
							button       => eprintln!("unknown button: {}", button)
						},
						state: msg.wParam as u32,
						x:     GET_X_LPARAM(msg.lParam),
						y:     GET_Y_LPARAM(msg.lParam)
					},
					WM_MOUSEMOVE                 => Event::PointerMotion {
						time:  msg.time,
						state: msg.wParam as u32,
						x:     GET_X_LPARAM(msg.lParam),
						y:     GET_Y_LPARAM(msg.lParam)
					},
					_ => Event::PlatformWin32(msg),
				}),
				_ => None
			}
		}
	}
	
	fn map(&mut self) -> Result<()> {
		match unsafe { ShowWindow(self.0, SW_SHOW) } {
			TRUE => Ok(()),
			_ => Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
	}
	
	fn unmap(&mut self) -> Result<()> {
		match unsafe { ShowWindow(self.0, SW_HIDE) } {
			TRUE => Ok(()),
			_ => Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
	}
	
	fn get_state(&mut self) -> WindowState {
		unimplemented!()
	}
	
	fn set_state(&mut self, state: WindowState) -> Result<()> {
		use super::WindowState::*;
		if state == WindowState::Fullscreen {
			unsafe {
				let monitor = MonitorFromWindow(self.0, MONITOR_DEFAULTTONEAREST);
				let mut info = MONITORINFO::default();
				if GetMonitorInfoA(monitor, &mut info) == FALSE {
					return Err(Error::PlatformWin32(unsafe { GetLastError() }))
				}
				
				let window = CreateWindowExA(
					0,
					DEFAULT_WINDOW_CLASS_NAME,
					name,
					WS_OVERLAPPEDWINDOW,
					info.rcMonitor.left,
					info.rcMonitor.top,
					info.rcMonitor.right - info.rcMonitor.left,
					info.rcMonitor.bottom - info.rcMonitor.top,
					null(),
					null(),
					HINSTANCE,
					null()
				);
				if window == null_mut() {
					return Err(Error::PlatformWin32(unsafe { GetLastError() }))
				}
				self.0 = window;
			}
			return Ok(());
		}
		
		match unsafe { ShowWindow(self.0, match state {
			Default    => SW_SHOWNORMAL,
			Minimized  => SW_MINIMIZE,
			Maximized  => SW_MAXIMIZE,
			Fullscreen => unreachable!()
		}) } {
			TRUE => Ok(()),
			_ => Err(Error::PlatformWin32(unsafe { GetLastError() })),
		}
	}
	
	fn get_rect(&self) -> Result<VkRect2D> {
		let mut rect: RECT = RECT::default();
		match unsafe { GetWindowRect(self.0, &mut rect as LPRECT) } {
			TRUE => Ok(kRect2D {
				offset: VkOffset2D {
					x: rect.left as i32,
					y: rect.top as i32,
				},
				extent: VkExtent2D {
					width: (rect.right - rect.left) as u32,
					height: (rect.bottom - rect.top) as u32,
				},
			}),
			_ => Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
	}
	
	fn set_rect(&mut self, rect: VkRect2D) -> Result<()> {
		match unsafe { MoveWindow(
			self.0,
			rect.offset.x,
			rect.offset.x,
			rect.extent.width as i32,
			rect.extent.height as i32,
			FALSE
		) } {
			TRUE => Ok(()),
			FALSE => Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
	}
	
	fn set_title(&mut self, name: &str) -> Result<()> {
		let mut name = String::from(name);
		name.push('\0');
		match unsafe { SetWindowTextA(self.0, name.as_ptr() as *const i8) } {
			TRUE => Ok(()),
			_ => Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
	}
	
	fn get_pointer_pos(&mut self) -> Result<(i32, i32)> {
		let mut point = POINT::default();
		match unsafe { GetCursorPos(&mut point) } {
			TRUE => Ok((point.x, point.y)),
			_ => Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
	}
	
	fn set_pointer_pos(&mut self, x: i32, y: i32) -> Result<()> {
		match unsafe { SetCursorPos(x, y) } {
			TRUE => Ok(()),
			_ => Err(Error::PlatformWin32(unsafe { GetLastError() }))
		}
	}
}

impl Drop for WindowImpl {
	fn drop(&mut self) {
		unsafe { DestroyWindow(self.0); }
	}
}

extern "system" fn window_proc(hwnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> isize {
	eprintln!("dummy window proc was called, that was probably not intended");
	0
}