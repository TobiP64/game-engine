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

/*use super::*;
use vk::VkFormat;

pub type Path = Box<dyn Source<Box<dyn PathReader>>>;
pub type AltPaths = Vec<Path>;

pub trait MeshReader: std::io::Read + Send + Sync {
	fn get_descriptor(&mut self) -> MeshDescriptor;
}

#[derive(Clone, Debug, Default)]
pub struct PathDescriptor {
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
	pub fn new<T: Copy + Send + Sync + 'static>(info: MeshDescriptor, data: &'static [T]) -> Mesh {
		Box::new(Self(info, crate::utils::to_bytes(data)))
	}
}

#[async_trait::async_trait]
impl Source<Box<dyn MeshReader>> for StaticMeshSource {
	async fn open(&self) -> std::io::Result<Box<dyn MeshReader>> {
		Ok(Box::new(self.clone()))
	}
}

impl MeshReader for StaticMeshSource {
	fn get_descriptor(&mut self) -> MeshDescriptor {
		self.0.clone()
	}
}

impl std::io::Read for StaticMeshSource {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.1.read(buf)
	}
}*/