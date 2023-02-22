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
    associated_type_bounds,
    once_cell,
    available_concurrency,
    map_first_last
)]
#![warn(clippy::all)]
#![allow()]

mod asset;
mod builder;
mod executor;
mod logger;
mod plugin;
mod registry;

pub use {asset::*, builder::*, plugin::*, registry::*};

pub type BoxedFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// Send a desktop notification. Use this function to alert the user of a critical error, where
/// wayland or vulkan is not available.
pub fn notify(title: &str, text: &str) {
    match std::process::Command::new("notify-send")
        .arg(title)
        .arg(text)
        .status()
    {
        Ok(s) if s.success() => log::trace!("notification {} {} sent", title, text),
        _ => log::error!("notification {} {} failed to send", title, text),
    }
}

pub fn trace_memory(label: &str, name: &str, delta: isize, used: usize, total: usize) {
    if cfg!(feature = "trace-memory") {
        log::trace!(
            "[MEMORY][{}] {}: -{}B, used: {}B total: {}B",
            label,
            name,
            delta,
            used,
            total
        );
    }
}

pub fn trace_oom(label: &str, name: &str) {
    if cfg!(feature = "trace-memory") {
        log::trace!("[MEMORY][{}] {}: out of memory", label, name);
    }
}
