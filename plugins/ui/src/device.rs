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

#![allow(dead_code)]

use {
	std::{ptr, sync::Arc},
	gpgpu::misc::set_object_names,
	vk::*,
	ecs::World
};

static SHADER_UI_COLORED_VERT:                &[u8] = include_bytes!("../../app/res/shader/ui_colored.vert.spv");
static SHADER_UI_COLORED_GRADIENT_VERT:       &[u8] = include_bytes!("../../app/res/shader/ui_colored_gradient.vert.spv");
static SHADER_UI_COLORED_FRAG:                &[u8] = include_bytes!("../../app/res/shader/ui_colored.frag.spv");
static SHADER_UI_CURVES_VERT:                 &[u8] = include_bytes!("../../app/res/shader/ui_curves.vert.spv");
static SHADER_UI_CURVES_GRADIENT_VERT:        &[u8] = include_bytes!("../../app/res/shader/ui_curves_gradient.vert.spv");
static SHADER_UI_CURVES_FRAG:                 &[u8] = include_bytes!("../../app/res/shader/ui_curves.frag.spv");
static SHADER_UI_TEXTURED_VERT:               &[u8] = include_bytes!("../../app/res/shader/ui_textured.vert.spv");
static SHADER_UI_TEXTURED_GRADIENT_VERT:      &[u8] = include_bytes!("../../app/res/shader/ui_textured_gradient.vert.spv");
static SHADER_UI_TEXTURED_FRAG:               &[u8] = include_bytes!("../../app/res/shader/ui_textured.frag.spv");
static SHADER_UI_DYN_COLORED_VERT:            &[u8] = include_bytes!("../../app/res/shader/ui_dyn_colored.vert.spv");
static SHADER_UI_DYN_COLORED_GRADIENT_VERT:   &[u8] = include_bytes!("../../app/res/shader/ui_dyn_colored_gradient.vert.spv");
static SHADER_UI_DYN_TEXTURED_VERT:           &[u8] = include_bytes!("../../app/res/shader/ui_dyn_textured.vert.spv");
static SHADER_UI_DYN_TEXTURED_GRADIENT_VERT:  &[u8] = include_bytes!("../../app/res/shader/ui_dyn_textured_gradient.vert.spv");
static SHADER_UI_SDF_FRAG:                    &[u8] = include_bytes!("../../app/res/shader/ui_sdf.frag.spv");
static SHADER_UI_GEN_SDF_COMP:                &[u8] = include_bytes!("../../app/res/shader/ui_sdf_gen.comp.spv");
static SHADER_GLYPHS_VERT:                    &[u8] = include_bytes!("../../app/res/shader/ui_glyphs.vert.spv");
static SHADER_GLYPHS_GEOM:                    &[u8] = include_bytes!("../../app/res/shader/ui_glyphs.geom.spv");
static SHADER_SDF_GEN_COMP:                   &[u8] = include_bytes!("../../app/res/shader/ui_sdf_gen.comp.spv");
static SHADER_GLYPHS_FRAG:                    &[u8] = include_bytes!("../../app/res/shader/ui_sdf.frag.spv");

#[derive(Debug, Default)]
pub struct DeviceContext {
	pub(crate) sampler:                                 VkSampler,
	pub(crate) desc_set_layout_ui_textured:             VkDescriptorSetLayout,
	pub(crate) desc_set_layout_ui_dyn:                  VkDescriptorSetLayout,
	pub(crate) desc_set_layout_ui_glyphs:               VkDescriptorSetLayout,
	pub(crate) desc_set_layout_ui_sdf_gen:              VkDescriptorSetLayout,
	pub(crate) pipeline_layout_ui:                      VkPipelineLayout,
	pub(crate) pipeline_layout_ui_textured:             VkPipelineLayout,
	pub(crate) pipeline_layout_ui_glyphs:               VkPipelineLayout,
	pub(crate) pipeline_layout_ui_sdf_gen:              VkPipelineLayout,
	pub(crate) shader_module_ui_colored_vert:           VkShaderModule,
	pub(crate) shader_module_ui_colored_gradient_vert:  VkShaderModule,
	pub(crate) shader_module_ui_colored_frag:           VkShaderModule,
	pub(crate) shader_module_ui_curves_vert:            VkShaderModule,
	pub(crate) shader_module_ui_curves_gradient_vert:   VkShaderModule,
	pub(crate) shader_module_ui_curves_frag:            VkShaderModule,
	pub(crate) shader_module_ui_textured_vert:          VkShaderModule,
	pub(crate) shader_module_ui_textured_gradient_vert: VkShaderModule,
	pub(crate) shader_module_ui_textured_frag:          VkShaderModule,
	pub(crate) shader_module_ui_sdf_frag:               VkShaderModule,
	pub(crate) shader_module_ui_glyphs_vert:            VkShaderModule,
	pub(crate) shader_module_ui_glyphs_geom:            VkShaderModule,
	pub(crate) pipeline_ui_sdf_gen:                     VkPipeline
}

impl DeviceContext {
	#[allow(clippy::cast_ptr_alignment)]
	pub(crate) fn create(device: &gpgpu::DeviceRootContext<Arc<World>>) -> Result<Self, VkResult> {
		let mut self_ = Self::default();
		
		device.device.createSampler(&VkSamplerCreateInfo {
			sType:                   VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
			pNext:                   None,
			flags:                   0,
			magFilter:               VK_FILTER_LINEAR,
			minFilter:               VK_FILTER_LINEAR,
			mipmapMode:              VK_SAMPLER_MIPMAP_MODE_LINEAR,
			addressModeU:            VK_SAMPLER_ADDRESS_MODE_REPEAT,
			addressModeV:            VK_SAMPLER_ADDRESS_MODE_REPEAT,
			addressModeW:            VK_SAMPLER_ADDRESS_MODE_REPEAT,
			mipLodBias:              0.0,
			anisotropyEnable:        device.device_info.vk10_features.samplerAnisotropy,
			maxAnisotropy:           device.device_info.vk10_properties.limits.maxSamplerAnisotropy,
			compareEnable:           VK_FALSE,
			compareOp:               VK_COMPARE_OP_NEVER,
			minLod:                  0.0,
			maxLod:                  1.0,
			borderColor:             VK_BORDER_COLOR_INT_TRANSPARENT_BLACK,
			unnormalizedCoordinates: VK_FALSE
		}, device.allocator, &mut self_.sampler)?;
		
		device.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        None,
			flags:        0,
			bindingCount: 1,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (binding = 0) uniform sampler2D splTexture / splSDF
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_FRAGMENT_BIT as u32,
					pImmutableSamplers: &self_.sampler
				}
			].as_ptr()
		}, device.allocator, &mut self_.desc_set_layout_ui_textured)?;
		
		device.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        None,
			flags:        0,
			bindingCount: 2,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (binding = 0) uniform sampler2D splSDF
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_FRAGMENT_BIT as u32,
					pImmutableSamplers: &self_.sampler
				},
				VkDescriptorSetLayoutBinding { // layout (binding = 1) readonly buffer SSBOGlyphs
					binding:            1,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_GEOMETRY_BIT as u32,
					pImmutableSamplers: ptr::null()
				}
			].as_ptr()
		}, device.allocator, &mut self_.desc_set_layout_ui_glyphs)?;
		
		device.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        None,
			flags:        0,
			bindingCount: 2,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (binding = 0) readonly buffer SSBOShapes
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (binding = 1) uniform writeonly image2D sdf
					binding:            1,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				}
			].as_ptr()
		}, device.allocator, &mut self_.desc_set_layout_ui_sdf_gen)?;
		
		device.device.createPipelineLayout(&VkPipelineLayoutCreateInfo {
			sType:                  VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext:                  None,
			flags:                  0,
			setLayoutCount:         0,
			pSetLayouts:            ptr::null(),
			pushConstantRangeCount: 0,
			pPushConstantRanges:    ptr::null()
		}, device.allocator, &mut self_.pipeline_layout_ui)?;
		
		device.device.createPipelineLayout(&VkPipelineLayoutCreateInfo {
			sType:                  VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext:                  None,
			flags:                  0,
			setLayoutCount:         1,
			pSetLayouts:            [self_.desc_set_layout_ui_textured].as_ptr(),
			pushConstantRangeCount: 0,
			pPushConstantRanges:    ptr::null()
		}, device.allocator, &mut self_.pipeline_layout_ui_textured)?;
		
		
		device.device.createPipelineLayout(&VkPipelineLayoutCreateInfo {
			sType:                  VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext:                  None,
			flags:                  0,
			setLayoutCount:         1,
			pSetLayouts:            [self_.desc_set_layout_ui_glyphs].as_ptr(),
			pushConstantRangeCount: 0,
			pPushConstantRanges:    ptr::null()
		}, device.allocator, &mut self_.pipeline_layout_ui_glyphs)?;
		
		device.device.createPipelineLayout(&VkPipelineLayoutCreateInfo {
			sType:                  VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext:                  None,
			flags:                  0,
			setLayoutCount:         1,
			pSetLayouts:            [self_.desc_set_layout_ui_sdf_gen].as_ptr(),
			pushConstantRangeCount: 1,
			pPushConstantRanges:    [
				VkPushConstantRange {
					stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as _,
					offset:     0,
					size:       24
				}
			].as_ptr()
		}, device.allocator, &mut self_.pipeline_layout_ui_sdf_gen)?;
		
		let info = VkShaderModuleCreateInfo {
			sType:    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
			pNext:    None,
			flags:    0,
			..VkShaderModuleCreateInfo::default()
		};
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_COLORED_VERT.len(),
			pCode:    SHADER_UI_COLORED_VERT.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_colored_vert)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_COLORED_GRADIENT_VERT.len(),
			pCode:    SHADER_UI_COLORED_GRADIENT_VERT.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_colored_gradient_vert)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_COLORED_FRAG.len(),
			pCode:    SHADER_UI_COLORED_FRAG.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_colored_frag)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_CURVES_VERT.len(),
			pCode:    SHADER_UI_CURVES_VERT.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_curves_vert)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_CURVES_GRADIENT_VERT.len(),
			pCode:    SHADER_UI_CURVES_GRADIENT_VERT.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_curves_gradient_vert)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_CURVES_FRAG.len(),
			pCode:    SHADER_UI_CURVES_FRAG.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_curves_frag)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_TEXTURED_VERT.len(),
			pCode:    SHADER_UI_TEXTURED_VERT.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_textured_vert)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_TEXTURED_GRADIENT_VERT.len(),
			pCode:    SHADER_UI_TEXTURED_GRADIENT_VERT.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_textured_gradient_vert)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_TEXTURED_FRAG.len(),
			pCode:    SHADER_UI_TEXTURED_FRAG.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_textured_frag)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_UI_SDF_FRAG.len(),
			pCode:    SHADER_UI_SDF_FRAG.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_sdf_frag)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_GLYPHS_VERT.len(),
			pCode:    SHADER_GLYPHS_VERT.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_glyphs_vert)?;
		
		device.device.createShaderModule(&VkShaderModuleCreateInfo {
			codeSize: SHADER_GLYPHS_GEOM.len(),
			pCode:    SHADER_GLYPHS_GEOM.as_ptr() as _,
			..info
		}, device.allocator, &mut self_.shader_module_ui_glyphs_geom)?;
		
		let mut module = VK_NULL_HANDLE;
		let mut pipelines = [VK_NULL_HANDLE; 1];
		device.device.createComputePipelines(device.pipeline_cache, &[
			VkComputePipelineCreateInfo {
				sType: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
				pNext: None,
				flags: 0,
				stage: VkPipelineShaderStageCreateInfo {
					sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
					pNext: None,
					flags: 0,
					stage: VK_SHADER_STAGE_COMPUTE_BIT,
					module: {
						device.device.createShaderModule(&VkShaderModuleCreateInfo {
							sType:    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
							pNext:    None,
							flags:    0,
							codeSize: SHADER_SDF_GEN_COMP.len(),
							pCode:    SHADER_SDF_GEN_COMP.as_ptr() as _
						}, device.allocator, &mut module)?;
						module
					},
					pName: "main\0".as_ptr(),
					pSpecializationInfo: None
				},
				layout:             self_.pipeline_layout_ui_sdf_gen,
				basePipelineHandle: VK_NULL_HANDLE,
				basePipelineIndex:  !0
			}
		], device.allocator, &mut pipelines)?;
		
		[self_.pipeline_ui_sdf_gen] = pipelines;
		device.device.destroyShaderModule(module, device.allocator);
		
		set_object_names(device.instance_info.vk12_debug_utils, &device.device, &[
			(VK_OBJECT_TYPE_SAMPLER,               self_.sampler,                                 "ui_sampler\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_ui_textured,             "desc_set_layout_ui_textured\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_ui_glyphs,               "desc_set_layout_ui_glyphs\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_ui_sdf_gen,              "desc_set_layout_ui_sdf_gen\0"),
			(VK_OBJECT_TYPE_PIPELINE_LAYOUT,       self_.pipeline_layout_ui,                      "pipeline_layout_ui\0"),
			(VK_OBJECT_TYPE_PIPELINE_LAYOUT,       self_.pipeline_layout_ui_textured,             "pipeline_layout_ui_textured\0"),
			(VK_OBJECT_TYPE_PIPELINE_LAYOUT,       self_.pipeline_layout_ui_glyphs,               "pipeline_layout_ui_glyphs\0"),
			(VK_OBJECT_TYPE_PIPELINE_LAYOUT,       self_.pipeline_layout_ui_sdf_gen,              "pipeline_layout_ui_sdf_gen\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_colored_vert,           "shader_module_ui_colored_vert\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_colored_gradient_vert,  "shader_module_ui_colored_gradient_vert\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_colored_frag,           "shader_module_ui_colored_frag\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_curves_vert,            "shader_module_ui_curves_vert\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_curves_gradient_vert,   "shader_module_ui_curves_gradient_vert\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_curves_frag,            "shader_module_ui_curves_frag\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_textured_vert,          "shader_module_ui_textured_vert\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_textured_gradient_vert, "shader_module_ui_textured_gradient_vert\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_textured_frag,          "shader_module_ui_textured_frag\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_sdf_frag,               "shader_module_ui_sdf_frag\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_glyphs_vert,            "shader_module_ui_glyphs_vert\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE,         self_.shader_module_ui_glyphs_geom,            "shader_module_ui_glyphs_geom\0"),
			(VK_OBJECT_TYPE_PIPELINE,              self_.pipeline_ui_sdf_gen,                     "pipeline_ui_sdf_gen\0"),
		]);
		
		Ok(self_)
	}
	
	pub(crate) fn destroy(&mut self, device: &gpgpu::DeviceRootContext<Arc<World>>) -> Result<(), VkResult> {
		device.device.destroyPipeline(self.pipeline_ui_sdf_gen, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_glyphs_vert, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_glyphs_geom, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_sdf_frag, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_textured_frag, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_textured_gradient_vert, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_textured_vert, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_colored_frag, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_colored_gradient_vert, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_colored_vert, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_curves_frag, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_curves_gradient_vert, device.allocator);
		device.device.destroyShaderModule(self.shader_module_ui_curves_vert, device.allocator);
		device.device.destroyPipelineLayout(self.pipeline_layout_ui_sdf_gen, device.allocator);
		device.device.destroyPipelineLayout(self.pipeline_layout_ui_glyphs, device.allocator);
		device.device.destroyPipelineLayout(self.pipeline_layout_ui_textured, device.allocator);
		device.device.destroyPipelineLayout(self.pipeline_layout_ui, device.allocator);
		device.device.destroyDescriptorSetLayout(self.desc_set_layout_ui_sdf_gen, device.allocator);
		device.device.destroyDescriptorSetLayout(self.desc_set_layout_ui_glyphs, device.allocator);
		device.device.destroyDescriptorSetLayout(self.desc_set_layout_ui_dyn, device.allocator);
		device.device.destroyDescriptorSetLayout(self.desc_set_layout_ui_textured, device.allocator);
		device.device.destroySampler(self.sampler, device.allocator);
		Ok(())
	}
}