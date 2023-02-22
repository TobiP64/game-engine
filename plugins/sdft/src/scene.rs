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
	engine_core::{trace_oom, trace_memory},
	std::{
		ptr,
		io,
		mem,
		sync::{*, atomic::*},
		collections::VecDeque
	},
	gpgpu::{mem::*, misc::*, plugins::UpdateResult},
	ecs::*,
	::scene::*,
	custom_sync::mpbc
};

static SCENE_SDF_FORMATS: [VkFormat; 2] = [
	VK_FORMAT_R8_SNORM,
	VK_FORMAT_R16_SNORM
];

static SCENE_IDS_FORMATS: [VkFormat; 3] = [
	VK_FORMAT_R8_UINT,
	VK_FORMAT_R16_UINT,
	VK_FORMAT_R32_UINT
];

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct EntityInstanceId(InstanceId);

#[derive(Clone, Debug, Default)]
struct Pending(Vec<Entity>);

#[derive(Copy, Clone, Debug)]
pub struct SceneOptions {
	pub sdf_extent:        Vec3<usize>,
	pub ids_extent:        Vec3<usize>,
	pub sdf_format:        VkFormat,
	pub ids_format:        VkFormat,
	pub init_instances:    usize,
	pub init_sdfs:         usize,
	pub init_materials:    usize,
	pub min_instances:     usize,
	pub min_sdfs:          usize,
	pub min_materials:     usize,
	pub max_instances:     usize,
	pub max_sdfs:          usize,
	pub max_materials:     usize,
	pub local_sdf_gen:     bool,
	pub local_sdf_upd:     bool,
	pub samples:           u32,
	pub bounces:           u32,
	pub min_step_size:     f32,
	pub min_dist:          f32,
	pub min_dist_global:   f32,
	pub max_dist:          f32,
	pub min_steps:         u32,
	pub max_steps:         u32,
	pub normal_polls:      u32,
	pub normal_smoothness: f32,
}

impl SceneOptions {
	fn validate(self) -> Self {
		debug_assert!(self.sdf_extent.0 > 0 && self.sdf_extent.1 > 0 && self.sdf_extent.2 > 0);
		debug_assert!(self.ids_extent.0 > 0 && self.ids_extent.1 > 0 && self.ids_extent.2 > 0);
		debug_assert!(SCENE_SDF_FORMATS.contains(&self.sdf_format));
		debug_assert!(SCENE_IDS_FORMATS.contains(&self.ids_format));
		debug_assert!(self.max_instances > 0 && self.max_instances <= std::u16::MAX as usize);
		debug_assert!(self.max_sdfs      > 0 && self.max_sdfs      <= std::u16::MAX as usize);
		debug_assert!(self.max_materials > 0 && self.max_materials <= std::u16::MAX as usize);
		debug_assert!(self.min_instances <= self.max_instances);
		debug_assert!(self.min_sdfs      <= self.max_sdfs);
		debug_assert!(self.min_materials <= self.max_materials);
		debug_assert!(self.init_instances >= self.min_instances && self.init_instances <= self.max_instances);
		debug_assert!(self.init_sdfs      >= self.min_sdfs      && self.init_sdfs      <= self.max_sdfs);
		debug_assert!(self.init_materials >= self.min_materials && self.init_materials <= self.max_materials);
		debug_assert!(self.init_instances > 0);
		debug_assert!(self.init_sdfs      > 0);
		debug_assert!(self.init_materials > 0);
		self
	}
}

impl Default for SceneOptions {
	fn default() -> Self {
		Self {
			sdf_extent:        Vec3::from(0),
			ids_extent:        Vec3::from(512),
			sdf_format:        VK_FORMAT_R16_SNORM,
			ids_format:        VK_FORMAT_R16_UINT,
			init_instances:    64,
			init_sdfs:         64,
			init_materials:    64,
			min_instances:     0,
			min_sdfs:          0,
			min_materials:     0,
			max_instances:     std::u16::MAX as _,
			max_sdfs:          std::u16::MAX as _,
			max_materials:     std::u16::MAX as _,
			local_sdf_gen:     false,
			local_sdf_upd:     false,
			samples:           1,
			bounces:           1,
			min_step_size:     1e-6,
			min_dist:          1e-6,
			min_dist_global:   0.0,
			max_dist:          1.0,
			min_steps:         0,
			max_steps:         0,
			normal_polls:      1,
			normal_smoothness: 1.0
		}
	}
}

#[derive(Clone, Debug, Default)]
struct HostCopies {
	sdf: Vec<VkBufferImageCopy>,
	ids: Vec<VkBufferImageCopy>
}

#[derive(Debug, Default)]
pub struct SceneContext {
	options:                  SceneOptions,
	desc_set_draw_general:    VkDescriptorSet,
	desc_set_draw_sdfs:       VkDescriptorSet,
	desc_set_draw_materials:  VkDescriptorSet,
	desc_set_upd:             VkDescriptorSet,
	/// separate descriptor pool for all sdf and material textures
	desc_pool:                VkDescriptorPool,
	desc_pool_size:           usize,
	/// This command buffer is used for local sdf generation/updates.
	/// `None` if local generation and updates are disabled.
	cmd_buffer:               Option<VkCommandBufferImpl>,
	scene_sdf_memory:         VkDeviceMemory,
	scene_sdf_image:          VkImage,
	scene_sdf_view:           VkImageView,
	/// if the scene sdf is host visible and host update is enabled, this points to the image
	scene_sdf_data:           AtomicPtr<u8>,
	scene_ids_image:          VkImage,
	scene_ids_view:           VkImageView,
	/// if the scene ids is host visible and host update is enabled, this points to the image
	scene_ids_data:           AtomicPtr<u8>,
	scene_updates:            Mutex<VecDeque<(Entity, Option<Instance>)>>,
	camera_data_off:          VkDeviceSize,
	instances_unused:         VecDeque<InstanceId>,
	instance_events_sdf:      mpbc::Receiver<Event>,
	instance_events_mat:      mpbc::Receiver<Event>,
	instance_events_trans:    mpbc::Receiver<Event>,
	instance_data_off:        VkDeviceSize,
	instance_data_cnt:        usize,
	instance_data_cap:        usize,
	lights_data_off:          VkDeviceSize,
	lights_data_cnt:          usize,
	lights_data_cap:          usize,
	material_updates:         mpbc::Receiver<Event>,
	material_unused:          VecDeque<ResourceId>,
	material_data_off:        VkDeviceSize,
	material_data_cnt:        usize,
	material_data_cap:        usize,
	sdf_updates:              mpbc::Receiver<Event>,
	sdf_unused:               VecDeque<ResourceId>,
	sdf_cnt:                  usize,
	sdf_cap:                  usize,
	host_copies:              Arc<Mutex<HostCopies>>,
}

impl SceneContext {
	pub(crate) fn create(ctx: &DeviceContext, context: &gpgpu::DeviceRootContext<Arc<World>>, scene: &Arc<World>) -> Result<Self, VkResult> {
		let options = scene.query::<&SceneOptions>().iter().next().unwrap();
		
		let mut self_ = Self {
			options:           *options,
			desc_pool_size:    options.init_sdfs + options.init_materials,
			instance_data_cap: options.init_instances,
			material_data_cap: options.init_materials,
			sdf_cap:           options.init_sdfs,
			..Self::default()
		};
		
		// allocate memory, create resources
		
		(self_.instance_data_off, self_.instance_data_cap) = context.dynamic_buffer_alloc
			.alloc_scaled::<Instance>(self_.instance_data_cap, mem::align_of::<Instance>(), self_.options.max_instances)
			.expect("failed to alloc instance memory");
		
		(self_.material_data_off, self_.material_data_cap) = context.local_buffer_alloc
			.alloc_scaled::<MaterialFactors>(self_.material_data_cap, mem::align_of::<MaterialFactors>(), self_.options.max_materials)
			.expect("failed to alloc material memory");
		
		(self_.camera_data_off, _) = context.dynamic_buffer_alloc
			.alloc_scaled::<RtCamera>(1, mem::align_of::<RtCamera>(), 1)
			.expect("failed to alloc camera memory");
		
		context.device.createImage(&VkImageCreateInfo {
			format: options.sdf_format,
			extent: VkExtent3D {
				width:  options.sdf_extent.0 as _,
				height: options.sdf_extent.1 as _,
				depth:  options.sdf_extent.2 as _
			},
			..SDF_IMAGE
		}, context.allocator, &mut self_.scene_sdf_image)?;
		
		context.device.createImage(&VkImageCreateInfo {
			format: options.ids_format,
			extent: VkExtent3D {
				width:  options.ids_extent.0 as _,
				height: options.ids_extent.1 as _,
				depth:  options.ids_extent.2 as _
			},
			..SDF_IMAGE
		}, context.allocator, &mut self_.scene_ids_image)?;
		
		context.alloc.alloc_dedicated(
			&context.device_info.vk10_memory_properties,
			&MEMORY_PROPERTY_FLAGS_LOCAL,
			context.allocator,
			&[],
			&[self_.scene_sdf_image, self_.scene_ids_image]
		)?;
		
		context.device.createImageView(&VkImageViewCreateInfo {
			sType:            VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			image:            self_.scene_sdf_image,
			viewType:         VK_IMAGE_VIEW_TYPE_3D,
			format:           VK_FORMAT_R16_SNORM,
			subresourceRange: VkImageSubresourceRange {
				aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
				baseMipLevel:   0,
				levelCount:     1,
				baseArrayLayer: 0,
				layerCount:     1
			},
			..VkImageViewCreateInfo::default()
		}, context.allocator, &mut self_.scene_sdf_view)?;
		
		context.device.createImageView(&VkImageViewCreateInfo {
			sType:            VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			image:            self_.scene_ids_image,
			viewType:         VK_IMAGE_VIEW_TYPE_3D,
			format:           VK_FORMAT_R16_UINT,
			subresourceRange: VkImageSubresourceRange {
				aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
				baseMipLevel:   0,
				levelCount:     1,
				baseArrayLayer: 0,
				layerCount:     1
			},
			..VkImageViewCreateInfo::default()
		}, context.allocator, &mut self_.scene_ids_view)?;
		
		// allocate desc sets
		
		context.device.createDescriptorPool(&VkDescriptorPoolCreateInfo {
			sType:         VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
			pNext:         None,
			flags:         VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT as _,
			maxSets:       2,
			poolSizeCount: 1,
			pPoolSizes:    &VkDescriptorPoolSize {
				r#type:          VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
				descriptorCount: self_.desc_pool_size as _
			}
		}, context.allocator, &mut self_.desc_pool);
		
		let mut desc_sets = [VK_NULL_HANDLE; 4];
		context.device.allocateDescriptorSets(&VkDescriptorSetAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			pNext:              None,
			descriptorPool:     context.desc_pool,
			descriptorSetCount: 2,
			pSetLayouts:        [ctx.desc_set_layout_draw_general, ctx.desc_set_layout_upd].as_ptr()
		}, &mut desc_sets[..2])?;
		
		context.device.allocateDescriptorSets(&VkDescriptorSetAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			pNext:              Some((&VkDescriptorSetVariableDescriptorCountAllocateInfo {
				sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
				pNext:              None,
				descriptorSetCount: 2,
				pDescriptorCounts:  [self_.sdf_cap as _, self_.material_data_cap as _].as_ptr()
			}).into()),
			descriptorPool:     self_.desc_pool,
			descriptorSetCount: 2,
			pSetLayouts:        [ctx.desc_set_layout_draw_textures, ctx.desc_set_layout_draw_textures].as_ptr()
		}, &mut desc_sets[2..])?;
		
		[
			self_.desc_set_draw_general,
			self_.desc_set_upd,
			self_.desc_set_draw_sdfs,
			self_.desc_set_draw_materials
		] = desc_sets;
		
		// set object names
		
		set_object_names(context.instance_info.vk12_debug_utils, &context.device, &[
			(VK_OBJECT_TYPE_DESCRIPTOR_SET, self_.desc_set_draw_general,   "desc_set_draw_general\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET, self_.desc_set_draw_sdfs,      "desc_set_draw_sdfs\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET, self_.desc_set_draw_materials, "desc_set_draw_materials\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET, self_.desc_set_upd,            "desc_set_upd\0"),
			(VK_OBJECT_TYPE_IMAGE,          self_.scene_sdf_image,         "scene_sdf_image\0"),
			(VK_OBJECT_TYPE_IMAGE_VIEW,     self_.scene_sdf_view,          "scene_sdf_view\0"),
			(VK_OBJECT_TYPE_IMAGE,          self_.scene_ids_image,         "scene_ids_image\0"),
			(VK_OBJECT_TYPE_IMAGE_VIEW,     self_.scene_ids_view,          "scene_ids_view\0")
		])?;
		
		// update desc sets
		
		context.device.updateDescriptorSets(&[
			VkWriteDescriptorSet { // layout (set = 0, binding = 0) uniform Camera
				sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:          self_.desc_set_draw_general,
				dstBinding:      0,
				descriptorCount: 1,
				descriptorType:  VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
				pBufferInfo:     &VkDescriptorBufferInfo {
					buffer: context.dynamic_buffer,
					offset: self_.camera_data_off,
					range:  mem::size_of::<RtCamera>() as _
				},
				..VkWriteDescriptorSet::default()
			},
			VkWriteDescriptorSet { // layout (set = 0, binding = 1) uniform sampler3D scene_sdf;
				sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:          self_.desc_set_draw_general,
				dstBinding:      1,
				descriptorCount: 1,
				descriptorType:  VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
				pImageInfo:      &VkDescriptorImageInfo {
					sampler:     VK_NULL_HANDLE,
					imageView:   self_.scene_sdf_view,
					imageLayout: VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
				},
				..VkWriteDescriptorSet::default()
			},
			VkWriteDescriptorSet { // layout (set = 0, binding = 2) uniform sampler3D scene_ids;
				sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:          self_.desc_set_draw_general,
				dstBinding:      2,
				descriptorCount: 1,
				descriptorType:  VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
				pImageInfo:      &VkDescriptorImageInfo {
					sampler:     VK_NULL_HANDLE,
					imageView:   self_.scene_ids_view,
					imageLayout: VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
				},
				..VkWriteDescriptorSet::default()
			},
			VkWriteDescriptorSet { // layout (set = 0, binding = 3) readonly buffer Instances { uint instance_count; Instance instances[ ]; };
				sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:          self_.desc_set_draw_general,
				dstBinding:      3,
				descriptorCount: 1,
				descriptorType:  VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
				pBufferInfo:     &VkDescriptorBufferInfo {
					buffer: context.dynamic_buffer,
					offset: self_.instance_data_off,
					range:  VK_WHOLE_SIZE // bind whole size and use sparse bind to extend memory
				},
				..VkWriteDescriptorSet::default()
			},
			VkWriteDescriptorSet { // layout (set = 0, binding = 4) readonly buffer Materials { Material materials[ ]; };
				sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:          self_.desc_set_draw_general,
				dstBinding:      4,
				descriptorCount: 1,
				descriptorType:  VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
				pBufferInfo:     &VkDescriptorBufferInfo {
					buffer: context.local_buffer,
					offset: self_.material_data_off,
					range:  VK_WHOLE_SIZE // bind whole size and use sparse bind to extend memory
				},
				..VkWriteDescriptorSet::default()
			},
			// TODO update lights descriptor
		], &[]);
		
		Ok(self_)
	}
	
	pub(crate) fn destroy(&mut self, context: &gpgpu::DeviceRootContext<Arc<World>>) -> Result<(), VkResult> {
		context.device.destroyDescriptorPool(self.desc_pool, context.allocator);
		context.device.destroyImageView(self.scene_sdf_view, context.allocator);
		context.device.destroyImageView(self.scene_ids_view, context.allocator);
		context.alloc.unbind_images(&[self.scene_sdf_image, self.scene_ids_image], context.allocator);
		context.device.destroyImage(self.scene_sdf_image, context.allocator);
		context.device.destroyImage(self.scene_ids_image, context.allocator);
		context.device.freeMemory(self.scene_sdf_memory, context.allocator);
		Ok(())
	}
	
	pub(crate) fn update(
		&mut self,
		device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		device_ctx: &DeviceContext,
		scene:      &Arc<World>
	) -> Result<UpdateResult, VkResult> {
		let (result, added, updated, removed, t) =
			(UpdateResult::Reuse, 0, 0, 0, std::time::Instant::now());
		
		self.update_scene_sdf_flush(device, device_ctx);
		
		for update in std::iter::empty::<()>() {
			let offset = Vec3(0, 0, 0);
			let extent = Vec3(1, 1, 1);
			
			self.update_scene_sdf(device, device_ctx, offset, extent, std::iter::empty());
		}
		
		Ok(UpdateResult::Reuse)
	}
	
	fn realloc_desc_sets(
		&mut self,
		device:    &gpgpu::DeviceRootContext<Arc<World>>,
		dev_ctx:   &DeviceContext,
		sdfs:      impl IntoIterator<Item = (ResourceId, VkImageView)>,
		materials: impl IntoIterator<Item = (ResourceId, VkImageView)>
	) -> Result<(), VkResult> {
		device.device.resetDescriptorPool(self.desc_pool, 0)?;
		
		let mut desc_sets = [VK_NULL_HANDLE; 2];
		device.device.allocateDescriptorSets(&VkDescriptorSetAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			pNext:              Some((&VkDescriptorSetVariableDescriptorCountAllocateInfo {
				sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
				pNext:              None,
				descriptorSetCount: 2,
				pDescriptorCounts:  [self.sdf_cap as u32, self.material_data_cap as u32].as_ptr()
			}).into()),
			descriptorPool:     device.desc_pool,
			descriptorSetCount: 2,
			pSetLayouts:        [dev_ctx.desc_set_layout_draw_textures; 2].as_ptr()
		}, &mut desc_sets)?;
		
		[self.desc_set_draw_sdfs, self.desc_set_draw_materials] = desc_sets;
		
		let sdfs = sdfs.into_iter();
		let materials = materials.into_iter();
		let mut infos = Vec::with_capacity(sdfs.size_hint().0);
		let mut infos2 = Vec::with_capacity(materials.size_hint().0);
		
		device.device.updateDescriptorSets(&sdfs
			.map(|(id, view)| VkWriteDescriptorSet {
				sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:          self.desc_set_draw_sdfs,
				dstArrayElement: id as _,
				descriptorCount: 1,
				descriptorType:  VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
				pImageInfo:      {
					let info = VkDescriptorImageInfo {
						sampler:     VK_NULL_HANDLE,
						imageView:   view,
						imageLayout: VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
					};
					infos.push(info);
					infos.last().unwrap()
				},
				..VkWriteDescriptorSet::default()
			})
			.chain(materials.map(|(id, view)| VkWriteDescriptorSet {
				sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:          self.desc_set_draw_materials,
				dstArrayElement: id as _,
				descriptorCount: 1,
				descriptorType:  VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
				pImageInfo:      {
					let info = VkDescriptorImageInfo {
						sampler:     VK_NULL_HANDLE,
						imageView:   view,
						imageLayout: VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
					};
					infos2.push(info);
					infos2.last().unwrap()
				},
				..VkWriteDescriptorSet::default()
			}))
			.collect::<Vec<_>>(),
										   &[]
		);
		
		Ok(())
	}
	
	fn realloc_instance_data(
		&mut self,
		device:  &gpgpu::DeviceRootContext<Arc<World>>,
		dev_ctx: &DeviceContext,
		result:  &mut UpdateResult,
		new_len: usize
	) -> Result<(), VkResult> {
		let old_len = self.instance_data_cnt;
		
		let result2 = if new_len > self.instance_data_cnt {
			if new_len > self.options.max_instances && dev_ctx.options.fail_on_oom {
				return Err(VK_ERROR_OUT_OF_HOST_MEMORY);
			}
			
			device.dynamic_buffer_alloc.grow_scaled::<Instance>(
				self.instance_data_off, self.instance_data_cnt, new_len)
		} else if new_len + 128 <= self.instance_data_cnt
			&& self.instance_data_cnt >= self.options.min_instances + 128 {
			device.dynamic_buffer_alloc.shrink_scaled::<Instance>(
				self.instance_data_off, self.instance_data_cnt, new_len)
		} else {
			return Ok(());
		};
		
		if let Ok(size) = result2 {
			self.instance_data_cnt = size;
		} else {
			log::trace!("[memory] instance data: grow/shrink failed, reallocating...");
			
			match device.dynamic_buffer_alloc.alloc_scaled::<Instance>(
				new_len, mem::size_of::<Instance>(), self.options.max_instances)
			{
				Err(()) if dev_ctx.options.fail_on_oom => return Err(VK_ERROR_OUT_OF_HOST_MEMORY),
				Err(()) => trace_oom("VK_API", "instance data"),
				Ok((offset, size)) => {
					device.alloc.host_copy(
						device.dynamic_buffer,
						self.instance_data_off,
						device.dynamic_buffer,
						offset,
						(new_len.min(self.instance_data_cnt) * mem::size_of::<Instance>()) as _
					)?;
					
					device.dynamic_buffer_alloc.dealloc_scaled::<Instance>(
						self.instance_data_off, self.instance_data_cnt);
					self.instance_data_off = offset;
					self.instance_data_cnt = size;
					
					// offset changed, descriptor update required
					
					*result = UpdateResult::Redraw;
					device.device.updateDescriptorSets(&[
						VkWriteDescriptorSet {
							sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
							dstSet:          self.desc_set_draw_general,
							dstBinding:      3,
							descriptorCount: 1,
							descriptorType:  VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
							pBufferInfo:     &VkDescriptorBufferInfo {
								buffer: device.dynamic_buffer,
								offset,
								range:  VK_WHOLE_SIZE // bind whole size and use sparse bind to extend memory
							},
							..VkWriteDescriptorSet::default()
						}
					], &[]);
				}
			}
		}
		
		trace_memory("VK_API", "instance data",
					 ((new_len - old_len) * mem::size_of::<Instance>()) as _,
					 self.instance_data_cnt * mem::size_of::<Instance>(),
					 self.instance_data_cap * mem::size_of::<Instance>());
		Ok(())
	}
	
	async fn load_sdf(
		&mut self,
		entity: Entity,
		source: &mut Texture,
		device: &gpgpu::DeviceRootContext<Arc<World>>,
		ctx:    &mut DeviceContext
	) -> Result<SdfData, Error> {
		let (descriptor, mut reader) = source.open(true, false).await?;
		
		if descriptor.cube || descriptor.extent.0 == 0 || descriptor.extent.1 == 0 || descriptor.extent.2 == 0 {
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				format!("entity:texture #{:04X} is not a 3D texture, aborting loading", entity)
			).into());
		}
		
		if descriptor.levels != 1 { log::warn!("entity:texture#{:04X} has more than 1 level, discarding other levels", entity) }
		if descriptor.layers != 1 { log::warn!("entity:texture#{:04X} has more than 1 layer, discarding other layers", entity) }
		
		let mut image = VK_NULL_HANDLE;
		device.device.createImage(&VkImageCreateInfo {
			sType:                 VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
			pNext:                 None,
			flags:                 0,
			imageType:             VK_IMAGE_TYPE_3D,
			format:                descriptor.format,
			extent:                VkExtent3D {
				width:  descriptor.extent.0 as _,
				height: descriptor.extent.1 as _,
				depth:  descriptor.extent.2 as _
			},
			mipLevels:             1,
			arrayLayers:           1,
			samples:               VK_SAMPLE_COUNT_1_BIT,
			tiling:                VK_IMAGE_TILING_OPTIMAL,
			usage:                 VK_IMAGE_USAGE_SAMPLED_BIT as u32 | VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32,
			sharingMode:           VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices:   ptr::null(),
			initialLayout:         VK_IMAGE_LAYOUT_PREINITIALIZED
		}, device.allocator, &mut image)?;
		
		let mut view = VK_NULL_HANDLE;
		device.device.createImageView(&VkImageViewCreateInfo {
			sType:            VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			pNext:            None,
			flags:            0,
			image,
			viewType:         VK_IMAGE_VIEW_TYPE_3D,
			format:           descriptor.format,
			components:       VkComponentMapping::default(),
			subresourceRange: VkImageSubresourceRange {
				aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
				baseMipLevel:   0,
				levelCount:     1,
				baseArrayLayer: 0,
				layerCount:     1
			}
		}, device.allocator, &mut view)?;
		
		device.alloc.bind_image(image, device.allocator)?;
		
		let id = self.sdf_unused.pop_front().unwrap_or(self.sdf_cnt as _);
		self.sdf_cnt += 1;
		ctx.desc_updates.push_back(Default::default());
		let (write, info) = ctx.desc_updates.back_mut().unwrap();
		
		*info = VkDescriptorImageInfo {
			sampler:     VK_NULL_HANDLE,
			imageView:   view,
			imageLayout: VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
		};
		
		*write = VkWriteDescriptorSet {
			sType:            VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
			pNext:            None,
			dstSet:           self.desc_set_draw_sdfs,
			dstBinding:       0,
			dstArrayElement:  id as _,
			descriptorCount:  1,
			descriptorType:   VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
			pImageInfo:       info,
			pBufferInfo:      ptr::null(),
			pTexelBufferView: ptr::null()
		};
		
		let len = descriptor.extent.mulc() * gpgpu::misc::get_format_size(descriptor.format);
		
		#[allow(non_snake_case)]
			let subresourceRange = VkImageSubresourceRange {
			aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
			baseMipLevel:   0,
			levelCount:     1,
			baseArrayLayer: 0,
			layerCount:     1
		};
		
		write_image(
			&mut reader,
			device.transfer_buffer,
			&device.transfer_buffer_alloc,
			image,
			VkBufferImageCopy {
				bufferRowLength:   descriptor.extent.0 as _,
				bufferImageHeight: descriptor.extent.1 as _,
				imageSubresource:  VkImageSubresourceLayers {
					aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
					mipLevel:       0,
					baseArrayLayer: 0,
					layerCount:     descriptor.layers as _
				},
				imageExtent:        VkExtent3D {
					width:  descriptor.extent.0 as _,
					height: descriptor.extent.1 as _,
					depth:  descriptor.extent.2 as _
				},
				..VkBufferImageCopy::default()
			},
			|cmd_buffer| cmd_buffer.cmdPipelineBarrier(
				VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT as _,
				VK_PIPELINE_STAGE_TRANSFER_BIT as _,
				0,
				&[],
				&[],
				&[
					VkImageMemoryBarrier {
						sType:               VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
						pNext:               None,
						srcAccessMask:       0,
						dstAccessMask:       VK_ACCESS_TRANSFER_WRITE_BIT as _,
						oldLayout:           VK_IMAGE_LAYOUT_UNDEFINED,
						newLayout:           VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
						srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
						dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
						image,
						subresourceRange
					}
				]
			),
			|cmd_buffer| cmd_buffer.cmdPipelineBarrier(
				VK_PIPELINE_STAGE_TRANSFER_BIT as _,
				VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as _,
				0,
				&[],
				&[],
				&[
					VkImageMemoryBarrier {
						sType:               VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
						pNext:               None,
						srcAccessMask:       VK_ACCESS_TRANSFER_WRITE_BIT as _,
						dstAccessMask:       VK_ACCESS_SHADER_READ_BIT as _,
						oldLayout:           VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
						newLayout:           VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
						srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
						dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
						image,
						subresourceRange
					}
				]
			),
			&device.transfer_cmds
		).await
			.map_err(|_| io::Error::new(io::ErrorKind::Other, "transfer failed"))?;
		
		device.transfer_ready.notify_all();
		device.transfer_wait.wait_async().await;
		log::trace!("uploaded sdf #{:x}", entity);
		Ok(SdfData { image, view, id })
	}
	
	fn destroy_sdf(&mut self, entity: Entity, data: SdfData, device: &gpgpu::DeviceRootContext<Arc<World>>) {
		device.alloc.bind(None, &HeapsBindInfo { unbind_images: &[data.image], ..HeapsBindInfo::default() }, device.allocator);
		device.device.destroyImageView(data.view, device.allocator);
		device.device.destroyImage(data.image, device.allocator);
		self.sdf_unused.push_back(data.id);
		self.sdf_cnt -= 1;
		log::trace!("destroyed sdf #{:x} (resource id: {:?})", entity, data.id)
	}
	
	fn destroy_material(&mut self, entity: Entity, data: MaterialData, device: &gpgpu::DeviceRootContext<Arc<World>>) {
		device.alloc.bind(None, &HeapsBindInfo { unbind_images: &[data.image], ..HeapsBindInfo::default() }, device.allocator);
		device.device.destroyImageView(data.view, device.allocator);
		device.device.destroyImage(data.image, device.allocator);
		self.material_unused.push_back(data.id);
		self.material_data_cnt -= 1;
		log::trace!("destroyed material #{:x} (resource id: {:?})", entity, data.id);
	}
	
	fn update_scene_sdf(
		&mut self,
		device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		device_ctx: &DeviceContext,
		offset:     Vec3<u32>,
		extent:     Vec3<u32>,
		instances:  impl IntoIterator<Item = Entity>
	) {
		if self.options.local_sdf_upd {
			let mut push_consts = [0u8; 12];
			push_consts[..4].copy_from_slice(&extent.0.to_le_bytes());
			push_consts[4..8].copy_from_slice(&extent.1.to_le_bytes());
			push_consts[8..].copy_from_slice(&extent.2.to_le_bytes());
			
			let cmd_buffer = self.cmd_buffer.as_ref().unwrap();
			
			cmd_buffer.cmdPushConstants(
				device_ctx.pipeline_layout_upd,
				VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as _,
				0,
				&push_consts
			);
			
			cmd_buffer.cmdDispatchBase(
				offset.0,
				offset.1,
				offset.2,
				extent.0 / 4,
				extent.1 / 4,
				extent.2 / 4
			);
		} else {
			let options = self.options;
			let host_copies = self.host_copies.clone();
			let device = device.clone();
			
			engine_core::spawn(async move {
				
				// allocate memory
				
				let len = extent.mulc() as usize * (get_format_size(options.sdf_format)
					+ get_format_size(options.ids_format));
				
				let mut sdf = Vec::<u8, _>::new_in(device.transfer_buffer_alloc.clone());
				sdf.resize(len, 0);
				
				// generate sdf
				
				// TODO
				
				// schedule copy
				
				let mut copy = VkBufferImageCopy {
					bufferOffset:      device.transfer_buffer_alloc.get_offset(sdf.as_ptr()),
					bufferRowLength:   extent.0,
					bufferImageHeight: extent.1,
					imageSubresource:  VkImageSubresourceLayers {
						aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
						mipLevel:       0,
						baseArrayLayer: 0,
						layerCount:     1
					},
					imageOffset: VkOffset3D {
						x: offset.0 as _,
						y: offset.1 as _,
						z: offset.2 as _
					},
					imageExtent: VkExtent3D {
						width:  extent.0,
						height: extent.1,
						depth:  extent.2
					}
				};
				
				struct SyncWrapper<T>(T);
				unsafe impl<T> Send for SyncWrapper<T> {}
				unsafe impl<T> Sync for SyncWrapper<T> {}
				
				let mut host_copies = SyncWrapper(host_copies.lock().expect("failed to lock host_copies"));
				host_copies.0.sdf.push(copy);
				copy.bufferOffset += (extent.mulc() as usize * get_format_size(options.sdf_format)) as VkDeviceSize;
				host_copies.0.ids.push(copy);
				std::mem::drop(host_copies);
				
				device.transfer_wait.wait_async().await;
			});
		}
	}
	
	fn update_scene_sdf_flush(
		&mut self,
		device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		device_ctx: &DeviceContext,
	) {
		let scene_sdf_image = self.scene_sdf_image;
		let scene_ids_image = self.scene_ids_image;
		let host_copies = self.host_copies.clone();
		
		// TODO submit directly to queue
		let device2 = device.clone();
		device.transfer_cmds.lock().unwrap().record(move |cmd_buffer| {
			let host_copies = std::mem::take(&mut*host_copies.lock()
				.expect("failed to lock host_copies"));
			
			let mut barrier = VkImageMemoryBarrier {
				sType:               VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
				pNext:               None,
				srcAccessMask:       VK_ACCESS_SHADER_READ_BIT as _,
				dstAccessMask:       VK_ACCESS_TRANSFER_WRITE_BIT as _,
				oldLayout:           VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
				newLayout:           VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
				srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
				dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
				image:               VK_NULL_HANDLE,
				subresourceRange:    VkImageSubresourceRange {
					aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
					baseMipLevel:   0,
					levelCount:     1,
					baseArrayLayer: 0,
					layerCount:     1
				}
			};
			
			cmd_buffer.cmdPipelineBarrier(
				VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as _,
				VK_PIPELINE_STAGE_TRANSFER_BIT as _,
				0,
				&[],
				&[],
				&[
					VkImageMemoryBarrier { image: scene_sdf_image, ..barrier },
					VkImageMemoryBarrier { image: scene_ids_image, ..barrier }
				]
			);
			
			cmd_buffer.cmdCopyBufferToImage(
				device2.transfer_buffer,
				scene_sdf_image,
				VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
				&host_copies.sdf
			);
			
			cmd_buffer.cmdCopyBufferToImage(
				device2.transfer_buffer,
				scene_ids_image,
				VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
				&host_copies.ids
			);
			
			barrier.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT as _;
			barrier.dstAccessMask = VK_ACCESS_SHADER_READ_BIT as _;
			barrier.oldLayout     = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
			barrier.newLayout     = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
			
			cmd_buffer.cmdPipelineBarrier(
				VK_PIPELINE_STAGE_TRANSFER_BIT as _,
				VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as _,
				0,
				&[],
				&[],
				&[
					VkImageMemoryBarrier { image: scene_sdf_image, ..barrier },
					VkImageMemoryBarrier { image: scene_ids_image, ..barrier }
				]
			);
		});
		
		device.transfer_ready.notify_all();
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SdfHandle(pub Handle<Box<Texture>>);

#[derive(Copy, Clone, Debug, Default)]
struct SdfData {
	id:    ResourceId,
	image: VkImage,
	view:  VkImageView
}

#[derive(Copy, Clone, Debug)]
struct SdfDataHostGen {
	image: VkImage,
	data:  *mut u8
}

impl Default for SdfDataHostGen {
	fn default() -> Self {
		Self { image: VK_NULL_HANDLE, data: ptr::null_mut() }
	}
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct MaterialFactors {
	pub albedo:            Vec3<f32>,
	pub metalness:         f32,
	pub emissive:          Vec3<f32>,
	pub roughness:         f32,
	pub normal_smoothness: f32
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
struct MaterialFactors2 {
	albedo:            Vec3<f32>,
	metalness:         f32,
	emissive:          Vec3<f32>,
	roughness:         f32,
	offset:            Vec3<f32>,
	normal_smoothness: f32,
	extent:            Vec3<f32>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MaterialHandle(pub Handle<Box<Texture>>);

#[derive(Copy, Clone, Debug, Default)]
struct MaterialData {
	id:    ResourceId,
	image: VkImage,
	view:  VkImageView
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
struct RtCamera {
	pos:               Vec3<f32>,
	_pad0:             u8,
	lower_left_corner: Vec3<f32>,
	_pad1:             u8,
	horizontal:        Vec3<f32>,
	_pad2:             u8,
	vertical:          Vec3<f32>
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Instance {
	pub transform: Mat4<f32>,
	pub sdf:       u16,
	pub material:  u16
}

impl Instance {
	fn from_bytes(mut reader: impl std::io::Read) -> std::io::Result<Self> {
		let mut buf = [0u8; 68];
		std::io::Read::read_exact(&mut reader, &mut buf)?;
		Ok(Self {
			transform: Mat4(
				Vec4(
					f32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
					f32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
					f32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
					f32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]])
				),
				Vec4(
					f32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]),
					f32::from_le_bytes([buf[20], buf[12], buf[22], buf[23]]),
					f32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]),
					f32::from_le_bytes([buf[28], buf[29], buf[30], buf[31]])
				),
				Vec4(
					f32::from_le_bytes([buf[32], buf[33], buf[34], buf[35]]),
					f32::from_le_bytes([buf[36], buf[37], buf[38], buf[39]]),
					f32::from_le_bytes([buf[40], buf[41], buf[42], buf[43]]),
					f32::from_le_bytes([buf[44], buf[45], buf[46], buf[47]])
				),
				Vec4(
					f32::from_le_bytes([buf[48], buf[49], buf[50], buf[51]]),
					f32::from_le_bytes([buf[52], buf[53], buf[54], buf[55]]),
					f32::from_le_bytes([buf[56], buf[57], buf[58], buf[59]]),
					f32::from_le_bytes([buf[60], buf[61], buf[62], buf[63]])
				)
			),
			sdf:      u16::from_le_bytes([buf[64], buf[65]]),
			material: u16::from_le_bytes([buf[66], buf[67]])
		})
	}
	
	fn to_bytes(self) -> [u8; 68] {
		let mut buf = [0u8; 68];
		buf[0 .. 4].copy_from_slice(&(self.transform.0).0.to_le_bytes());
		buf[4 .. 8].copy_from_slice(&(self.transform.0).1.to_le_bytes());
		buf[8 ..12].copy_from_slice(&(self.transform.0).2.to_le_bytes());
		buf[12..16].copy_from_slice(&(self.transform.0).3.to_le_bytes());
		buf[16..20].copy_from_slice(&(self.transform.1).0.to_le_bytes());
		buf[20..24].copy_from_slice(&(self.transform.1).1.to_le_bytes());
		buf[24..28].copy_from_slice(&(self.transform.1).2.to_le_bytes());
		buf[28..32].copy_from_slice(&(self.transform.1).3.to_le_bytes());
		buf[32..36].copy_from_slice(&(self.transform.2).0.to_le_bytes());
		buf[36..40].copy_from_slice(&(self.transform.2).1.to_le_bytes());
		buf[40..44].copy_from_slice(&(self.transform.2).2.to_le_bytes());
		buf[44..48].copy_from_slice(&(self.transform.2).3.to_le_bytes());
		buf[48..52].copy_from_slice(&(self.transform.3).0.to_le_bytes());
		buf[52..56].copy_from_slice(&(self.transform.3).1.to_le_bytes());
		buf[56..60].copy_from_slice(&(self.transform.3).2.to_le_bytes());
		buf[60..64].copy_from_slice(&(self.transform.3).3.to_le_bytes());
		buf[64..66].copy_from_slice(&self.sdf.to_le_bytes());
		buf[66..68].copy_from_slice(&self.material.to_le_bytes());
		buf
	}
}

static SDF_IMAGE: VkImageCreateInfo = VkImageCreateInfo {
	sType:                 VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
	pNext:                 None,
	flags:                 0,
	imageType:             VK_IMAGE_TYPE_3D,
	format:                VK_FORMAT_R16_SFLOAT,
	extent:                VkExtent3D { width: 0, height: 0, depth: 0 },
	mipLevels:             1,
	arrayLayers:           1,
	samples:               VK_SAMPLE_COUNT_1_BIT,
	tiling:                VK_IMAGE_TILING_OPTIMAL,
	usage:                 VK_IMAGE_USAGE_STORAGE_BIT as u32 | VK_IMAGE_USAGE_SAMPLED_BIT as u32,
	sharingMode:           VK_SHARING_MODE_EXCLUSIVE,
	queueFamilyIndexCount: 0,
	pQueueFamilyIndices:   ptr::null(),
	initialLayout:         VK_IMAGE_LAYOUT_UNDEFINED
};