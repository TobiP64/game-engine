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
	super::{*, scene::*},
	std::{io, ptr},
	engine_core::*,
	math::*,
	ecs::*,
	::scene::*,
	gpgpu::{mem::*, misc::*},
	vec_map::VecMap,
};

#[derive(Debug, Clone)]
pub struct Loader {
	registry: Registry,
	world:    Arc<World>,
	device:   Arc<gpgpu::DeviceRootContext<World>>,
	context:  Arc<DeviceContext>
}

impl Loader {
	async fn load_mesh(
		&self,
		world:  &World,
		entity: Entity,
		source: &dyn Source<(MeshDescriptor, Box<dyn MeshReader>)>
	) -> io::Result<()> {
		use {::scene::mesh::MeshAttribute::*, std::io::Read};
		
		world.add_component(entity, ResourceState::<MeshData>::Unloaded);
		let (descriptor, reader) = source.open(true, false).await?;
		
		let pipeline = match descriptor.attributes.as_slice() {
			[Pos(VK_FORMAT_R32G32_SFLOAT)]                                                                     => UiPipeline::Colored,
			[Pos(VK_FORMAT_R32G32_SFLOAT), Color(VK_FORMAT_R32G32B32A32_SFLOAT)]                               => UiPipeline::ColoredGradient,
			[Pos(VK_FORMAT_R32G32_SFLOAT), Tex(VK_FORMAT_R32G32_SFLOAT)]                                       => UiPipeline::ColoredTextured,
			[Pos(VK_FORMAT_R32G32_SFLOAT), Color(VK_FORMAT_R32G32B32A32_SFLOAT), Tex(VK_FORMAT_R32G32_SFLOAT)] => UiPipeline::ColoredTexturedGradient,
			_ => return Err(io::Error::new(io::ErrorKind::InvalidData, "mesh attributes not supported"))
		};
		
		let vertex_stride = pipeline.vertex_size();
		let len = descriptor.vertex_count * vertex_stride;
		let (transfer_offset, transfer_len) = self.device.transfer_buffer_alloc
			.alloc(len, 1, len)
			.map_err(|_| io::Error::new(io::ErrorKind::Other, "transfer alloc failed"))?;
		let (local_offset, _) = self.device.local_buffer_alloc
			.alloc(len, vertex_stride, len)
			.map_err(|_| io::Error::new(io::ErrorKind::Other, "local alloc failed"))?;
		
		self.device.alloc.access(self.device.transfer_buffer)
			.write_all_to(transfer_offset as _, reader.take(len as _))?;
		
		let transfer_buffer = self.device.transfer_buffer;
		let local_buffer    = self.device.local_buffer;
		self.device.transfer_cmds.lock().unwrap().record(move |cmd_buffer| {
			cmd_buffer.cmdCopyBuffer(
				transfer_buffer,
				local_buffer,
				&[
					VkBufferCopy {
						srcOffset: transfer_offset,
						dstOffset: local_offset,
						size:      len as _
					}
				]
			);
		});
		
		self.device.transfer_ready.notify_all();
		self.device.transfer_wait.wait_async().await;
		self.device.transfer_buffer_alloc.dealloc(transfer_offset, transfer_len);
		
		world.add_component(entity, ResourceState::Loaded(MeshData {
			offset: local_offset as _,
			count:  descriptor.vertex_count as _,
			pipeline
		}));
		
		log::trace!("[RESOURCES] uploaded mesh #{:04x} to VkBuffer#{:X} @ {:#X}-{:#X} ({} vertices)",
					entity, self.device.local_buffer, local_offset, local_offset + len as VkDeviceSize, descriptor.vertex_count);
		Ok(())
	}
	
	async fn unload_mesh(&self, world: &World, entity: Entity) -> io::Result<()> {
		match world.remove_component::<ResourceState<MeshData>>(entity) {
			Some(ResourceState::Loaded(data)) => {
				self.device.local_buffer_alloc.dealloc(data.offset, data.len());
				log::trace!("[RESOURCES] unloaded mesh #{:04x}", entity);
			},
			Some(ResourceState::Unloaded) => log::warn!(
				"[RESOURCES] attempted to unload mesh #{:04x}, which was never loaded", entity),
			None                        => log::warn!(
				"[RESOURCES] attempted to unload mesh #{:04x}, which does not exist", entity)
		}
		
		Ok(())
	}
	
	async fn load_texture(
		&self,
		world:  &World,
		entity: Entity,
		source: &dyn Source<(TextureDescriptor, Box<dyn TextureReader>)>
	) -> io::Result<()> {
		world.add_component(entity, ResourceState::<TextureData>::Unloaded);
		
		let (descriptor, mut reader) = source.open(true, false).await?;
		
		let mut image = VK_NULL_HANDLE;
		self.device.device.createImage(&VkImageCreateInfo {
			sType:                 VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
			pNext:                 None,
			flags:                 0,
			imageType:             VK_IMAGE_TYPE_2D,
			format:                descriptor.format,
			extent:                VkExtent3D {
				width:  descriptor.extent.0 as _,
				height: descriptor.extent.1 as _,
				depth:  1
			},
			mipLevels:             descriptor.levels as _,
			arrayLayers:           descriptor.layers as _,
			samples:               VK_SAMPLE_COUNT_1_BIT,
			tiling:                VK_IMAGE_TILING_OPTIMAL,
			usage:                 VK_IMAGE_USAGE_SAMPLED_BIT as u32 | VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32,
			sharingMode:           VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices:   ptr::null(),
			initialLayout:         VK_IMAGE_LAYOUT_PREINITIALIZED
		}, self.device.allocator, &mut image);
		
		let mut view = VK_NULL_HANDLE;
		self.device.device.createImageView(&VkImageViewCreateInfo {
			sType:            VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			pNext:            None,
			flags:            0,
			image,
			viewType:         VK_IMAGE_VIEW_TYPE_2D,
			format:           descriptor.format,
			components:       VkComponentMapping::default(),
			subresourceRange: VkImageSubresourceRange {
				aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
				baseMipLevel:   0,
				levelCount:     descriptor.levels as _,
				baseArrayLayer: 0,
				layerCount:     descriptor.layers as _
			}
		}, self.device.allocator, &mut view);
		
		let mut desc_sets = [VK_NULL_HANDLE; 1];
		self.device.device.allocateDescriptorSets(&VkDescriptorSetAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			pNext:              None,
			descriptorPool:     self.device.desc_pool,
			descriptorSetCount: 1,
			pSetLayouts:        &self.context.desc_set_layout_ui_textured
		}, &mut desc_sets);
		
		self.device.alloc.bind(None, &HeapsBindInfo {
			bind_images: &[image],
			..Default::default()
		}, self.device.allocator);
		
		// TODO move loop to write_image
		for i in 0..descriptor.levels {
			write_image(
				&mut reader,
				self.device.transfer_buffer,
				&self.device.transfer_buffer_alloc,
				image,
				VkBufferImageCopy {
					bufferOffset:      0,
					bufferRowLength:   descriptor.extent.0 as _,
					bufferImageHeight: descriptor.extent.1 as _,
					imageSubresource:  VkImageSubresourceLayers {
						aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
						mipLevel:       i as _,
						baseArrayLayer: 0,
						layerCount:     descriptor.layers as _
					},
					imageExtent:        VkExtent3D {
						width:  descriptor.extent.0 as _,
						height: descriptor.extent.1 as _,
						depth:  1
					},
					..VkBufferImageCopy::default()
				},
				|_| {},
				|_| {},
				&self.device.transfer_cmds
			).await.map_err(|_| io::Error::new(io::ErrorKind::Other, "transfer failed"))?;
		}
		
		world.add_component(entity, ResourceState::Loaded(TextureData {
			image,
			view,
			desc_set: desc_sets[0]
		}));
		
		log::trace!("[RESOURCES] uploaded texture #{:04x} as VkImage#{:X} with VkImageView#{:X} (format: {:?})",
					entity, image, view, descriptor.format);
		Ok(())
	}
	
	async fn unload_texture(&self, world: &World, entity: Entity) -> io::Result<()> {
		match world.remove_component::<ResourceState<TextureData>>(entity) {
			Some(ResourceState::Loaded(data)) => {
				self.device.alloc.unbind_image(data.image, self.device.allocator);
				self.device.device.freeDescriptorSets(self.device.desc_pool, &[data.desc_set]);
				self.device.device.destroyImageView(data.view, self.device.allocator);
				self.device.device.destroyImage(data.image, self.device.allocator);
				log::trace!("[RESOURCES] unloaded texture #{:04x}", entity);
			},
			Some(ResourceState::Unloaded) => log::warn!(
				"[RESOURCES] attempted to unload texture #{:04x}, which was never loaded", entity),
			None                        => log::warn!(
				"[RESOURCES] attempted to unload texture #{:04x}, which does not exist", entity)
		}
		
		Ok(())
	}
	
	async fn load_font(
		&self,
		world:  &World,
		entity: Entity,
		source: &dyn Source<(FontDescriptor, Box<dyn FontReader>)>
	) -> io::Result<()> {
		const DEFAULT_LINE_HEIGHT: usize = 128;
		const DEFAULT_PADDING:     usize = 8;
		const DEFAULT_GLYPH_COUNT: usize = (b'~' - b' ') as usize;
		
		world.add_component(entity, ResourceState::<FontData>::Unloaded);
		
		let (desc, mut reader) = source.open(true, false).await?;
		
		// PROCESS DATA ----------------------------------------------------------------------------
		
		let scale = 1f32 / (desc.ascender - desc.descender) as f32;
		let texel_scale = scale * DEFAULT_LINE_HEIGHT as f32;
		let _padding = DEFAULT_PADDING as f32 / DEFAULT_LINE_HEIGHT as f32;
		
		let mut i = 0;
		let mut width = 0u32;
		let mut glyphs_host = VecMap::with_capacity(DEFAULT_GLYPH_COUNT + 1);
		let mut glyphs_local = Vec::<[f32; 8], _>::with_capacity_in(
			DEFAULT_GLYPH_COUNT,
			AllocatorWithLayout::new(&self.device.transfer_buffer_alloc).with_min_align(8)
		);
		let mut glyphs_gen_host = Vec::with_capacity(DEFAULT_GLYPH_COUNT);
		let mut glyphs_gen_local = Vec::<LocalSdfGenVertex, _>::with_capacity_in(
			DEFAULT_GLYPH_COUNT,
			AllocatorWithLayout::new(&self.device.transfer_buffer_alloc).with_min_align(8)
		);
		
		for ch in ' '..='~' {
			let glyph: Glyph = match reader.get_glyph(ch) {
				Some(v) => v,
				None    => continue
			};
			
			let extent    = (glyph.metrics.bbox_max - glyph.metrics.bbox_min).map(|v| v as f32);
			let extent_px = (extent * texel_scale).map(|v| v as u32);
			
			glyphs_host.insert(ch as char, HostGlyphData {
				idx:          if extent.0 == 0f32 || extent.1 == 0f32 { -1 } else { i },
				advance:      Vec2(glyph.metrics.advance.0 as f32 * scale, glyph.metrics.advance.1 as f32 * scale),
				kern_advance: (' '..='~')
					.map(|ch1| (ch1, reader.get_kerning(ch, ch1).unwrap_or(0) as f32 * scale))
					.collect(),
			});
			
			if extent.0 == 0f32 || extent.1 == 0f32 {
				continue;
			}
			
			i += 1;
			
			// mirror the uv coords on both axis
			glyphs_local.push([
				glyph.metrics.bbox_min.0 as f32 * scale,// - padding,
				glyph.metrics.bbox_min.1 as f32 * -scale,// - padding,
				glyph.metrics.bbox_max.0 as f32 * scale,// + padding,
				glyph.metrics.bbox_max.1 as f32 * -scale,// + padding,
				(width + extent_px.0/* + DEFAULT_PADDING*/) as f32,
				extent.1 * scale,// + padding,
				width as f32,
				0f32
			]);
			
			glyphs_gen_host.push((
				glyphs_gen_local.len() * size_of::<LocalSdfGenVertex>(),
				LocalSdfGenData {
					offset: Vec2(width, 0),
					extent: extent_px,// + Vec2::from(DEFAULT_PADDING * 2),
					scale:  1f32,
					count:  glyph.path.len() as _
				}
			));
			
			let bbox_min = Vec2(glyph.metrics.bbox_min.0 as f32, glyph.metrics.bbox_min.1 as f32);
			let scale = Vec2::from(1f32) / extent;
			let pad = 1f32;//Vec2::from(1f32) - Vec2::from(DEFAULT_PADDING as f32) / (extent * texel_scale);
			
			glyphs_gen_local.extend(glyph.path.iter().map(|&e| match e {
				PathElement::Move(p0) => LocalSdfGenVertex {
					r#type: 0,
					p0:     (p0 - bbox_min) * scale * pad,
					..LocalSdfGenVertex::default()
				},
				PathElement::Curve1(p0) => LocalSdfGenVertex {
					r#type: 1,
					p0:     (p0 - bbox_min) * scale * pad,
					..LocalSdfGenVertex::default()
				},
				PathElement::Curve2(p0, p1) => LocalSdfGenVertex {
					r#type: 2,
					p0:     (p1 - bbox_min) * scale * pad,
					p1:     (p0 - bbox_min) * scale * pad,
					..LocalSdfGenVertex::default()
				},
				PathElement::Curve3(p0, p1, p2) => LocalSdfGenVertex {
					r#type: 3,
					p0:     (p2 - bbox_min) * scale * pad,
					p1:     (p1 - bbox_min) * scale * pad,
					p2:     (p0 - bbox_min) * scale * pad,
					..LocalSdfGenVertex::default()
				}
			}));
			
			width += extent_px.0;// + DEFAULT_PADDING * 2;
		}
		
		// normalize x coord
		glyphs_local.iter_mut().for_each(|glyph| {
			glyph[4] /= width as f32;
			glyph[6] /= width as f32;
		});
		
		std::mem::drop(reader);
		
		let glyphs_local_transfer_offset = self.device.transfer_buffer_alloc
			.get_offset(glyphs_local.as_ptr());
		let glyphs_local_transfer_len = glyphs_local.len() * size_of::<[f32; 8]>();
		let glyphs_gen_local_offset = self.device.transfer_buffer_alloc
			.get_offset(glyphs_gen_local.as_ptr());
		let glyphs_gen_local_len = glyphs_gen_local.len() * size_of::<LocalSdfGenVertex>();
		let (glyphs_local_offset, glyphs_local_len) = self.device.local_buffer_alloc
			.alloc(glyphs_local_transfer_len, self.device.device_info
				.vk10_properties.limits.minStorageBufferOffsetAlignment as _, glyphs_local_transfer_len)
			.map_err(|_| io::Error::new(io::ErrorKind::Other, "local alloc failed"))?;
		
		// CREATE RESOURCE -------------------------------------------------------------------------
		
		let mut sdf_image = VK_NULL_HANDLE;
		self.device.device.createImage(&VkImageCreateInfo {
			sType:                 VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
			pNext:                 None,
			flags:                 0,
			imageType:             VK_IMAGE_TYPE_2D,
			format:                VK_FORMAT_R16_SNORM,
			extent:                VkExtent3D {
				width,
				height: DEFAULT_LINE_HEIGHT as _,
				depth:  1
			},
			mipLevels:             1,
			arrayLayers:           1,
			samples:               VK_SAMPLE_COUNT_1_BIT,
			tiling:                VK_IMAGE_TILING_OPTIMAL,
			usage:                 VK_IMAGE_USAGE_SAMPLED_BIT as u32
				| VK_IMAGE_USAGE_STORAGE_BIT as u32
				| VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32,
			sharingMode:           VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices:   ptr::null(),
			initialLayout:         VK_IMAGE_LAYOUT_UNDEFINED
		}, self.device.allocator, &mut sdf_image);
		
		self.device.alloc.add(choose_memory_types(
			&self.device.device_info.vk10_memory_properties,
			!0,
			&MEMORY_PROPERTY_FLAGS_LOCAL_PREFERRED
		), &[], &[sdf_image]);
		self.device.alloc.bind(None, &HeapsBindInfo::bind_images(&[sdf_image]), self.device.allocator);
		
		let mut sdf_view = VK_NULL_HANDLE;
		self.device.device.createImageView(&VkImageViewCreateInfo {
			sType:            VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			pNext:            None,
			flags:            0,
			image:            sdf_image,
			viewType:         VK_IMAGE_VIEW_TYPE_2D,
			format:           VK_FORMAT_R16_SNORM,
			components:       VkComponentMapping::default(),
			subresourceRange: VkImageSubresourceRange {
				aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
				baseMipLevel:   0,
				levelCount:     1,
				baseArrayLayer: 0,
				layerCount:     1
			}
		}, self.device.allocator, &mut sdf_view);
		
		// DESC SETS -------------------------------------------------------------------------------
		
		let layouts = [self.context.desc_set_layout_ui_glyphs, self.context.desc_set_layout_ui_sdf_gen];
		let mut desc_sets = [VK_NULL_HANDLE; 2];
		self.device.device.allocateDescriptorSets(&VkDescriptorSetAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			pNext:              None,
			descriptorPool:     self.device.desc_pool,
			descriptorSetCount: 2,
			pSetLayouts:        layouts.as_ptr()
		}, &mut desc_sets);
		
		self.device.device.updateDescriptorSets(&[
			VkWriteDescriptorSet {
				sType:            VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:           desc_sets[0],
				dstBinding:       0,
				dstArrayElement:  0,
				descriptorCount:  1,
				descriptorType:   VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
				pImageInfo:       &VkDescriptorImageInfo {
					sampler:     VK_NULL_HANDLE,
					imageView:   sdf_view,
					imageLayout: VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
				},
				..VkWriteDescriptorSet::default()
			},
			VkWriteDescriptorSet {
				sType:            VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:           desc_sets[0],
				dstBinding:       1,
				dstArrayElement:  0,
				descriptorCount:  1,
				descriptorType:   VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
				pBufferInfo:      &VkDescriptorBufferInfo {
					buffer: self.device.local_buffer,
					offset: glyphs_local_offset,
					range:  glyphs_local_len as _
				},
				..VkWriteDescriptorSet::default()
			},
			VkWriteDescriptorSet {
				sType:            VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:           desc_sets[1],
				dstBinding:       0,
				dstArrayElement:  0,
				descriptorCount:  1,
				descriptorType:   VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC,
				pBufferInfo:      &VkDescriptorBufferInfo {
					buffer: self.device.transfer_buffer,
					offset: glyphs_gen_local_offset,
					range:  glyphs_gen_local_len as _
				},
				..VkWriteDescriptorSet::default()
			},
			VkWriteDescriptorSet {
				sType:            VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
				dstSet:           desc_sets[1],
				dstBinding:       1,
				dstArrayElement:  0,
				descriptorCount:  1,
				descriptorType:   VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
				pImageInfo:       &VkDescriptorImageInfo {
					sampler:     VK_NULL_HANDLE,
					imageView:   sdf_view,
					imageLayout: VK_IMAGE_LAYOUT_GENERAL
				},
				..VkWriteDescriptorSet::default()
			}
		], &[]);
		
		// COMMAND BUFFER --------------------------------------------------------------------------
		
		let transfer_buffer = self.device.transfer_buffer;
		let local_buffer    = self.device.local_buffer;
		let pipeline        = self.context.pipeline_ui_sdf_gen;
		let pipeline_layout = self.context.pipeline_layout_ui_sdf_gen;
		let len             = glyphs_gen_local.len() * size_of::<LocalSdfGenVertex>();
		self.device.transfer_cmds.lock().unwrap().record(move |cmd_buffer| {
			cmd_buffer.cmdCopyBuffer(
				transfer_buffer,
				local_buffer,
				&[
					VkBufferCopy {
						srcOffset: glyphs_local_transfer_offset,
						dstOffset: glyphs_local_offset,
						size:      glyphs_local_transfer_len as _
					}
				]
			);
			
			let mut barrier = [
				VkImageMemoryBarrier {
					sType:               VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
					pNext:               None,
					srcAccessMask:       0,
					dstAccessMask:       VK_ACCESS_TRANSFER_WRITE_BIT as _,
					oldLayout:           VK_IMAGE_LAYOUT_UNDEFINED,
					newLayout:           VK_IMAGE_LAYOUT_GENERAL,
					srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
					dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
					image:               sdf_image,
					subresourceRange:    VkImageSubresourceRange {
						aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
						baseMipLevel:   0,
						levelCount:     1,
						baseArrayLayer: 0,
						layerCount:     1
					}
				}
			];
			
			cmd_buffer.cmdPipelineBarrier(
				VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT as _,
				VK_PIPELINE_STAGE_TRANSFER_BIT as _,
				0,
				&[],
				&[],
				&barrier
			);
			
			cmd_buffer.cmdClearColorImage(
				sdf_image,
				VK_IMAGE_LAYOUT_GENERAL,
				&VkClearColorValue { float32: [1.0, 1.0, 1.0, 1.0] },
				&[
					VkImageSubresourceRange {
						aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as _,
						baseMipLevel:   0,
						levelCount:     1,
						baseArrayLayer: 0,
						layerCount:     1
					}
				]
			);
			
			barrier[0].srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT as _;
			barrier[0].dstAccessMask = VK_ACCESS_SHADER_WRITE_BIT as _;
			barrier[0].oldLayout = VK_IMAGE_LAYOUT_GENERAL;
			
			cmd_buffer.cmdPipelineBarrier(
				VK_PIPELINE_STAGE_TRANSFER_BIT as u32 | VK_PIPELINE_STAGE_HOST_BIT as u32,
				VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as _,
				0,
				&[],
				&[
					VkBufferMemoryBarrier {
						sType:                VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
						pNext:                None,
						srcAccessMask:        VK_ACCESS_HOST_WRITE_BIT as _,
						dstAccessMask:        VK_ACCESS_SHADER_READ_BIT as _,
						srcQueueFamilyIndex:  VK_QUEUE_FAMILY_IGNORED,
						dstQueueFamilyIndex:  VK_QUEUE_FAMILY_IGNORED,
						buffer:               transfer_buffer,
						offset:               glyphs_gen_local_offset,
						size:                 len as _,
					}
				],
				&barrier
			);
			
			cmd_buffer.cmdBindPipeline(VK_PIPELINE_BIND_POINT_COMPUTE, pipeline);
			
			for (offset, data) in &glyphs_gen_host {
				cmd_buffer.cmdPushConstants(
					pipeline_layout,
					VK_SHADER_STAGE_COMPUTE_BIT as _,
					0,
					&data.to_bytes()
				);
				
				cmd_buffer.cmdBindDescriptorSets(
					VK_PIPELINE_BIND_POINT_COMPUTE,
					pipeline_layout,
					0,
					&[desc_sets[1]],
					&[*offset as _]
				);
				
				cmd_buffer.cmdDispatch(data.extent.0, data.extent.1, 1);
			}
			
			barrier[0].srcAccessMask = VK_ACCESS_SHADER_WRITE_BIT as _;
			barrier[0].dstAccessMask = VK_ACCESS_SHADER_READ_BIT as _;
			barrier[0].newLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
			cmd_buffer.cmdPipelineBarrier(
				VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as _,
				VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as _,
				0,
				&[],
				&[],
				&barrier
			);
		});
		
		// FINISH ---------------------------------------------------------------------------------
		
		self.device.transfer_ready.notify_all();
		self.device.transfer_wait.wait_async().await;
		self.device.device.freeDescriptorSets(self.device.desc_pool, &desc_sets[1..]);
		
		world.add_component(entity, ResourceState::Loaded(FontData {
			sdf_image,
			sdf_view,
			desc_set:            desc_sets[0],
			glyphs_host,
			glyphs_local_offset,
			glyphs_local_len,
			ascender:            desc.ascender as f32 * scale,
			descender:           desc.descender as f32 * scale,
			line_gap:            desc.line_gap as f32 * scale
		}));
		
		log::trace!("[RESOURCES] uploaded font #{:04x} to VkBuffer#{:X} @ {:#X}-{:#X}, VkImage#{:X} with VkImageView#{:X}",
					entity, self.device.local_buffer, glyphs_local_offset,
					glyphs_local_offset + glyphs_local_len as VkDeviceSize,
					sdf_image, sdf_view);
		Ok(())
	}
	
	async fn unload_font(&self, world: &World, entity: Entity) -> io::Result<()> {
		match world.remove_component::<ResourceState<FontData>>(entity) {
			Some(ResourceState::Loaded(data)) => {
				self.device.local_buffer_alloc.dealloc(data.glyphs_local_offset, data.glyphs_local_len);
				self.device.device.freeDescriptorSets(self.device.desc_pool, &[data.desc_set]);
				self.device.device.destroyImageView(data.sdf_view, self.device.allocator);
				self.device.device.destroyImage(data.sdf_image, self.device.allocator);
				self.device.alloc.bind(None, &HeapsBindInfo::unbind_images(&[data.sdf_image]), self.device.allocator);
				log::trace!("[RESOURCES] unloaded font #{:04x}", entity);
			},
			Some(ResourceState::Unloaded) => log::warn!(
				"[RESOURCES] attempted to unload font #{:04x}, which was never loaded", entity),
			None                          => log::warn!(
				"[RESOURCES] attempted to unload font #{:04x}, which does not exist", entity)
		}
		
		Ok(())
	}
}

#[async_trait::async_trait]
impl engine_core::Loader<(MeshDescriptor, Box<dyn MeshReader>)> for Loader {
	async fn load(&self, entity: Entity, source: &dyn Source<(MeshDescriptor, Box<dyn MeshReader>)>) -> io::Result<()> {
		self.load_mesh(&self.world, entity, source).await
	}
	
	async fn destroy(&self, entity: usize) -> io::Result<()> {
		self.unload_mesh(&self.world, entity).await
	}
}

#[async_trait::async_trait]
impl engine_core::Loader<(TextureDescriptor, Box<dyn TextureReader>)> for Loader {
	async fn load(&self, entity: Entity, source: &dyn Source<(TextureDescriptor, Box<dyn TextureReader>)>) -> io::Result<()> {
		self.load_texture(&self.world, entity, source).await
	}
	
	async fn destroy(&self, entity: usize) -> io::Result<()> {
		self.unload_texture(&self.world, entity).await
	}
}

#[async_trait::async_trait]
impl engine_core::Loader<(FontDescriptor, Box<dyn FontReader>)> for Loader {
	async fn load(&self, entity: Entity, source: &dyn Source<(FontDescriptor, Box<dyn FontReader>)>) -> io::Result<()> {
		self.load_font(&self.world, entity, source).await
	}
	
	async fn destroy(&self, entity: usize) -> io::Result<()> {
		self.unload_font(&self.world, entity).await
	}
}