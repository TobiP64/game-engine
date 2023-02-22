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
	super::misc::{choose_memory_type, compatibility},
	crate::utils::{align, generic_seek},
	std::{
		io,
		ptr::{null_mut, null},
		sync::{*, atomic::*},
		mem::ManuallyDrop,
		collections::{BTreeSet, BTreeMap},
		borrow::Borrow,
		iter::FromIterator
	},
	vk::*,
	vec_map::VecMap
};

pub use self::{sub_alloc::*, access::*};

/// Device local memory with full GPU read/write/atomic speed.
pub const MEMORY_PROPERTY_FLAGS_LOCAL: [u32; 2] = [
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
		| VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT as u32
];

pub const MEMORY_PROPERTY_FLAGS_LOCAL_PREFERRED: [u32; 4] = [
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
		| VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT as u32,
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
	VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
];

/// Memory that is suitable for both, uploads and downloads, to/from the GPU.
pub const MEMORY_PROPERTY_FLAGS_DYNAMIC: [u32; 1] = [
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
];

pub const MEMORY_PROPERTY_FLAGS_DYNAMIC_PREFERRED: [u32; 2] = [
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
	VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
];

/// Memory that is most suitable for uploading data to the GPU.
pub const MEMORY_PROPERTY_FLAGS_UPLOAD: [u32; 1] = [
	VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
];

/// Memory that is most suitable for downloading data from the GPU.
pub const MEMORY_PROPERTY_FLAGS_DOWNLOAD: [u32; 2] = [
	VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_CACHED_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
	VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32
		| VK_MEMORY_PROPERTY_HOST_CACHED_BIT as u32
];

type VkResource = u64;

enum BindMode<'a> {
	Full(&'a mut VkDeviceMemory, &'a mut VkDeviceSize),
	Sparse(VkDeviceSize, VkDeviceSize, &'a mut Vec<VkSparseMemoryBind>),
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ResourceBind {
	pub offset: VkDeviceSize,
	pub size:   VkDeviceSize
}

#[derive(Debug, Default, Copy, Clone)]
pub struct HeapsBindInfo<'a> {
	pub bind_buffers:                    &'a [VkBuffer],
	pub bind_images:                     &'a [VkImage],
	pub bind_sparse_buffers:             &'a [(VkBuffer, &'a [ResourceBind])],
	pub bind_sparse_images:              &'a [(VkImage, &'a [ResourceBind])],
	pub unbind_buffers:                  &'a [VkBuffer],
	pub unbind_images:                   &'a [VkImage],
	pub unbind_sparse_buffers:           &'a [(VkBuffer, &'a [ResourceBind])],
	pub unbind_sparse_images:            &'a [(VkImage, &'a [ResourceBind])],
	pub dedicated_bind_buffers:          &'a [VkBuffer],
	pub dedicated_bind_images:           &'a [VkImage],
	pub dedicated_bind_sparse_buffers:   &'a [VkBuffer],
	pub dedicated_bind_sparse_images:    &'a [VkImage],
	pub dedicated_unbind_buffers:        &'a [VkBuffer],
	pub dedicated_unbind_images:         &'a [VkImage],
	pub dedicated_unbind_sparse_buffers: &'a [VkBuffer],
	pub dedicated_unbind_sparse_images:  &'a [VkImage],
	pub wait_semaphores:                 &'a [VkSemaphore],
	pub signal_semaphores:               &'a [VkSemaphore],
	pub fence:                           VkFence
}

impl<'a> HeapsBindInfo<'a> {
	pub fn bind_buffers(buffers: &'a [VkBuffer]) -> Self {
		Self { bind_buffers: buffers, ..Self::default() }
	}
	
	pub fn bind_images(images: &'a [VkImage]) -> Self {
		Self { bind_images: images, ..Self::default() }
	}
	
	pub fn unbind_buffers(buffers: &'a [VkBuffer]) -> Self {
		Self { unbind_buffers: buffers, ..Self::default() }
	}
	
	pub fn unbind_images(images: &'a [VkImage]) -> Self {
		Self { unbind_images: images, ..Self::default() }
	}
}

#[derive(Debug, Copy, Clone)]
pub struct HeapsMemorySelectionInfo<'a> {
	pub memory_properties: &'a VkPhysicalDeviceMemoryProperties,
	pub memory_type_bits:  u32,
	pub property_flags:    &'a [VkMemoryPropertyFlags]
}

#[derive(Debug, Copy, Clone)]
pub struct HeapsExtensionInfo {
	pub version:                   u32,
	pub bind_memory_2:             bool,
	pub get_memory_requirements_2: bool,
	pub memory_priority:           bool,
	pub memory_budget:             bool
}

#[derive(Debug)]
pub struct Heaps {
	device: ManuallyDrop<VkDeviceImpl>,
	compat: HeapsCompatibilityLayer,
	config: HeapsConfig,
	props:  VkPhysicalDeviceMemoryProperties,
	sync:   RwLock<HeapsSync>
}

struct HeapsCompatibilityLayer {
	memory_priority:                bool,
	memory_budget:                  bool,
	get_buffer_memory_requirements: fn(&VkDeviceImpl, &VkBufferMemoryRequirementsInfo2, &mut VkMemoryRequirements2),
	get_image_memory_requirements:  fn(&VkDeviceImpl, &VkImageMemoryRequirementsInfo2, &mut VkMemoryRequirements2),
	bind_buffer_memory:             fn(&VkDeviceImpl, &[VkBindBufferMemoryInfo]) -> VkResult,
	bind_image_memory:              fn(&VkDeviceImpl, &[VkBindImageMemoryInfo]) -> VkResult
}

impl Default for HeapsCompatibilityLayer {
	fn default() -> Self {
		Self {
			memory_priority:                false,
			memory_budget:                  false,
			get_buffer_memory_requirements: compatibility::vk10GetBufferMemoryRequirements,
			get_image_memory_requirements:  compatibility::vk10GetImageMemoryRequirements,
			bind_buffer_memory:             compatibility::vk10BindBufferMemory,
			bind_image_memory:              compatibility::vk10BindImageMemory
		}
	}
}

impl std::fmt::Debug for HeapsCompatibilityLayer {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GpaCompatibilityLayer")
			.field("memory_priority", &self.memory_priority)
			.field("memory_budget", &self.memory_budget)
			.finish()
	}
}

#[derive(Debug, Default)]
struct HeapsConfig {
	block_size: VkDeviceSize,
	min_blocks: usize,
	max_blocks: usize
}

#[derive(Default, Debug)]
struct HeapsSync {
	types:     u32,
	memories:  [HeapsMemoryType; 32],
	resources: VecMap<VkResource, HeapResource>
}

#[derive(Debug, Default)]
struct HeapsMemoryType {
	blocks:    Vec<HeapMemory>,
	free_len:  BTreeSet<HeapMemoryRange>
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct HeapMemoryRange {
	len: VkDeviceSize,
	idx: usize,
	off: VkDeviceSize
}

impl HeapMemoryRange {
	fn new(len: VkDeviceSize, idx: usize,  off: VkDeviceSize) -> Self {
		Self { len, idx, off }
	}
}

impl Borrow<VkDeviceSize> for HeapMemoryRange {
	fn borrow(&self) -> &VkDeviceSize {
		&self.len
	}
}

impl HeapsMemoryType {
	fn remove_by_offset(&mut self, idx: usize, offset: VkDeviceSize) -> Option<VkDeviceSize> {
		let len = *self.blocks[idx].free_off.get(&offset)?;
		self.free_len.remove(&HeapMemoryRange::new(len, idx, offset))
			.then_some(())?;
		self.blocks[idx].free_off.remove(&offset)
			.expect("failed to remove free entry");
		Some(len)
	}
	
	fn insert(&mut self, idx: usize, offset: VkDeviceSize, len: VkDeviceSize) {
		self.blocks[idx].free_off.insert(offset, len);
		self.free_len.insert(HeapMemoryRange::new(len, idx, offset));
	}
}

#[derive(Debug, Default)]
struct HeapMemory {
	handle:    VkDeviceMemory,
	dedicated: bool,
	free_off:  BTreeMap<VkDeviceSize, VkDeviceSize>,
	data:      AtomicPtr<u8>, // required for Send + Sync + Default
	refs:      AtomicUsize
}

#[derive(Default)]
struct HeapResource {
	requirements: VkMemoryRequirements,
	ranges:       BTreeMap<VkDeviceSize, HeapMemoryRange>
}

impl std::fmt::Debug for HeapResource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		struct RangesDbg<'a>(&'a BTreeMap<VkDeviceSize, HeapMemoryRange>);
		
		impl std::fmt::Debug for RangesDbg<'_> {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.debug_list()
					.entries(self.0.iter().map(|(k, v)| RangeDbg(*k, *v)))
					.finish()
			}
		}
		
		struct RangeDbg(VkDeviceSize, HeapMemoryRange);
		
		impl std::fmt::Debug for RangeDbg {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				let Self(res_off, HeapMemoryRange { len, idx, off }) = *self;
				write!(f, "{:08X} - {:08X} @ VkMemory#{:X} {:08X} - {:08X} ({} bytes)",
					   res_off, res_off + len, idx, off, off + len, len)
			}
		}
		
		f.debug_struct(std::any::type_name::<Self>())
			.field("requirements", &self.requirements)
			.field("ranges", &RangesDbg(&self.ranges))
			.finish()
	}
}

impl Heaps {
	pub fn create(
		device:         &VkDeviceImpl,
		props:          VkPhysicalDeviceMemoryProperties,
		block_size:     VkDeviceSize,
		extension_info: &HeapsExtensionInfo
	) -> Self {
		debug_assert!(block_size <= std::u32::MAX as VkDeviceSize);
		Self {
			device:        ManuallyDrop::new(device.clone()),
			compat:        HeapsCompatibilityLayer {
				memory_priority:                extension_info.memory_priority,
				memory_budget:                  extension_info.memory_budget,
				get_buffer_memory_requirements: match (extension_info.version, extension_info.get_memory_requirements_2) {
					(VK_API_VERSION_1_0, false) => compatibility::vk10GetBufferMemoryRequirements,
					(VK_API_VERSION_1_0, true)  => VkDeviceImpl::getBufferMemoryRequirements2KHR,
					_                           => VkDeviceImpl::getBufferMemoryRequirements2
				},
				get_image_memory_requirements:  match (extension_info.version, extension_info.get_memory_requirements_2) {
					(VK_API_VERSION_1_0, false) => compatibility::vk10GetImageMemoryRequirements,
					(VK_API_VERSION_1_0, true)  => VkDeviceImpl::getImageMemoryRequirements2KHR,
					_                           => VkDeviceImpl::getImageMemoryRequirements2
				},
				bind_buffer_memory:            match (extension_info.version, extension_info.bind_memory_2) {
					(VK_API_VERSION_1_0, false) => compatibility::vk10BindBufferMemory,
					(VK_API_VERSION_1_0, true)  => VkDeviceImpl::bindBufferMemory2KHR,
					_                           => VkDeviceImpl::bindBufferMemory2
				},
				bind_image_memory:             match (extension_info.version, extension_info.bind_memory_2) {
					(VK_API_VERSION_1_0, false) => compatibility::vk10BindImageMemory,
					(VK_API_VERSION_1_0, true)  => VkDeviceImpl::bindImageMemory2KHR,
					_                           => VkDeviceImpl::bindImageMemory2
				}
			},
			config:        HeapsConfig { block_size, min_blocks: 0, max_blocks: 16 },
			props,
			sync:          RwLock::default()
		}
	}
	
	pub fn alloc_dedicated(
		&self,
		mem_props:    &VkPhysicalDeviceMemoryProperties,
		prop_flags:   &[u32],
		allocator:    Option<&VkAllocationCallbacks>,
		buffers:      &[VkBuffer],
		images:       &[VkImage]
	) -> VkResult {
		let mut guard = self.sync.write().expect("failed to acquire lock");
		let mut requirements = VkMemoryRequirements2 {
			sType: VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2,
			pNext: None,
			memoryRequirements: VkMemoryRequirements::default()
		};
		
		let mut allocate_info = VkMemoryAllocateInfo {
			sType:           VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
			pNext:           None,
			allocationSize:  0,
			memoryTypeIndex: !0
		};
		
		let mut buffer_binds = Vec::with_capacity(buffers.len());
		let mut image_binds = Vec::with_capacity(images.len());
		
		for buffer in buffers.iter().copied() {
			self.device.getBufferMemoryRequirements2(&VkBufferMemoryRequirementsInfo2 {
				sType:  VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2,
				pNext:  None,
				buffer
			}, &mut requirements);
			
			allocate_info.allocationSize = align(
				allocate_info.allocationSize,
				requirements.memoryRequirements.alignment
			);
			
			buffer_binds.push(VkBindBufferMemoryInfo {
				sType:        VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO,
				pNext:        None,
				buffer,
				memory:       VK_NULL_HANDLE,
				memoryOffset: allocate_info.allocationSize
			});
			
			allocate_info.allocationSize += requirements.memoryRequirements.size;
			allocate_info.memoryTypeIndex &= requirements.memoryRequirements.memoryTypeBits;
			
			guard.resources.insert(buffer, HeapResource {
				requirements: requirements.memoryRequirements,
				ranges: BTreeMap::from_iter(Some((0, HeapMemoryRange {
					len: requirements.memoryRequirements.size,
					idx: !0,
					off: 0
				})))
			});
		}
		
		for image in images.iter().copied() {
			self.device.getImageMemoryRequirements2(&VkImageMemoryRequirementsInfo2 {
				sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2,
				pNext: None,
				image
			}, &mut requirements);
			
			allocate_info.allocationSize = align(
				allocate_info.allocationSize,
				requirements.memoryRequirements.alignment
			);
			
			image_binds.push(VkBindImageMemoryInfo {
				sType:        VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO,
				pNext:        None,
				image,
				memory:       VK_NULL_HANDLE,
				memoryOffset: allocate_info.allocationSize
			});
			
			allocate_info.allocationSize += requirements.memoryRequirements.size;
			allocate_info.memoryTypeIndex &= requirements.memoryRequirements.memoryTypeBits;
			
			guard.resources.insert(image, HeapResource {
				requirements: requirements.memoryRequirements,
				ranges: BTreeMap::from_iter(Some((0, HeapMemoryRange {
					len: requirements.memoryRequirements.size,
					idx: !0,
					off: 0
				})))
			});
		}
		
		allocate_info.memoryTypeIndex = choose_memory_type(
			mem_props,
			allocate_info.memoryTypeIndex,
			prop_flags
		).expect("failed to find suitable memory type");
		
		let mut memory = VK_NULL_HANDLE;
		self.device.allocateMemory(&allocate_info, allocator, &mut memory);
		buffer_binds.iter_mut().for_each(|info| info.memory = memory);
		image_binds.iter_mut().for_each(|info| info.memory = memory);
		if !buffer_binds.is_empty() { self.device.bindBufferMemory2(buffer_binds.as_slice())?; }
		if !image_binds.is_empty() { self.device.bindImageMemory2(image_binds.as_slice())?; }
		
		let type_idx = allocate_info.memoryTypeIndex as usize;
		let idx = guard.memories[type_idx].blocks.len();
		guard.memories[type_idx].blocks.push(HeapMemory { handle: memory, dedicated: true, ..HeapMemory::default() });
		
		for buf in buffers { guard.resources[buf].ranges.get_mut(&0).unwrap().idx = idx | (type_idx << 32); }
		for img in images { guard.resources[img].ranges.get_mut(&0).unwrap().idx = idx | (type_idx << 32); }
		
		std::mem::drop(guard);
		log::trace!("[MEMORY] allocated memory block VkMemory#{:X} (type: {}, size: {:#X}B, dedicated)",
					memory, type_idx, self.config.block_size);
		
		VK_SUCCESS
	}
	
	pub fn add(&self, memory_types: u32, buffers: &[VkBuffer], images: &[VkImage]) {
		let mut guard = self.sync.write().expect("failed to acquire lock");
		let mut requirements = VkMemoryRequirements2 {
			sType:              VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2,
			pNext:              None,
			memoryRequirements: VkMemoryRequirements::default()
		};
		
		for buffer in buffers.iter().copied() {
			(self.compat.get_buffer_memory_requirements)(&self.device, &VkBufferMemoryRequirementsInfo2 {
				sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2,
				pNext: None,
				buffer
			}, &mut requirements);
			
			requirements.memoryRequirements.memoryTypeBits &= memory_types;
			guard.resources.insert(buffer, HeapResource { requirements: requirements.memoryRequirements, ..Default::default() });
		}
		
		for image in images.iter().copied() {
			(self.compat.get_image_memory_requirements)(&self.device, &VkImageMemoryRequirementsInfo2 {
				sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2,
				pNext: None,
				image
			}, &mut requirements);
			
			requirements.memoryRequirements.memoryTypeBits &= memory_types;
			guard.resources.insert(image, HeapResource { requirements: requirements.memoryRequirements, ..Default::default() });
		}
	}
	
	pub fn remove(&mut self, buffers: &[VkBuffer], images: &[VkImage]) {
		let mut guard = self.sync.write().expect("failed to acquire lock");
		buffers.iter().for_each(|buffer| { guard.resources.remove(buffer); });
		images.iter().for_each(|image| { guard.resources.remove(image); });
	}
	
	fn bind_internal(
		&self,
		sync:      &mut HeapsSync,
		handle:    VkResource,
		mut mode:  BindMode,
		allocator: Option<&VkAllocationCallbacks>
	) -> VkResult {
		let HeapsSync { resources, memories, types, .. } = sync;
		let resource = &mut resources[&handle];
		let requirements = resource.requirements;
		let (offset, size, min_size) = match &mode {
			BindMode::Full(..) => (0, requirements.size, requirements.size),
			BindMode::Sparse(offset, size, ..) => (*offset, *size, requirements.alignment)
		};
		let mut remaining = size;
		let type_idx = (0..32).find(|i| (requirements.memoryTypeBits & *types) >> i & 1 == 1)
			.or_else(|| (0..32).find(|i| requirements.memoryTypeBits >> i & 1 == 1))
			.expect("no suitable memory type");
		*types |= 1 << type_idx;
		let memories: &mut HeapsMemoryType = &mut memories[type_idx];
		
		if requirements.size > self.config.block_size {
			return VK_ERROR_INITIALIZATION_FAILED;
		}
		
		let mut min_range_len = min_size;
		
		'outer: loop {
			for free_range in memories.free_len.range(min_range_len..) {
				
				// check if range meets size/align requirements
				
				let region_off  = free_range.off;
				let region_len  = free_range.len;
				let region_end  = region_off + region_len;
				let aligned_off = align(region_off, requirements.alignment);
				let aligned_len = region_len - (aligned_off - region_off);
				
				if min_size > aligned_len { continue; }
				
				let aligned_len = remaining.min(aligned_len - aligned_len % min_size);
				let aligned_end = aligned_off + aligned_len;
				debug_assert!(aligned_off >= region_off && aligned_end <= region_end);
				
				// remove range from free list
				
				let mut free_range = *free_range;
				min_range_len = free_range.len + 1;
				
				if !memories.free_len.remove(&free_range) {
					log::trace!("[MEMORY] memory free ranges list is out of date");
					continue 'outer;
				};
				
				memories.blocks[free_range.idx].free_off.remove(&region_off)
					.expect("memory free ranges list is out of date");
				
				if region_off != aligned_off {
					memories.insert(free_range.idx, free_range.off, aligned_off - free_range.off);
				}
				
				if region_end != aligned_end {
					memories.insert(free_range.idx, aligned_end, region_end - aligned_end);
				}
				
				// add range to resource
				
				let res_off = offset + size - remaining;
				
				if resource.ranges.range(res_off..res_off + aligned_len).next().is_some() {
					panic!("range is already bound");
				}
				
				free_range.idx |= type_idx << 32;
				
				match (
					resource.ranges.range(0..=res_off).next_back(),	        // prev range
					resource.ranges.get(&(res_off + aligned_len)).copied()	// next range
				) {
					(Some((&prev_range_off, &prev_range)), next_range) if prev_range_off + prev_range.len == res_off
						&& prev_range.idx == free_range.idx
						&& prev_range.off + prev_range.len == free_range.off => {
						resource.ranges.get_mut(&prev_range_off).unwrap().len += aligned_len;
						
						match next_range {
							Some(next_range) if next_range.idx == free_range.idx
								&& next_range.off == free_range.off + aligned_len => {
								resource.ranges.get_mut(&prev_range_off).unwrap().len += next_range.len;
								resource.ranges.remove(&(res_off + aligned_len));
							},
							_ => ()
						}
					}
					(None, Some(next_range)) if next_range.idx == free_range.idx
						&& next_range.off == free_range.off + aligned_len => {
						let mut range = resource.ranges.remove(&(res_off + aligned_len)).unwrap();
						range.off -= aligned_len;
						range.len += aligned_len;
						resource.ranges.insert(res_off, range);
					}
					_ => {
						resource.ranges.insert(res_off, HeapMemoryRange {
							len: aligned_len, idx: free_range.idx, off: free_range.off });
					}
				}
				
				// add range to bindings list
				
				match &mut mode {
					BindMode::Full(mem, off) => {
						**mem = memories.blocks[free_range.idx & 0xFFFF_FFFF].handle;
						**off = aligned_off;
						return VK_SUCCESS;
					}
					BindMode::Sparse(.., infos) => infos.push(VkSparseMemoryBind {
						resourceOffset: res_off,
						size:           aligned_len,
						memory:         memories.blocks[free_range.idx & 0xFFFF_FFFF].handle,
						memoryOffset:   aligned_off,
						flags:          0
					})
				}
				
				log::trace!("[MEMORY] Resource#{:X}: freed range {:08X} - {:08X} @ VkMemory#{:X} (type {}) {:08X} - {:08X} ({} bytes)",
					handle, res_off, res_off + aligned_len, memories.blocks[free_range.idx & 0xFFFF_FFFF].handle, type_idx, aligned_off, aligned_end, aligned_len);
				
				remaining -= aligned_len;
				if remaining == 0 { return VK_SUCCESS; }
				
				continue 'outer;
			}
			
			// out of memory, alloc new block
			
			let idx = memories.blocks.iter()
				.enumerate()
				.find(|(_, mem)| mem.handle == VK_NULL_HANDLE)
				.map(|(i, _)| i)
				.unwrap_or(memories.blocks.len());
			
			if idx > self.config.max_blocks {
				return VK_ERROR_OUT_OF_POOL_MEMORY;
			}
			
			let mut memory = VK_NULL_HANDLE;
			self.device.allocateMemory(&VkMemoryAllocateInfo {
				sType:           VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
				pNext:           None,
				allocationSize:  self.config.block_size,
				memoryTypeIndex: type_idx as _,
			}, allocator, &mut memory)?;
			
			memories.blocks.insert(idx, HeapMemory {
				handle:   memory,
				free_off: BTreeMap::from_iter(Some((0, self.config.block_size))),
				..HeapMemory::default()
			});
			
			memories.free_len.insert(HeapMemoryRange {
				len: self.config.block_size,
				idx,
				off: 0
			});
			
			log::trace!("[MEMORY] allocated memory block VkMemory#{:X} (type: {}, size: {:#X}B)",
						memory, type_idx, self.config.block_size);
		}
	}
	
	fn unbind_internal(
		&self,
		sync:      &mut HeapsSync,
		handle:    VkResource,
		mut mode:  BindMode,
		_allocator: Option<&VkAllocationCallbacks>
	) -> VkResult {
		fn free_range(
			res:     VkResource,
			res_off: VkDeviceSize,
			mem:     VkDeviceMemory,
			mem_idx: usize,
			mem_off: VkDeviceSize,
			len:     VkDeviceSize,
			mode:    &mut BindMode,
			msg:     &str
		) {
			log::trace!("[MEMORY] Resource#{:X}: freed range {:08X} - {:08X} @ VkMemory#{:X} (type {}) {:08X} - {:08X} ({} bytes, {})",
				res, res_off, res_off + len, mem, mem_idx >> 32, mem_off, mem_off + len, len, msg);
			
			if let BindMode::Sparse(.., infos) = mode {
				infos.push(VkSparseMemoryBind {
					resourceOffset: res_off,
					size:           len,
					..VkSparseMemoryBind::default()
				});
			}
		}
		
		let resource: &mut HeapResource = &mut sync.resources[&handle];
		let requirements = resource.requirements;
		let (offset, size) = match &mode {
			BindMode::Full(..) => (0, requirements.size),
			BindMode::Sparse(offset, size, ..) => (*offset, *size)
		};
		let end = offset + size;
		
		debug_assert!(offset + size <= requirements.size);
		
		let (off, len)  = (offset, size);
		let start_range = resource.ranges.range(..off).last().map(|(k, v)| (*k, *v));
		let end_range   = resource.ranges.range(..off + len).last().map(|(k, v)| (*k, *v));
		
		// if the unbound range is fully contained within a range, split this range up
		if start_range.is_some() && start_range == end_range {
			let (range_off, range) = if let Some(v) = start_range { v } else { unreachable!() };
			let upd_range_len = off - range_off;
			let upd_range_end = range_off + range.len;
			let free_off = range.off + upd_range_len;
			let new_range = HeapMemoryRange {
				len: upd_range_end - end,
				idx: range.idx,
				off: range.off + upd_range_len + len
			};
			
			resource.ranges.get_mut(&range_off).unwrap().len = upd_range_len; // update the previous range
			resource.ranges.insert(end, new_range);                     // insert the next range
			sync.memories[range.idx >> 32].insert(range.idx & 0xFFFF_FFFF, free_off, len);
			
			free_range(
				handle,
				off,
				sync.memories[range.idx >> 32].blocks[range.idx & 0xFFFF_FFFF].handle,
				range.idx,
				free_off,
				len,
				&mut mode,
				"contained"
			);
			
			VK_SUCCESS
		} else {
			// if the first range is not fully contained within the unbound range, free only the end of the first range
			if let Some((start_range_off, start_range)) = start_range {
				let upd_range_len = off - start_range_off;
				resource.ranges.get_mut(&start_range_off).unwrap().len = upd_range_len;
				let free_off = start_range.off + upd_range_len;
				let free_len = start_range.len - upd_range_len + sync.memories[start_range.idx >> 32]
					.remove_by_offset(start_range.idx & 0xFFFF_FFFF, free_off)
					.unwrap_or(0);
				
				sync.memories[start_range.idx >> 32].insert(start_range.idx & 0xFFFF_FFFF, free_off, free_len);
				
				free_range(
					handle,
					start_range_off,
					sync.memories[start_range.idx >> 32].blocks[start_range.idx & 0xFFFF_FFFF].handle,
					start_range.idx,
					free_off,
					free_len,
					&mut mode,
					"partial start"
				);
			}
			
			// if the last range is not fully contained within the unbound range, free only the beginning of the last range
			if let Some((end_range_off, _)) = end_range {
				let end_range = resource.ranges.remove(&end_range_off).unwrap();
				let free_len = off - end_range_off;
				let upd_range = HeapMemoryRange {
					len: end_range.len - free_len,
					idx: end_range.idx,
					off: end_range.off + free_len
				};
				
				resource.ranges.insert(end, upd_range);
				
				if let Some((_, len)) = sync.memories[upd_range.idx >> 32].blocks[upd_range.idx & 0xFFFF_FFFF].free_off
					.range_mut(..end_range.off)
					.last()
					.filter(|&(&off, &mut len)| off + len == end_range.off)
				{
					*len += free_len
				}
				
				free_range(
					handle,
					end_range_off,
					sync.memories[end_range.idx >> 32].blocks[end_range.idx & 0xFFFF_FFFF].handle,
					end_range.idx,
					end_range.off,
					free_len,
					&mut mode,
					"partial end"
				);
			}
			
			// free remaining ranges
			for (offset, range) in resource.ranges
				.drain_filter(|off, rng| *off >= offset && *off + rng.len <= offset + size)
			{
				sync.memories[range.idx >> 32].insert(range.idx & 0xFFFF_FFFF, range.off, range.len);
				
				free_range(
					handle,
					offset,
					sync.memories[range.idx >> 32].blocks[range.idx & 0xFFFF_FFFF].handle,
					range.idx,
					range.off,
					range.len,
					&mut mode,
					"full"
				);
			}
			
			// TODO free empty memory blocks (just check if there is only one free range that spans the whole memory block and free the block if this is the case)
			
			VK_SUCCESS
		}
	}
	
	pub fn bind(
		&self,
		queue:     Option<&VkQueueImpl>,
		info:      &HeapsBindInfo,
		allocator: Option<&VkAllocationCallbacks>
	) -> VkResult {
		let mut guard = self.sync.write().expect("failed to acquire lock");
		
		if !info.bind_buffers.is_empty() {
			(self.compat.bind_buffer_memory)(&self.device, &info.bind_buffers.iter().copied().map(|buffer| {
				let mut info = VkBindBufferMemoryInfo {
					sType:        VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO,
					pNext:        None,
					buffer,
					memory:       VK_NULL_HANDLE,
					memoryOffset: 0
				};
				
				self.bind_internal(&mut guard, buffer, BindMode::Full(&mut info.memory, &mut info.memoryOffset), allocator)?;
				Ok(info)
			}).collect::<Result<Vec<_>, VkResult>>()?)?;
		}
		
		if !info.bind_images.is_empty() {
			(self.compat.bind_image_memory)(&self.device, &info.bind_images.iter().copied().map(|image| {
				let mut info = VkBindImageMemoryInfo {
					sType:        VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO,
					pNext:        None,
					image,
					memory:       VK_NULL_HANDLE,
					memoryOffset: 0
				};
				
				self.bind_internal(&mut guard, image, BindMode::Full(&mut info.memory, &mut info.memoryOffset), allocator)?;
				Ok(info)
			}).collect::<Result<Vec<_>, VkResult>>()?)?;
		}
		
		info.unbind_buffers.iter().copied().try_for_each(|buffer| {
			self.unbind_internal(&mut guard, buffer, BindMode::Full(&mut 0, &mut 0), allocator)?;
			Ok::<_, VkResult>(())
		})?;
		
		info.unbind_images.iter().copied().try_for_each(|image| {
			self.unbind_internal(&mut guard, image, BindMode::Full(&mut 0, &mut 0), allocator)?;
			Ok::<_, VkResult>(())
		})?;
		
		if !info.dedicated_bind_buffers.is_empty() {
		
		}
		
		if !info.dedicated_bind_images.is_empty() {
			unimplemented!()
		}
		
		if info.bind_sparse_buffers.is_empty() && info.bind_sparse_images.is_empty()
			&& info.unbind_sparse_buffers.is_empty() && info.unbind_sparse_images.is_empty() {
			return VK_SUCCESS;
		}
		
		let mut buffer_infos = Vec::with_capacity(info.bind_sparse_buffers.len() + info.unbind_sparse_buffers.len());
		let mut image_infos = Vec::with_capacity(info.bind_sparse_images.len() + info.unbind_sparse_images.len());
		let mut binds_buf = Vec::with_capacity(info.bind_sparse_buffers.len() + info.unbind_sparse_buffers.len()
				+ info.bind_sparse_images.len() + info.unbind_sparse_images.len());
		
		for (buffer, binds) in info.bind_sparse_buffers.iter().copied() {
			let offset = binds_buf.len();
			
			for ResourceBind { offset, size } in binds {
				self.bind_internal(&mut guard, buffer, BindMode::Sparse(*offset, *size, &mut binds_buf), allocator)?;
			}
			
			buffer_infos.push(VkSparseBufferMemoryBindInfo {
				buffer,
				bindCount: binds_buf[offset..].len() as _,
				pBinds:    binds_buf[offset..].as_ptr()
			});
		}
		
		for (buffer, binds) in info.unbind_sparse_buffers.iter().copied() {
			let offset = binds_buf.len();
			
			for ResourceBind { offset, size } in binds {
				self.unbind_internal(&mut guard, buffer, BindMode::Sparse(*offset, *size, &mut binds_buf), allocator)?;
			}
			
			buffer_infos.push(VkSparseBufferMemoryBindInfo {
				buffer,
				bindCount: binds_buf[offset..].len() as _,
				pBinds:    binds_buf[offset..].as_ptr()
			});
		}
		
		for (image, binds) in info.bind_sparse_images.iter().copied() {
			let offset = binds_buf.len();
			
			for ResourceBind { offset, size } in binds {
				self.bind_internal(&mut guard, image, BindMode::Sparse(*offset, *size, &mut binds_buf), allocator)?;
			}
			
			image_infos.push(VkSparseImageOpaqueMemoryBindInfo {
				image,
				bindCount: binds_buf[offset..].len() as _,
				pBinds:    binds_buf[offset..].as_ptr()
			});
		}
		
		for (image, binds) in info.unbind_sparse_images.iter().copied() {
			let offset = binds_buf.len();
			
			for ResourceBind { offset, size } in binds {
				self.unbind_internal(&mut guard, image, BindMode::Sparse(*offset, *size, &mut binds_buf), allocator)?;
			}
			
			image_infos.push(VkSparseImageOpaqueMemoryBindInfo {
				image,
				bindCount: binds_buf[offset..].len() as _,
				pBinds:    binds_buf[offset..].as_ptr()
			});
		}
		
		queue.unwrap().bindSparse(&[
			VkBindSparseInfo {
				sType:                VK_STRUCTURE_TYPE_BIND_SPARSE_INFO,
				pNext:                None,
				waitSemaphoreCount:   info.wait_semaphores.len() as _,
				pWaitSemaphores:      info.wait_semaphores.as_ptr(),
				bufferBindCount:      buffer_infos.len() as _,
				pBufferBinds:         buffer_infos.as_ptr(),
				imageOpaqueBindCount: image_infos.len() as _,
				pImageOpaqueBinds:    image_infos.as_ptr(),
				imageBindCount:       0,
				pImageBinds:          null(),
				signalSemaphoreCount: info.signal_semaphores.len() as _,
				pSignalSemaphores:    info.signal_semaphores.as_ptr()
			}
		], info.fence)
	}
	
	/// Returns the memory region for the given offset of the given host-visible resource,
	/// mapping the memory if the data pointer is null.
	pub fn map_region(&self, handle: VkResource, offset: VkDeviceSize) -> Result<MemoryRegion, VkResult> {
		let guard = self.sync.read().expect("failed to acquire lock");
		let ranges: &BTreeMap<VkDeviceSize, HeapMemoryRange> = &guard.resources[&handle].ranges;
		let range = ranges.range(..offset)
			.rev()
			.take(1)
			.chain(ranges.range(offset..))
			.find(|(off, rng)| **off <= offset && **off + rng.len > offset);
		
		let range = match range {
			Some(v) => v,
			None => panic!("range with offset {:#X} not bound for resource #{:X}", offset, handle)
		};
		
		let memory = &guard
			.memories[range.1.idx >> 32]
			.blocks[range.1.idx & 0xFFFF_FFFF];
		
		if memory.data.load(Ordering::SeqCst).is_null() {
			let ptr = null_mut();
			self.device.mapMemory(
				memory.handle,
				0,
				VK_WHOLE_SIZE,
				0,
				&mut VkAnyMut::new(unsafe { &mut*ptr })
			)?;
			
			if memory.data.compare_exchange(null_mut(), ptr, Ordering::SeqCst, Ordering::SeqCst).is_err() {
				log::warn!("race condition detected");
			}
		}
		
		Ok(MemoryRegion {
			alloc:  self,
			memory: range.1.idx,
			data:   unsafe { std::slice::from_raw_parts_mut(memory.data
				.load(Ordering::SeqCst)
				.offset((range.1.off + offset) as _), range.1.len as _) }
		})
	}
	
	pub fn get_dedicated_memory(&self, resource: VkResource) -> VkDeviceMemory {
		let sync = self.sync.read().expect("failed to acquire lock");
		let mem_idx = sync.resources[&resource].ranges
			.first_key_value()
			.expect("resource has not been bound")
			.1.idx;
		
		sync.memories[mem_idx >> 32].blocks[mem_idx & 0xFFFF_FFFF].handle
	}
	
	pub fn access(&self, handle: VkResource) -> MemoryAccessor {
		let size = self.sync.read().expect("failed to acquire lock").resources[&handle].requirements.size;
		MemoryAccessor::new(self, handle, size as _)
	}
	
	pub fn maintain(&mut self, _command_buffer: &VkCommandBufferImpl) {
		unimplemented!()
	}
	
	/// Destroys this allocator.
	pub fn destroy(self, allocator: Option<&VkAllocationCallbacks>) {
		let guard = self.sync.write().expect("failed to acquire lock");
		guard.memories.iter().map(|m| m.blocks.iter()).flatten().for_each(|memory|
			self.device.freeMemory(memory.handle, allocator));
	}
	
	pub fn host_copy(
		&self,
		src:     VkResource,
		src_off: VkDeviceSize,
		dst:     VkResource,
		dst_off: VkDeviceSize,
		len:     VkDeviceSize
	) -> Result<(), VkResult> {
		let mut copied = 0;
		
		while copied < len {
			let src = self.map_region(src, src_off + copied)?;
			let mut dst = self.map_region(dst, dst_off + copied)?;
			let len = (len - copied).min(src.len() as _).min(dst.len() as _);
			
			if len == 0 { return Err(VK_ERROR_UNKNOWN); }
			dst[..len as usize].copy_from_slice(&src[..len as usize]);
			copied += len;
		}
		
		Ok(())
	}
	
	pub fn bind_buffer(&self, buffer: VkBuffer, allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { bind_buffers: &[buffer], ..HeapsBindInfo::default() }, allocator)
	}
	
	pub fn bind_image(&self, image: VkImage, allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { bind_images: &[image], ..HeapsBindInfo::default() }, allocator)
	}
	
	pub fn bind_buffers(&self, buffers: &[VkBuffer], allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { bind_buffers: buffers, ..HeapsBindInfo::default() }, allocator)
	}
	
	pub fn bind_images(&self, images: &[VkImage], allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { bind_images: images, ..HeapsBindInfo::default() }, allocator)
	}
	
	pub fn unbind_buffer(&self, buffer: VkBuffer, allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { unbind_buffers: &[buffer], ..HeapsBindInfo::default() }, allocator)
	}
	
	pub fn unbind_image(&self, image: VkImage, allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { unbind_images: &[image], ..HeapsBindInfo::default() }, allocator)
	}
	
	pub fn unbind_buffers(&self, buffers: &[VkBuffer], allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { unbind_buffers: buffers, ..HeapsBindInfo::default() }, allocator)
	}
	
	pub fn unbind_images(&self, images: &[VkImage], allocator: Option<&VkAllocationCallbacks>) -> VkResult {
		self.bind(None, &HeapsBindInfo { unbind_images: images, ..HeapsBindInfo::default() }, allocator)
	}
}

impl Default for Heaps {
	fn default() -> Self {
		Self {
			device: ManuallyDrop::new(VkDeviceImpl { handle: VK_NULL_HANDLE, table: unsafe { Arc::from_raw(null()) } }),
			compat: HeapsCompatibilityLayer::default(),
			config: HeapsConfig::default(),
			props:  VkPhysicalDeviceMemoryProperties::default(),
			sync:   RwLock::default()
		}
	}
}

pub mod sub_alloc {
	use super::*;
	use std::{iter::FromIterator, mem::size_of};
	use std::ops::Deref;
	
	#[derive(Clone, Debug, Default)]
	pub struct DummyAlloc {
		offset: VkDeviceSize,
		ranges: BTreeMap<VkDeviceSize, VkDeviceSize>
	}
	
	impl DummyAlloc {
		pub fn new(offset: VkDeviceSize, size: usize) -> Self {
			Self { offset, ranges: BTreeMap::from_iter(Some((0, size as _))) }
		}
		
		pub fn bytes_free(&self) -> VkDeviceSize {
			self.ranges.values().sum()
		}
		
		/// Resizes the underlying memory block
		pub fn resize(&mut self, _val: usize) {
			unimplemented!()
		}
		
		pub fn alloc(&mut self, size: VkDeviceSize, align: VkDeviceSize) -> Result<(VkDeviceSize, usize), ()> {
			for (off, len) in self.ranges.iter_mut() {
				let len_mut = len;
				let (off, len) = (*off, *len_mut);
				let aligned_off = crate::utils::align(off, align);
				let aligned_len = len as isize - (aligned_off - off) as isize;
				
				if aligned_len < size as isize {
					continue;
				}
				
				if off != aligned_off {
					*len_mut = aligned_off - off;
				} else {
					self.ranges.remove(&off);
				}
				
				if off + len != aligned_off + size {
					self.ranges.insert(aligned_off + size, off + len - (aligned_off + size));
				}
				
				return Ok((self.offset + aligned_off, size as _))
			}
			
			Err(())
		}
		
		pub fn free(&mut self, offset: VkDeviceSize, size: usize) {
			let mut size = size as VkDeviceSize;
			
			if let Some((prev_off, prev_len)) = self.ranges.range_mut(..offset).next_back() {
				if *prev_off + *prev_len == offset {
					*prev_len += size;
					
					let __tmp_prev_off__ = *prev_off;
					if let Some(next_len) = self.ranges.remove(&(offset + size)) {
						//*prev_len += next_len;
						*self.ranges.get_mut(&__tmp_prev_off__).unwrap() += next_len;
					}
					
					return;
				}
			}
			
			if let Some(next_len) = self.ranges.remove(&(offset + size)) {
				size += next_len;
			}
			
			self.ranges.insert(offset, size);
		}
	}
	
	impl std::fmt::Display for DummyAlloc {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			for (off, len) in &self.ranges {
				writeln!(f, "{:08X}-{:08X} {:08X}", *off, *off + *len, *len)?;
			}
			
			Ok(())
		}
	}
	
	pub trait RemoteAllocRef: std::fmt::Debug {
		/// Attempts to allocate a new memory block.
		fn alloc(&self, size: usize, align: usize, capacity: usize) -> Result<(VkDeviceSize, usize), ()>;
		
		/// Frees a memory block, panicking if the offset is invalid.
		fn dealloc(&self, offset: VkDeviceSize, size: usize);
		
		/// Attempts to grow the memory block in place, returning Err(()) on failure, or the new size otherwise.
		fn grow(&self, offset: VkDeviceSize, size: usize, new_size: usize) -> Result<usize, ()>;
		
		/// Attempts to shrink the memory block in place, returning Err(()) on failure, or the new size otherwise.
		fn shrink(&self, offset: VkDeviceSize, size: usize, new_size: usize) -> Result<usize, ()>;
	}
	
	impl<T: Deref + std::fmt::Debug> RemoteAllocRef for T where T::Target: RemoteAllocRef {
		fn alloc(&self, size: usize, align: usize, capacity: usize) -> Result<(u64, usize), ()> {
			self.deref().alloc(size, align, capacity)
		}
		
		fn dealloc(&self, offset: u64, size: usize) {
			self.deref().dealloc(offset, size)
		}
		
		fn grow(&self, offset: u64, size: usize, new_size: usize) -> Result<usize, ()> {
			self.deref().grow(offset, size, new_size)
		}
		
		fn shrink(&self, offset: u64, size: usize, new_size: usize) -> Result<usize, ()> {
			self.deref().shrink(offset, size, new_size)
		}
	}
	
	pub trait RemoteAllocRefScaled: RemoteAllocRef {
		/// Attempts to allocate a new memory block.
		fn alloc_scaled<T>(&self, size: usize, align: usize, capacity: usize) -> Result<(VkDeviceSize, usize), ()> {
			self.alloc(size * size_of::<T>(), align, capacity * size_of::<T>())
				.map(|(off, size)| (off, size / size_of::<T>()))
		}
		
		/// Frees a memory block, panicking if the offset is invalid.
		fn dealloc_scaled<T>(&self, offset: VkDeviceSize, size: usize) {
			self.dealloc(offset, size * size_of::<T>())
		}
		
		/// Attempts to grow the memory block in place, returning Err(()) on failure, or the new size otherwise.
		fn grow_scaled<T>(&self, offset: VkDeviceSize, size: usize, new_size: usize) -> Result<usize, ()> {
			self.grow(offset, size * size_of::<T>(), new_size * size_of::<T>())
				.map(|size| size / size_of::<T>())
		}
		
		/// Attempts to shrink the memory block in place, returning Err(()) on failure, or the new size otherwise.
		fn shrink_scaled<T>(&self, offset: VkDeviceSize, size: usize, new_size: usize) -> Result<usize, ()> {
			self.shrink(offset, size * size_of::<T>(), new_size * size_of::<T>())
				.map(|size| size / size_of::<T>())
		}
	}
	
	impl<T: RemoteAllocRef> RemoteAllocRefScaled for T {}
	
	#[derive(Debug)]
	pub struct GpaSubAlloc {
		gpa:        Weak<Heaps>,
		pub buffer: VkBuffer,
		alloc:      Mutex<DummyAlloc>
	}
	
	impl GpaSubAlloc {
		pub fn new(alloc: &Arc<Heaps>, buffer: VkBuffer) -> Self {
			let size = alloc.sync.read().expect("failed to acquire lock")
				.resources[&buffer].requirements.size;
			Self { gpa: Arc::downgrade(alloc), buffer, alloc: Mutex::new(DummyAlloc::new(0, size as _)) }
		}
	}
	
	impl RemoteAllocRef for GpaSubAlloc {
		fn alloc(&self, size: usize, align: usize, _capacity: usize) -> Result<(VkDeviceSize, usize), ()> {
			self.alloc.lock().unwrap().alloc(size as _, align as _)
		}
		
		fn dealloc(&self, offset: VkDeviceSize, size: usize) {
			self.alloc.lock().unwrap().free(offset, size)
		}
		
		fn grow(&self, _offset: VkDeviceSize, _size: usize, _new_size: usize) -> Result<usize, ()> {
			Err(())
		}
		
		fn shrink(&self, _offset: VkDeviceSize, _size: usize, _new_size: usize) -> Result<usize, ()> {
			Err(())
		}
	}
	
	#[derive(Debug)]
	pub struct GpaSparseSubAllocInner {
		alloc:         Arc<Heaps>,
		pub buffer:    VkBuffer,
		//                                        offset,        length,       capacity
		regions:       vec_map::VecMap<VkDeviceSize, (VkDeviceSize, VkDeviceSize)>,
		size:          VkDeviceSize,
		align:         VkDeviceSize,
		bind_buffer:   Vec<(VkDeviceSize, VkDeviceSize)>,
		unbind_buffer: Vec<(VkDeviceSize, VkDeviceSize)>,
		flush_wait:    Vec<std::task::Waker>,
		compat_mode:   bool
	}
	
	impl GpaSparseSubAllocInner {
		fn new(alloc: Arc<Heaps>, buffer: VkBuffer, compat_mode: bool) -> Self {
			let requirements = alloc.sync.read().expect("failed to acquire lock").resources[&buffer].requirements;
			Self {
				alloc,
				buffer,
				regions:       vec_map::VecMap::new(),
				size:          requirements.size,
				align:         requirements.alignment,
				bind_buffer:   Vec::new(),
				unbind_buffer: Vec::new(),
				flush_wait:    Vec::new(),
				compat_mode
			}
		}
		
		fn alloc(&mut self, _size: usize, _align: usize, _capacity: usize) -> Result<(VkDeviceSize, usize), ()> {
			unimplemented!()
			/*debug_assert!(size > 0 && capacity > 0);
			let off = self.regions.inner().last().map_or(0, |(off, (_, len))| off + len);
			
			if off + capacity as VkDeviceSize > self.size {
				log::error!("allocation capacity can not be satisfied (requested capacity: {}, available capacity: {})",
							capacity, self.size - off);
				//return Err(());
			}
			
			self.regions.insert(off, (0, super::align(capacity as VkDeviceSize, self.align)));
			Ok((off, self.grow(off, 0, size).unwrap()))*/
		}
		
		fn free(&mut self, _offset: VkDeviceSize, _size: usize) {
			unimplemented!()
			//self.shrink(offset, size, 0).unwrap();
			//self.regions.remove(&offset).unwrap();
		}
		
		fn grow(&mut self, _offset: VkDeviceSize, _size: usize, _new_size: usize) -> Result<usize, ()> {
			unimplemented!()
			/*debug_assert!(size > 0);
			let (len, cap) = self.regions.get_mut(&offset).unwrap();
			
			if self.compat_mode && *len != 0 {
				return Err(());
			}
			
			let new_len = if size == VK_WHOLE_SIZE as usize {
				*cap
			} else {
				align(size as VkDeviceSize, self.align)
			};
			
			if *len + new_len > *cap {
				return Err(());
			}
			
			self.bind_buffer.push((offset + *len, new_len));
			*len += new_len;
			Ok(new_len as _)*/
		}
		
		fn shrink(&mut self, _offset: VkDeviceSize, _size: usize, _new_size: usize) -> Result<usize, ()> {
			unimplemented!()
			/*debug_assert!(size > 0);
			let (len, _) = self.regions.get_mut(&offset).unwrap();
			let new_len = if size as VkDeviceSize == VK_WHOLE_SIZE {
				*len
			} else {
				align(size as VkDeviceSize, self.align)
			};
			
			if new_len > *len {
				return Err(())
			}
			
			self.unbind_buffer.push((offset + *len - new_len, new_len));
			*len += new_len;
			Ok(new_len as _)*/
		}
	}
	
	#[derive(Debug)]
	pub struct GpaSparseSubAlloc(Mutex<GpaSparseSubAllocInner>);
	
	impl GpaSparseSubAlloc {
		pub fn new(alloc: Arc<Heaps>, buffer: VkBuffer, compat_mode: bool) -> Self {
			Self(Mutex::new(GpaSparseSubAllocInner::new(alloc, buffer, compat_mode)))
		}
	}
	
	impl RemoteAllocRef for GpaSparseSubAlloc {
		fn alloc(&self, size: usize, align: usize, capacity: usize) -> Result<(VkDeviceSize, usize), ()> {
			self.0.lock().map_err(|_| ())?.alloc(size, align, capacity)
		}
		
		fn dealloc(&self, offset: VkDeviceSize, size: usize) {
			if let Ok(mut alloc) = self.0.lock() {
				alloc.free(offset, size);
			}
		}
		
		fn grow(&self, offset: VkDeviceSize, size: usize, new_size: usize) -> Result<usize, ()> {
			self.0.lock().map_err(|_| ())?.grow(offset, size, new_size)
		}
		
		fn shrink(&self, offset: VkDeviceSize, size: usize, new_size: usize) -> Result<usize, ()> {
			self.0.lock().map_err(|_| ())?.shrink(offset, size, new_size)
		}
	}
}

pub async fn write_buffer(
	_src:        &mut impl std::io::Read,
	_src_buffer: VkBuffer,
	_src_alloc:  &MappedSubAlloc,
	_dst_buffer: VkBuffer,
	_dst_off:    VkDeviceSize,
	_before:     impl FnOnce(&VkCommandBufferImpl),
	_after:      impl FnOnce(&VkCommandBufferImpl),
	_cmds:       &Mutex<super::cmd::Cmds>
) -> Result<(), VkResult> {
	unimplemented!()
}

pub async fn write_image(
	_src:        &mut impl std::io::Read,
	_src_buffer: VkBuffer,
	_src_alloc:  &MappedSubAlloc,
	_dst_image:  VkImage,
	_dst_copy:   VkBufferImageCopy,
	_before:     impl FnOnce(&VkCommandBufferImpl),
	_after:      impl FnOnce(&VkCommandBufferImpl),
	_cmds:       &Mutex<super::cmd::Cmds>
) -> Result<(), VkResult> {
	unimplemented!()
}

/// Access primitives to mapped host-visible memory.
mod access {
	use super::*;
	use std::{alloc::*, ptr::NonNull};
	
	pub struct MemoryRegion<'a> {
		pub(super) alloc:  &'a Heaps,
		pub(super) memory: usize,
		pub(super) data:   &'a mut [u8]
	}
	
	impl<'a> MemoryRegion<'a> {
		pub fn into_ptr(self) -> *mut [u8] {
			let ptr = self.data as *mut [u8];
			std::mem::forget(self);
			ptr
		}
	}
	
	impl std::fmt::Debug for MemoryRegion<'_> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("MappedMemoryRegion")
				.field("memory", &self.memory)
				.field("data", &"[...]")
				.finish()
		}
	}
	
	impl Drop for MemoryRegion<'_> {
		fn drop(&mut self) {
			self.alloc.sync.read().unwrap()
				.memories[self.memory >> 32]
				.blocks[self.memory & 0xFFFF_FFFF]
				.refs.fetch_sub(1, Ordering::SeqCst);
		}
	}
	
	impl std::ops::Deref for MemoryRegion<'_> {
		type Target = [u8];
		
		fn deref(&self) -> &Self::Target {
			&*self.data
		}
	}
	
	impl std::ops::DerefMut for MemoryRegion<'_> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			self.data
		}
	}
	
	/// Struct for reading/writing to host-visible memory
	#[derive(Debug, Clone)]
	pub struct MemoryAccessor<'a> {
		alloc:  &'a Heaps,
		handle: VkResource,
		size:   usize,
		pos:    usize
	}
	
	impl<'a> MemoryAccessor<'a> {
		pub fn new(alloc: &'a Heaps, handle: VkResource, size: usize) -> Self {
			Self { alloc, handle, size, pos: 0 }
		}
		
		pub fn read(&mut self, offset: usize, buf: &mut [u8]) -> io::Result<usize> {
			self.read_to(offset, buf)
		}
		
		pub fn write(&mut self, offset: usize, buf: &[u8]) -> io::Result<usize> {
			self.write_to(offset, buf)
		}
		
		pub fn read_exact(&mut self, mut offset: usize, mut buf: &mut [u8]) -> std::io::Result<()> {
			while !buf.is_empty() {
				match self.read(offset, buf)? {
					0 => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer")),
					n => {
						offset -= n;
						buf = &mut buf[n..];
					}
				}
			}
			
			Ok(())
		}
		
		pub fn write_all(&mut self, mut offset: usize, mut buf: &[u8]) -> std::io::Result<()> {
			while !buf.is_empty() {
				match self.write(offset, buf)? {
					0 => return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write whole buffer")),
					n => {
						offset -= n;
						buf = &buf[n..];
					}
				}
			}
			
			Ok(())
		}
		
		pub fn read_to(&mut self, offset: usize, mut dst: impl io::Write) -> io::Result<usize> {
			dst.write(&*self.alloc
				.map_region(self.handle, offset as _)
				.map_err(Self::vk_result_io_err)?)
		}
		
		pub fn write_to(&mut self, offset: usize, mut src: impl io::Read) -> io::Result<usize> {
			src.read(&mut*self.alloc
				.map_region(self.handle, offset as _)
				.map_err(Self::vk_result_io_err)?)
		}
		
		pub fn read_exact_to(&mut self, mut offset: usize, mut dst: impl io::Write) -> std::io::Result<()> {
			loop {
				let range = self.alloc
					.map_region(self.handle, offset as _)
					.map_err(Self::vk_result_io_err)?;
				
				if range.is_empty() {
					return Ok(());
				}
				
				dst.write_all(&*range)?;
				offset += range.len();
			}
		}
		
		pub fn write_all_to(&mut self, mut offset: usize, mut src: impl io::Read) -> std::io::Result<()> {
			loop {
				let mut range = self.alloc
					.map_region(self.handle, offset as _)
					.map_err(Self::vk_result_io_err)?;
				
				if range.is_empty() {
					return Ok(());
				}
				
				match src.read_exact(&mut*range) {
					Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
					Err(e) => return Err(e),
					_ => ()
				}
				
				offset += range.len();
			}
		}
		
		fn vk_result_io_err(e: VkResult) -> std::io::Error {
			io::Error::new(io::ErrorKind::Other, e)
		}
	}
	
	impl io::Write for MemoryAccessor<'_> {
		fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
			Self::write(self, self.pos, buf).map(|len| { self.pos += len; len })
		}
		
		fn flush(&mut self) -> io::Result<()> {
			Ok(()) // no flushing required, as we write directly to host memory
		}
	}
	
	impl io::Read for MemoryAccessor<'_> {
		fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
			Self::read(self, self.pos, &mut buf).map(|len| { self.pos += len; len })
		}
	}
	
	impl io::Seek for MemoryAccessor<'_> {
		fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
			generic_seek(pos, &mut self.pos, self.size)
		}
		
		fn stream_len(&mut self) -> io::Result<u64> {
			Ok(self.size as _)
		}
		
		fn stream_position(&mut self) -> io::Result<u64> {
			Ok(self.pos as _)
		}
	}
	
	#[derive(Debug, Clone)]
	pub struct MappedSubAlloc(Arc<MappedSubAllocInner>);
	
	#[derive(Debug)]
	pub struct MappedSubAllocInner {
		alloc: Box<dyn RemoteAllocRef + Send + Sync>,
		data:  AtomicPtr<u8>
	}
	
	impl MappedSubAlloc {
		pub fn new(alloc: impl RemoteAllocRef + Send + Sync + 'static, data: MemoryRegion) -> Self {
			let self_ = Self(Arc::new(MappedSubAllocInner { alloc: Box::new(alloc), data: AtomicPtr::new(data.data.as_mut_ptr()) }));
			std::mem::forget(data);
			self_
		}
		
		pub fn get_offset<T>(&self, ptr: *const T) -> VkDeviceSize {
			(ptr as *const u8 as usize - self.0.data.load(Ordering::Relaxed) as usize) as _
		}
		
		pub fn get_index<T>(&self, ptr: *const T) -> u32 {
			debug_assert!((ptr as usize - self.0.data.load(Ordering::Relaxed) as usize) % std::mem::size_of::<T>() == 0, "offset was not aligned properly");
			((ptr as usize - self.0.data.load(Ordering::Relaxed) as usize) / std::mem::size_of::<T>()) as _
		}
	}
	
	impl std::ops::Deref for MappedSubAlloc {
		type Target = dyn RemoteAllocRef;
		
		fn deref(&self) -> &Self::Target {
			&*self.0.alloc
		}
	}
	
	unsafe impl std::alloc::Allocator for MappedSubAlloc {
		fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
			match self.0.alloc.alloc(layout.size(), layout.align(), layout.align()) {
				Ok((offset, size)) => Ok(unsafe { NonNull::new_unchecked(std::slice::from_raw_parts_mut(
					self.0.data.load(Ordering::SeqCst).wrapping_add(offset as _), size)) }),
				Err(()) => Err(AllocError)
			}
		}
		
		unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
			self.0.alloc.dealloc(ptr.as_ptr().offset_from(self.0.data.load(Ordering::SeqCst)) as _, layout.size());
		}
	}
}