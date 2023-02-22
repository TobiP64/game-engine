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
	std::{ptr, mem::size_of, collections::VecDeque},
	gpgpu::misc::set_object_names,
	custom_sync::AsyncCondvar,
};

static SHADER_DRAW: &[u8]     = include_bytes!("../../app/res/shader/sdf_trace.comp.spv");
static SHADER_UPD:  &[u8]     = include_bytes!("../../app/res/shader/sdf_upd.comp.spv");
static SHADER_GEN:  &[u8]     = include_bytes!("../../app/res/shader/sdf_gen.comp.spv");

static SDF_FORMATS: [VkFormat; 8] = [
	VK_FORMAT_R8_SNORM,
	VK_FORMAT_R16_SNORM,
	VK_FORMAT_R16_SFLOAT,
	VK_FORMAT_R32_SFLOAT,
	VK_FORMAT_R8G8_SNORM,
	VK_FORMAT_R16G16_SNORM,
	VK_FORMAT_R16G16_SFLOAT,
	VK_FORMAT_R32G32_SFLOAT
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TexturingMode {
	Disabled,
	BiPlanarMapping,
	TriPlanarMapping
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AmbientOcclusionMode {
	Disabled,
	FiveTap,
	MultiRay5Tap,
	ConeTraced
}

#[derive(Copy, Clone, Debug)]
pub struct DeviceOptions {
	pub fail_on_oom:       bool,
	pub disable_sparse:    bool,
	pub gpu_sdf_gen:       bool,
	pub gpu_sdf_upd:       bool,
	pub sdf_format:        VkFormat,
	pub texturing:         TexturingMode,
	pub ambient_occlusion: AmbientOcclusionMode
}

impl DeviceOptions {
	pub fn validate(self) -> Self {
		debug_assert!(
			SDF_FORMATS.contains(&self.sdf_format),
			"invalid sdf format: expected one of {:?}, but was {:?}",
			SDF_FORMATS, self.sdf_format
		);
		self
	}
}

impl Default for DeviceOptions {
	fn default() -> Self {
		Self {
			fail_on_oom:       false,
			disable_sparse:    true,
			gpu_sdf_gen:       false,
			gpu_sdf_upd:       false,
			sdf_format:        VK_FORMAT_R16_SNORM,
			texturing:         TexturingMode::Disabled,
			ambient_occlusion: AmbientOcclusionMode::Disabled
		}
	}
}

#[derive(Debug, Default)]
pub struct DeviceContext {
	pub(crate) options:                        DeviceOptions,
	pub(crate) scene_sdf_sampler:              VkSampler,
	pub(crate) scene_ids_sampler:              VkSampler,
	pub(crate) desc_set_layout_draw_general:   VkDescriptorSetLayout,
	pub(crate) desc_set_layout_draw_textures:  VkDescriptorSetLayout,
	pub(crate) desc_set_layout_draw_out_image: VkDescriptorSetLayout,
	pub(crate) desc_set_layout_upd:            VkDescriptorSetLayout,
	pub(crate) desc_set_layout_gen:            VkDescriptorSetLayout,
	pub(crate) pipeline_layout_draw:           VkPipelineLayout,
	pub(crate) pipeline_layout_upd:            VkPipelineLayout,
	pub(crate) pipeline_layout_gen:            VkPipelineLayout,
	pub(crate) shader_module_draw:             VkShaderModule,
	pub(crate) shader_module_upd:              VkShaderModule,
	pub(crate) shader_module_gen:              VkShaderModule,
	pub(crate) pipeline_draw:                  VkPipeline,
	pub(crate) pipeline_upd:                   VkPipeline,
	pub(crate) pipeline_gen:                   VkPipeline,
	pub(crate) desc_updates:                   VecDeque<(VkWriteDescriptorSet<'static>, VkDescriptorImageInfo)>,
	pub(crate) desc_update_wait:               AsyncCondvar
}

impl DeviceContext {
	#[allow(clippy::cast_ptr_alignment)]
	pub(crate) fn create(context: &gpgpu::DeviceRootContext<Arc<World>>) -> Result<Self, VkResult> {
		let mut self_ = Self::default();
		
		// create samplers
		
		context.device.createSampler(&VkSamplerCreateInfo {
			sType:        VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
			magFilter:    VK_FILTER_LINEAR,
			minFilter:    VK_FILTER_LINEAR,
			mipmapMode:   VK_SAMPLER_MIPMAP_MODE_NEAREST, // VK_SAMPLER_MIPMAP_MODE_LINEAR
			addressModeU: VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
			addressModeV: VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
			addressModeW: VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
			borderColor:  VK_BORDER_COLOR_INT_OPAQUE_BLACK,
			..VkSamplerCreateInfo::default()
		}, context.allocator, &mut self_.scene_sdf_sampler)?;
		
		context.device.createSampler(&VkSamplerCreateInfo {
			sType:        VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
			magFilter:    VK_FILTER_NEAREST,
			minFilter:    VK_FILTER_NEAREST,
			mipmapMode:   VK_SAMPLER_MIPMAP_MODE_NEAREST,
			addressModeU: VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
			addressModeV: VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
			addressModeW: VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
			borderColor:  VK_BORDER_COLOR_INT_OPAQUE_BLACK,
			..VkSamplerCreateInfo::default()
		}, context.allocator, &mut self_.scene_ids_sampler)?;
		
		// create desc set layouts
		
		context.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        None,
			flags:        0,
			bindingCount: 7,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 0) uniform Camera
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 1) uniform sampler3D scene_sdf;
					binding:            1,
					descriptorType:     VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: &self_.scene_sdf_sampler
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 2) uniform sampler3D scene_ids;
					binding:            2,
					descriptorType:     VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: &self_.scene_ids_sampler
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 3) readonly buffer   Instances { uint instance_count; Instance instances[ ]; };
					binding:            3,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 4) readonly buffer   Materials { Material materials[ ]; };
					binding:            4,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 5) readonly buffer   Lights    { uint lights[ ]; };
					binding:            5,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 6) uniform sampler   sdf_sampler;
					binding:            6,
					descriptorType:     VK_DESCRIPTOR_TYPE_SAMPLER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: &self_.scene_sdf_sampler
				}
			].as_ptr()
		}, context.allocator, &mut self_.desc_set_layout_draw_general)?;
		
		context.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        Some(VkAnyRef::new(&VkDescriptorSetLayoutBindingFlagsCreateInfo {
				sType:         VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
				pNext:         None,
				bindingCount:  1,
				pBindingFlags: [
					VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT as u32
						| VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT as u32
						| VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT as u32
						| VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT as u32
				].as_ptr()
			})),
			flags:        VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT as _,
			bindingCount: 1,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (set = 1/2, binding = 0) uniform texture2D/3D sdfs[ ]/materials_spl[ ];
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
					descriptorCount:    DESC_LIMIT,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				}
			].as_ptr()
		}, context.allocator, &mut self_.desc_set_layout_draw_textures)?;
		
		context.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        None,
			flags:        0,
			bindingCount: 1,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (set = 3, binding = 0) uniform writeonly image2D out_image;
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				}
			].as_ptr()
		}, context.allocator, &mut self_.desc_set_layout_draw_out_image)?;
		
		context.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        None,
			flags:        0,
			bindingCount: 4,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 0, r16f) uniform writeonly image3D scene_sdf;
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 1, r16 ) uniform writeonly image3D scene_ids;
					binding:            1,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 2) readonly buffer ssbo_instances { uint  instance_count; Instance instances[ ]; };
					binding:            2,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (set = 0, binding = 3) uniform sampler sdf_sampler;
					binding:            3,
					descriptorType:     VK_DESCRIPTOR_TYPE_SAMPLER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: &self_.scene_sdf_sampler
				}
			].as_ptr()
		}, context.allocator, &mut self_.desc_set_layout_upd)?;
		
		context.device.createDescriptorSetLayout(&VkDescriptorSetLayoutCreateInfo {
			sType:        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext:        None,
			flags:        0,
			bindingCount: 2,
			pBindings:    [
				VkDescriptorSetLayoutBinding { // layout (binding = 0, r16f) uniform writeonly image3D sdf;
					binding:            0,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				},
				VkDescriptorSetLayoutBinding { // layout (binding = 1) readonly buffer data { ... }
					binding:            1,
					descriptorType:     VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
					descriptorCount:    1,
					stageFlags:         VK_SHADER_STAGE_COMPUTE_BIT as u32,
					pImmutableSamplers: ptr::null()
				}
			].as_ptr()
		}, context.allocator, &mut self_.desc_set_layout_gen)?;
		
		// create pipelines
		
		context.device.createPipelineLayout(&VkPipelineLayoutCreateInfo {
			sType:                  VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext:                  None,
			flags:                  0,
			setLayoutCount:         4,
			pSetLayouts:            [
				self_.desc_set_layout_draw_general,
				self_.desc_set_layout_draw_textures,
				self_.desc_set_layout_draw_textures,
				self_.desc_set_layout_draw_out_image
			].as_ptr(),
			pushConstantRangeCount: 1,
			pPushConstantRanges:    &VkPushConstantRange {
				stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as _,
				offset:     0,
				size:       size_of::<RenderParams>() as _
			}
		}, context.allocator, &mut self_.pipeline_layout_draw)?;
		
		context.device.createPipelineLayout(&VkPipelineLayoutCreateInfo {
			sType:                  VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext:                  None,
			flags:                  0,
			setLayoutCount:         2,
			pSetLayouts:            [
				self_.desc_set_layout_upd,
				self_.desc_set_layout_draw_textures
			].as_ptr(),
			pushConstantRangeCount: 1,
			pPushConstantRanges:    &VkPushConstantRange {
				stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as _,
				offset:     0,
				size:       size_of::<UpdateParams>() as _
			}
		}, context.allocator, &mut self_.pipeline_layout_upd)?;
		
		context.device.createPipelineLayout(&VkPipelineLayoutCreateInfo {
			sType:                  VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext:                  None,
			flags:                  0,
			setLayoutCount:         1,
			pSetLayouts:            &self_.desc_set_layout_gen,
			pushConstantRangeCount: 0,
			pPushConstantRanges:    ptr::null()
		}, context.allocator, &mut self_.pipeline_layout_gen)?;
		
		context.device.createShaderModule(&VkShaderModuleCreateInfo {
			sType:    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
			pNext:    None,
			flags:    0,
			codeSize: SHADER_DRAW.len(),
			pCode:    SHADER_DRAW.as_ptr() as _
		}, context.allocator, &mut self_.shader_module_draw)?;
		
		context.device.createShaderModule(&VkShaderModuleCreateInfo {
			sType:    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
			pNext:    None,
			flags:    0,
			codeSize: SHADER_UPD.len(),
			pCode:    SHADER_UPD.as_ptr() as _
		}, context.allocator, &mut self_.shader_module_upd)?;
		
		context.device.createShaderModule(&VkShaderModuleCreateInfo {
			sType:    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
			pNext:    None,
			flags:    0,
			codeSize: SHADER_GEN.len(),
			pCode:    SHADER_GEN.as_ptr() as _
		}, context.allocator, &mut self_.shader_module_gen)?;
		
		let mut pipelines = [VK_NULL_HANDLE; 3];
		context.device.createComputePipelines(context.pipeline_cache, &[
			VkComputePipelineCreateInfo {
				stage:  VkPipelineShaderStageCreateInfo {
					module: self_.shader_module_draw,
					..DEFAULT_PIPELINE_INFO.stage
				},
				layout: self_.pipeline_layout_draw,
				..DEFAULT_PIPELINE_INFO
			},
			VkComputePipelineCreateInfo {
				stage:  VkPipelineShaderStageCreateInfo {
					module: self_.shader_module_upd,
					..DEFAULT_PIPELINE_INFO.stage
				},
				layout: self_.pipeline_layout_upd,
				..DEFAULT_PIPELINE_INFO
			},
			VkComputePipelineCreateInfo {
				stage:  VkPipelineShaderStageCreateInfo {
					module: self_.shader_module_gen,
					..DEFAULT_PIPELINE_INFO.stage
				},
				layout: self_.pipeline_layout_gen,
				..DEFAULT_PIPELINE_INFO
			}
		], context.allocator, &mut pipelines)?;
		
		[self_.pipeline_draw, self_.pipeline_upd, self_.pipeline_gen] = pipelines;
		
		// set object names
		
		set_object_names(context.instance_info.vk12_debug_utils, &context.device, &[
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_draw_general, "desc_set_layout_draw_general\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_draw_textures, "desc_set_layout_draw_textures\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_draw_out_image, "desc_set_layout_draw_out_image\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_upd, "desc_set_layout_upd\0"),
			(VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT, self_.desc_set_layout_gen, "desc_set_layout_gen\0"),
			(VK_OBJECT_TYPE_PIPELINE_LAYOUT, self_.pipeline_layout_draw, "pipeline_layout_draw\0"),
			(VK_OBJECT_TYPE_PIPELINE_LAYOUT, self_.pipeline_layout_upd, "pipeline_layout_upd\0"),
			(VK_OBJECT_TYPE_PIPELINE_LAYOUT, self_.pipeline_layout_gen, "pipeline_layout_gen\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE, self_.shader_module_draw, "shader_module_draw\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE, self_.shader_module_upd, "shader_module_upd\0"),
			(VK_OBJECT_TYPE_SHADER_MODULE, self_.shader_module_gen, "shader_module_gen\0"),
			(VK_OBJECT_TYPE_PIPELINE, self_.pipeline_draw, "pipeline_draw\0"),
			(VK_OBJECT_TYPE_PIPELINE, self_.pipeline_upd, "pipeline_upd\0"),
			(VK_OBJECT_TYPE_PIPELINE, self_.pipeline_gen, "pipeline_gen\0"),
			(VK_OBJECT_TYPE_SAMPLER, self_.scene_sdf_sampler, "scene_sdf_sampler\0"),
			(VK_OBJECT_TYPE_SAMPLER, self_.scene_ids_sampler, "scene_ids_sampler\0")
		])?;
		
		Ok(self_)
	}
	
	pub(crate) fn destroy(&mut self, context: &gpgpu::DeviceRootContext<Arc<World>>) -> Result<(), VkResult> {
		context.device.destroyPipeline(self.pipeline_gen, context.allocator);
		context.device.destroyPipeline(self.pipeline_upd, context.allocator);
		context.device.destroyPipeline(self.pipeline_draw, context.allocator);
		context.device.destroyShaderModule(self.shader_module_gen, context.allocator);
		context.device.destroyShaderModule(self.shader_module_upd, context.allocator);
		context.device.destroyShaderModule(self.shader_module_draw, context.allocator);
		context.device.destroyPipelineLayout(self.pipeline_layout_gen, context.allocator);
		context.device.destroyPipelineLayout(self.pipeline_layout_upd, context.allocator);
		context.device.destroyPipelineLayout(self.pipeline_layout_draw, context.allocator);
		context.device.destroyDescriptorSetLayout(self.desc_set_layout_gen, context.allocator);
		context.device.destroyDescriptorSetLayout(self.desc_set_layout_upd, context.allocator);
		context.device.destroyDescriptorSetLayout(self.desc_set_layout_draw_general, context.allocator);
		context.device.destroyDescriptorSetLayout(self.desc_set_layout_draw_textures, context.allocator);
		context.device.destroyDescriptorSetLayout(self.desc_set_layout_draw_out_image, context.allocator);
		context.device.destroySampler(self.scene_ids_sampler, context.allocator);
		context.device.destroySampler(self.scene_sdf_sampler, context.allocator);
		Ok(())
	}
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
struct RenderParams {
	extent:            Vec2<u32>,
	samples:           u32,
	bounces:           u32,
	min_step_size:     f32,
	min_dist:          f32,
	min_dist_global:   f32,
	max_dist:          f32,
	min_steps:         u32,
	max_steps:         u32,
	normal_polls:      u32,
	normal_smoothness: f32,
	features:          u32
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
struct UpdateParams {
	extent: Vec3<u32>
}

static DEFAULT_PIPELINE_INFO: VkComputePipelineCreateInfo = VkComputePipelineCreateInfo {
	sType:              VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
	pNext:              None,
	flags:              0,
	stage:              VkPipelineShaderStageCreateInfo {
		sType:               VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
		pNext:               None,
		flags:               0,
		stage:               VK_SHADER_STAGE_COMPUTE_BIT,
		module:              VK_NULL_HANDLE,
		pName:               "main\0".as_ptr(),
		pSpecializationInfo: None
	},
	layout:             VK_NULL_HANDLE,
	basePipelineHandle: VK_NULL_HANDLE,
	basePipelineIndex:  !0
};