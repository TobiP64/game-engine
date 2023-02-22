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

use {vk::*};

/*use custom_collections::VecMap;

pub trait TransferCmdBuffer: std::fmt::Debug {
	fn copy_buffer(&mut self, src_buffer: VkBuffer, dst_bufer: VkBuffer, regions: &[VkBufferCopy]);
	
	fn copy_buffer_to_image(&mut self, src_buffer: VkBuffer, dst_image: VkImage, dst_image_layout: VkImageLayout, regions: &[VkBufferImageCopy]);
	
	fn copy_image_to_buffer(&mut self, src_image: VkImage, src_image_layout: VkImageLayout, dst_buffer: VkBuffer, regions: &[VkBufferImageCopy]);
}

#[derive(Debug, Default, Clone)]
pub struct Barrier {
	pub memory: Vec<VkMemoryBarrier<'static>>,
	pub buffer: Vec<VkBufferMemoryBarrier<'static>>,
	pub image:  Vec<VkImageMemoryBarrier<'static>>
}

#[derive(Debug, Default, Clone)]
pub struct Barriers {
	pub before:  VecMap<(VkPipelineStageFlags, VkPipelineStageFlags, VkDependencyFlags), Barrier>,
	pub between: VecMap<(VkPipelineStageFlags, VkPipelineStageFlags, VkDependencyFlags), Barrier>,
	pub after:   VecMap<(VkPipelineStageFlags, VkPipelineStageFlags, VkDependencyFlags), Barrier>,
}

#[derive(Debug, Default, Clone)]
pub struct TransferCmds {
	pub barriers:         Barriers,
	pub read_buffer:      VecMap<VkBuffer, Vec<VkBufferCopy>>,
	pub read_image:       VecMap<VkImage, (VkImageLayout, Vec<VkBufferImageCopy>)>,
	pub write_buffer:     VecMap<VkBuffer, Vec<VkBufferCopy>>,
	pub write_image:      VecMap<VkImage, (VkImageLayout, Vec<VkBufferImageCopy>)>
}

impl TransferCmds {
	pub fn record(&mut self, cmd_buffer: &VkCommandBufferImpl) {
		unimplemented!() // TODO TransferCmds impl
	}
	
	pub fn pipeline_barrier_before(
		&self,
		src_stage_mask:           VkPipelineStageFlags,
		dst_stage_mask:           VkPipelineStageFlags,
		dependency_flags:         VkDependencyFlags,
		p_memory_barriers:        &[VkMemoryBarrier],
		p_buffer_memory_barriers: &[VkBufferMemoryBarrier],
		p_image_memory_barriers:  &[VkImageMemoryBarrier]
	) {
		unimplemented!() // TODO TransferCmds impl
	}
}*/

#[derive(Default)]
pub struct Cmds(pub Vec<Box<dyn FnMut(&VkCommandBufferImpl) + Send + Sync>>);

impl Cmds {
	pub fn record(&mut self, cmds: impl FnMut(&VkCommandBufferImpl) + Send + Sync + 'static) {
		self.0.push(Box::new(cmds));
	}
	
	pub fn flush(&mut self, cmd_buffer: &VkCommandBufferImpl) {
		self.0.drain(..).for_each(|mut cmd| cmd(cmd_buffer));
	}
}

impl std::fmt::Debug for Cmds {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Cmds").finish()
	}
}

pub const NOOP: CmdNoOp = CmdNoOp;

#[derive(Copy, Clone)]
pub struct CmdNoOp;

/*impl FnOnce<&VkCommandBufferImpl> for CmdNoOp {
	type Output = ();
	
	extern "rust-call" fn call_once(self, _: &VkCommandBufferImpl) -> Self::Output {}
}

impl FnMut<&VkCommandBufferImpl> for CmdNoOp {
	extern "rust-call" fn call_mut(&mut self, _: &VkCommandBufferImpl) -> Self::Output {}
}*/