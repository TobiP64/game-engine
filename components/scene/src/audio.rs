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

use super::*;

pub type Audio = Box<dyn Source<Box<dyn AudioData>>>;

pub trait AudioData: std::io::Read {
	fn get_descriptor(&mut self) -> AudioDescriptor;
}

#[derive(Copy, Clone, Debug)]
pub struct AudioDescriptor {
	pub frequency: usize,
	pub format:    vk::VkFormat
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioSource {
	pub handle:       Handle<Audio>,
	pub radiation:    Radiation,
	pub range:        f32,
	pub fall_off_min: f32,
	pub fall_off_max: f32,
	pub pitch:        f32,
	pub gain:         f32
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Radiation {
	OmniDirectional,
	Conical { angle: f32 },
	Planar
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Listener {
	pub gain: f32
}