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
	crate::{DeviceRootContext, utils::*},
	std::{thread::JoinHandle, alloc::*, ptr::NonNull},
	vk::*,
	log::*
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct QueryData {
	pub input_assembly_vertices:                    u64,
	pub input_assembly_primitives:                  u64,
	pub vertex_shader_invocations:                  u64,
	pub geometry_shader_invocations:                u64,
	pub geometry_shader_primitives:                 u64,
	pub clipping_invocations:                       u64,
	pub clipping_primitives:                        u64,
	pub fragment_shader_invocations:                u64,
	pub tessellation_control_shader_patches:        u64,
	pub tessellation_evaluation_shader_invocations: u64,
	pub compute_shader_invocations:                 u64
}

impl QueryData {
	pub fn from_bytes(b: &[u8]) -> Self {
		debug_assert_eq!(b.len(), 88);
		Self {
			input_assembly_vertices:                    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
			input_assembly_primitives:                  u64::from_le_bytes([b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
			vertex_shader_invocations:                  u64::from_le_bytes([b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
			geometry_shader_invocations:                u64::from_le_bytes([b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]),
			geometry_shader_primitives:                 u64::from_le_bytes([b[32], b[33], b[34], b[35], b[36], b[37], b[38], b[39]]),
			clipping_invocations:                       u64::from_le_bytes([b[40], b[41], b[42], b[43], b[44], b[45], b[46], b[47]]),
			clipping_primitives:                        u64::from_le_bytes([b[48], b[49], b[50], b[51], b[52], b[53], b[54], b[55]]),
			fragment_shader_invocations:                u64::from_le_bytes([b[56], b[57], b[58], b[59], b[60], b[61], b[62], b[63]]),
			tessellation_control_shader_patches:        u64::from_le_bytes([b[64], b[65], b[66], b[67], b[68], b[69], b[70], b[71]]),
			tessellation_evaluation_shader_invocations: u64::from_le_bytes([b[72], b[73], b[74], b[75], b[76], b[77], b[78], b[79]]),
			compute_shader_invocations:                 u64::from_le_bytes([b[80], b[81], b[82], b[83], b[84], b[85], b[86], b[87]])
		}
	}
}

impl std::fmt::Display for QueryData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, r#"================= QUERY POOL RESULTS ===============
INPUT_ASSEMBLY_VERTICES:                    {:>8}
INPUT_ASSEMBLY_PRIMITIVES:                  {:>8}
VERTEX_SHADER_INVOCATIONS:                  {:>8}
GEOMETRY_SHADER_INVOCATIONS:                {:>8}
GEOMETRY_SHADER_PRIMITIVES:                 {:>8}
CLIPPING_INVOCATIONS:                       {:>8}
CLIPPING_PRIMITIVES:                        {:>8}
FRAGMENT_SHADER_INVOCATIONS:                {:>8}
TESSELLATION_CONTROL_SHADER_PATCHES:        {:>8}
TESSELLATION_EVALUATION_SHADER_INVOCATIONS: {:>8}
COMPUTE_SHADER_INVOCATIONS:                 {:>8}
====================================================="#,
				 self.input_assembly_vertices,
				 self.input_assembly_primitives,
				 self.vertex_shader_invocations,
				 self.geometry_shader_invocations,
				 self.geometry_shader_primitives,
				 self.clipping_invocations,
				 self.clipping_primitives,
				 self.fragment_shader_invocations,
				 self.tessellation_control_shader_patches,
				 self.tessellation_evaluation_shader_invocations,
				 self.compute_shader_invocations)
	}
}

/// Chooses a suitable memory type for the given parameters.
pub fn choose_memory_type(
	memory_properties: &VkPhysicalDeviceMemoryProperties,
	memory_type_bits:  u32,
	property_flags:    &[VkMemoryPropertyFlags]
) -> Option<u32> {
	let mut property_flags_idx = property_flags.len();
	memory_properties.memoryTypes
		.iter()
		.enumerate()
		.take(memory_properties.memoryTypeCount as usize)
		.filter(|(type_idx, _)| 1 << *type_idx as u32 & memory_type_bits != 0)
		.filter_map(|(type_idx, r#type)| property_flags
			.iter()
			.enumerate()
			.take(property_flags_idx)
			.find(|(_, flags)| r#type.propertyFlags == **flags)
			.map(|(flags_idx, _)| {
				property_flags_idx = flags_idx;
				type_idx
			}))
		.last()
		.map(|idx| idx as _)
}

/// Chooses a suitable memory type for the given parameters.
pub fn choose_memory_types(
	memory_properties: &VkPhysicalDeviceMemoryProperties,
	memory_type_bits:  u32,
	property_flags:    &[VkMemoryPropertyFlags]
) -> u32 {
	let mut property_flags_idx = property_flags.len();
	memory_properties.memoryTypes
		.iter()
		.enumerate()
		.take(memory_properties.memoryTypeCount as usize)
		.filter(|(type_idx, _)| 1 << *type_idx as u32 & memory_type_bits != 0)
		.filter_map(|(type_idx, r#type)| property_flags
			.iter()
			.enumerate()
			.take(property_flags_idx)
			.find(|(_, flags)| r#type.propertyFlags == **flags)
			.map(|(flags_idx, _)| {
				property_flags_idx = flags_idx;
				type_idx
			}))
		.fold(0, |bits, i| bits | (1 << i))
}

/// Chooses a suitable format for the given parameters.
pub fn choose_format(
	physical_device: &VkPhysicalDeviceImpl,
	properties:      VkFormatProperties,
	formats:         &[VkFormat]
) -> Option<VkFormat> {
	let mut props = VkFormatProperties2 {
		sType:            VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2,
		pNext:            None,
		formatProperties: VkFormatProperties::default()
	};
	
	formats.iter().copied().find(|format| {
		physical_device.getFormatProperties2(*format, &mut props);
		props.formatProperties.linearTilingFeatures & properties.linearTilingFeatures == properties.linearTilingFeatures
			&& props.formatProperties.optimalTilingFeatures & properties.optimalTilingFeatures == properties.optimalTilingFeatures
			&& props.formatProperties.bufferFeatures & properties.bufferFeatures == properties.bufferFeatures
	})
}

pub fn set_object_names(dbg: bool, device: &VkDeviceImpl, infos: &[(VkObjectType, u64, &str)]) -> VkResult {
	if !dbg { return VK_SUCCESS; }
	
	let mut info = VkDebugUtilsObjectNameInfoEXT {
		sType:        VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
		pNext:        None,
		..VkDebugUtilsObjectNameInfoEXT::default()
	};
	
	for (ty, handle, name) in infos {
		info.objectType = *ty;
		info.objectHandle = *handle;
		info.pObjectName = name.as_ptr();
		device.setDebugUtilsObjectNameEXT(&info)?;
	}
	
	VK_SUCCESS
}

pub fn spawn_transfer_thread<S: 'static + Send, T: 'static + Send>(device_ctx: &std::sync::Arc<DeviceRootContext<S, T>>) -> std::io::Result<JoinHandle<()>> where std::sync::Weak<DeviceRootContext<S, T>>: Send {
	let device_ctx_weak = std::sync::Arc::downgrade(device_ctx);
	let transfer_ready = device_ctx.transfer_ready.clone();
	std::thread::Builder::new()
		.name("gpgpu-transfer".to_string())
		.stack_size(0x1000)
		.spawn(move || loop {
			transfer_ready.wait();
			std::thread::sleep(std::time::Duration::from_millis(100));
			
			match device_ctx_weak.upgrade() {
				Some(device_ctx) => match device_ctx.transfer() {
					VK_SUCCESS => (),
					e => log::error!("[GPGPU] root device context #{:X}: transfer failed: {}", device_ctx.device.handle, e)
				},
				None => return
			}
		})
}

pub extern "C" fn messenger_callback(
	message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
	message_types:    VkDebugUtilsMessageTypeFlagsEXT,
	p_callback_data:  &VkDebugUtilsMessengerCallbackDataEXT,
	_p_user_data:     &u8
) -> VkBool32 {
	if std::thread::panicking() {
		return VK_FALSE;
	}
	
	unsafe {
		if ffi_str_eq(p_callback_data.pMessageIdName, "Loader Message\0".as_ptr()) {
			return VK_FALSE;
		}
		
		let obj = *p_callback_data.pObjects;
		let msg = format!(
			"{}{} [{:#x}]: {:?} [{:#x}] (`{}`): {}",
			{
				let mut buf = String::new();
				if message_types & VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT as u32 != 0 {
					buf.push_str("[GENERAL] ");
				}
				
				if message_types & VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT as u32 != 0 {
					buf.push_str("[PERFORMANCE] ");
				}
				
				if message_types & VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT as u32 != 0 {
					buf.push_str("[VALIDATION] ");
				}
				buf
			},
			str_convert(p_callback_data.pMessageIdName),
			p_callback_data.messageIdNumber,
			obj.objectType,
			obj.objectHandle,
			try_str_convert(obj.pObjectName).unwrap_or("null"),
			str_convert(p_callback_data.pMessage)
		);
		
		match message_severity {
			VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT => trace!("{}", msg),
			VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT    => info!("{}", msg),
			VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT => warn!("{}", msg),
			VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT   => {
				error!("{}", msg);
				panic!("{}", msg)
			},
		}
		
		VK_FALSE
	}
}

pub fn get_format_size(format: VkFormat) -> usize {
	match format {
		VK_FORMAT_R4G4_UNORM_PACK8                    => 1,
		VK_FORMAT_R4G4B4A4_UNORM_PACK16               => 2,
		VK_FORMAT_B4G4R4A4_UNORM_PACK16               => 2,
		VK_FORMAT_R5G6B5_UNORM_PACK16                 => 2,
		VK_FORMAT_B5G6R5_UNORM_PACK16                 => 2,
		VK_FORMAT_R5G5B5A1_UNORM_PACK16               => 2,
		VK_FORMAT_B5G5R5A1_UNORM_PACK16               => 2,
		VK_FORMAT_A1R5G5B5_UNORM_PACK16               => 2,
		VK_FORMAT_R8_UNORM                            => 1,
		VK_FORMAT_R8_SNORM                            => 1,
		VK_FORMAT_R8_USCALED                          => 1,
		VK_FORMAT_R8_SSCALED                          => 1,
		VK_FORMAT_R8_UINT                             => 1,
		VK_FORMAT_R8_SINT                             => 1,
		VK_FORMAT_R8_SRGB                             => 1,
		VK_FORMAT_R8G8_UNORM                          => 2,
		VK_FORMAT_R8G8_SNORM                          => 2,
		VK_FORMAT_R8G8_USCALED                        => 2,
		VK_FORMAT_R8G8_SSCALED                        => 2,
		VK_FORMAT_R8G8_UINT                           => 2,
		VK_FORMAT_R8G8_SINT                           => 2,
		VK_FORMAT_R8G8_SRGB                           => 2,
		VK_FORMAT_R8G8B8_UNORM                        => 3,
		VK_FORMAT_R8G8B8_SNORM                        => 3,
		VK_FORMAT_R8G8B8_USCALED                      => 3,
		VK_FORMAT_R8G8B8_SSCALED                      => 3,
		VK_FORMAT_R8G8B8_UINT                         => 3,
		VK_FORMAT_R8G8B8_SINT                         => 3,
		VK_FORMAT_R8G8B8_SRGB                         => 3,
		VK_FORMAT_B8G8R8_UNORM                        => 3,
		VK_FORMAT_B8G8R8_SNORM                        => 3,
		VK_FORMAT_B8G8R8_USCALED                      => 3,
		VK_FORMAT_B8G8R8_SSCALED                      => 3,
		VK_FORMAT_B8G8R8_UINT                         => 3,
		VK_FORMAT_B8G8R8_SINT                         => 3,
		VK_FORMAT_B8G8R8_SRGB                         => 3,
		VK_FORMAT_R8G8B8A8_UNORM                      => 4,
		VK_FORMAT_R8G8B8A8_SNORM                      => 4,
		VK_FORMAT_R8G8B8A8_USCALED                    => 4,
		VK_FORMAT_R8G8B8A8_SSCALED                    => 4,
		VK_FORMAT_R8G8B8A8_UINT                       => 4,
		VK_FORMAT_R8G8B8A8_SINT                       => 4,
		VK_FORMAT_R8G8B8A8_SRGB                       => 4,
		VK_FORMAT_B8G8R8A8_UNORM                      => 4,
		VK_FORMAT_B8G8R8A8_SNORM                      => 4,
		VK_FORMAT_B8G8R8A8_USCALED                    => 4,
		VK_FORMAT_B8G8R8A8_SSCALED                    => 4,
		VK_FORMAT_B8G8R8A8_UINT                       => 4,
		VK_FORMAT_B8G8R8A8_SINT                       => 4,
		VK_FORMAT_B8G8R8A8_SRGB                       => 4,
		VK_FORMAT_A8B8G8R8_UNORM_PACK32               => 4,
		VK_FORMAT_A8B8G8R8_SNORM_PACK32               => 4,
		VK_FORMAT_A8B8G8R8_USCALED_PACK32             => 4,
		VK_FORMAT_A8B8G8R8_SSCALED_PACK32             => 4,
		VK_FORMAT_A8B8G8R8_UINT_PACK32                => 4,
		VK_FORMAT_A8B8G8R8_SINT_PACK32                => 4,
		VK_FORMAT_A8B8G8R8_SRGB_PACK32                => 4,
		VK_FORMAT_A2R10G10B10_UNORM_PACK32            => 4,
		VK_FORMAT_A2R10G10B10_SNORM_PACK32            => 4,
		VK_FORMAT_A2R10G10B10_USCALED_PACK32          => 4,
		VK_FORMAT_A2R10G10B10_SSCALED_PACK32          => 4,
		VK_FORMAT_A2R10G10B10_UINT_PACK32             => 4,
		VK_FORMAT_A2R10G10B10_SINT_PACK32             => 4,
		VK_FORMAT_A2B10G10R10_UNORM_PACK32            => 4,
		VK_FORMAT_A2B10G10R10_SNORM_PACK32            => 4,
		VK_FORMAT_A2B10G10R10_USCALED_PACK32          => 4,
		VK_FORMAT_A2B10G10R10_SSCALED_PACK32          => 4,
		VK_FORMAT_A2B10G10R10_UINT_PACK32             => 4,
		VK_FORMAT_A2B10G10R10_SINT_PACK32             => 4,
		VK_FORMAT_R16_UNORM                           => 2,
		VK_FORMAT_R16_SNORM                           => 2,
		VK_FORMAT_R16_USCALED                         => 2,
		VK_FORMAT_R16_SSCALED                         => 2,
		VK_FORMAT_R16_UINT                            => 2,
		VK_FORMAT_R16_SINT                            => 2,
		VK_FORMAT_R16_SFLOAT                          => 2,
		VK_FORMAT_R16G16_UNORM                        => 4,
		VK_FORMAT_R16G16_SNORM                        => 4,
		VK_FORMAT_R16G16_USCALED                      => 4,
		VK_FORMAT_R16G16_SSCALED                      => 4,
		VK_FORMAT_R16G16_UINT                         => 4,
		VK_FORMAT_R16G16_SINT                         => 4,
		VK_FORMAT_R16G16_SFLOAT                       => 4,
		VK_FORMAT_R16G16B16_UNORM                     => 6,
		VK_FORMAT_R16G16B16_SNORM                     => 6,
		VK_FORMAT_R16G16B16_USCALED                   => 6,
		VK_FORMAT_R16G16B16_SSCALED                   => 6,
		VK_FORMAT_R16G16B16_UINT                      => 6,
		VK_FORMAT_R16G16B16_SINT                      => 6,
		VK_FORMAT_R16G16B16_SFLOAT                    => 6,
		VK_FORMAT_R16G16B16A16_UNORM                  => 8,
		VK_FORMAT_R16G16B16A16_SNORM                  => 8,
		VK_FORMAT_R16G16B16A16_USCALED                => 8,
		VK_FORMAT_R16G16B16A16_SSCALED                => 8,
		VK_FORMAT_R16G16B16A16_UINT                   => 8,
		VK_FORMAT_R16G16B16A16_SINT                   => 8,
		VK_FORMAT_R16G16B16A16_SFLOAT                 => 8,
		VK_FORMAT_R32_UINT                            => 4,
		VK_FORMAT_R32_SINT                            => 4,
		VK_FORMAT_R32_SFLOAT                          => 4,
		VK_FORMAT_R32G32_UINT                         => 8,
		VK_FORMAT_R32G32_SINT                         => 8,
		VK_FORMAT_R32G32_SFLOAT                       => 8,
		VK_FORMAT_R32G32B32_UINT                      => 12,
		VK_FORMAT_R32G32B32_SINT                      => 12,
		VK_FORMAT_R32G32B32_SFLOAT                    => 12,
		VK_FORMAT_R32G32B32A32_UINT                   => 16,
		VK_FORMAT_R32G32B32A32_SINT                   => 16,
		VK_FORMAT_R32G32B32A32_SFLOAT                 => 16,
		VK_FORMAT_R64_UINT                            => 8,
		VK_FORMAT_R64_SINT                            => 8,
		VK_FORMAT_R64_SFLOAT                          => 8,
		VK_FORMAT_R64G64_UINT                         => 16,
		VK_FORMAT_R64G64_SINT                         => 16,
		VK_FORMAT_R64G64_SFLOAT                       => 16,
		VK_FORMAT_R64G64B64_UINT                      => 24,
		VK_FORMAT_R64G64B64_SINT                      => 24,
		VK_FORMAT_R64G64B64_SFLOAT                    => 24,
		VK_FORMAT_R64G64B64A64_UINT                   => 32,
		VK_FORMAT_R64G64B64A64_SINT                   => 32,
		VK_FORMAT_R64G64B64A64_SFLOAT                 => 32,
		VK_FORMAT_B10G11R11_UFLOAT_PACK32             => 4,
		VK_FORMAT_E5B9G9R9_UFLOAT_PACK32              => 4,
		VK_FORMAT_D16_UNORM                           => 2,
		VK_FORMAT_X8_D24_UNORM_PACK32                 => 4,
		VK_FORMAT_D32_SFLOAT                          => 4,
		VK_FORMAT_S8_UINT                             => 1,
		VK_FORMAT_D16_UNORM_S8_UINT                   => 0,
		VK_FORMAT_D24_UNORM_S8_UINT                   => 0,
		VK_FORMAT_D32_SFLOAT_S8_UINT                  => 0,
		VK_FORMAT_BC1_RGB_UNORM_BLOCK                 => 16,
		VK_FORMAT_BC1_RGB_SRGB_BLOCK                  => 16,
		VK_FORMAT_BC1_RGBA_UNORM_BLOCK                => 16,
		VK_FORMAT_BC1_RGBA_SRGB_BLOCK                 => 16,
		VK_FORMAT_BC2_UNORM_BLOCK                     => 16,
		VK_FORMAT_BC2_SRGB_BLOCK                      => 16,
		VK_FORMAT_BC3_UNORM_BLOCK                     => 16,
		VK_FORMAT_BC3_SRGB_BLOCK                      => 16,
		VK_FORMAT_BC4_UNORM_BLOCK                     => 16,
		VK_FORMAT_BC4_SNORM_BLOCK                     => 16,
		VK_FORMAT_BC5_UNORM_BLOCK                     => 16,
		VK_FORMAT_BC5_SNORM_BLOCK                     => 16,
		VK_FORMAT_BC6H_UFLOAT_BLOCK                   => 16,
		VK_FORMAT_BC6H_SFLOAT_BLOCK                   => 16,
		VK_FORMAT_BC7_UNORM_BLOCK                     => 16,
		VK_FORMAT_BC7_SRGB_BLOCK                      => 16,
		VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK             => 0,
		VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK              => 0,
		VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK           => 0,
		VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK            => 0,
		VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK           => 0,
		VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK            => 0,
		VK_FORMAT_EAC_R11_UNORM_BLOCK                 => 0,
		VK_FORMAT_EAC_R11_SNORM_BLOCK                 => 0,
		VK_FORMAT_EAC_R11G11_UNORM_BLOCK              => 0,
		VK_FORMAT_EAC_R11G11_SNORM_BLOCK              => 0,
		VK_FORMAT_ASTC_4x4_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_4x4_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_5x4_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_5x4_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_5x5_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_5x5_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_6x5_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_6x5_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_6x6_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_6x6_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_8x5_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_8x5_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_8x6_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_8x6_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_8x8_UNORM_BLOCK                => 0,
		VK_FORMAT_ASTC_8x8_SRGB_BLOCK                 => 0,
		VK_FORMAT_ASTC_10x5_UNORM_BLOCK               => 0,
		VK_FORMAT_ASTC_10x5_SRGB_BLOCK                => 0,
		VK_FORMAT_ASTC_10x6_UNORM_BLOCK               => 0,
		VK_FORMAT_ASTC_10x6_SRGB_BLOCK                => 0,
		VK_FORMAT_ASTC_10x8_UNORM_BLOCK               => 0,
		VK_FORMAT_ASTC_10x8_SRGB_BLOCK                => 0,
		VK_FORMAT_ASTC_10x10_UNORM_BLOCK              => 0,
		VK_FORMAT_ASTC_10x10_SRGB_BLOCK               => 0,
		VK_FORMAT_ASTC_12x10_UNORM_BLOCK              => 0,
		VK_FORMAT_ASTC_12x10_SRGB_BLOCK               => 0,
		VK_FORMAT_ASTC_12x12_UNORM_BLOCK              => 0,
		VK_FORMAT_ASTC_12x12_SRGB_BLOCK               => 0,
		_ => 0
	}
}

#[derive(Debug, Default)]
pub struct AllocatorWithLayout<T: Allocator>  {
	pub inner:     T,
	pub min_align: usize
}

impl<T: Allocator> AllocatorWithLayout<T> {
	pub fn new(alloc: T) -> Self {
		Self { inner: alloc, min_align: 0 }
	}
	
	pub fn with_min_align(mut self, min_align: usize) -> Self {
		self.min_align = min_align;
		self
	}
	
	fn get_layout(&self, layout: Layout) -> Layout {
		unsafe { Layout::from_size_align_unchecked(
			layout.size(), layout.align().max(self.min_align)) }
	}
}

impl<T: Allocator + Clone> Clone for AllocatorWithLayout<T> {
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone(), min_align: self.min_align }
	}
}

impl<T: Allocator> std::ops::Deref for AllocatorWithLayout<T> {
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

unsafe impl<T: Allocator>  std::alloc::Allocator for AllocatorWithLayout<T> {
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		self.inner.allocate(self.get_layout(layout))
	}
	
	fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		self.inner.allocate_zeroed(self.get_layout(layout))
	}
	
	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
		self.inner.deallocate(ptr, self.get_layout(layout))
	}
	
	unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		self.inner.grow(ptr, self.get_layout(old_layout), self.get_layout(new_layout))
	}
	
	unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		self.inner.grow_zeroed(ptr, self.get_layout(old_layout), self.get_layout(new_layout))
	}
	
	unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		self.inner.shrink(ptr, self.get_layout(old_layout), self.get_layout(new_layout))
	}
}

pub mod compatibility {
	#![allow(non_snake_case)]
	use vk::*;
	use log::warn;
	
	pub fn vk10GetBufferMemoryRequirements(
		device:              &VkDeviceImpl,
		pInfo:               &VkBufferMemoryRequirementsInfo2,
		pMemoryRequirements: &mut VkMemoryRequirements2
	) {
		if pInfo.pNext.is_some() {
			warn!("[VK_COMPATIBILITY_LAYER] vk10GetBufferMemoryRequirements: pInfo.pNext was not null");
		}
		
		if pMemoryRequirements.pNext.is_some() {
			warn!("[VK_COMPATIBILITY_LAYER] vk10GetBufferMemoryRequirements: pMemoryRequirements.pNext was not null");
		}
		
		device.getBufferMemoryRequirements(
			pInfo.buffer,
			&mut pMemoryRequirements.memoryRequirements
		);
	}
	
	pub fn vk10GetImageMemoryRequirements(
		device:              &VkDeviceImpl,
		pInfo:               &VkImageMemoryRequirementsInfo2,
		pMemoryRequirements: &mut VkMemoryRequirements2
	) {
		if pInfo.pNext.is_some() {
			warn!("[VK_COMPATIBILITY_LAYER] vk10GetImageMemoryRequirements: pInfo.pNext was not null");
		}
		
		if pMemoryRequirements.pNext.is_some() {
			warn!("[VK_COMPATIBILITY_LAYER] vk10GetImageMemoryRequirements: pMemoryRequirements.pNext was not null");
		}
		
		device.getBufferMemoryRequirements(
			pInfo.image,
			&mut pMemoryRequirements.memoryRequirements
		);
	}
	
	pub fn vk10BindBufferMemory(
		device:     &VkDeviceImpl,
		pBindInfos: &[VkBindBufferMemoryInfo]
	) -> VkResult {
		for info in pBindInfos {
			device.bindBufferMemory(info.buffer, info.memory, info.memoryOffset)?;
		}
		VK_SUCCESS
	}
	
	pub fn vk10BindImageMemory(
		device:     &VkDeviceImpl,
		pBindInfos: &[VkBindImageMemoryInfo]
	) -> VkResult {
		for info in pBindInfos {
			device.bindImageMemory(info.image, info.memory, info.memoryOffset)?;
		}
		VK_SUCCESS
	}
}