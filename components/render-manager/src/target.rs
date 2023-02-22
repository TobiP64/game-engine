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
	crate::{*, render_graph::*, plugins::*, misc::*},
	std::{ptr, sync::Arc, lazy::SyncOnceCell},
	vk::*
};

pub trait Target {
	/// Creates a `VkSurfaceKHR` from this `Target`.
	fn create_surface(
		&self,
		instance:  &vk::VkInstanceImpl,
		next:      Option<VkAnyRef>,
		allocator: Option<&vk::VkAllocationCallbacks>,
		surface:   &mut VkSurfaceKHR
	) -> VkResult;
}

impl<D: AsRef<wayland::WlDisplay>, S: AsRef<wayland::WlSurface>> Target for (D, S) {
	fn create_surface(
		&self,
		instance:  &VkInstanceImpl,
		next:      Option<VkAnyRef>,
		allocator: Option<&VkAllocationCallbacks>,
		surface:   &mut VkSurfaceKHR,
	) -> VkResult {
		let (display, wl_surface) = *self;
		instance.createWaylandSurfaceKHR(&VkWaylandSurfaceCreateInfoKHR {
			sType:   VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
			pNext:   next,
			flags:   0,
			display: unsafe { external::wl_display::from_ptr(display.as_ref() as *const _ as *mut wayland::WlDisplay).unwrap() },
			surface: unsafe { external::wl_surface::from_ptr(wl_surface.as_ref() as *const _ as *mut wayland::WlSurface).unwrap() },
		}, allocator, surface)
	}
}

#[derive(Debug)]
pub struct TargetRootContext<S: 'static, T: 'static> {
	pub device_ctx:                Arc<DeviceRootContext<S, T>>,
	pub contexts:                  SyncOnceCell<Vec<Box<dyn TargetContext<T>>>>,
	pub render_graph:              RenderGraph<S, T>,
	pub semaphore_image_available: VkSemaphore,
	pub semaphore_render_finished: VkSemaphore,
	pub surface:                   VkSurfaceKHR,
	pub swapchain:                 VkSwapchainKHR,
	pub extent:                    VkExtent2D,
	pub images:                    Vec<SwapchainImage>,
	pub query_pool:                VkQueryPool
}

#[derive(Debug)]
pub struct SwapchainImage {
	pub image:       VkImage,
	pub image_view:  VkImageView,
	pub framebuffer: VkFramebuffer,
	pub cmd_buffer:  VkCommandBufferImpl
}

impl<S: 'static, T: 'static + Target> TargetRootContext<S, T> {
	pub fn new<'a>(
		device_ctx:   Arc<DeviceRootContext<S, T>>,
		scene_ctx:    Arc<SceneRootContext<S, T>>,
		render_graph: RenderGraph<S, T>,
		target:       T,
		surface:      Option<VkSurfaceKHR>,
		extent:       VkExtent2D,
		plugins:      impl IntoIterator<Item = &'a dyn PluginRootContext<S, T>>
	) -> Result<Arc<Self>, VkResult> {
		let surface  = match surface {
			Some(v) => v,
			None => {
				let mut surface = VK_NULL_HANDLE;
				target.create_surface(&device_ctx.instance, None, device_ctx.allocator, &mut surface)?;
				surface
			}
		};
		
		let semaphore_info = VkSemaphoreCreateInfo {
			sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
			pNext: None,
			flags: 0
		};
		
		let mut semaphore_image_available = VK_NULL_HANDLE;
		let mut semaphore_render_finished = VK_NULL_HANDLE;
		device_ctx.device.createSemaphore(&semaphore_info, device_ctx.allocator, &mut semaphore_image_available)?;
		device_ctx.device.createSemaphore(&semaphore_info, device_ctx.allocator, &mut semaphore_render_finished)?;
		
		let mut query_pool = VK_NULL_HANDLE;
		device_ctx.device.createQueryPool(&VkQueryPoolCreateInfo {
			sType:              VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
			pNext:              None,
			flags:              0,
			queryType:          VK_QUERY_TYPE_PIPELINE_STATISTICS,
			queryCount:         1,
			pipelineStatistics: 0x7FF
		}, device_ctx.allocator, &mut query_pool)?;
		
		set_object_names(device_ctx.instance_info.vk12_debug_utils, &device_ctx.device, &[
			(VK_OBJECT_TYPE_SEMAPHORE, semaphore_image_available, "semaphore_image_available\0"),
			(VK_OBJECT_TYPE_SEMAPHORE, semaphore_render_finished, "semaphore_render_finished\0")
		])?;
		
		let mut self_ = Arc::new(Self {
			extent:            VkExtent2D::default(),
			semaphore_image_available,
			semaphore_render_finished,
			surface,
			swapchain:         VK_NULL_HANDLE,
			images:            Vec::new(),
			query_pool,
			device_ctx,
			render_graph,
			contexts:          SyncOnceCell::new()
		});
		
		let contexts = plugins.into_iter()
			.map(|plugin| plugin.create_target_context(&self_))
			.collect::<Result<Vec<_>, _>>()
			.expect("failed to create device context");
		self_.contexts.set(contexts)
			.expect("failed to set contexts");
		
		self_.update(extent)?;
		log::debug!("[GPGPU] root surface context #{:X}: created", surface);
		Ok(self_)
	}
	
	pub fn update(&mut self, extent: VkExtent2D) -> VkResult {
		let DeviceRootContext {
			allocator,
			physical_device,
			device,
			device_info,
			primary_queue_family,
			primary_cmd_pool,
			..
		}: &DeviceRootContext<S, T> = &*self.device_ctx;
		let allocator = *allocator;
		
		//
		// SWAPCHAIN
		//
		
		let surface_caps = if device_info.vk12_get_surface_capabilities_2 {
			let info = VkPhysicalDeviceSurfaceInfo2KHR {
				sType:   VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
				pNext:   None,
				surface: self.surface
			};
			
			let mut capabilities = VkSurfaceCapabilities2KHR {
				sType:               VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
				pNext:               None,
				surfaceCapabilities: VkSurfaceCapabilitiesKHR::default()
			};
			
			physical_device.getSurfaceCapabilities2KHR(&info, &mut capabilities)?;
			capabilities.surfaceCapabilities
		} else {
			let mut capabilities = VkSurfaceCapabilitiesKHR::default();
			physical_device.getSurfaceCapabilitiesKHR(self.surface, &mut capabilities)?;
			capabilities
		};
		
		let format = 'outer0: {
			let mut format_count = 0;
			physical_device.getSurfaceFormatsKHR(self.surface, &mut format_count, None)?;
			let mut formats = vec![Default::default(); format_count as _];
			physical_device.getSurfaceFormatsKHR(self.surface, &mut format_count, Some(&mut formats))?;
			
			for format in formats.iter() {
				if let VkSurfaceFormatKHR {
					format: VK_FORMAT_A2B10G10R10_UNORM_PACK32 | VK_FORMAT_B8G8R8A8_UNORM,
					colorSpace: VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
				} = format { break 'outer0 *format; }
			}
			
			log::error!("[GPGPU] root surface context #{:X}: failed to find suitable surface format", self.surface);
			return VK_ERROR_INITIALIZATION_FAILED;
		};
		
		let old_swapchain = self.swapchain;
		let swapchain_info = VkSwapchainCreateInfoKHR {
			sType:                 VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
			pNext:                 None,
			flags:                 0,
			surface:               self.surface,
			minImageCount:         surface_caps.minImageCount,
			imageFormat:           format.format,
			imageColorSpace:       format.colorSpace,
			imageExtent:           match surface_caps.currentExtent {
				VkExtent2D { width: 0xFFFF_FFFF, height: 0xFFFF_FFFF } => VkExtent2D {
					width:  extent.width.clamp(
						surface_caps.minImageExtent.width, surface_caps.maxImageExtent.width),
					height: extent.height.clamp(
						surface_caps.minImageExtent.height, surface_caps.maxImageExtent.height)
				},
				extent => extent
			},
			imageArrayLayers:      1,
			imageUsage:            VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32 | VK_IMAGE_USAGE_STORAGE_BIT as u32 | VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32,
			imageSharingMode:      VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 1,
			pQueueFamilyIndices:   primary_queue_family,
			preTransform:          surface_caps.currentTransform,
			compositeAlpha:        VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
			presentMode:           {
				let mut mode_count = 0;
				physical_device.getSurfacePresentModesKHR(self.surface, &mut mode_count, None)?;
				let mut present_modes = vec![VK_PRESENT_MODE_IMMEDIATE_KHR; mode_count as _];
				physical_device.getSurfacePresentModesKHR(self.surface, &mut mode_count, Some(&mut present_modes))?;
				
				if present_modes.contains(&VK_PRESENT_MODE_MAILBOX_KHR) {
					VK_PRESENT_MODE_MAILBOX_KHR
				} else {
					VK_PRESENT_MODE_FIFO_KHR
				}
			},
			clipped:               VK_TRUE,
			oldSwapchain:          old_swapchain
		};
		
		self.extent = swapchain_info.imageExtent;
		device.createSwapchainKHR(&swapchain_info, allocator, &mut self.swapchain)?;
		
		if old_swapchain != VK_NULL_HANDLE {
			device.destroySwapchainKHR(old_swapchain, allocator);
		}
		
		//
		// SWAPCHAIN IMAGE RESOURCES
		//
		
		let mut image_view_info = VkImageViewCreateInfo {
			sType:      VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			pNext:      None,
			flags:      0,
			image:      VK_NULL_HANDLE,
			viewType:   VK_IMAGE_VIEW_TYPE_2D,
			format:     swapchain_info.imageFormat,
			components: VkComponentMapping {
				r: VK_COMPONENT_SWIZZLE_IDENTITY,
				g: VK_COMPONENT_SWIZZLE_IDENTITY,
				b: VK_COMPONENT_SWIZZLE_IDENTITY,
				a: VK_COMPONENT_SWIZZLE_IDENTITY
			},
			subresourceRange: VkImageSubresourceRange {
				aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as u32,
				baseMipLevel:   0,
				levelCount:     1,
				baseArrayLayer: 0,
				layerCount:     1
			}
		};
		
		let mut framebuffer_info_attachments = [VK_NULL_HANDLE];
		let framebuffer_info = VkFramebufferCreateInfo {
			sType:           VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
			pNext:           None,
			flags:           0,
			renderPass:      self.render_pass,
			attachmentCount: framebuffer_info_attachments.len() as _,
			pAttachments:    framebuffer_info_attachments.as_ptr(),
			width:           swapchain_info.imageExtent.width,
			height:          swapchain_info.imageExtent.height,
			layers:          1
		};
		
		let mut image_count = 0;
		device.getSwapchainImagesKHR(self.swapchain, &mut image_count, None)?;
		let mut images_ = vec![Default::default(); image_count as _];
		device.getSwapchainImagesKHR(self.swapchain, &mut image_count, Some(&mut images_))?;
		
		let cmd_buffer_alloc_info = VkCommandBufferAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
			pNext:              None,
			commandPool:        *primary_cmd_pool,
			level:              VK_COMMAND_BUFFER_LEVEL_PRIMARY,
			commandBufferCount: image_count
		};
		
		let mut command_buffers = vec![Default::default(); cmd_buffer_alloc_info.commandBufferCount as _];
		device.allocateCommandBuffers(&cmd_buffer_alloc_info, &mut command_buffers)?;
		
		for SwapchainImage { image_view, framebuffer, cmd_buffer, .. } in self.images.drain(..) {
			device.destroyFramebuffer(framebuffer, allocator);
			device.destroyImageView(image_view, allocator);
			device.freeCommandBuffers(*primary_cmd_pool, &[cmd_buffer.handle]);
		}
		
		for (i, &image) in images_.iter().enumerate() {
			image_view_info.image = image;
			let mut image_view = VK_NULL_HANDLE;
			device.createImageView(&image_view_info, allocator, &mut image_view)?;
			
			framebuffer_info_attachments[0] = image_view;
			let mut framebuffer = VK_NULL_HANDLE;
			device.createFramebuffer(&framebuffer_info, allocator, &mut framebuffer)?;
			
			self.images.push(SwapchainImage {
				image,
				image_view,
				framebuffer,
				cmd_buffer: VkCommandBufferImpl::new(command_buffers[i], device)
			});
		}
		
		//
		// DEBUG
		//
		
		set_object_names(self.device_ctx.instance_info.vk12_debug_utils, &self.device_ctx.device, &[
			(VK_OBJECT_TYPE_SWAPCHAIN_KHR, self.swapchain, "swapchain\0")
		]);
		
		for (i, SwapchainImage { image, image_view, framebuffer, cmd_buffer, .. }) in self.images.iter().enumerate() {
			set_object_names(self.device_ctx.instance_info.vk12_debug_utils, &self.device_ctx.device, &[
				(VK_OBJECT_TYPE_IMAGE, *image, &format!("swapchain_image_{}\0", i)),
				(VK_OBJECT_TYPE_IMAGE_VIEW, *image_view, &format!("swapchain_image_view_{}\0", i)),
				(VK_OBJECT_TYPE_FRAMEBUFFER, *framebuffer, &format!("swapchain_framebuffer_{}\0", i)),
				(VK_OBJECT_TYPE_COMMAND_BUFFER, cmd_buffer.handle, &format!("swapchain_cmd_buffer_{}\0", i))
			]);
		}
		
		VK_SUCCESS
	}
	
	/*pub fn update(&mut self, surface_extent: VkExtent2D) -> VkResult {
		let DeviceRootContext {
			allocator,
			physical_device,
			device,
			device_info,
			primary_queue_family: queue_family,
			primary_cmd_pool: command_pool,
			..
		}: &DeviceRootContext<S, T> = &*self.device_ctx;
		let allocator = *allocator;
		
		//
		// SWAPCHAIN
		//
		
		let surface_caps = if device_info.vk12_get_surface_capabilities_2 {
			let info = VkPhysicalDeviceSurfaceInfo2KHR {
				sType:   VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
				pNext:   None,
				surface: self.surface
			};
			
			let mut capabilities = VkSurfaceCapabilities2KHR {
				sType:               VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
				pNext:               None,
				surfaceCapabilities: VkSurfaceCapabilitiesKHR::default()
			};
			
			physical_device.getSurfaceCapabilities2KHR(&info, &mut capabilities)?;
			capabilities.surfaceCapabilities
		} else {
			let mut capabilities = VkSurfaceCapabilitiesKHR::default();
			physical_device.getSurfaceCapabilitiesKHR(self.surface, &mut capabilities)?;
			capabilities
		};
		
		let format = 'outer0: {
			let mut format_count = 0;
			physical_device.getSurfaceFormatsKHR(self.surface, &mut format_count, None)?;
			let mut formats = vec![Default::default(); format_count as _];
			physical_device.getSurfaceFormatsKHR(self.surface, &mut format_count, Some(&mut formats))?;
			
			for format in formats.iter() {
				if let VkSurfaceFormatKHR {
					format: VK_FORMAT_A2B10G10R10_UNORM_PACK32 | VK_FORMAT_B8G8R8A8_UNORM,
					colorSpace: VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
				} = format { break 'outer0 *format; }
			}
			
			log::error!("[GPGPU] root surface context #{:X}: failed to find suitable surface format", self.surface);
			return VK_ERROR_INITIALIZATION_FAILED;
		};
		
		let old_swapchain = self.swapchain;
		let swapchain_info = VkSwapchainCreateInfoKHR {
			sType:                 VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
			pNext:                 None,
			flags:                 0,
			surface:               self.surface,
			minImageCount:         surface_caps.minImageCount,
			imageFormat:           format.format,
			imageColorSpace:       format.colorSpace,
			imageExtent:           match surface_caps.currentExtent {
				VkExtent2D { width: 0xFFFF_FFFF, height: 0xFFFF_FFFF } => VkExtent2D {
					width:  surface_extent.width.clamp(
						surface_caps.minImageExtent.width, surface_caps.maxImageExtent.width),
					height: surface_extent.height.clamp(
						surface_caps.minImageExtent.height, surface_caps.maxImageExtent.height)
				},
				extent => extent
			},
			imageArrayLayers:      1,
			imageUsage:            VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32 | VK_IMAGE_USAGE_STORAGE_BIT as u32 | VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32,
			imageSharingMode:      VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 1,
			pQueueFamilyIndices:   queue_family,
			preTransform:          surface_caps.currentTransform,
			compositeAlpha:        VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
			presentMode:           {
				let mut mode_count = 0;
				physical_device.getSurfacePresentModesKHR(self.surface, &mut mode_count, None)?;
				let mut present_modes = vec![VK_PRESENT_MODE_IMMEDIATE_KHR; mode_count as _];
				physical_device.getSurfacePresentModesKHR(self.surface, &mut mode_count, Some(&mut present_modes))?;
				
				if present_modes.contains(&VK_PRESENT_MODE_MAILBOX_KHR) {
					VK_PRESENT_MODE_MAILBOX_KHR
				} else {
					VK_PRESENT_MODE_FIFO_KHR
				}
			},
			clipped:               VK_TRUE,
			oldSwapchain:          old_swapchain
		};
		
		self.extent = swapchain_info.imageExtent;
		device.createSwapchainKHR(&swapchain_info, allocator, &mut self.swapchain)?;
		
		if old_swapchain != VK_NULL_HANDLE {
			device.destroySwapchainKHR(old_swapchain, allocator);
		}
		
		//
		// RENDER PASS
		//
		
		if self.render_pass != VK_NULL_HANDLE {
			device.destroyRenderPass(self.render_pass, allocator);
		}
		
		if device_info.vk11_create_renderpass_2 {
			device.createRenderPass2KHR(&VkRenderPassCreateInfo2KHR  {
				sType:                   VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR,
				pNext:                   None,
				flags:                   0,
				attachmentCount:         1,
				pAttachments:            [
					VkAttachmentDescription2KHR {
						sType:          VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR,
						pNext:          None,
						flags:          0,
						format:         swapchain_info.imageFormat,
						samples:        VK_SAMPLE_COUNT_1_BIT,
						loadOp:         VK_ATTACHMENT_LOAD_OP_CLEAR,
						storeOp:        VK_ATTACHMENT_STORE_OP_STORE,
						stencilLoadOp:  VK_ATTACHMENT_LOAD_OP_CLEAR,
						stencilStoreOp: VK_ATTACHMENT_STORE_OP_STORE,
						initialLayout:  VK_IMAGE_LAYOUT_UNDEFINED,
						finalLayout:    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR
					}
				].as_ptr(),
				subpassCount:            1,
				pSubpasses:              [
					VkSubpassDescription2KHR {
						sType:                   VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR,
						pNext:                   None,
						flags:                   0,
						pipelineBindPoint:       VK_PIPELINE_BIND_POINT_GRAPHICS,
						viewMask:                0,
						inputAttachmentCount:    0,
						pInputAttachments:       ptr::null(),
						colorAttachmentCount:    1,
						pColorAttachments:       [
							VkAttachmentReference2KHR {
								sType:      VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR,
								pNext:      None,
								attachment: 0,
								layout:     VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
								aspectMask: VK_IMAGE_ASPECT_COLOR_BIT as u32 | VK_IMAGE_ASPECT_DEPTH_BIT as u32
							}
						].as_ptr(),
						pResolveAttachments:     ptr::null(),
						pDepthStencilAttachment: None,
						preserveAttachmentCount: 0,
						pPreserveAttachments:    ptr::null()
					}
				].as_ptr(),
				dependencyCount:         1,
				pDependencies:           [
					VkSubpassDependency2KHR {
						sType:           VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR,
						pNext:           None,
						srcSubpass:      VK_SUBPASS_EXTERNAL,
						dstSubpass:      0,
						srcStageMask:    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32,
						dstStageMask:    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32,
						srcAccessMask:   0,
						dstAccessMask:   VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as u32 | VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32,
						dependencyFlags: 0,
						viewOffset:      0
					}
				].as_ptr(),
				correlatedViewMaskCount: 0,
				pCorrelatedViewMasks:    ptr::null()
			}, allocator, &mut self.render_pass)?;
		} else {
			device.createRenderPass(&VkRenderPassCreateInfo {
				sType:           VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
				pNext:           None,
				flags:           0,
				attachmentCount: 1,
				pAttachments:    [
					VkAttachmentDescription {
						flags:          0,
						format:         swapchain_info.imageFormat,
						samples:        VK_SAMPLE_COUNT_1_BIT,
						loadOp:         VK_ATTACHMENT_LOAD_OP_DONT_CARE,
						storeOp:        VK_ATTACHMENT_STORE_OP_DONT_CARE,
						stencilLoadOp:  VK_ATTACHMENT_LOAD_OP_DONT_CARE,
						stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
						initialLayout:  VK_IMAGE_LAYOUT_UNDEFINED,
						finalLayout:    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR
					}
				].as_ptr(),
				subpassCount:    1,
				pSubpasses:      [
					VkSubpassDescription {
						flags:                   0,
						pipelineBindPoint:       VK_PIPELINE_BIND_POINT_GRAPHICS,
						inputAttachmentCount:    0,
						pInputAttachments:       ptr::null(),
						colorAttachmentCount:    1,
						pColorAttachments:       [
							VkAttachmentReference {
								attachment: 0,
								layout:     VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
							}
						].as_ptr(),
						pResolveAttachments:     ptr::null(),
						pDepthStencilAttachment: None,
						preserveAttachmentCount: 0,
						pPreserveAttachments:    ptr::null()
					}
				].as_ptr(),
				dependencyCount: 1,
				pDependencies:   [
					VkSubpassDependency {
						srcSubpass:      VK_SUBPASS_EXTERNAL,
						dstSubpass:      0,
						srcStageMask:    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32,
						dstStageMask:    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32,
						srcAccessMask:   0,
						dstAccessMask:   VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as u32 | VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32,
						dependencyFlags: 0,
					}
				].as_ptr()
			}, allocator, &mut self.render_pass)?;
		}
		
		//
		// SWAPCHAIN IMAGE RESOURCES
		//
		
		let mut image_view_info = VkImageViewCreateInfo {
			sType:      VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			pNext:      None,
			flags:      0,
			image:      VK_NULL_HANDLE,
			viewType:   VK_IMAGE_VIEW_TYPE_2D,
			format:     swapchain_info.imageFormat,
			components: VkComponentMapping {
				r: VK_COMPONENT_SWIZZLE_IDENTITY,
				g: VK_COMPONENT_SWIZZLE_IDENTITY,
				b: VK_COMPONENT_SWIZZLE_IDENTITY,
				a: VK_COMPONENT_SWIZZLE_IDENTITY
			},
			subresourceRange: VkImageSubresourceRange {
				aspectMask:     VK_IMAGE_ASPECT_COLOR_BIT as u32,
				baseMipLevel:   0,
				levelCount:     1,
				baseArrayLayer: 0,
				layerCount:     1
			}
		};
		
		let mut framebuffer_info_attachments = [VK_NULL_HANDLE];
		let framebuffer_info = VkFramebufferCreateInfo {
			sType:           VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
			pNext:           None,
			flags:           0,
			renderPass:      self.render_pass,
			attachmentCount: framebuffer_info_attachments.len() as _,
			pAttachments:    framebuffer_info_attachments.as_ptr(),
			width:           swapchain_info.imageExtent.width,
			height:          swapchain_info.imageExtent.height,
			layers:          1
		};
		
		let mut image_count = 0;
		device.getSwapchainImagesKHR(self.swapchain, &mut image_count, None)?;
		let mut images_ = vec![Default::default(); image_count as _];
		device.getSwapchainImagesKHR(self.swapchain, &mut image_count, Some(&mut images_))?;
		
		let cmd_buffer_alloc_info = VkCommandBufferAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
			pNext:              None,
			commandPool:        *command_pool,
			level:              VK_COMMAND_BUFFER_LEVEL_PRIMARY,
			commandBufferCount: image_count
		};
		
		let mut command_buffers = vec![Default::default(); cmd_buffer_alloc_info.commandBufferCount as _];
		device.allocateCommandBuffers(&cmd_buffer_alloc_info, &mut command_buffers)?;
		
		for SwapchainImage { image_view, framebuffer, cmd_buffer, .. } in self.images.drain(..) {
			device.destroyFramebuffer(framebuffer, allocator);
			device.destroyImageView(image_view, allocator);
			device.freeCommandBuffers(*command_pool, &[cmd_buffer.handle]);
		}
		
		for (i, &image) in images_.iter().enumerate() {
			image_view_info.image = image;
			let mut image_view = VK_NULL_HANDLE;
			device.createImageView(&image_view_info, allocator, &mut image_view)?;
			
			framebuffer_info_attachments[0] = image_view;
			let mut framebuffer = VK_NULL_HANDLE;
			device.createFramebuffer(&framebuffer_info, allocator, &mut framebuffer)?;
			
			self.images.push(SwapchainImage {
				image,
				image_view,
				framebuffer,
				cmd_buffer: VkCommandBufferImpl::new(command_buffers[i], device)
			});
		}
		
		//
		// PIPELINES
		//
		
		let view_ports = [
			VkViewport {
				x:        0.0,
				y:        0.0,
				width:    swapchain_info.imageExtent.width as _,
				height:   swapchain_info.imageExtent.height as _,
				minDepth: 0.0,
				maxDepth: 1.0
			}
		];
		
		let scissors = [
			VkRect2D {
				offset: VkOffset2D { x: 0, y: 0 },
				extent: swapchain_info.imageExtent
			}
		];
		
		let pipeline_template = VkGraphicsPipelineCreateInfo {
			sType:               VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
			pNext:               None,
			flags:               0,
			stageCount:          0,
			pStages:             ptr::null(),
			pVertexInputState:   None,
			pInputAssemblyState: Some(&VkPipelineInputAssemblyStateCreateInfo {
				sType:                  VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
				pNext:                  None,
				flags:                  0,
				topology:               VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
				primitiveRestartEnable: VK_FALSE
			}),
			pTessellationState:  None,
			pViewportState:      Some(&VkPipelineViewportStateCreateInfo {
				sType:         VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
				pNext:         None,
				flags:         0,
				viewportCount: view_ports.len() as _,
				pViewports:    view_ports.as_ptr(),
				scissorCount:  scissors.len() as _,
				pScissors:     scissors.as_ptr()
			}),
			pRasterizationState: &VkPipelineRasterizationStateCreateInfo {
				sType:                   VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
				pNext:                   None,
				flags:                   0,
				depthClampEnable:        VK_FALSE,
				rasterizerDiscardEnable: VK_FALSE,
				polygonMode:             VK_POLYGON_MODE_FILL,
				cullMode:                VK_CULL_MODE_NONE as _,
				frontFace:               VK_FRONT_FACE_COUNTER_CLOCKWISE,
				depthBiasEnable:         VK_FALSE,
				depthBiasConstantFactor: 0.0,
				depthBiasClamp:          0.0,
				depthBiasSlopeFactor:    0.0,
				lineWidth:               1.0
			},
			pMultisampleState:   Some(&VkPipelineMultisampleStateCreateInfo {
				sType:                 VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
				pNext:                 None,
				flags:                 0,
				rasterizationSamples:  VK_SAMPLE_COUNT_1_BIT,
				sampleShadingEnable:   VK_FALSE,
				minSampleShading:      0.0,
				pSampleMask:           ptr::null(),
				alphaToCoverageEnable: VK_FALSE,
				alphaToOneEnable:      VK_FALSE
			}),
			pDepthStencilState:  None,
			pColorBlendState:    Some(&VkPipelineColorBlendStateCreateInfo {
				sType:           VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
				pNext:           None,
				flags:           0,
				logicOpEnable:   0,
				logicOp:         VK_LOGIC_OP_CLEAR,
				attachmentCount: 1,
				pAttachments:    [
					VkPipelineColorBlendAttachmentState {
						blendEnable:         VK_TRUE,
						srcColorBlendFactor: VK_BLEND_FACTOR_SRC_ALPHA,
						dstColorBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
						colorBlendOp:        VK_BLEND_OP_ADD,
						srcAlphaBlendFactor: VK_BLEND_FACTOR_SRC_ALPHA,
						dstAlphaBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
						alphaBlendOp:        VK_BLEND_OP_ADD,
						colorWriteMask:      0xF
					}
				].as_ptr(),
				blendConstants: [0.0, 0.0, 0.0, 0.0]
			}),
			pDynamicState:       None,
			layout:              VK_NULL_HANDLE,
			renderPass:          self.render_pass,
			subpass:             0,
			basePipelineHandle:  VK_NULL_HANDLE,
			basePipelineIndex:   !0
		};
		
		let self__ = unsafe { (self as *const Self).as_ref().unwrap() };
		let contexts = self.device_ctx.contexts.get().unwrap();
		
		for i in 0..self.device_ctx.systems.len() {
			self.device_ctx.systems[i].target_ctx_update(
				&mut *self.contexts[i],
				&*contexts[i],
				&self.device_ctx,
				self__,
				&pipeline_template
			)?;
		}
		
		//
		// DEBUG
		//
		
		set_object_names(self.device_ctx.instance_info.vk12_debug_utils, &self.device_ctx.device, &[
			(VK_OBJECT_TYPE_SWAPCHAIN_KHR, self.swapchain, "swapchain\0"),
			(VK_OBJECT_TYPE_RENDER_PASS, self.render_pass, "render_pass\0")
		]);
		
		for (i, SwapchainImage { image, image_view, framebuffer, cmd_buffer, .. }) in self.images.iter().enumerate() {
			set_object_names(self.device_ctx.instance_info.vk12_debug_utils, &self.device_ctx.device, &[
				(VK_OBJECT_TYPE_IMAGE, *image, &format!("swapchain_image_{}\0", i)),
				(VK_OBJECT_TYPE_IMAGE_VIEW, *image_view, &format!("swapchain_image_view_{}\0", i)),
				(VK_OBJECT_TYPE_FRAMEBUFFER, *framebuffer, &format!("swapchain_framebuffer_{}\0", i)),
				(VK_OBJECT_TYPE_COMMAND_BUFFER, cmd_buffer.handle, &format!("swapchain_cmd_buffer_{}\0", i))
			]);
		}
		
		VK_SUCCESS
	}
	
	pub fn record(&mut self) -> Result<(), VkResult> {
		let DeviceRootContext {
			//instance_extensions,
			device_info,
			..
		} = &*self.device_ctx;
		
		self.update_cmd_buffer.reset(0)?;
		self.update_cmd_buffer.begin(&VkCommandBufferBeginInfo {
			sType:            VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
			pNext:            None,
			flags:            0,
			pInheritanceInfo: None
		})?;
		self.update_cmd_buffer.end();
		
		let mut render_pass_begin_info = VkRenderPassBeginInfo {
			sType:           VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
			pNext:           None,
			renderPass:      self.render_pass,
			framebuffer:     VK_NULL_HANDLE,
			renderArea:      VkRect2D { offset: VkOffset2D { x: 0, y: 0 }, extent: self.extent },
			clearValueCount: 1,
			pClearValues:    [
				VkClearValue { color: VkClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } }
			].as_ptr()
		};
		
		for (i, SwapchainImage { framebuffer, cmd_buffer, .. }) in self.images.iter().enumerate() {
			cmd_buffer.reset(0)?;
			cmd_buffer.begin(&VkCommandBufferBeginInfo {
				sType:            VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
				pNext:            None,
				flags:            0,
				pInheritanceInfo: None
			})?;
			
			render_pass_begin_info.framebuffer = *framebuffer;
			cmd_buffer.cmdResetQueryPool(self.query_pool, 0, 1);
			
			if device_info.vk11_create_renderpass_2 {
				cmd_buffer.cmdBeginRenderPass2KHR(&render_pass_begin_info, &VkSubpassBeginInfoKHR {
					sType:    VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR,
					pNext:    None,
					contents: VK_SUBPASS_CONTENTS_INLINE
				});
			} else {
				cmd_buffer.cmdBeginRenderPass(&render_pass_begin_info, VK_SUBPASS_CONTENTS_INLINE);
			}
			
			cmd_buffer.cmdBeginQuery(self.query_pool, 0, 0);
			
			let contexts = self.device_ctx.contexts.get().unwrap();
			#[allow(clippy::needless_range_loop)]
			for j in 0..self.device_ctx.systems.len() {
				self.device_ctx.systems[j].target_ctx_record(
					&*self.contexts[j],
					&*contexts[j],
					&*self.scene_ctx.contexts[j],
					&self.device_ctx,
					&self.scene_ctx.scene,
					self,
					cmd_buffer,
					InvocationId::new(0, 0, i)
				);
			}
			
			cmd_buffer.cmdEndQuery(self.query_pool, 0);
			
			if device_info.vk11_create_renderpass_2 {
				cmd_buffer.cmdEndRenderPass2KHR(&VkSubpassEndInfoKHR {
					sType: VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR,
					pNext: None
				});
			} else {
				cmd_buffer.cmdEndRenderPass();
			}
			
			cmd_buffer.end();
		}
		
		Ok(())
	}*/
	
	pub fn submit(&mut self) -> Result<(), VkResult> {
		let Self {
			device_ctx,
			semaphore_image_available,
			semaphore_render_finished,
			swapchain,
			images,
			query_pool,
			..
		} = self;
		
		let DeviceRootContext {
			device,
			primary_queue,
			..
		}: &DeviceRootContext<S, T> = &*device_ctx;
		
		primary_queue.waitIdle()?;
		let mut image_index = !0;
		device.acquireNextImage2KHR(&VkAcquireNextImageInfoKHR {
			sType:      VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR,
			pNext:      None,
			swapchain:  *swapchain,
			timeout:    std::u64::MAX,
			semaphore:  *semaphore_image_available,
			fence:      VK_NULL_HANDLE,
			deviceMask: 1
		}, &mut image_index)?;
		
		primary_queue.submit(&[
			VkSubmitInfo {
				sType:                VK_STRUCTURE_TYPE_SUBMIT_INFO,
				pNext:                None,
				waitSemaphoreCount:   1,
				pWaitSemaphores:      [
					//*semaphore_update_finished,
					*semaphore_image_available
				].as_ptr(),
				pWaitDstStageMask:    [
					//VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as u32,
					VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32
				].as_ptr(),
				commandBufferCount:   1,
				pCommandBuffers:      &images[image_index as usize].cmd_buffer.handle,
				signalSemaphoreCount: 1,
				pSignalSemaphores:    semaphore_render_finished
			}
		], VK_NULL_HANDLE)?;
		
		primary_queue.presentKHR(&VkPresentInfoKHR {
			sType:              VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
			pNext:              None,
			waitSemaphoreCount: 1,
			pWaitSemaphores:    semaphore_render_finished,
			swapchainCount:     1,
			pSwapchains:        swapchain,
			pImageIndices:      &image_index,
			pResults:           ptr::null_mut()
		})?;
		
		let err = primary_queue.waitIdle()?;
		
		if cfg!(feature = "trace-performance") {
			let mut data = [0u8; 88];
			device_ctx.device.getQueryPoolResults(
				*query_pool,
				0,
				1,
				&mut data,
				std::mem::size_of::<u64>() as _,
				VK_QUERY_RESULT_64_BIT as u32
			)?;
			
			log::info!("{}\x1b[13F", QueryData::from_bytes(&data));
		}
		
		match err {
			VK_SUBOPTIMAL_KHR => Err(VK_SUBOPTIMAL_KHR),
			_ => Ok(())
		}
	}
	
	pub fn get_ctx<C: std::any::Any>(&self) -> Option<&C> {
		self.contexts.get()
			.expect("contexts not initialized")
			.iter()
			.find_map(|v| v.as_any().downcast_ref::<C>())
	}
	
	pub fn framebuffers<'a>(&'a self) -> impl Iterator<Item = VkFramebuffer> + 'a {
		self.images.iter().map(|v| v.framebuffer)
	}
}

impl<S: 'static, T: 'static> std::ops::Drop for TargetRootContext<S, T> {
	fn drop(&mut self) {
		if std::thread::panicking() {
			log::warn!("[GPGPU] root surface context {:#X}: aborting destruction due to panic", self.surface);
			return;
		}
		
		for context in self.contexts.get().unwrap() {
			context.destroy().unwrap();
		}
		
		self.device_ctx.primary_queue.waitIdle();
		
		for SwapchainImage { image_view, framebuffer, cmd_buffer, .. } in &self.images {
			self.device_ctx.device.freeCommandBuffers(self.device_ctx.primary_cmd_pool, &[cmd_buffer.handle]);
			self.device_ctx.device.destroyFramebuffer(*framebuffer, self.device_ctx.allocator);
			self.device_ctx.device.destroyImageView(*image_view, self.device_ctx.allocator);
		}
		
		self.device_ctx.device.destroySwapchainKHR(self.swapchain, self.device_ctx.allocator);
		self.device_ctx.device.destroyQueryPool(self.query_pool, self.device_ctx.allocator);
		self.device_ctx.device.destroySemaphore(self.semaphore_image_available, self.device_ctx.allocator);
		self.device_ctx.device.destroySemaphore(self.semaphore_render_finished, self.device_ctx.allocator);
		self.device_ctx.instance.destroySurfaceKHR(self.surface, self.device_ctx.allocator);
		log::debug!("[GPGPU] root surface context {:#X}: destroyed", self.surface);
	}
}
