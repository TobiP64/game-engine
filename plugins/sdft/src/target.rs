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

#[derive(Debug)]
pub struct TargetContext {
	desc_sets_draw_out_image: Vec<VkDescriptorSet>
}

impl TargetContext {
	pub(crate) fn create() -> Result<Self, VkResult> {
		Ok(Self {
			desc_sets_draw_out_image: Vec::new()
		})
	}
	
	pub(crate) fn update(
		&mut self,
		device_ctx: &DeviceContext,
		device:     &gpgpu::DeviceRootContext<Arc<World>>,
		target:     &gpgpu::TargetRootContext<Arc<World>>
	) -> Result<(), VkResult> {
		if !self.desc_sets_draw_out_image.is_empty() {
			device.device.freeDescriptorSets(device.desc_pool, &self.desc_sets_draw_out_image);
		}
		
		self.desc_sets_draw_out_image.resize(target.images.len(), VK_NULL_HANDLE);
		let set_layouts = vec![device_ctx.desc_set_layout_draw_out_image; target.images.len()];
		
		device.device.allocateDescriptorSets(&VkDescriptorSetAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			pNext:              None,
			descriptorPool:     device.desc_pool,
			descriptorSetCount: set_layouts.len() as _,
			pSetLayouts:        set_layouts.as_ptr()
		}, &mut self.desc_sets_draw_out_image)?;
		
		let mut image_info_buf = Vec::new();
		device.device.updateDescriptorSets(&self.desc_sets_draw_out_image.iter().enumerate().map(|(i, desc_set)| VkWriteDescriptorSet {
			sType:           VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
			dstSet:          *desc_set,
			descriptorCount: 1,
			descriptorType:  VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
			pImageInfo:      {
				let info = VkDescriptorImageInfo {
					sampler:     VK_NULL_HANDLE,
					imageView:   target.images[i].image_view,
					imageLayout: VK_IMAGE_LAYOUT_GENERAL
				};
				image_info_buf.push(info);
				image_info_buf.last().unwrap()
			},
			..VkWriteDescriptorSet::default()
		}).collect::<Vec<_>>(), &[]);
		
		Ok(())
	}
	
	pub(crate) fn destroy(&mut self, device: &gpgpu::DeviceRootContext<Arc<World>>) -> Result<(), VkResult> {
		device.device.freeDescriptorSets(device.desc_pool, &self.desc_sets_draw_out_image);
		Ok(())
	}
}