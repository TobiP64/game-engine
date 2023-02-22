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

#![feature(allocator_api, destructuring_assignment)]
#![warn(clippy::all)]
#![allow(clippy::too_many_arguments)]

use {
	ecs::World,
	gpgpu::{*, plugins::*},
	std::{mem::size_of, sync::Arc},
	vk::*
};

pub use {
	device::DeviceContext,
	loader::Loader,
	self::scene::SceneContext,
	target::TargetContext,
	scene_components::*,
	::scene::ui::*
};

pub mod device;
pub mod scene;
pub mod target;
pub mod loader;
pub mod scene_components;

type VkAlloc = gpgpu::misc::AllocatorWithLayout<gpgpu::mem::MappedSubAlloc>;
pub type GlyphIndex = u32;

//const MAX_DYN_SLOTS: usize = 4;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FontRender {
    Bitmap,
    Sdf,
    Curves
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UiRender;

#[derive(Debug)]
pub struct RootContext;

impl gpgpu::plugins::PluginRootContext<Arc<World>> for RootContext {
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
		_device_ctx: &Self::DeviceContext,
		device:      &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		world:       &Arc<World>
    ) -> Result<Self::SceneContext, Self::Error> {
        Self::SceneContext::create(device, world)
    }
    
    fn scene_ctx_update(
		&self,
		ctx:         &mut Self::SceneContext,
		_device_ctx: &Self::DeviceContext,
		device:      &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		world:       &Arc<World>
    ) -> Result<UpdateResult, Self::Error> {
        ctx.update(device, world)
    }
    
    fn target_ctx_create(
		&self,
		_device_ctx: &Self::DeviceContext,
		_device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		_target:     &TargetRootContext<Arc<World>>
    ) -> Result<Self::TargetContext, Self::Error> {
        Ok(Self::TargetContext::default())
    }
    
    fn target_ctx_update(
		&self,
		ctx:        &mut Self::TargetContext,
		device_ctx: &Self::DeviceContext,
		device:     &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		_target:    &TargetRootContext<Arc<World>>,
		template:   &VkGraphicsPipelineCreateInfo
    ) -> Result<(), Self::Error> {
        ctx.update(device_ctx, device, template)
    }
    
    fn target_ctx_destroy(
		&self,
		ctx:         &mut Self::TargetContext,
		_device_ctx: &Self::DeviceContext,
		device:      &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		_target:     &TargetRootContext<Arc<World>>
    ) -> Result<(), Self::Error> {
        ctx.destroy(device)
    }
    
    fn target_ctx_record(
		&self,
		ctx:            &Self::TargetContext,
		_device_ctx:    &Self::DeviceContext,
		scene_ctx:      &Self::SceneContext,
		device:         &Arc<gpgpu::DeviceRootContext<Arc<World>>>,
		_scene:         &Arc<World>,
		_target:        &TargetRootContext<Arc<World>>,
		cmd_buffer:     &VkCommandBufferImpl,
		_invocation_id: InvocationId
    ) {
        if scene_ctx.colored.commands.is_empty()
            && scene_ctx.colored_gradient.commands.is_empty()
            && scene_ctx.textured.is_empty()
            && scene_ctx.textured_gradient.is_empty() {
            return;
        }
        
        let max_draws = device.device_info.vk10_properties.limits.maxDrawIndirectCount;
        
        cmd_buffer.cmdBindVertexBuffers(
            0,
            &[device.local_buffer, device.dynamic_buffer],
            &[0, 0]
        );
        
        if !scene_ctx.colored.commands.is_empty() {
            cmd_buffer.cmdBindPipeline(VK_PIPELINE_BIND_POINT_GRAPHICS, ctx.pipeline_colored);
            cmd_buffer.cmdDrawIndirect(
                device.dynamic_buffer,
                device.dynamic_buffer_alloc.get_offset(scene_ctx.colored.commands.as_ptr()),
                max_draws.min(scene_ctx.colored.commands.len() as u32),
                size_of::<VkDrawIndirectCommand>() as _
            );
        }
        
        if !scene_ctx.colored_gradient.commands.is_empty() {
            cmd_buffer.cmdBindPipeline(VK_PIPELINE_BIND_POINT_GRAPHICS, ctx.pipeline_colored_gradient);
            cmd_buffer.cmdDrawIndirect(
                device.dynamic_buffer,
                device.dynamic_buffer_alloc.get_offset(scene_ctx.colored_gradient.commands.as_ptr()),
                max_draws.min(scene_ctx.colored_gradient.commands.len() as u32),
                size_of::<VkDrawIndirectCommand>() as _
            );
        }
        
        /*if !scene_ctx.textured.is_empty() {
			cmd_buffer.cmdBindPipeline(VK_PIPELINE_BIND_POINT_GRAPHICS, ctx.pipeline_textured);
			
			for (_, batch) in scene_ctx.textured.iter() {
				cmd_buffer.cmdBindDescriptorSets(
					VK_PIPELINE_BIND_POINT_GRAPHICS,
					device_ctx.pipeline_layout_ui_textured,
					0,
					&[batch.data.desc_set],
					&[]
				);
				
				cmd_buffer.cmdDrawIndirect(
					device.dynamic_buffer,
					device.dynamic_buffer_alloc.get_offset(batch.commands.as_ptr()),
					max_draws.min(batch.commands.len() as u32),
					size_of::<VkDrawIndirectCommand>() as _
				);
			}
		}
		
		if !scene_ctx.textured_gradient.is_empty() {
			cmd_buffer.cmdBindPipeline(VK_PIPELINE_BIND_POINT_GRAPHICS, ctx.pipeline_textured_gradient);
			
			for (_, batch) in scene_ctx.textured_gradient.iter() {
				cmd_buffer.cmdBindDescriptorSets(
					VK_PIPELINE_BIND_POINT_GRAPHICS,
					device_ctx.pipeline_layout_ui_textured,
					0,
					&[batch.data.desc_set],
					&[]
				);
				
				cmd_buffer.cmdDrawIndirect(
					device.dynamic_buffer,
					device.dynamic_buffer_alloc.get_offset(batch.commands.as_ptr()),
					max_draws.min(batch.commands.len() as u32),
					size_of::<VkDrawIndirectCommand>() as _
				);
			}
		}
		
		cmd_buffer.cmdBindVertexBuffers(
			0,
			&[device.dynamic_buffer, device.dynamic_buffer],
			&[0, 0]
		);
		
		cmd_buffer.cmdBindPipeline(
			VK_PIPELINE_BIND_POINT_GRAPHICS,
			ctx.pipeline_glyphs
		);
		
		for (_, batch) in &scene_ctx.glyphs {
			if batch.pending { continue; }
			
			cmd_buffer.cmdBindDescriptorSets(
				VK_PIPELINE_BIND_POINT_GRAPHICS,
				device_ctx.pipeline_layout_ui_glyphs,
				0,
				&[batch.desc_set],
				&[]
			);
			
			let cmd_offset = device.dynamic_buffer_alloc.get_offset(batch.commands.as_ptr());
			
			for offset in (0..batch.commands.len()).step_by(
				device.device_info.properties10.limits.maxDrawIndirectCount as _
			) {
				cmd_buffer.cmdDrawIndirect(
					device.dynamic_buffer,
					cmd_offset + (offset * size_of::<(VkDrawIndirectCommand, LocalInstanceData)>()) as VkDeviceSize,
					device.device_info.properties10.limits.maxDrawIndirectCount
						.min((batch.commands.len() - offset) as u32),
					size_of::<(VkDrawIndirectCommand, LocalInstanceData)>() as _
				);
			}
		}*/
    }
}
