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

use {crate::BoxedFuture, ecs::Entity, std::io};

/// A source specifies where to read a resource from and how to interpret the data.
pub trait Source<T>: Send + Sync + std::fmt::Debug {
	fn open(&self, read: bool, write: bool) -> BoxedFuture<io::Result<T>>;
}

/// A simple source that only contains a path. This is sufficient for most use cases.
#[derive(Debug, Copy, Clone)]
pub struct PathSource<T: AsRef<std::path::Path>>(pub T);

/// A simple source that references data included in the binary.
#[derive(Debug, Copy, Clone)]
pub struct CachedSource<T: AsRef<[u8]>>(pub T);

/// A Loader specifies what to do with data read from a `Source`
pub trait Loader<T, D: 'static + Send + Sync = ()> {
	fn create(&self, _entity: Entity, _info: D) -> BoxedFuture<io::Result<()>> {
		Box::pin(async move {
			Err(io::Error::new(io::ErrorKind::Other, "loader does not support creating assets"))
		})
	}

	fn destroy(&self, _entity: Entity) -> BoxedFuture<io::Result<()>> {
		Box::pin(async move {
			Err(io::Error::new(io::ErrorKind::Other, "loader does not support destroying assets"))
		})
	}

	fn load(&self, _entity: Entity, _source: &dyn Source<T>) -> BoxedFuture<io::Result<()>> {
		Box::pin(async move {
			Err(io::Error::new(io::ErrorKind::Other, "loader does not support loading assets"))
		})
	}

	fn save(&self, _entity: Entity, _source: &dyn Source<T>) -> BoxedFuture<io::Result<()>> {
		Box::pin(async move {
			Err(io::Error::new(io::ErrorKind::Other, "loader does not support saving assets"))
		})
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResourceState<T> {
	Unloaded,
	Partially(f32),
	Loaded(T),
	Failed(String)
}

impl<T> Default for ResourceState<T> {
	fn default() -> Self {
		Self::Unloaded
	}
}