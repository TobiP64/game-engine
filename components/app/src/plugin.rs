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

use crate::registry::Registry;

pub trait Plugin {
    const NAME: &'static str;

    fn setup(&mut self, registry: &Registry);
}

pub fn load(path: &str) -> Result<ExternalPlugin, libloading::Error> {
    let mut plugin = ExternalPlugin {
        lib:     libloading::Library::new(path)?,
        name:    "",
        setup:   { fn dummy(_: &Registry) {} dummy },
        destroy: { fn dummy(/*_: &Registry*/) {} dummy },
    };

    plugin.name    = unsafe { *plugin.lib.get::<&'static str>(b"plugin_name\0")? };
    plugin.setup   = unsafe {*plugin.lib.get::<fn(registr: &Registry)>(b"plugin_setup\0")? };
    plugin.destroy = unsafe { *plugin.lib.get::<fn(/*registr: &Registry*/)>(b"plugin_destroy\0")? };

    Ok(plugin)
}

pub struct ExternalPlugin {
    lib:     libloading::Library,
    name:    &'static str,
    setup:   fn(registry: &Registry),
    destroy: fn(/*registry: &Registry*/),
}

impl Plugin for ExternalPlugin {
    const NAME: &'static str = "<external>";

    fn setup(&mut self, registry: &Registry) {
        (self.setup)(registry);
    }
}

impl Drop for ExternalPlugin {
    fn drop(&mut self) {
        (self.destroy)();
    }
}

impl std::fmt::Debug for ExternalPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("name", &self.name)
            .finish()
    }
}
