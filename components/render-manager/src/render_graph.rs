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

use crate::{DeviceRootContext, TargetRootContext};
use std::sync::Arc;
use vk::*;

pub fn build<S: 'static, T: 'static>() -> Builder {
	RenderGraph::<S, T>::build()
}

pub trait RenderGraphNode {
	fn name(&self) -> &str;
	
	fn enabled(&self) -> bool {
		false
	}
	
	fn destroy(&mut self) -> Result<(), ()> {
		Ok(())
	}
}

impl RenderGraphNode for () {
	fn name(&self) -> &str {
		"dummy_node"
	}
}

pub trait RenderGraphNodeHost: RenderGraphNode {
	fn execute(&mut self) -> Result<(), ()> {
		Ok(())
	}
}

impl RenderGraphNodeHost for () {}

pub trait RenderGraphNodeLocal: RenderGraphNode {
	fn update(&mut self, extent: VkExtent2D) {}
	
	fn record(&self, _cmd_buffer: &VkCommandBuffer, invocation_id: usize) {}
	
	fn subpass_cfg(&self) -> &[VkSubpassDescription2] {
		&[]
	}
}

impl RenderGraphNodeLocal for () {}

/// A render graph. Passed to the RootTargetContext.
#[derive(Debug, Clone)]
pub struct RenderGraph<S: 'static, T: 'static> {
	device_ctx: Arc<DeviceRootContext<S, T>>
}

impl<S: 'static, T: 'static> RenderGraph<S, T> {
	pub fn build() -> Builder {
		Builder::default()
	}
	
	pub fn new(
		builder:    &Builder,
		device_ctx: Arc<DeviceRootContext<S, T>>
	) -> Result<Self, VkResult> {
		Ok(Self {
			device_ctx
		})
	}
	
	pub fn update(&mut self, extent: VkExtent2D) -> VkResult {
		VK_SUCCESS
	}
	
	pub fn record(&self, cmd_buffer: &VkCommandBuffer, invocation_id: usize) {
	
	}
	
	pub fn dispatch(&self) -> VkResult {
		VK_SUCCESS
	}
}

#[derive(Default)]
pub struct Builder {
	stages: Vec<StageBuilder>
}

impl Builder {
	/// Stages are usually completely independent from one another, they are recorded in
	/// separate command buffers and sometimes dispatched to different queues.
	pub fn stage(mut self, f: impl FnOnce(StageBuilder) -> StageBuilder) -> Self {
		let builder = f(StageBuilder::default());
		self.stages.push(builder);
		self
	}
	
	pub fn finish<S, T>(&self, device_ctx: &Arc<DeviceRootContext<S, T>>) -> Result<RenderGraph<S, T>, VkResult> {
		RenderGraph::new(self, device_ctx.clone())
	}
}

#[derive(Default)]
pub struct StageBuilder {
	name:            Option<String>,
	jobs:            Vec<StageJob>,
	pre_conditions:  Vec<VkSemaphore>,
	post_conditions: Vec<VkSemaphore>,
}

enum StageJob {
	Pass(PassBuilder),
	Node(NodeBuilder),
	/// All nodes prior to the barrier must be finished before executing jobs after the barrier.
	Barrier,
	Flush
}

impl StageBuilder {
	pub fn name(mut self, name: impl ToString) -> Self {
		self.name = Some(name.to_string());
		self
	}
	
	pub fn pre_condition(mut self, semaphore: VkSemaphore) -> Self {
		self.pre_conditions.push(semaphore);
		self
	}
	
	pub fn post_condition(mut self, semaphore: VkSemaphore) -> Self {
		self.post_conditions.push(semaphore);
		self
	}
	
	pub fn pass(mut self, f: impl FnOnce(PassBuilder) -> PassBuilder) -> Self {
		let builder = f(PassBuilder::default());
		self.jobs.push(StageJob::Pass(builder));
		self
	}
	
	pub fn node(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
		let builder = f(NodeBuilder::default());
		self.jobs.push(StageJob::Node(builder));
		self
	}
	
	pub fn barrier(mut self) -> Self {
		self.jobs.push(StageJob::Barrier);
		self
	}
	
	/// Used between host and local nodes, to ensure transferred data is not cached
	pub fn transfer_flush(mut self) -> Self {
		self.jobs.push(StageJob::Flush);
		self
	}
}

#[derive(Default)]
pub struct PassBuilder {
	name:      Option<String>,
	subpasses: Vec<SubpassBuilder>
}

impl PassBuilder {
	pub fn name(mut self, name: impl ToString) -> Self {
		self.name = Some(name.to_string());
		self
	}
	
	pub fn target_ctx<S, T>(mut self, target: &TargetRootContext<S, T>) -> Self {
		
		self
	}
	
	pub fn subpass(mut self, f: impl FnOnce(SubpassBuilder) -> SubpassBuilder) -> Self {
		let builder = f(SubpassBuilder::default());
		self.subpasses.push(builder);
		self
	}
}

#[derive(Default)]
pub struct SubpassBuilder {
	name:  Option<String>,
	nodes: Vec<NodeBuilder>
}

impl SubpassBuilder {
	pub fn name(mut self, name: impl ToString) -> Self {
		self.name = Some(name.to_string());
		self
	}
	
	pub fn node(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
		let builder = f(NodeBuilder::default());
		self.nodes.push(builder);
		self
	}
}

#[derive(Default)]
pub struct NodeBuilder {
	name:            Option<String>,
	dependencies:    Vec<u8>,
	queues:          usize,
	job:             Option<NodeJob>,
	pre_conditions:  Vec<VkSemaphore>,
	post_conditions: Vec<VkSemaphore>,
}

impl NodeBuilder {
	pub fn name(mut self, name: impl ToString) -> Self {
		self.name = Some(name.to_string());
		self
	}
	
	pub fn pre_condition(mut self, semaphore: VkSemaphore) -> Self {
		self.pre_conditions.push(semaphore);
		self
	}
	
	pub fn post_condition(mut self, semaphore: VkSemaphore) -> Self {
		self.post_conditions.push(semaphore);
		self
	}
	
	pub fn dependency(mut self, node: u8) -> Self {
		self.dependencies.push(node);
		self
	}
	
	pub fn dependencies(mut self, nodes: impl IntoIterator<Item = u8>) -> Self {
		self.dependencies = nodes.into_iter().collect();
		self
	}
	
	pub fn queue(mut self, queue: usize) -> Self {
		self.queues |= 1 << queue;
		self
	}
	
	pub fn queues(mut self, queues: usize) -> Self {
		self.queues = queues;
		self
	}
	
	pub fn host_job(mut self, job: impl RenderGraphNodeHost + 'static) -> Self {
		debug_assert!(self.job.is_none());
		self.job = Some(NodeJob::Host(Box::new(job)));
		self
	}
	
	pub fn local_job(mut self, job: impl RenderGraphNodeLocal + 'static) -> Self {
		debug_assert!(self.job.is_none());
		self.job = Some(NodeJob::Local(Box::new(job)));
		self
	}
}

pub enum NodeJob {
	Host(Box<dyn RenderGraphNodeHost>),
	Local(Box<dyn RenderGraphNodeLocal>)
}