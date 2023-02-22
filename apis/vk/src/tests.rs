use super::*;
use std::ptr::null;

#[test]
fn test() {
	let mut v: u32 = 0;
	VkInstanceImpl::enumerateVersion(&mut v);
	eprintln!("vulkan instance version is {}.{}.{}", VK_VERSION_MAJOR(v), VK_VERSION_MINOR(v), VK_VERSION_PATCH(v));
	
	let instance_info = VkInstanceCreateInfo {
		sType:                   VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
		pNext:                   null(),
		flags:                   0,
		pApplicationInfo: &VkApplicationInfo {
			sType:              VK_STRUCTURE_TYPE_APPLICATION_INFO,
			pNext:              null(),
			pApplicationName:   "test app\0".as_ptr(),
			applicationVersion: VK_MAKE_VERSION(0, 0, 1),
			pEngineName:        "test engine\0".as_ptr(),
			engineVersion:      VK_MAKE_VERSION(0, 0, 1),
			apiVersion:         VK_MAKE_VERSION(1, 1, 0),
		},
		enabledLayerCount:       0,
		ppEnabledLayerNames:     null(),
		enabledExtensionCount:   0,
		ppEnabledExtensionNames: null()
	};
	
	let mut instance = VK_NULL_HANDLE;
	assert_eq!(VkInstanceImpl::create(&instance_info, null(), &mut instance), VK_SUCCESS);
	let instance = unsafe { VkInstanceImpl::new(instance) };
}