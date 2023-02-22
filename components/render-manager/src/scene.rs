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
	crate::{*, plugins::*},
	std::{sync::Arc, lazy::SyncOnceCell}
};

#[derive(Debug)]
pub struct SceneRootContext<S: 'static, T: 'static> {
	pub device_ctx: Arc<DeviceRootContext<S, T>>,
	pub scene:      S,
	pub contexts:   SyncOnceCell<Vec<Box<dyn SceneContext<S>>>>,
}

impl<S: 'static, T: 'static> SceneRootContext<S, T> {
	pub fn new<'a>(
		device_ctx: Arc<DeviceRootContext<S, T>>,
		scene:      S,
		plugins:    impl IntoIterator<Item = &'a dyn PluginRootContext<S, T>>
	) -> Result<Arc<Self>, Error> {
		let mut self_ = Arc::new(Self { device_ctx, scene, contexts: SyncOnceCell::new() });
		
		let contexts = plugins.into_iter()
			.map(|plugin| plugin.create_scene_context(&self_))
			.collect::<Result<Vec<_>, _>>()?;
		self_.contexts.set(contexts)
			.expect("failed to set contexts");
		
		log::debug!("[GPGPU] root scene context #{:?}: created", &self_.scene as *const _);
		Ok(self_)
	}
	
	pub fn get_ctx<C: std::any::Any>(&self) -> Option<&C> {
		self.contexts.get()
			.expect("contexts not initialized")
			.iter()
			.find_map(|v| v.as_any().downcast_ref::<C>())
	}
}

impl<S: 'static, T: 'static> Drop for SceneRootContext<S, T> {
	fn drop(&mut self) {
		if std::thread::panicking() {
			log::warn!("[GPGPU] root scene context #{:?}: aborting destruction due to panic", &self.scene as *const _);
			return;
		}
		
		for context in self.device_ctx.contexts.get().unwrap() {
			context.destroy().unwrap();
		}
		
		log::debug!("[GPGPU] root scene context #{:?}: destroyed", &self.scene as *const _);
	}
}