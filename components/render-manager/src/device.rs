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
	crate::{*, mem::*, cmd, plugins::*, cfg::*, utils::*, misc::*},
	std::{ptr, sync::*, lazy::SyncOnceCell},
	custom_sync::AsyncCondvar,
	vk::*
};

#[derive(Debug)]
pub struct InstanceInfo {
	pub version:                               u32,
	pub vk10_get_physical_device_properties_2: bool,
	pub vk12_debug_utils:                      bool
}

#[derive(Debug)]
pub struct DeviceInfo {
	pub vk10_features:                       VkPhysicalDeviceFeatures,
	pub vk10_properties:                     VkPhysicalDeviceProperties,
	pub vk10_memory_properties:              VkPhysicalDeviceMemoryProperties,
	pub vk10_bind_memory_2:                  bool,
	pub vk10_get_memory_requirements_2:      bool,
	pub vk11_features:                       VkPhysicalDeviceVulkan11Features<'static>,
	pub vk11_properties:                     VkPhysicalDeviceVulkan11Properties<'static>,
	pub vk11_draw_indirect_count:            bool,
	pub vk11_create_renderpass_2:            bool,
	pub vk11_descriptor_indexing:            bool,
	pub vk11_descriptor_indexing_properties: VkPhysicalDeviceDescriptorIndexingProperties<'static>,
	pub vk11_descriptor_indexing_features:   VkPhysicalDeviceDescriptorIndexingFeatures<'static>,
	pub vk12_features:                       VkPhysicalDeviceVulkan12Features<'static>,
	pub vk12_properties:                     VkPhysicalDeviceVulkan12Properties<'static>,
	pub vk12_get_surface_capabilities_2:     bool,
	pub layers:                              Vec<*const u8>,
	pub extensions:                          Vec<*const u8>,
}

#[derive(Debug)]
pub struct DeviceRootContext<S: 'static, T: 'static> {
	pub instance:                   VkInstanceImpl,
	pub instance_info:              InstanceInfo,
	pub allocator:                  Option<&'static VkAllocationCallbacks<'static>>,
	pub dbg_messenger:              VkDebugUtilsMessengerEXT,
	pub physical_device:            VkPhysicalDeviceImpl,
	pub device:                     VkDeviceImpl,
	pub device_info:                DeviceInfo,
	pub primary_queue_family:       u32,
	pub primary_queue:              VkQueueImpl,
	pub primary_cmd_pool:           VkCommandPool,
	pub async_compute_queue_family: u32,
	pub async_compute_queue:        VkQueueImpl,
	pub async_compute_cmd_pool:     VkCommandPool,
	pub transfer_queue_family:      u32,
	pub transfer_queue:             VkQueueImpl,
	pub transfer_cmd_pool:          VkCommandPool,
	pub transfer_cmd_buffer:        VkCommandBufferImpl,
	pub transfer_cmds:              Mutex<cmd::Cmds>,
	pub transfer_fence:             VkFence,
	pub transfer_wait:              AsyncCondvar,
	pub transfer_ready:             Arc<AsyncCondvar>,
	pub pipeline_cache:             VkPipelineCache,
	pub desc_pool:                  VkDescriptorPool,
	pub alloc:                      Arc<Heaps>,
	pub transfer_buffer:            VkBuffer,
	pub transfer_buffer_alloc:      MappedSubAlloc,
	pub dynamic_buffer:             VkBuffer,
	pub dynamic_buffer_alloc:       MappedSubAlloc,
	pub local_buffer:               VkBuffer,
	pub local_buffer_alloc:         Box<dyn RemoteAllocRef + Send + Sync>,
	pub contexts:                   SyncOnceCell<Vec<Box<dyn DeviceContext<S, T>>>>
}

impl<S: 'static, T: 'static> DeviceRootContext<S, T> {
	#[allow(clippy::cast_ptr_alignment)]
	pub fn create<'a>(
		cfg:             &Config,
		targets:         &[&dyn target::Target],
		surfaces:        &mut [VkSurfaceKHR],
		window_sys_exts: &[*const u8],
		plugins:         impl IntoIterator<Item = &'a dyn PluginRootContext<S, T>>
	) -> Result<Arc<Self>, VkResult> {
		let allocator = cfg.allocator;
		let t = std::time::Instant::now();
		
		//
		// instance --------------------------------------------------------------------------------
		//
		
		let instance_version = {
			let mut version = 0;
			VkInstanceImpl::enumerateVersion(&mut version)?;
			if VK_VERSION_MAJOR(version) < VK_VERSION_MAJOR(cfg.application_info.apiVersion)
				|| VK_VERSION_MINOR(version) < VK_VERSION_MINOR(cfg.application_info.apiVersion)
				|| VK_VERSION_PATCH(version) < VK_VERSION_PATCH(cfg.application_info.apiVersion) {
				panic!("unsupported instance version: {}.{}.{}, required: {}.{}.{} or higher",
					   VK_VERSION_MAJOR(version), VK_VERSION_MINOR(version), VK_VERSION_PATCH(version),
					   VK_VERSION_MAJOR(cfg.application_info.apiVersion),
					   VK_VERSION_MINOR(cfg.application_info.apiVersion),
					   VK_VERSION_PATCH(cfg.application_info.apiVersion));
			}
			version
		};
		
		let instance_layers = {
			let mut layer_count = 0u32;
			VkInstanceImpl::enumerateLayerProperties(&mut layer_count, None)?;
			let mut layer_props = vec![Default::default(); layer_count as _];
			VkInstanceImpl::enumerateLayerProperties(&mut layer_count, Some(&mut layer_props))?;
			
			'outer0: for required_layer in INSTANCE_LAYERS.iter() {
				for layer in layer_props.iter() {
					if ffi_str_eq(layer.layerName.as_ptr(), *required_layer) { continue 'outer0; }
				}
				log::warn!("instance layer not available: {}", string_convert(*required_layer));
			}
			
			INSTANCE_LAYERS.to_vec().into_boxed_slice()
		};
		
		let instance_extensions = {
			let mut ext_count = 0u32;
			VkInstanceImpl::enumerateExtensionProperties(None, &mut ext_count, None)?;
			let mut ext_props = vec![Default::default(); ext_count as _];
			VkInstanceImpl::enumerateExtensionProperties(None, &mut ext_count, Some(&mut ext_props))?;
			
			let mut required_extensions = Vec::from(INSTANCE_EXTENSIONS);
			required_extensions.extend_from_slice(window_sys_exts);
			
			'outer1: for required_ext in required_extensions.iter() {
				for extension in ext_props.iter() {
					if ffi_str_eq(extension.extensionName.as_ptr(), *required_ext) { continue 'outer1; }
				}
				log::warn!("instance extension not available: {}", string_convert(*required_ext))
			}
			
			required_extensions.into_boxed_slice()
		};
		
		let instance_info = InstanceInfo {
			version: instance_version,
			vk10_get_physical_device_properties_2: instance_extensions
				.contains_ffi_str(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME),
			vk12_debug_utils: instance_extensions
				.contains_ffi_str(VK_EXT_DEBUG_UTILS_EXTENSION_NAME)
		};
		
		let messenger_info = VkDebugUtilsMessengerCreateInfoEXT {
			sType:           VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
			pNext:           None,
			flags:           0,
			messageSeverity: cfg.debug_msg_flags,
			messageType:     VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT as u32
				| VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT as u32
				| VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT as u32,
			pfnUserCallback: messenger_callback,
			pUserData:       None
		};
		
		let mut instance = VK_NULL_HANDLE;
		VkInstanceImpl::create(&VkInstanceCreateInfo {
			sType:            VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			pNext:            instance_info.vk12_debug_utils.then_some((&messenger_info).into()),
			flags:            0,
			pApplicationInfo: Some(&VkApplicationInfo {
				apiVersion: instance_version,
				..cfg.application_info
			}),
			enabledLayerCount:       instance_layers.len() as _,
			ppEnabledLayerNames:     instance_layers.as_ptr(),
			enabledExtensionCount:   instance_extensions.len() as _,
			ppEnabledExtensionNames: instance_extensions.as_ptr()
		}, allocator, &mut instance)?;
		let instance = VkInstanceImpl::new(instance);
		
		let mut dbg_messenger = VK_NULL_HANDLE;
		if instance_info.vk12_debug_utils {
			instance.createDebugUtilsMessengerEXT(&messenger_info, allocator, &mut dbg_messenger)?;
		}
		
		//
		// surfaces --------------------------------------------------------------------------------
		//
		
		for i in 0..targets.len() {
			targets[i].create_surface(&instance, None, allocator, &mut surfaces[i]);
		}
		
		//
		// device ----------------------------------------------------------------------------------
		//
		
		let mut dev = create_device(
			&instance,
			cfg.device_profiles,
			surfaces.iter().copied().collect::<Vec<_>>().as_slice(),
			allocator
		)?.expect("failed to find any suitable device");
		
		let physical_device      = dev.physical_device;
		let device               = dev.device;
		let primary_queue_family = dev.queues[0].0;
		let primary_queue        = dev.queues[0].1.remove(0);
		
		let mut features_descriptor_indexing = VkPhysicalDeviceDescriptorIndexingFeaturesEXT {
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT,
			pNext: None,
			.. VkPhysicalDeviceDescriptorIndexingFeaturesEXT::default()
		};
		
		let mut features = VkPhysicalDeviceFeatures2 {
			sType:    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
			pNext:    Some((&mut features_descriptor_indexing).into()),
			features: VkPhysicalDeviceFeatures::default()
		};
		
		physical_device.getFeatures2(&mut features);
		
		let mut properties_descriptor_indexing = VkPhysicalDeviceDescriptorIndexingPropertiesEXT {
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT,
			pNext: None,
			.. VkPhysicalDeviceDescriptorIndexingPropertiesEXT::default()
		};
		
		let mut properties = VkPhysicalDeviceProperties2 {
			sType:      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2,
			pNext:      Some((&mut properties_descriptor_indexing).into()),
			properties: VkPhysicalDeviceProperties::default()
		};
		
		physical_device.getProperties2(&mut properties);
		
		let mut memory_props = VkPhysicalDeviceMemoryProperties2 {
			sType:            VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
			pNext:            None,
			memoryProperties: VkPhysicalDeviceMemoryProperties::default()
		};
		
		physical_device.getMemoryProperties2(&mut memory_props);
		
		let device_info = DeviceInfo {
			vk10_features:                       features.features,
			vk10_properties:                     properties.properties,
			vk10_memory_properties:              memory_props.memoryProperties,
			vk10_bind_memory_2:                  properties.properties.apiVersion > VK_API_VERSION_1_0 || dev.extensions.contains_ffi_str(VK_KHR_BIND_MEMORY_2_EXTENSION_NAME),
			vk10_get_memory_requirements_2:      properties.properties.apiVersion > VK_API_VERSION_1_0 || dev.extensions.contains_ffi_str(VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME),
			vk11_features:                       Default::default(),
			vk11_properties:                     Default::default(),
			vk11_draw_indirect_count:            properties.properties.apiVersion > VK_API_VERSION_1_1 || dev.extensions.contains_ffi_str(VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME),
			vk11_create_renderpass_2:            properties.properties.apiVersion > VK_API_VERSION_1_1 || dev.extensions.contains_ffi_str(VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME),
			vk11_descriptor_indexing:            properties.properties.apiVersion > VK_API_VERSION_1_1 || dev.extensions.contains_ffi_str(VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME),
			vk11_descriptor_indexing_properties: properties_descriptor_indexing,
			vk11_descriptor_indexing_features:   features_descriptor_indexing,
			vk12_features:                       Default::default(),
			vk12_properties:                     Default::default(),
			vk12_get_surface_capabilities_2:     dev.extensions.contains_ffi_str(VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME),
			layers:                              dev.layers,
			extensions:                          dev.extensions
		};
		
		if !device_info.vk10_bind_memory_2 {
			log::warn!("[GPGPU] extension VK_KHR_bind_memory2 not supported, falling back to Vulkan Core 1.0 equivalent");
		}
		
		if !device_info.vk10_get_memory_requirements_2 {
			log::warn!("[GPGPU] extension VK_KHR_get_memory_requirements2 not supported, falling back to Vulkan Core 1.0 equivalent");
		}
		
		if !device_info.vk11_create_renderpass_2 {
			log::warn!("[GPGPU] extension VK_KHR_create_renderpass2 not supported, falling back to Vulkan Core 1.1 equivalent");
		}
		
		if !device_info.vk12_get_surface_capabilities_2 {
			log::warn!("[GPGPU] extension VK_KHR_get_surface_capabilities2 not supported, falling back to Vulkan Core 1.2 equivalent");
		}
		
		//
		// resources -------------------------------------------------------------------------------
		//
		
		// CMD POOL
		
		let mut primary_cmd_pool = VK_NULL_HANDLE;
		device.createCommandPool(&VkCommandPoolCreateInfo {
			sType:            VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
			pNext:            None,
			flags:            VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32,
			queueFamilyIndex: primary_queue_family
		}, allocator, &mut primary_cmd_pool)?;
		
		let mut cmd_buffer_transfer = [VK_NULL_HANDLE];
		device.allocateCommandBuffers(&VkCommandBufferAllocateInfo {
			sType:              VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
			pNext:              None,
			commandPool: primary_cmd_pool,
			level:              VK_COMMAND_BUFFER_LEVEL_PRIMARY,
			commandBufferCount: 1
		}, &mut cmd_buffer_transfer);
		let transfer_cmd_buffer = VkCommandBufferImpl::new(cmd_buffer_transfer[0], &device);
		
		// PIPELINE CACHE
		
		let cache_data = std::fs::read(cfg.pipeline_cache_path);
		let mut pipeline_cache = VK_NULL_HANDLE;
		device.createPipelineCache(&VkPipelineCacheCreateInfo {
			sType:           VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
			pNext:           None,
			flags:           0,
			initialDataSize: cache_data.as_ref().map_or(0, Vec::len),
			pInitialData:    cache_data.as_ref().map_or(ptr::null(), |v| v.as_ptr() as _)
		}, allocator, &mut pipeline_cache)?;
		
		// DESCRIPTOR POOL
		
		let mut desc_pool = VK_NULL_HANDLE;
		device.createDescriptorPool(&VkDescriptorPoolCreateInfo {
			sType:          VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
			pNext:          None,
			flags:          VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT as u32
				| VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT as u32,
			maxSets:        64,
			poolSizeCount:  5,
			pPoolSizes:     [
				VkDescriptorPoolSize {
					r#type:          VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
					descriptorCount: 64
				},
				VkDescriptorPoolSize {
					r#type:          VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
					descriptorCount: 64
				},
				VkDescriptorPoolSize {
					r#type:          VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC,
					descriptorCount: 64
				},
				VkDescriptorPoolSize {
					r#type:          VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
					descriptorCount: 64
				},
				VkDescriptorPoolSize {
					r#type:          VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
					descriptorCount: 64
				}
			].as_ptr()
		}, allocator, &mut desc_pool)?;
		
		// ALLOCATOR
		
		let alloc = Arc::new(Heaps::create(
			&device,
			memory_props.memoryProperties,
			cfg.dynamic_memory_block_size,
			&HeapsExtensionInfo {
				version:                   instance_version,
				bind_memory_2:             device_info.vk10_bind_memory_2,
				get_memory_requirements_2: device_info.vk10_get_memory_requirements_2,
				memory_priority:           false,
				memory_budget:             false
			}
		));
		
		// TRANSFER BUFFER
		
		let mut transfer_buffer = VK_NULL_HANDLE;
		
		device.createBuffer(&VkBufferCreateInfo {
			sType:                 VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
			pNext:                 None,
			flags:                 0,
			size:                  cfg.staging_buffer_size,
			usage:                 VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32
				| VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32
				| VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as u32,
			sharingMode:           VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices:   ptr::null()
		}, allocator, &mut transfer_buffer);
		
		alloc.alloc_dedicated(
			&memory_props.memoryProperties,
			&MEMORY_PROPERTY_FLAGS_DYNAMIC_PREFERRED,
			allocator,
			&[transfer_buffer],
			&[]
		)?;
		
		let transfer_buffer_alloc = MappedSubAlloc::new(
			GpaSubAlloc::new(&alloc, transfer_buffer),
			alloc.map_region(transfer_buffer, 0).unwrap()
		);
		//let transfer_buffer_alloc_sprs = GpaSparseSubAlloc::new(alloc.clone(), transfer_buffer, true);
		
		transfer_cmd_buffer.begin(&VkCommandBufferBeginInfo {
			sType:            VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
			pNext:            None,
			flags:            VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT as _,
			pInheritanceInfo: None
		})?;
		
		let mut transfer_fence = VK_NULL_HANDLE;
		device.createFence(&VkFenceCreateInfo {
			sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
			pNext: None,
			flags: 0
		}, allocator, &mut transfer_fence)?;
		
		// DYNAMIC BUFFER
		
		let mut dynamic_buffer = VK_NULL_HANDLE;
		device.createBuffer(&VkBufferCreateInfo {
			sType:                 VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
			pNext:                 None,
			flags:                 0,
			size:                  cfg.dynamic_buffer_size,
			usage:                 VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT as u32
				| VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32
				| VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32
				| VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as u32,
			sharingMode:           VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices:   ptr::null()
		}, allocator, &mut dynamic_buffer)?;
		
		alloc.add(choose_memory_types(
			&memory_props.memoryProperties, !0, &MEMORY_PROPERTY_FLAGS_DYNAMIC_PREFERRED), &[dynamic_buffer], &[]);
		alloc.bind(None, &HeapsBindInfo::bind_buffers(&[dynamic_buffer]), allocator)?;
		let dynamic_buffer_alloc = MappedSubAlloc::new(
			GpaSubAlloc::new(&alloc, dynamic_buffer),
			alloc.map_region(dynamic_buffer, 0)?
		);
		//let dynamic_buffer_alloc_sprs = GpaSparseSubAlloc::new(alloc.clone(), dynamic_buffer, true);
		
		// LOCAL BUFFER
		
		let mut local_buffer = VK_NULL_HANDLE;
		device.createBuffer(&VkBufferCreateInfo {
			sType:                 VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
			pNext:                 None,
			flags:                 0,
			size:                  cfg.local_buffer_size,
			usage:                 VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32
				| VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32
				| VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT as u32
				| VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32
				| VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32
				| VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32
				| VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as u32,
			sharingMode:           VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices:   ptr::null()
		}, allocator, &mut local_buffer)?;
		
		alloc.add(choose_memory_types(
			&memory_props.memoryProperties, !0, &MEMORY_PROPERTY_FLAGS_LOCAL_PREFERRED), &[local_buffer], &[]);
		alloc.bind(None, &HeapsBindInfo::bind_buffers(&[local_buffer]), allocator)?;
		let local_buffer_alloc = Box::new(GpaSubAlloc::new(&alloc, local_buffer));
		//let local_buffer_alloc_sprs = GpaSparseSubAlloc::new(alloc.clone(), local_buffer, true);
		
		//
		// debug -----------------------------------------------------------------------------------
		//
		
		set_object_names(instance_info.vk12_debug_utils, &device, &[
			(VK_OBJECT_TYPE_INSTANCE, instance.handle, "instance\0"),
			//(VK_OBJECT_TYPE_PHYSICAL_DEVICE, physical_device.handle, "physical_device\0"),
			(VK_OBJECT_TYPE_DEVICE, device.handle, "device\0"),
			(VK_OBJECT_TYPE_QUEUE, primary_queue.handle, "primary_queue\0"),
			(VK_OBJECT_TYPE_BUFFER, local_buffer, "local_buffer\0"),
			(VK_OBJECT_TYPE_BUFFER, dynamic_buffer, "dynamic_buffer\0"),
		]);
		
		let async_compute_queue = primary_queue.clone();
		let transfer_queue = primary_queue.clone();
		
		let ctx = Arc::new(Self {
			instance,
			instance_info,
			allocator,
			dbg_messenger,
			physical_device,
			device,
			device_info,
			primary_queue_family,
			primary_queue,
			primary_cmd_pool,
			async_compute_queue_family: primary_queue_family,
			async_compute_queue,
			async_compute_cmd_pool:     primary_cmd_pool,
			transfer_queue_family:      primary_queue_family,
			transfer_queue,
			transfer_cmd_pool:          primary_cmd_pool,
			transfer_cmd_buffer,
			transfer_fence,
			transfer_cmds:              Mutex::default(),
			transfer_wait:              AsyncCondvar::new(),
			transfer_ready:             Arc::new(AsyncCondvar::new()),
			pipeline_cache,
			desc_pool,
			alloc,
			transfer_buffer,
			transfer_buffer_alloc,
			dynamic_buffer,
			dynamic_buffer_alloc,
			local_buffer,
			local_buffer_alloc,
			contexts:                   SyncOnceCell::new()
		});
		
		let contexts = plugins.into_iter()
			.map(|plugin| plugin.create_device_context(&ctx))
			.collect::<Result<Vec<_>, _>>()
			.expect("failed to create device context");
		ctx.contexts.set(contexts)
			.expect("failed to set device contexts");
		
		log::debug!("[GPGPU] root device context #{:X}: created ({}ms)", ctx.device.handle, t.elapsed().as_millis());
		Ok(ctx)
	}
	
	pub fn write_pipeline_cache(&self, path: impl AsRef<std::path::Path>) -> VkResult {
		self.device.waitIdle()?;
		let mut data_size = 0;
		self.device.getPipelineCacheData(self.pipeline_cache, &mut data_size, None)?;
		let mut data = vec![0u8; data_size].into_boxed_slice();
		self.device.getPipelineCacheData(self.pipeline_cache, &mut data_size, Some(&mut data))?;
		
		if std::fs::write(path, &data).is_err() {
			log::error!("[GPGPU] root device context #{:X}: failed to write pipeline cache", self.device.handle);
			return VK_ERROR_UNKNOWN;
		}
		
		log::debug!("[GPGPU] root device context #{:X}: wrote pipeline cache", self.device.handle);
		VK_SUCCESS
	}
	
	pub fn transfer(&self) -> VkResult {
		let mut cmds = std::mem::take(&mut*self.transfer_cmds.lock().unwrap());
		
		if cmds.0.is_empty() {
			return VK_SUCCESS;
		}
		
		let t = std::time::Instant::now();
		self.transfer_cmd_buffer.reset(0)?;
		self.transfer_cmd_buffer.begin(&VkCommandBufferBeginInfo {
			sType:            VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
			pNext:            None,
			flags:            0,
			pInheritanceInfo: None
		});
		
		cmds.flush(&self.transfer_cmd_buffer);
		self.transfer_cmd_buffer.end();
		self.transfer_queue.submit(&[VkSubmitInfo {
			sType:                VK_STRUCTURE_TYPE_SUBMIT_INFO,
			pNext:                None,
			waitSemaphoreCount:   0,
			pWaitSemaphores:      ptr::null(),
			pWaitDstStageMask:    ptr::null(),
			commandBufferCount:   1,
			pCommandBuffers:      &self.transfer_cmd_buffer.handle,
			signalSemaphoreCount: 0,
			pSignalSemaphores:    ptr::null()
		}], self.transfer_fence);
		
		self.device.waitForFences(&[self.transfer_fence], VK_FALSE, !0)?;
		self.device.resetFences(&[self.transfer_fence])?;
		self.transfer_wait.notify_all();
		log::info!("[GPGPU] root device context #{:X}: transfer completed ({}ms)", self.device.handle, t.elapsed().as_millis());
		VK_SUCCESS
	}
	
	pub fn get_ctx<C: std::any::Any>(&self) -> Option<&C> {
		self.contexts.get()
			.expect("contexts not initialized")
			.iter()
			.find_map(|v| v.as_any().downcast_ref::<C>())
	}
}

impl<S: 'static, T: 'static> Drop for DeviceRootContext<S, T> {
	fn drop(&mut self) {
		if std::thread::panicking() {
			log::warn!("[GPGPU] root device context #{:X}: aborting destruction due to panic", self.device.handle);
			return;
		}
		
		let t = std::time::Instant::now();
		let self_ = unsafe { (self as *const Self).as_ref().unwrap() };
		let contexts = self.contexts.get_mut().unwrap();
		for context in contexts {
			context.destroy().unwrap();
		}
		
		let alloc = Arc::try_unwrap(std::mem::replace(
			&mut self.alloc, Arc::new(Heaps::default()))).unwrap();
		
		self.device.destroyFence(self.transfer_fence, self.allocator);
		self.device.destroyBuffer(self.transfer_buffer, self.allocator);
		self.device.destroyBuffer(self.local_buffer, self.allocator);
		self.device.destroyBuffer(self.dynamic_buffer, self.allocator);
		alloc.bind(None, &HeapsBindInfo {
			unbind_buffers: &[self.local_buffer, self.dynamic_buffer],
			..HeapsBindInfo::default()
		}, self.allocator);
		alloc.destroy(self.allocator);
		self.device.destroyDescriptorPool(self.desc_pool, self.allocator);
		self.device.destroyPipelineCache(self.pipeline_cache, self.allocator);
		self.device.destroyCommandPool(self.primary_cmd_pool, self.allocator);
		self.device.destroy(self.allocator);
		self.instance.destroyDebugUtilsMessengerEXT(self.dbg_messenger, self.allocator);
		self.instance.destroy(self.allocator);
		log::debug!("[GPGPU] root device context #{:X}: destroyed ({}ms)", self.device.handle, t.elapsed().as_millis());
	}
}

#[derive(Debug)]
pub struct DeviceProfile<'a> {
	pub versions:            &'a [u32],
	pub types:               &'a [VkPhysicalDeviceType],
	pub features:            &'a VkPhysicalDeviceFeatures2<'a>,
	pub layers:              &'a [*const u8],
	pub extensions:          &'a [*const u8],
	pub optional_features:   &'a VkPhysicalDeviceFeatures2<'a>,
	pub optional_layers:     &'a [*const u8],
	pub optional_extensions: &'a [*const u8],
	pub queues:              &'a [&'a [(u32, &'a [f32])]]
}

#[derive(Debug)]
pub struct SelectedDeviceInfo {
	pub physical_device: VkPhysicalDeviceImpl,
	pub device:          VkDeviceImpl,
	pub properties:      VkPhysicalDeviceProperties2<'static>,
	pub features:        VkPhysicalDeviceFeatures2<'static>,
	pub layers:          Vec<*const u8>,
	pub extensions:      Vec<*const u8>,
	pub queue_profile:   usize,
	pub queues:          Vec<(u32, Vec<VkQueueImpl>)>
}

/// Chooses the first physical device that suits our needs. The regular user will only have two
/// physical devices, so it doesn't make sense to iterate over all devices and pick the one with
/// the highest rating.
fn create_device(
	instance:  &VkInstanceImpl,
	profiles:  &[DeviceProfile],
	surfaces:  &[VkSurfaceKHR],
	allocator: Option<&VkAllocationCallbacks>
) -> Result<Option<SelectedDeviceInfo>, VkResult> {
	let mut device_count = 0;
	instance.enumeratePhysicalDevices(&mut device_count, None)?;
	//let mut physical_devices = vec![Default::default(); device_count as _];
	let mut physical_devices = vec![Default::default(), device_count as _];
	instance.enumeratePhysicalDevices(&mut device_count, Some(&mut physical_devices))?;
	
	for profile in profiles {
		'devices: for physical_device in physical_devices.iter() {
			let physical_device = VkPhysicalDeviceImpl::new(*physical_device, instance);
			
			// properties
			
			let mut properties = VkPhysicalDeviceProperties2 {
				sType:      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2,
				pNext:      None,
				properties: VkPhysicalDeviceProperties::default()
			};
			
			physical_device.getProperties2(&mut properties);
			
			if !profile.versions.iter().copied().any(|version|
				VK_VERSION_MAJOR(properties.properties.apiVersion) == VK_VERSION_MAJOR(version)
					&& VK_VERSION_MINOR(properties.properties.apiVersion) >= VK_VERSION_MINOR(version)) {
				log::warn!("[GPGPU] skipping physical device: unsupported api version (was `{}.{}.{}`)",
					  VK_VERSION_MAJOR(properties.properties.apiVersion),
					  VK_VERSION_MINOR(properties.properties.apiVersion),
					  VK_VERSION_PATCH(properties.properties.apiVersion));
				continue 'devices;
			}
			
			// features
			
			let mut features = VkPhysicalDeviceFeatures2 {
				sType:    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
				pNext:    None,
				features: VkPhysicalDeviceFeatures::default()
			};
			
			physical_device.getFeatures2(&mut features);
			
			let available_features = vk_features_to_bools(&features.features);
			let required_features  = vk_features_to_bools(&profile.features.features);
			let optional_features  = vk_features_to_bools(&profile.optional_features.features);
			
			if (0..available_features.len()).any(|i|
				available_features[i] == VK_FALSE && required_features[i] == VK_TRUE) {
				log::warn!("[GPGPU] skipping physical device: unsupported feature");
				continue 'devices;
			}
			
			let mut features = profile.features.features;
			let features_tmp = vk_features_to_bools_mut(&mut features);
			
			(0..features_tmp.len())
				.filter( |i| optional_features[*i] == VK_TRUE
					&& available_features[*i] == VK_TRUE)
				.for_each(|i| features_tmp[i] = VK_TRUE);
			
			// queues
			
			let mut queue_family_count = 0;
			physical_device.getQueueFamilyProperties(&mut queue_family_count, None);
			let mut queue_family_props = vec![Default::default(); queue_family_count as _];
			physical_device.getQueueFamilyProperties(&mut queue_family_count, Some(&mut queue_family_props));
			
			let (queue_profile, queues) = 'queues_outer: {
				'profiles: for (i, queue_profile) in profile.queues.iter().enumerate() {
					let mut queues = Vec::new();
					
					'queues: for (flags, priorities) in queue_profile.iter() {
						for (i, props) in queue_family_props.iter().enumerate() {
							let skip = (props.queueCount as usize) < priorities.len()
								|| props.queueFlags & *flags != *flags & !0x20
								|| flags & 0x20 != 0
								&& !surfaces.iter().all(|surface| {
								let mut supported = 0u32;
								physical_device.getSurfaceSupportKHR(
									i as _, *surface, &mut supported);
								supported == VK_TRUE
							});
							
							if skip { continue; }
							
							queues.push(VkDeviceQueueCreateInfo {
								sType:            VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
								pNext:            None,
								flags:            0,
								queueFamilyIndex: i as _,
								queueCount:       priorities.len() as _,
								pQueuePriorities: priorities.as_ptr()
							});
							
							continue 'queues;
						}
						
						continue 'profiles;
					}
					
					break 'queues_outer (i, queues);
				}
				
				log::warn!("[GPGPU] skipping physical device: couldn't find a suitable queue");
				continue 'devices;
			};
			
			// layers
			
			let mut layer_count = 0;
			physical_device.enumerateDeviceLayerProperties(&mut layer_count, None)?;
			let mut layer_props = vec![Default::default(); layer_count as _];
			physical_device.enumerateDeviceLayerProperties(&mut layer_count, Some(&mut layer_props))?;
			
			'layers: for cfg_layer in profile.layers.iter() {
				for layer in layer_props.iter() {
					if ffi_str_eq(layer.layerName.as_ptr(), *cfg_layer) {
						continue 'layers;
					}
				}
				
				log::warn!("[GPGPU] skipping physical device: layer `{}` not supported",
					  string_convert(*cfg_layer));
				continue 'devices;
			}
			
			let layers = profile.optional_layers.iter().copied()
				.filter(|layer| layer_props.iter().any(|props|
					ffi_str_eq(props.layerName.as_ptr(), *layer)))
				.chain(profile.layers.iter().copied())
				.collect::<Vec<_>>();
			
			// extensions
			
			let mut extension_count = 0;
			physical_device.enumerateDeviceExtensionProperties(None, &mut extension_count, None)?;
			let mut extension_props = vec![Default::default(); extension_count as _];
			physical_device.enumerateDeviceExtensionProperties(None, &mut extension_count, Some(&mut extension_props))?;
			
			'extensions: for cfg_ext in profile.extensions.iter() {
				for ext in extension_props.iter() {
					if ffi_str_eq(ext.extensionName.as_ptr(), *cfg_ext) {
						continue 'extensions;
					}
				}
				
				log::warn!("[GPGPU] [INIT] skipping physical device: extension `{}` not supported",
					  string_convert(*cfg_ext));
				continue 'devices;
			}
			
			let extensions = profile.optional_extensions.iter().copied()
				.filter(|extension| extension_props.iter().any(|props|
					ffi_str_eq(props.extensionName.as_ptr(), *extension)))
				.chain(profile.extensions.iter().copied())
				.collect::<Vec<_>>();
			
			let mut device = VK_NULL_HANDLE;
			physical_device.createDevice(&VkDeviceCreateInfo {
				sType:                   VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
				pNext:                   Some((&VkPhysicalDeviceVulkan12Features {
					sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
					drawIndirectCount:                            VK_TRUE,
					timelineSemaphore:                            VK_TRUE,
					descriptorIndexing:                           VK_TRUE,
					shaderSampledImageArrayNonUniformIndexing:    VK_TRUE,
					shaderStorageImageArrayNonUniformIndexing:    VK_TRUE,
					descriptorBindingSampledImageUpdateAfterBind: VK_TRUE,
					descriptorBindingStorageImageUpdateAfterBind: VK_TRUE,
					descriptorBindingUpdateUnusedWhilePending:    VK_TRUE,
					descriptorBindingPartiallyBound:              VK_TRUE,
					descriptorBindingVariableDescriptorCount:     VK_TRUE,
					runtimeDescriptorArray:                       VK_TRUE,
					..Default::default()
				}).into()),
				flags:                   0,
				queueCreateInfoCount:    queues.len() as _,
				pQueueCreateInfos:       queues.as_ptr(),
				enabledLayerCount:       layers.len() as _,
				ppEnabledLayerNames:     layers.as_ptr(),
				enabledExtensionCount:   extensions.len() as _,
				ppEnabledExtensionNames: extensions.as_ptr(),
				pEnabledFeatures:        Some(&features)
			}, allocator, &mut device)?;
			let device = VkDeviceImpl::new(device, instance);
			
			let queues = queues.iter()
				.map(|queue_info| (queue_info.queueFamilyIndex, (0..queue_info.queueCount)
					.map(|i| {
						let mut queue = VK_NULL_HANDLE;
						device.getQueue(queue_info.queueFamilyIndex, i, &mut queue);
						VkQueueImpl::new(queue, &device)
					})
					.collect::<Vec<_>>()))
				.collect::<Vec<_>>();
			
			return Ok(Some(SelectedDeviceInfo {
				physical_device,
				device,
				properties,
				features: VkPhysicalDeviceFeatures2 {
					sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
					pNext: None,
					features
				},
				layers,
				extensions,
				queue_profile,
				queues
			}));
		}
	}
	
	Ok(None)
}