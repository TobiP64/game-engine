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
	gpgpu::{plugins::DEFAULT_STAGE, misc::set_object_names}
};

#[derive(Debug, Default)]
pub struct TargetContext {
	pub(crate) pipeline_colored:           VkPipeline, // colored 2d triangles
	pub(crate) pipeline_colored_gradient:  VkPipeline, // colored 2d triangles
	pub(crate) pipeline_curves:            VkPipeline, // colored 2d curves
	pub(crate) pipeline_curves_gradient:   VkPipeline, // colored 2d curves
	pub(crate) pipeline_textured:          VkPipeline, // textured 2d triangles
	pub(crate) pipeline_textured_gradient: VkPipeline, // textured 2d triangles
	pub(crate) pipeline_sdf:               VkPipeline, // sdf textures (e.g. vector graphics)
	pub(crate) pipeline_glyphs:            VkPipeline  // glyphs
}

impl TargetContext {
	pub(crate) fn create(
		device_ctx: &DeviceContext,
		device:     &gpgpu::DeviceRootContext<Arc<World>>,
		template:   &VkGraphicsPipelineCreateInfo
	) -> Result<Self, VkResult> {
		let stages_ui_colored = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_colored_vert,
				.. DEFAULT_STAGE
			},
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_FRAGMENT_BIT,
				module: device_ctx.shader_module_ui_colored_frag,
				.. DEFAULT_STAGE
			}
		];
		
		let stages_ui_colored_gradient = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_colored_gradient_vert,
				.. DEFAULT_STAGE
			},
			stages_ui_colored[1]
		];
		
		let stages_ui_curves = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_curves_vert,
				.. DEFAULT_STAGE
			},
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_FRAGMENT_BIT,
				module: device_ctx.shader_module_ui_curves_frag,
				.. DEFAULT_STAGE
			}
		];
		
		let stages_ui_curves_gradient = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_curves_gradient_vert,
				.. DEFAULT_STAGE
			},
			stages_ui_curves[1]
		];
		
		let stages_ui_textured = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_textured_vert,
				.. DEFAULT_STAGE
			},
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_FRAGMENT_BIT,
				module: device_ctx.shader_module_ui_textured_frag,
				.. DEFAULT_STAGE
			}
		];
		
		let stages_ui_textured_gradient = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_textured_gradient_vert,
				.. DEFAULT_STAGE
			},
			stages_ui_textured[1]
		];
		
		let stages_ui_sdf = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_textured_vert,
				.. DEFAULT_STAGE
			},
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_FRAGMENT_BIT,
				module: device_ctx.shader_module_ui_sdf_frag,
				.. DEFAULT_STAGE
			}
		];
		
		let stages_glyphs = [
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_VERTEX_BIT,
				module: device_ctx.shader_module_ui_glyphs_vert,
				.. DEFAULT_STAGE
			},
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_GEOMETRY_BIT,
				module: device_ctx.shader_module_ui_glyphs_geom,
				.. DEFAULT_STAGE
			},
			VkPipelineShaderStageCreateInfo {
				stage:  VK_SHADER_STAGE_FRAGMENT_BIT,
				module: device_ctx.shader_module_ui_sdf_frag,
				.. DEFAULT_STAGE
			}
		];
		
		let mut pipelines = [VK_NULL_HANDLE; 8];
		device.device.createGraphicsPipelines(device.pipeline_cache, &[
			VkGraphicsPipelineCreateInfo {
				stageCount:        stages_ui_colored.len() as _,
				pStages:           stages_ui_colored.as_ptr(),
				pVertexInputState: Some(&PIPELINE_VERTEX_INPUT_STATE_DEFAULT),
				layout:            device_ctx.pipeline_layout_ui,
				..*template
			},
			VkGraphicsPipelineCreateInfo {
				stageCount:        stages_ui_colored_gradient.len() as _,
				pStages:           stages_ui_colored_gradient.as_ptr(),
				pVertexInputState: Some(&PIPELINE_VERTEX_INPUT_STATE_GRADIENT),
				layout:            device_ctx.pipeline_layout_ui,
				basePipelineIndex: 0,
				..*template
			},
			VkGraphicsPipelineCreateInfo {
				stageCount:        stages_ui_curves.len() as _,
				pStages:           stages_ui_curves.as_ptr(),
				pVertexInputState: Some(&PIPELINE_VERTEX_INPUT_STATE_DEFAULT),
				layout:            device_ctx.pipeline_layout_ui,
				basePipelineIndex: 0,
				..*template
			},
			VkGraphicsPipelineCreateInfo {
				stageCount:        stages_ui_curves_gradient.len() as _,
				pStages:           stages_ui_curves_gradient.as_ptr(),
				pVertexInputState: Some(&PIPELINE_VERTEX_INPUT_STATE_GRADIENT),
				layout:            device_ctx.pipeline_layout_ui,
				basePipelineIndex: 0,
				..*template
			},
			VkGraphicsPipelineCreateInfo {
				stageCount:        stages_ui_textured.len() as _,
				pStages:           stages_ui_textured.as_ptr(),
				pVertexInputState: Some(&PIPELINE_VERTEX_INPUT_STATE_TEXTURED),
				layout:            device_ctx.pipeline_layout_ui_textured,
				basePipelineIndex: 0,
				..*template
			},
			VkGraphicsPipelineCreateInfo {
				stageCount:        stages_ui_textured_gradient.len() as _,
				pStages:           stages_ui_textured_gradient.as_ptr(),
				pVertexInputState: Some(&PIPELINE_VERTEX_INPUT_STATE_TEXTURED_GRADIENT),
				layout:            device_ctx.pipeline_layout_ui_textured,
				basePipelineIndex: 0,
				..*template
			},
			VkGraphicsPipelineCreateInfo {
				stageCount:        stages_ui_sdf.len() as _,
				pStages:           stages_ui_sdf.as_ptr(),
				pVertexInputState: Some(&PIPELINE_VERTEX_INPUT_STATE_TEXTURED),
				layout:            device_ctx.pipeline_layout_ui_textured,
				basePipelineIndex: 0,
				..*template
			},
			VkGraphicsPipelineCreateInfo {
				stageCount:          stages_glyphs.len() as _,
				pStages:             stages_glyphs.as_ptr(),
				pVertexInputState:   Some(&VkPipelineVertexInputStateCreateInfo {
					sType:                           VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
					pNext:                           None,
					flags:                           0,
					vertexBindingDescriptionCount:   2,
					pVertexBindingDescriptions:      [
						VkVertexInputBindingDescription { binding: 0, stride: 0x0C, inputRate: VK_VERTEX_INPUT_RATE_VERTEX },
						VkVertexInputBindingDescription { binding: 1, stride: 0x60, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE }
					].as_ptr(),
					vertexAttributeDescriptionCount: 7,
					pVertexAttributeDescriptions:    [
						VkVertexInputAttributeDescription { location: 0, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x00 }, // pos
						VkVertexInputAttributeDescription { location: 1, binding: 0, format: VK_FORMAT_R32_UINT,            offset: 0x08 }, // char
						VkVertexInputAttributeDescription { location: 2, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 }, // model[0]
						VkVertexInputAttributeDescription { location: 3, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 }, // model[1]
						VkVertexInputAttributeDescription { location: 4, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 }, // model[2]
						VkVertexInputAttributeDescription { location: 5, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 }, // model[3]
						VkVertexInputAttributeDescription { location: 6, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x50 }  // color
					].as_ptr()
				}),
				pInputAssemblyState: Some(&VkPipelineInputAssemblyStateCreateInfo {
					sType:                  VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
					pNext:                  None,
					flags:                  0,
					topology:               VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
					primitiveRestartEnable: VK_FALSE
				}),
				layout:              device_ctx.pipeline_layout_ui_glyphs,
				..*template
			}
		], device.allocator, &mut pipelines)?;
		
		set_object_names(device.instance_info.vk12_debug_utils, &device.device, &[
			(VK_OBJECT_TYPE_PIPELINE, pipelines[0], "pipeline_colored\0"),
			(VK_OBJECT_TYPE_PIPELINE, pipelines[1], "pipeline_colored_gradient\0"),
			(VK_OBJECT_TYPE_PIPELINE, pipelines[2], "pipeline_curves\0"),
			(VK_OBJECT_TYPE_PIPELINE, pipelines[3], "pipeline_curves_gradient\0"),
			(VK_OBJECT_TYPE_PIPELINE, pipelines[4], "pipeline_textured\0"),
			(VK_OBJECT_TYPE_PIPELINE, pipelines[5], "pipeline_textured_gradient\0"),
			(VK_OBJECT_TYPE_PIPELINE, pipelines[6], "pipeline_sdf\0"),
			(VK_OBJECT_TYPE_PIPELINE, pipelines[7], "pipeline_glyphs\0"),
		]);
		
		Ok(Self {
			pipeline_colored:           pipelines[0],
			pipeline_colored_gradient:  pipelines[1],
			pipeline_curves:            pipelines[2],
			pipeline_curves_gradient:   pipelines[3],
			pipeline_textured:          pipelines[4],
			pipeline_textured_gradient: pipelines[5],
			pipeline_sdf:               pipelines[6],
			pipeline_glyphs:            pipelines[7]
		})
	}
	
	pub(crate) fn update(
		&mut self,
		device_ctx: &DeviceContext,
		device:     &gpgpu::DeviceRootContext<Arc<World>>,
		template:   &VkGraphicsPipelineCreateInfo
	) -> Result<(), VkResult> {
		self.destroy(device)?;
		*self = Self::create(device_ctx, device, template)?;
		Ok(())
	}
	
	pub(crate) fn destroy(&mut self, device: &gpgpu::DeviceRootContext<Arc<World>>) -> Result<(), VkResult> {
		let dev = &device.device;
		dev.destroyPipeline(self.pipeline_colored, device.allocator);
		dev.destroyPipeline(self.pipeline_colored_gradient, device.allocator);
		dev.destroyPipeline(self.pipeline_curves, device.allocator);
		dev.destroyPipeline(self.pipeline_curves_gradient, device.allocator);
		dev.destroyPipeline(self.pipeline_textured, device.allocator);
		dev.destroyPipeline(self.pipeline_textured_gradient, device.allocator);
		dev.destroyPipeline(self.pipeline_sdf, device.allocator);
		dev.destroyPipeline(self.pipeline_glyphs, device.allocator);
		Ok(())
	}
}

static PIPELINE_VERTEX_INPUT_STATE_DEFAULT: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	sType:                           VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
	pNext:                           None,
	flags:                           0,
	vertexBindingDescriptionCount:   2,
	pVertexBindingDescriptions:      [
		VkVertexInputBindingDescription { binding: 0, stride: 0x8,  inputRate: VK_VERTEX_INPUT_RATE_VERTEX },
		VkVertexInputBindingDescription { binding: 1, stride: 0x50, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE }
	].as_ptr(),
	vertexAttributeDescriptionCount: 6,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0,  binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x00 }, // pos
		VkVertexInputAttributeDescription { location: 8,  binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x00 }, // model[0]
		VkVertexInputAttributeDescription { location: 9,  binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 }, // model[1]
		VkVertexInputAttributeDescription { location: 10, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 }, // model[2]
		VkVertexInputAttributeDescription { location: 11, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 }, // model[3]
		VkVertexInputAttributeDescription { location: 12, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 }  // color
	].as_ptr()
};

static PIPELINE_VERTEX_INPUT_STATE_GRADIENT: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	vertexAttributeDescriptionCount: 7,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location:  0, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x00 }, // pos
		VkVertexInputAttributeDescription { location:  1, binding: 0, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x08 }, // gradient
		VkVertexInputAttributeDescription { location:  8, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x00 }, // model[0]
		VkVertexInputAttributeDescription { location:  9, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 }, // model[1]
		VkVertexInputAttributeDescription { location: 10, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 }, // model[2]
		VkVertexInputAttributeDescription { location: 11, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 }, // model[3]
		VkVertexInputAttributeDescription { location: 12, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 }  // color
	].as_ptr(),
	..PIPELINE_VERTEX_INPUT_STATE_DEFAULT
};

static PIPELINE_VERTEX_INPUT_STATE_TEXTURED: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	vertexAttributeDescriptionCount: 7,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0,  binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x00 }, // pos
		VkVertexInputAttributeDescription { location: 2,  binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x08 }, // tex
		VkVertexInputAttributeDescription { location: 8,  binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x00 }, // model[0]
		VkVertexInputAttributeDescription { location: 9,  binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 }, // model[1]
		VkVertexInputAttributeDescription { location: 10, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 }, // model[2]
		VkVertexInputAttributeDescription { location: 11, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 }, // model[3]
		VkVertexInputAttributeDescription { location: 12, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 }  // color
	].as_ptr(),
	..PIPELINE_VERTEX_INPUT_STATE_DEFAULT
};

static PIPELINE_VERTEX_INPUT_STATE_TEXTURED_GRADIENT: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	vertexAttributeDescriptionCount: 8,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0,  binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x00 }, // pos
		VkVertexInputAttributeDescription { location: 1,  binding: 0, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x08 }, // gradient
		VkVertexInputAttributeDescription { location: 2,  binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x18 }, // tex
		VkVertexInputAttributeDescription { location: 8,  binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x00 }, // model[0]
		VkVertexInputAttributeDescription { location: 9,  binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 }, // model[1]
		VkVertexInputAttributeDescription { location: 10, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 }, // model[2]
		VkVertexInputAttributeDescription { location: 11, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 }, // model[3]
		VkVertexInputAttributeDescription { location: 12, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 }  // color
	].as_ptr(),
	..PIPELINE_VERTEX_INPUT_STATE_DEFAULT
};

//
// DYN -----------------------------------------------------------------------------------------
//

#[allow(dead_code)]
static PIPELINE_VERTEX_INPUT_STATE_UI_DYN_BASIC: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	sType:                           VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
	pNext:                           None,
	flags:                           0,
	vertexBindingDescriptionCount:   6,
	pVertexBindingDescriptions:      [
		VkVertexInputBindingDescription { binding: 0, stride: 0x08,  inputRate: VK_VERTEX_INPUT_RATE_VERTEX },
		VkVertexInputBindingDescription { binding: 1, stride: 0x50, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE },
		VkVertexInputBindingDescription { binding: 2, stride: 0x0C, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE },
		VkVertexInputBindingDescription { binding: 3, stride: 0x0C, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE },
		VkVertexInputBindingDescription { binding: 4, stride: 0x0C, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE },
		VkVertexInputBindingDescription { binding: 5, stride: 0x0C, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE }
	].as_ptr(),
	vertexAttributeDescriptionCount: 18,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x00 },
		VkVertexInputAttributeDescription { location: 2, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x00 },
		VkVertexInputAttributeDescription { location: 3, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 },
		VkVertexInputAttributeDescription { location: 4, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 },
		VkVertexInputAttributeDescription { location: 5, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 },
		VkVertexInputAttributeDescription { location: 6, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 },
		
		VkVertexInputAttributeDescription { location: 16, binding: 2, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 17, binding: 3, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 18, binding: 4, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 19, binding: 5, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		
		VkVertexInputAttributeDescription { location: 20, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 21, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 22, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 23, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		
		VkVertexInputAttributeDescription { location: 24, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 25, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 26, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 27, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 }
	].as_ptr()
};

#[allow(dead_code)]
static PIPELINE_VERTEX_INPUT_STATE_UI_DYN_GRADIENT: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	vertexAttributeDescriptionCount: 20,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x0 },
		VkVertexInputAttributeDescription { location: 1, binding: 0, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 2, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 3, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 },
		VkVertexInputAttributeDescription { location: 4, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 },
		VkVertexInputAttributeDescription { location: 5, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 },
		VkVertexInputAttributeDescription { location: 6, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 },
		
		VkVertexInputAttributeDescription { location: 16, binding: 2, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 17, binding: 3, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 18, binding: 4, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 19, binding: 5, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		
		VkVertexInputAttributeDescription { location: 20, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 21, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 22, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 23, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		
		VkVertexInputAttributeDescription { location: 24, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 25, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 26, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 27, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 }
	].as_ptr(),
	..PIPELINE_VERTEX_INPUT_STATE_UI_DYN_BASIC
};

#[allow(dead_code)]
static PIPELINE_VERTEX_INPUT_STATE_UI_DYN_TEXTURED: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	vertexAttributeDescriptionCount: 19,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x0 },
		VkVertexInputAttributeDescription { location: 2, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 3, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 },
		VkVertexInputAttributeDescription { location: 4, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 },
		VkVertexInputAttributeDescription { location: 5, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 },
		VkVertexInputAttributeDescription { location: 6, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 },
		VkVertexInputAttributeDescription { location: 7, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x8 },
		
		VkVertexInputAttributeDescription { location: 16, binding: 2, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 17, binding: 3, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 18, binding: 4, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 19, binding: 5, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		
		VkVertexInputAttributeDescription { location: 20, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 21, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 22, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 23, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		
		VkVertexInputAttributeDescription { location: 24, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 25, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 26, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 27, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 }
	].as_ptr(),
	..PIPELINE_VERTEX_INPUT_STATE_UI_DYN_BASIC
};

#[allow(dead_code)]
static PIPELINE_VERTEX_INPUT_STATE_UI_DYN_TEXTURED_GRADIENT: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	vertexAttributeDescriptionCount: 8,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x0 },
		VkVertexInputAttributeDescription { location: 1, binding: 0, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 2, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 3, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 },
		VkVertexInputAttributeDescription { location: 4, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 },
		VkVertexInputAttributeDescription { location: 5, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 },
		VkVertexInputAttributeDescription { location: 6, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 },
		VkVertexInputAttributeDescription { location: 7, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x18 },
		
		VkVertexInputAttributeDescription { location: 16, binding: 2, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 17, binding: 3, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 18, binding: 4, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 19, binding: 5, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		
		VkVertexInputAttributeDescription { location: 20, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 21, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 22, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 23, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		
		VkVertexInputAttributeDescription { location: 24, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 25, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 26, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 27, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 }
	].as_ptr(),
	..PIPELINE_VERTEX_INPUT_STATE_UI_DYN_BASIC
};

#[allow(dead_code)]
static PIPELINE_VERTEX_INPUT_STATE_UI_DYN_SDF: VkPipelineVertexInputStateCreateInfo = VkPipelineVertexInputStateCreateInfo {
	vertexAttributeDescriptionCount: 7,
	pVertexAttributeDescriptions:    [
		VkVertexInputAttributeDescription { location: 0, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x0 },
		VkVertexInputAttributeDescription { location: 2, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 3, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x10 },
		VkVertexInputAttributeDescription { location: 4, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x20 },
		VkVertexInputAttributeDescription { location: 5, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x30 },
		VkVertexInputAttributeDescription { location: 6, binding: 1, format: VK_FORMAT_R32G32B32A32_SFLOAT, offset: 0x40 },
		VkVertexInputAttributeDescription { location: 7, binding: 0, format: VK_FORMAT_R32G32_SFLOAT,       offset: 0x8 },
		
		VkVertexInputAttributeDescription { location: 16, binding: 2, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 17, binding: 3, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 18, binding: 4, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		VkVertexInputAttributeDescription { location: 19, binding: 5, format: VK_FORMAT_R32_UINT, offset: 0x0 },
		
		VkVertexInputAttributeDescription { location: 20, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 21, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 22, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		VkVertexInputAttributeDescription { location: 23, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x4 },
		
		VkVertexInputAttributeDescription { location: 24, binding: 2, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 25, binding: 3, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 26, binding: 4, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 },
		VkVertexInputAttributeDescription { location: 27, binding: 5, format: VK_FORMAT_R32_SFLOAT, offset: 0x8 }
	].as_ptr(),
	..PIPELINE_VERTEX_INPUT_STATE_UI_DYN_BASIC
};