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
use engine_core::BoxedFuture;

pub type Mesh = dyn Source<(MeshDescriptor, Box<dyn MeshReader>)>;
pub type AltMeshes = Vec<Mesh>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IoMode {
	Interleaved,
	DeInterleaved
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MeshType {
	Vk(vk::VkPrimitiveTopology),
	Path
}

pub trait MeshReader: std::io::Read + Send + Sync {
	fn io_mode(&mut self, _mode: Option<IoMode>) -> IoMode {
		IoMode::Interleaved
	}
}

#[derive(Clone, Debug, Default)]
pub struct MeshDescriptor {
	pub vertex_count: usize,
	pub layers:       usize,
	//pub levels:       Option<Vec<usize>>,
	pub attributes:   Vec<MeshAttribute>
}

impl MeshDescriptor {
	pub fn new(vertex_count: usize, attributes: &[MeshAttribute]) -> Self {
		Self { vertex_count, layers: 0, /*levels: None,*/ attributes: attributes.to_vec() }
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MeshAttribute {
	Pos(VkFormat),
	Tex(VkFormat),
	Color(VkFormat),
	Normal(VkFormat),
	Empty
}

impl Default for MeshAttribute {
	fn default() -> Self {
		Self::Empty
	}
}

#[derive(Debug, Clone)]
pub struct StaticMeshSource(pub MeshDescriptor, pub &'static [u8]);

impl StaticMeshSource {
	#[allow(clippy::new_ret_no_self)]
	pub fn new<T: Copy + Send + Sync + 'static>(info: MeshDescriptor, data: &'static [T]) -> Box<Mesh> {
		fn to_bytes<T: Copy + Send + Sync + 'static>(slice: &[T]) -> &[u8] {
			if cfg!(feature = "trace-unsafe") { log::trace!("TRACE-UNSAFE") }
			unsafe { std::slice::from_raw_parts(
				slice.as_ptr() as _, std::mem::size_of::<T>() * slice.len()) }
		}
		
		Box::new(Self(info, to_bytes(data)))
	}
}

impl Source<(MeshDescriptor, Box<dyn MeshReader>)> for StaticMeshSource {
	fn open(&self, _read: bool, _write: bool) -> BoxedFuture<std::io::Result<(MeshDescriptor, Box<dyn MeshReader>)>> {
		Box::pin(async move {
			Ok((self.0.clone(), Box::new(self.clone()) as _))
		})
	}
}

impl MeshReader for StaticMeshSource {}

impl std::io::Read for StaticMeshSource {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.1.read(buf)
	}
}