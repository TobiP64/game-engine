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

#![warn(clippy::all)]
#![forbid(unsafe_code)]
#![allow(clippy::should_implement_trait)]

#[cfg(feature = "ktx2")]
pub mod ktx2;
#[cfg(feature = "flac")]
pub mod flac;
#[cfg(feature = "opus")]
pub mod opus;
#[cfg(feature = "mkv")]
pub mod mkv;
#[cfg(feature = "ebml")]
pub mod ebml;
#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "xml")]
pub mod xml;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "gltf")]
pub mod gltf;
#[cfg(feature = "svg")]
pub mod svg;
#[cfg(feature = "protobuf")]
pub mod protobuf;
#[cfg(feature = "ber")]
pub mod ber;
#[cfg(feature = "yaml")]
pub mod yaml;
#[cfg(feature = "elf")]
pub mod elf;
#[cfg(feature = "dyn_repr")]
pub mod dyn_repr;
pub mod utils;

#[cfg(feature = "serde")]
pub use serde;
#[cfg(feature = "ttf")]
pub use ttf_parser as ttf;