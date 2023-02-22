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

use {
	super::*,
	std::{any::Any, sync::Arc},
	vk::*
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UpdateResult {
	Reuse,
	Redraw,
	Rerecord
}

impl std::ops::BitAnd for UpdateResult {
	type Output = Self;
	
	fn bitand(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Self::Rerecord, _) | (_, Self::Rerecord) => Self::Rerecord,
			(Self::Redraw, _) | (_, Self::Redraw) => Self::Redraw,
			_ => Self::Reuse,
		}
	}
}

impl std::ops::BitAndAssign for UpdateResult {
	fn bitand_assign(&mut self, rhs: Self) {
		*self = *self & rhs;
	}
}

impl std::iter::FromIterator<UpdateResult> for UpdateResult {
	fn from_iter<T: IntoIterator<Item = UpdateResult>>(iter: T) -> Self {
		iter.into_iter().fold(Self::Reuse, |v0, v1| v0 & v1)
	}
}

#[derive(Debug)]
pub struct Plugins<S: 'static, T: 'static> {
	plugins: Vec<Box<dyn PluginRootContext<S, T>>>
}

impl<S: 'static, T: 'static> Plugins<S, T> {
	pub fn new() -> Self {
		Self { plugins: Vec::new() }
	}
	
	pub fn add(mut self, plugin: impl PluginRootContext<S, T>) -> Self {
		self.plugins.push(Box::new(plugin));
		self
	}
}

impl<'a, S: 'static, T: 'static> IntoIterator for &'a Plugins<S, T> {
	type Item     = &'a dyn PluginRootContext<S, T>;
	type IntoIter = PluginsIter<'a, S, T>;
	
	fn into_iter(self) -> Self::IntoIter {
		PluginsIter(self.plugins.iter())
	}
}

pub struct PluginsIter<'a, S: 'static, T: 'static>(<&'a Vec<Box<dyn PluginRootContext<S, T>>> as IntoIterator>::IntoIter);

impl<'a, S: 'static, T: 'static> Iterator for PluginsIter<'a, S, T> {
	type Item = &'a dyn PluginRootContext<S, T>;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|v| &**v)
	}
}

pub type Error = Box<dyn std::fmt::Debug>;

pub trait PluginRootContext<S: 'static, T: 'static>: Any + Send + std::fmt::Debug {
	fn as_any(&self) -> &dyn Any;
	
	fn select_device(&self, device: &VkPhysicalDevice) -> bool {
		true
	}
	
	fn create_device_context(&self, device: &Arc<DeviceRootContext<S, T>>)-> Result<Box<dyn DeviceContext<S, T>>, Error>;
	
	fn create_scene_context(&self, scene_ctx: &Arc<SceneRootContext<S, T>>) -> Result<Box<dyn SceneContext<S>>, Error>;
	
	fn create_target_context(&self, target_ctx: &Arc<TargetRootContext<S, T>>) -> Result<Box<dyn TargetContext<T>>, Error>;
}

pub trait DeviceContext<S, T>: Any + Send + std::fmt::Debug {
	fn as_any(&self) -> &dyn Any;
	
	fn destroy(&mut self) -> Result<(), Error> {
		Ok(())
	}
}

pub trait SceneContext<S>: Any + Send + std::fmt::Debug {
	fn as_any(&self) -> &dyn Any;
	
	fn destroy(&mut self) -> Result<(), Error> {
		Ok(())
	}
}

pub trait TargetContext<T>: Any + Send + std::fmt::Debug {
	fn as_any(&self) -> &dyn Any;
	
	fn destroy(&mut self) -> Result<(), Error> {
		Ok(())
	}
}

pub mod batch {
	use {vec_map::VecMap, vk::VkDrawIndirectCommand, std::convert::identity};
	
	#[derive(Debug)]
	pub struct Batch<H: Copy + Ord, A: std::alloc::Allocator, DH = (), DL = (), D = ()> {
		pub batch:      VecMap<H, DH>,
		pub batch_data: Vec<DL, A>,
		pub data:       D
	}
	
	impl<H: Copy + Ord, A: std::alloc::Allocator, DH, DL, D> Batch<H, A, DH, DL, D> {
		pub fn new(alloc: A) -> Self where D: Default {
			Self {
				batch:      VecMap::new(),
				batch_data: Vec::new_in(alloc),
				data:       D::default()
			}
		}
		
		pub fn insert(&mut self, handle: H, instance: DH, data: DL) -> Result<(), ()> {
			match self.batch.inner().binary_search_by_key(&&handle, |(h, _)| h) {
				Ok(_) => Err(()),
				Err(i) => {
					self.batch.inner().insert(i, (handle, instance));
					self.batch_data.insert(i, data);
					Ok(())
				}
			}
		}
		
		pub fn remove(&mut self, handle: H) -> Result<(), ()> {
			match self.batch.inner().binary_search_by_key(&&handle, |(h, _)| h) {
				Err(_) => Err(()),
				Ok(i) => {
					self.batch.inner().remove(i);
					self.batch_data.remove(i);
					Ok(())
				}
			}
		}
	}
	
	impl<H: Copy + Ord, A: std::alloc::Allocator, DH, DL, D> std::ops::Index<H> for Batch<H, A, DH, DL, D> {
		type Output = DL;
		
		fn index(&self, index: H) -> &Self::Output {
			&self.batch_data[self.batch.binary_search_by_key(&&index, |(h, _)| h)
				.expect("instance is not present")]
		}
	}
	
	impl<H: Copy + Ord, A: std::alloc::Allocator, DH, DL, D> std::ops::IndexMut<H> for Batch<H, A, DH, DL, D> {
		fn index_mut(&mut self, index: H) -> &mut Self::Output {
			&mut self.batch_data[self.batch.binary_search_by_key(&&index, |(h, _)| h)
				.expect("instance is not present")]
		}
	}
	
	#[derive(Debug)]
	pub struct CmdBatch<H: Copy + Ord, A: std::alloc::Allocator, DH = (), DL = (), D = ()> {
		pub batch:      VecMap<H, DH>,
		pub batch_data: Vec<DL, A>,
		pub commands:   Vec<VkDrawIndirectCommand, A>,
		pub data:       D
	}
	
	impl<H: Copy + Ord, A: std::alloc::Allocator, DH, DL, D> CmdBatch<H, A, DH, DL, D> {
		pub fn new(alloc: A) -> Self where A: Clone, D: Default {
			Self {
				batch:      VecMap::new(),
				batch_data: Vec::new_in(alloc.clone()),
				commands:   Vec::new_in(alloc),
				data:       D::default()
			}
		}
		
		pub fn insert(&mut self, handle: H, instance: DH, data: DL, cmd: VkDrawIndirectCommand) -> Result<(), ()> {
			match self.batch.get_index_of(&handle) {
				Ok(_) => Err(()),
				Err(i) => {
					self.batch.inner().insert(i, (handle, instance));
					self.batch_data.insert(i, data);
					self.commands.insert(i, cmd);
					Ok(())
				}
			}
		}
		
		pub fn remove(&mut self, handle: H) -> Result<(), ()> {
			match self.batch.get_index_of(&handle) {
				Err(_) => Err(()),
				Ok(i) => {
					self.batch.inner().remove(i);
					self.batch_data.remove(i);
					self.commands.remove(i);
					Ok(())
				}
			}
		}
	}
	
	#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
	pub struct CmdBatchMeshData<Handle> {
		pub handle:       Handle,
		pub vertex_count: u32,
		pub first_vertex: u32,
	}
	
	impl<Handle> CmdBatchMeshData<Handle> {
		pub fn new(handle: Handle, vertex_count: u32, first_vertex: u32) -> Self {
			Self { handle, vertex_count, first_vertex }
		}
	}
	
	#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
	pub struct CmdBatchInstData<Handle, DataHost, DataLocal> {
		pub handle:     Handle,
		pub data_host:  DataHost,
		pub data_local: DataLocal
	}
	
	impl<Handle, DataHost, DataLocal> CmdBatchInstData<Handle, DataHost, DataLocal> {
		pub fn new(handle: Handle, host: DataHost, local: DataLocal) -> Self {
			Self { handle, data_host: host, data_local: local }
		}
	}
	
	impl<
		H: Copy + Ord,
		A: std::alloc::Allocator,
		DataLocal,
		Data,
		BatchHandle: Copy + Ord,
		BatchDataHost,
		BatchDataLocal,
		BatchData
	> CmdBatch<H, A, Batch<BatchHandle, A, BatchDataHost, BatchDataLocal, BatchData>, DataLocal, Data> {
		pub fn insert_batch(
			&mut self,
			mesh_handle:  H,
			vertex_count: u32,
			first_vertex: u32,
			batch:        Batch<BatchHandle, A, BatchDataHost, BatchDataLocal, BatchData>,
			data:         DataLocal,
			offset:       impl FnOnce(&Batch<BatchHandle, A, BatchDataHost, BatchDataLocal, BatchData>) -> u32
		) -> bool {
			let cmd = VkDrawIndirectCommand {
				vertexCount:   vertex_count,
				instanceCount: batch.batch.len() as _,
				firstVertex:   first_vertex,
				firstInstance: offset(&batch)
			};
			self.insert(mesh_handle, batch, data, cmd).is_ok()
		}
		
		pub fn insert_instance(
			&mut self,
			mesh_handle:  H,
			vertex_count: u32,
			first_vertex: u32,
			inst_handle:  BatchHandle,
			inst_host:    BatchDataHost,
			inst_local:   BatchDataLocal,
			batch:        impl FnOnce() -> Batch<BatchHandle, A, BatchDataHost, BatchDataLocal, BatchData>,
			offset:       impl FnOnce(&Batch<BatchHandle, A, BatchDataHost, BatchDataLocal, BatchData>) -> u32
		) -> bool {
			let result = match self.batch.get_index_of(&mesh_handle) {
				Ok(i) => Ok((&mut self.batch.inner()[i].1, &mut self.commands[i])),
				Err(i) => {
					let inner: &mut Vec<_> = self.batch.inner();
					inner.insert(i, (mesh_handle, batch()));
					
					self.commands.insert(i, VkDrawIndirectCommand {
						vertexCount:   vertex_count,
						instanceCount: 0,
						firstVertex:   first_vertex,
						firstInstance: 0
					});
					
					Err((&mut inner[i].1, &mut self.commands[i]))
				}
			};
			
			let inserted = result.is_ok();
			let (instances, command) = result.unwrap_or_else(identity);
			instances.insert(inst_handle, inst_host, inst_local)
				.expect("instance already present");
			command.instanceCount = instances.batch.len() as _;
			command.firstInstance = offset(instances);
			inserted
		}
		
		pub fn remove_instance(
			&mut self,
			//handle: Handle<Mesh>,
			instance: BatchHandle,
			offset:   impl FnOnce(&Batch<BatchHandle, A, BatchDataHost, BatchDataLocal, BatchData>) -> u32
		) -> Result<(), ()> {
			let inner: &mut Vec<_> = self.batch.inner();
			
			let i = 'outer: {
				for (i, (_, instances)) in inner.iter_mut().enumerate() {
					if instances.remove(instance).is_ok() {
						break 'outer i;
					}
				}
				
				return Err(());
			};
			
			let (_, instances) = &mut inner[i];
			
			self.commands[i].instanceCount = instances.batch.len() as _;
			self.commands[i].firstInstance = offset(instances);
			
			if instances.batch.is_empty() {
				inner.remove(i);
				self.commands.remove(i);
			}
			
			Ok(())
		}
	}
}

pub mod metrics {
	use std::{marker::PhantomData, sync::atomic::AtomicUsize};
	
	#[derive(Debug)]
	pub struct RenderSystemMetrics<T> {
		pub instances_added:    AtomicUsize,
		pub instances_updated:  AtomicUsize,
		pub instances_removed:  AtomicUsize,
		pub instances_time:     AtomicUsize,
		pub resources_uploaded: AtomicUsize,
		pub resources_unloaded: AtomicUsize,
		pub resources_time:     AtomicUsize,
		pub redraws:            AtomicUsize,
		pub marker:             std::marker::PhantomData<T>
	}
	
	impl<T> RenderSystemMetrics<T> {
		pub fn new() -> Self {
			Self::default()
		}
		
		pub fn reset(&self) {
			use std::sync::atomic::Ordering::*;
			self.instances_added.store(0, SeqCst);
			self.instances_updated.store(0, SeqCst);
			self.instances_removed.store(0, SeqCst);
			self.instances_time.store(0, SeqCst);
			self.resources_uploaded.store(0, SeqCst);
			self.resources_unloaded.store(0, SeqCst);
			self.resources_time.store(0, SeqCst);
		}
		
		pub fn update(
			&self,
			instances_added:    usize,
			instances_updated:  usize,
			instances_removed:  usize,
			instances_time:     usize,
			resources_uploaded: usize,
			resources_unloaded: usize,
			resources_time:     usize,
			redraws:            usize
		) {
			use std::sync::atomic::Ordering::*;
			if instances_added != 0 { self.instances_added.fetch_add(instances_added, SeqCst); }
			if instances_updated != 0 { self.instances_updated.fetch_add(instances_updated, SeqCst); }
			if instances_removed != 0 { self.instances_removed.fetch_add(instances_removed, SeqCst); }
			if instances_time != 0 { self.instances_time.fetch_add(instances_time, SeqCst); }
			if resources_uploaded != 0 { self.resources_uploaded.fetch_add(resources_uploaded, SeqCst); }
			if resources_unloaded != 0 { self.resources_unloaded.fetch_add(resources_unloaded, SeqCst); }
			if resources_time != 0 { self.resources_time.fetch_add(resources_time, SeqCst); }
			if redraws != 0 { self.redraws.fetch_add(redraws, SeqCst); }
		}
	}
	
	impl<T> Default for RenderSystemMetrics<T> {
		fn default() -> Self {
			Self {
				instances_added:    AtomicUsize::new(0),
				instances_updated:  AtomicUsize::new(0),
				instances_removed:  AtomicUsize::new(0),
				instances_time:     AtomicUsize::new(0),
				resources_uploaded: AtomicUsize::new(0),
				resources_unloaded: AtomicUsize::new(0),
				resources_time:     AtomicUsize::new(0),
				redraws:            AtomicUsize::new(0),
				marker:             PhantomData
			}
		}
	}
	
	impl<T> std::fmt::Display for RenderSystemMetrics<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			use std::sync::atomic::Ordering::*;
			write!(
				f,
				"Render System Metrics of `{}`: instances: +{} ~{} -{} ({}ms), resources: +{} ~0 -{} ({}ms), redraws: {}",
				std::any::type_name::<T>(),
				self.instances_added.load(Relaxed),
				self.instances_updated.load(Relaxed),
				self.instances_removed.load(Relaxed),
				self.instances_time.load(Relaxed),
				self.resources_uploaded.load(Relaxed),
				self.resources_unloaded.load(Relaxed),
				self.resources_time.load(Relaxed),
				self.redraws.load(Relaxed)
			)
		}
	}
}

pub static DEFAULT_STAGE: VkPipelineShaderStageCreateInfo = VkPipelineShaderStageCreateInfo {
	sType:  VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
	pNext:  None,
	flags:  0,
	stage:  VK_SHADER_STAGE_VERTEX_BIT,
	module: VK_NULL_HANDLE,
	pName:  "main\0".as_ptr(),
	pSpecializationInfo: None
};