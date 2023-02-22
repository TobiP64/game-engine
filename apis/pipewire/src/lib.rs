
#![allow(dead_code)]

use std::ptr::null;

pub fn init() {
    (unsafe { LIB_PIPEWIRE.pw_init })(0, null())
}

static mut LIB_PIPEWIRE: LibPipewire = unsafe { LibPipewire::uninit() };

struct LibPipewire {
    lib:                                   Option<libloading::Library>,
    pw_init:                               extern fn(u32, *const *const u8)
}

impl LibPipewire {
    const unsafe fn uninit() -> Self {
        extern fn abort() { panic!("libpipewire has not been loaded") }

        Self {
            lib:                                   None,
            pw_init:                               {
                extern fn load(argc: u32, argv: *const *const u8) {
                    unsafe { LIB_PIPEWIRE.load(); (LIB_PIPEWIRE.pw_init)(argc, argv) }
                }
                load
            }
        }
    }

    unsafe fn load(&mut self) {
        if self.lib.is_some() { return; }

        let lib                                    = libloading::Library::new("libpipewire-0.3.so.0").expect("failed to load libpipewire");
        self.pw_init                               = *lib.get(b"pw_init\0").expect("failed to load `pw_init`");
        self.lib                                   = Some(lib);
        log::trace!("loaded libpipewire");
    }

    fn get() -> &'static Self {
        unsafe { &LIB_PIPEWIRE }
    }
}