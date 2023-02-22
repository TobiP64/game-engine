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

// TODO material upload
// TODO implement gpu update

#![feature(allocator_api, destructuring_assignment)]
#![warn(clippy::all)]
#![allow(dead_code, unused_variables)]

use {
    std::sync::Arc,
    vk::*,
    math::*,
    ecs::World,
    gpgpu::plugins::{UpdateResult, InvocationId}
};

pub use self::{
    device::DeviceContext,
    scene::*,
    target::TargetContext
};

mod device;
mod scene;
mod target;

type VkAlloc = gpgpu::misc::AllocatorWithLayout<gpgpu::mem::MappedSubAlloc>;
type ResourceId = u16;
type InstanceId = u16;

pub const INVALID_ID: u16       = u16::MAX;
pub const TILE_SIZE:  Vec2<u32> = Vec2(8, 8);
pub const BLOCK_SIZE: Vec3<u32> = Vec3(4, 4, 4);
pub const DESC_LIMIT: u32       = std::u16::MAX as u32 - 1;

#[derive(Copy, Clone, Debug)]
pub enum RayGen {
    Constant { bounces: usize,  },
    Factor { initial: usize, factor: f64, bounces: usize }
}

#[derive(Debug)]
pub struct RootContext;

impl gpgpu::plugins::GpgpuSystem<Arc<World>> for RootContext {
    type Error         = VkResult;
    type DeviceContext = DeviceContext;
    type SceneContext  = SceneContext;
    type TargetContext = TargetContext;
    
    fn device_ctx_create(
        &self,
        device: &Arc<gpgpu::DeviceRootContext<Arc<World>>>
    ) -> Result<Self::DeviceContext, Self::Error> {
        Self::DeviceContext::create(device)
    }
    
    fn device_ctx_destroy(
        &self,
        ctx:    &mut Self::DeviceContext,
        device: &gpgpu::DeviceRootContext<Arc<World>>
    ) -> Result<(), Self::Error> {
        ctx.destroy(device)
    }
    
    fn scene_ctx_create(
        &self,
        device_ctx: &Self::DeviceContext,
        device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
        scene:      &Arc<World>
    ) -> Result<Self::SceneContext, Self::Error> {
        Self::SceneContext::create(device_ctx, device, scene)
    }
    
    fn scene_ctx_update(
        &self,
        ctx:         &mut Self::SceneContext,
        device_ctx:  &Self::DeviceContext,
        device:      &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
        scene:       &Arc<World>
    ) -> Result<UpdateResult, Self::Error> {
        ctx.update(device, device_ctx, scene)?;
        Ok(UpdateResult::Reuse)
    }
    
    fn scene_ctx_destroy(
        &self,
        ctx:         &mut Self::SceneContext,
        _device_ctx: &Self::DeviceContext,
        device:      &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
        _scene:      &Arc<World>
    ) -> Result<(), Self::Error> {
        ctx.destroy(device)
    }
    
    fn target_ctx_create(
        &self,
        _device_ctx: &Self::DeviceContext,
        _device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
        _target:     &gpgpu::TargetRootContext<Arc<World>>
    ) -> Result<Self::TargetContext, Self::Error> {
        Self::TargetContext::create()
    }
    
    fn target_ctx_update(
		&self,
		ctx:        &mut Self::TargetContext,
		device_ctx: &Self::DeviceContext,
		device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		target:     &gpgpu::TargetRootContext<Arc<World>>,
		_template:  &VkGraphicsPipelineCreateInfo
    ) -> Result<(), Self::Error> {
        ctx.update(device_ctx, device, target)
    }
    
    fn target_ctx_record(
		&self,
		ctx:           &Self::TargetContext,
		device_ctx:    &Self::DeviceContext,
		scene_ctx:     &Self::SceneContext,
		_device:       &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		_scene:        &Arc<World>,
		target:        &gpgpu::TargetRootContext<Arc<World>>,
		cmd_buffer:    &VkCommandBufferImpl,
		invocation_id: InvocationId
    ) {
        /*if invocation_id.stage == 0 {
			//if scene_ctx.instances.is_empty() { return; }
			
			cmd_buffer.cmdBindDescriptorSets(
				VK_PIPELINE_BIND_POINT_COMPUTE,
				device_ctx.pipeline_layout_draw,
				0,
				&[scene_ctx.desc_set_draw[0], scene_ctx.desc_set_draw[1], scene_ctx.desc_set_draw[2],
					ctx.desc_sets_draw_out_image[invocation_id.image]],
				&[]
			);
			cmd_buffer.cmdBindPipeline(VK_PIPELINE_BIND_POINT_COMPUTE, device_ctx.pipeline_draw);
			cmd_buffer.cmdDispatch(
				target.extent.width / TILE_SIZE.0,
				target.extent.height / TILE_SIZE.1,
				1
			);
		} else if invocation_id.stage == 1 {
			let mut updates = scene_ctx.scene_updates.try_lock().expect("failed to lock scene updates");
			
			if updates.is_empty() { return; }
			
			cmd_buffer.cmdBindDescriptorSets(
				VK_PIPELINE_BIND_POINT_COMPUTE,
				device_ctx.pipeline_layout_upd,
				0,
				&[scene_ctx.desc_set_upd, scene_ctx.desc_set_draw[1]],
				&[]
			);
			cmd_buffer.cmdBindPipeline(VK_PIPELINE_BIND_POINT_COMPUTE, device_ctx.pipeline_draw);
			
			// TODO dispatch updates
		}*/
    }
    
    fn target_ctx_destroy(
        &self,
        ctx:         &mut Self::TargetContext,
        _device_ctx: &Self::DeviceContext,
        device:      &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
        _target:     &gpgpu::TargetRootContext<Arc<World>>
    ) -> Result<(), Self::Error> {
        ctx.destroy(device)
    }
}

#[derive(Debug)]
pub enum Error {
    Vk(VkResult),
    Io(std::io::Error)
}

impl From<VkResult> for Error {
    fn from(v: VkResult) -> Self {
        Self::Vk(v)
    }
}

impl From<std::io::Error> for Error {
    fn from(v: std::io::Error) -> Self {
        Self::Io(v)
    }
}