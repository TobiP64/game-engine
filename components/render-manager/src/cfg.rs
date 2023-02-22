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

use vk::*;
use crate::DeviceProfile;

pub const PIPELINE_CACHE_PATH: &str = "./tmp/pipeline_cache";

#[cfg(feature = "renderdoc")]
pub const INSTANCE_LAYERS: [*const u8; 1] = ["VK_LAYER_RENDERDOC_Capture\0".as_ptr()];
#[cfg(all(not(feature = "renderdoc"), feature = "validation_layer"))]
pub const INSTANCE_LAYERS: [*const u8; 1] = ["VK_LAYER_KHRONOS_validation\0".as_ptr()];
#[cfg(all(not(feature = "renderdoc"), not(feature = "validation_layer")))]
pub const INSTANCE_LAYERS: &[*const u8] = &[];

#[cfg(all(not(feature = "renderdoc"), feature = "validation_layer"))]
pub const INSTANCE_EXTENSIONS: &[*const u8] = &[VK_EXT_DEBUG_UTILS_EXTENSION_NAME];
#[cfg(not(feature = "validation_layer"))]
pub const INSTANCE_EXTENSIONS: &[*const u8] = &[];

pub const DEFAULT_DEVICE_PROFILES: [DeviceProfile; 2] = [
	DeviceProfile {
		versions:            &[VK_API_VERSION_1_1],
		types:               &[VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU],
		features:            &DEFAULT_DEVICE_FEATURES,
		layers:              &[],
		extensions:          &[
			VK_KHR_SWAPCHAIN_EXTENSION_NAME,
			VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME
		],
		optional_features:   &DEFAULT_DEVICE_FEATURES_OPTIONAL,
		optional_layers:     &[],
		optional_extensions: &[
			VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME,
			VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME,
			VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME
		],
		queues:              &[
			&[(VK_QUEUE_COMPUTE_BIT as u32 | VK_QUEUE_GRAPHICS_BIT as u32
				   | VK_QUEUE_SPARSE_BINDING_BIT as u32 | 0x20, &[0f32])]
		]
	},
	DeviceProfile {
		versions:            &[VK_API_VERSION_1_0],
		types:               &[VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU],
		features:            &DEFAULT_DEVICE_FEATURES,
		layers:              &[],
		extensions:          &[
			VK_KHR_SWAPCHAIN_EXTENSION_NAME,
			VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME
		],
		optional_features:   &DEFAULT_DEVICE_FEATURES_OPTIONAL,
		optional_layers:     &[],
		optional_extensions: &[
			VK_KHR_BIND_MEMORY_2_EXTENSION_NAME,
			VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME,
			VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME,
			VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME,
			VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME
		],
		queues:              &[
			&[(VK_QUEUE_COMPUTE_BIT as u32 | VK_QUEUE_GRAPHICS_BIT as u32
				   | VK_QUEUE_SPARSE_BINDING_BIT as u32 | 0x20, &[0f32])]
		]
	}
];

const DEFAULT_DEVICE_FEATURES: VkPhysicalDeviceFeatures2 = VkPhysicalDeviceFeatures2 {
	sType:    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
	pNext:    None,
	features: VkPhysicalDeviceFeatures {
		robustBufferAccess:                      VK_FALSE,
		fullDrawIndexUint32:                     VK_FALSE,
		imageCubeArray:                          VK_FALSE,
		independentBlend:                        VK_FALSE,
		geometryShader:                          VK_TRUE,
		tessellationShader:                      VK_FALSE,
		sampleRateShading:                       VK_FALSE,
		dualSrcBlend:                            VK_FALSE,
		logicOp:                                 VK_FALSE,
		multiDrawIndirect:                       VK_TRUE,
		drawIndirectFirstInstance:               VK_TRUE,
		depthClamp:                              VK_FALSE,
		depthBiasClamp:                          VK_FALSE,
		fillModeNonSolid:                        VK_FALSE,
		depthBounds:                             VK_FALSE,
		wideLines:                               VK_FALSE,
		largePoints:                             VK_FALSE,
		alphaToOne:                              VK_FALSE,
		multiViewport:                           VK_FALSE,
		samplerAnisotropy:                       VK_TRUE,
		textureCompressionETC2:                  VK_FALSE,
		textureCompressionASTC_LDR:              VK_FALSE,
		textureCompressionBC:                    VK_TRUE,
		occlusionQueryPrecise:                   VK_FALSE,
		pipelineStatisticsQuery:                 VK_TRUE,
		vertexPipelineStoresAndAtomics:          VK_FALSE,
		fragmentStoresAndAtomics:                VK_FALSE,
		shaderTessellationAndGeometryPointSize:  VK_FALSE,
		shaderImageGatherExtended:               VK_FALSE,
		shaderStorageImageExtendedFormats:       VK_TRUE,
		shaderStorageImageMultisample:           VK_FALSE,
		shaderStorageImageReadWithoutFormat:     VK_FALSE,
		shaderStorageImageWriteWithoutFormat:    VK_TRUE,
		shaderUniformBufferArrayDynamicIndexing: VK_FALSE,
		shaderSampledImageArrayDynamicIndexing:  VK_FALSE,
		shaderStorageBufferArrayDynamicIndexing: VK_FALSE,
		shaderStorageImageArrayDynamicIndexing:  VK_FALSE,
		shaderClipDistance:                      VK_FALSE,
		shaderCullDistance:                      VK_FALSE,
		shaderFloat64:                           VK_FALSE,
		shaderInt64:                             VK_FALSE,
		shaderInt16:                             VK_FALSE,
		shaderResourceResidency:                 VK_FALSE,
		shaderResourceMinLod:                    VK_FALSE,
		sparseBinding:                           VK_TRUE,
		sparseResidencyBuffer:                   VK_FALSE,
		sparseResidencyImage2D:                  VK_FALSE,
		sparseResidencyImage3D:                  VK_FALSE,
		sparseResidency2Samples:                 VK_FALSE,
		sparseResidency4Samples:                 VK_FALSE,
		sparseResidency8Samples:                 VK_FALSE,
		sparseResidency16Samples:                VK_FALSE,
		sparseResidencyAliased:                  VK_FALSE,
		variableMultisampleRate:                 VK_FALSE,
		inheritedQueries:                        VK_FALSE
	}
};

const DEFAULT_DEVICE_FEATURES_OPTIONAL: VkPhysicalDeviceFeatures2 = VkPhysicalDeviceFeatures2 {
	sType:    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
	pNext:    None,
	features: VkPhysicalDeviceFeatures {
		robustBufferAccess:                      VK_FALSE,
		fullDrawIndexUint32:                     VK_FALSE,
		imageCubeArray:                          VK_FALSE,
		independentBlend:                        VK_FALSE,
		geometryShader:                          VK_TRUE,
		tessellationShader:                      VK_FALSE,
		sampleRateShading:                       VK_FALSE,
		dualSrcBlend:                            VK_FALSE,
		logicOp:                                 VK_FALSE,
		multiDrawIndirect:                       VK_FALSE,
		drawIndirectFirstInstance:               VK_TRUE,
		depthClamp:                              VK_FALSE,
		depthBiasClamp:                          VK_FALSE,
		fillModeNonSolid:                        VK_FALSE,
		depthBounds:                             VK_FALSE,
		wideLines:                               VK_FALSE,
		largePoints:                             VK_FALSE,
		alphaToOne:                              VK_FALSE,
		multiViewport:                           VK_FALSE,
		samplerAnisotropy:                       VK_TRUE,
		textureCompressionETC2:                  VK_TRUE,
		textureCompressionASTC_LDR:              VK_TRUE,
		textureCompressionBC:                    VK_TRUE,
		occlusionQueryPrecise:                   VK_FALSE,
		pipelineStatisticsQuery:                 VK_FALSE,
		vertexPipelineStoresAndAtomics:          VK_FALSE,
		fragmentStoresAndAtomics:                VK_FALSE,
		shaderTessellationAndGeometryPointSize:  VK_FALSE,
		shaderImageGatherExtended:               VK_FALSE,
		shaderStorageImageExtendedFormats:       VK_TRUE,
		shaderStorageImageMultisample:           VK_FALSE,
		shaderStorageImageReadWithoutFormat:     VK_FALSE,
		shaderStorageImageWriteWithoutFormat:    VK_TRUE,
		shaderUniformBufferArrayDynamicIndexing: VK_FALSE,
		shaderSampledImageArrayDynamicIndexing:  VK_FALSE,
		shaderStorageBufferArrayDynamicIndexing: VK_FALSE,
		shaderStorageImageArrayDynamicIndexing:  VK_FALSE,
		shaderClipDistance:                      VK_FALSE,
		shaderCullDistance:                      VK_FALSE,
		shaderFloat64:                           VK_FALSE,
		shaderInt64:                             VK_FALSE,
		shaderInt16:                             VK_FALSE,
		shaderResourceResidency:                 VK_FALSE,
		shaderResourceMinLod:                    VK_FALSE,
		sparseBinding:                           VK_TRUE,
		sparseResidencyBuffer:                   VK_FALSE,
		sparseResidencyImage2D:                  VK_FALSE,
		sparseResidencyImage3D:                  VK_FALSE,
		sparseResidency2Samples:                 VK_FALSE,
		sparseResidency4Samples:                 VK_FALSE,
		sparseResidency8Samples:                 VK_FALSE,
		sparseResidency16Samples:                VK_FALSE,
		sparseResidencyAliased:                  VK_FALSE,
		variableMultisampleRate:                 VK_FALSE,
		inheritedQueries:                        VK_FALSE
	}
};

pub const DEFAULT_CFG: Config = Config {
	device_profiles:     &DEFAULT_DEVICE_PROFILES,
	allocator:           None,
	application_info:    VkApplicationInfo {
		sType:              VK_STRUCTURE_TYPE_APPLICATION_INFO,
		pNext:              None,
		pApplicationName:   "Arcturos Game Engine Test Application\0".as_ptr(),
		applicationVersion: VK_MAKE_VERSION(0, 0, 1),
		pEngineName:        "Arcturos Game Engine\0".as_ptr(),
		engineVersion:      VK_MAKE_VERSION(0, 0, 1),
		apiVersion:         VK_API_VERSION_1_1
	},
	debug_msg_flags:           VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT as u32
		| VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT as u32,
	local_memory_block_size:   0x1000_0000, // 256MB
	local_buffer_size:         0x0100_0000,
	dynamic_memory_block_size: 0x1000_0000,  // 16MB
	dynamic_buffer_size:       0x0010_0000,
	staging_buffer_size:       0x0010_0000,
	pipeline_cache_path:       "./tmp/pipeline_cache"
};

#[derive(Debug)]
pub struct Config {
	pub device_profiles:           &'static [DeviceProfile<'static>],
	pub allocator:                 Option<&'static VkAllocationCallbacks<'static>>,
	pub application_info:          VkApplicationInfo<'static>,
	pub debug_msg_flags:           u32,
	pub local_memory_block_size:   VkDeviceSize,
	pub local_buffer_size:         VkDeviceSize,
	pub dynamic_memory_block_size: VkDeviceSize,
	pub dynamic_buffer_size:       VkDeviceSize,
	pub staging_buffer_size:       VkDeviceSize,
	pub pipeline_cache_path:       &'static str
}