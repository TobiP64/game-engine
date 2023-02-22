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

use {super::*, vk::VkFormat};

pub type AnimationSource = dyn Source<(AnimationDescriptor, Box<dyn std::io::Read>)>;
pub type AnimationSrc    = Box<AnimationSource>;

#[derive(Clone, Debug)]
pub struct AnimationDescriptor {
	pub tracks: Vec<Track>
}

#[derive(Clone, Debug)]
pub struct Track {
	pub frame_count:  usize,
	pub vertex_count: usize,
	pub attributes:   [MeshAttribute; 8]
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrackAttribute {
	Mesh(super::mesh::MeshAttribute),
	InstPos(VkFormat),
	InstRot(VkFormat),
	InstSca(VkFormat),
	InstCol(VkFormat),
	Empty
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LoopMode {
	Once,
	Loop(usize),
	Cycle(usize)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Animations<T>(Vec<Animation<T>>);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Animation<T> {
	pub handle:     Handle<T>,
	pub start_time: u64,
	pub speed:      f32,
	pub weight:     f32,
	pub loop_mode:  LoopMode
}