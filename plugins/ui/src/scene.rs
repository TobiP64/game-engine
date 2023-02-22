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

#![allow(dead_code)]

use {
	super::*,
	std::{alloc::Allocator, collections::BTreeMap},
	engine_core::ResourceState,
	gpgpu::{mem::MappedSubAlloc, misc::AllocatorWithLayout, plugins::batch::*},
	math::*,
	ecs::*,
	::scene::*,
	vec_map::VecMap
};

type GenericBatch<T> = CmdBatch<T, VkAlloc, Batch<Entity, VkAlloc, (), Instance>>;
type UiDynBatch      = CmdBatch<Entity, VkAlloc, DynData, Instance>;
type UiDynTexBatch   = CmdBatch<Entity, VkAlloc, DynTexData, Instance>;
type PendingBatch    = BTreeMap<Handle<Mesh>, BTreeMap<Option<Handle<Texture>>, Batch<Entity, VkAlloc, (), Instance>>>;

#[derive(Debug)]
pub struct SceneContext {
	pub(crate) pending:               PendingBatch,
	pub(crate) colored:               GenericBatch<Handle<Mesh>>,
	pub(crate) colored_gradient:      GenericBatch<Handle<Mesh>>,
	pub(crate) curves:                GenericBatch<Handle<Mesh>>,
	pub(crate) curves_gradient:       GenericBatch<Handle<Mesh>>,
	pub(crate) textured:              VecMap<Handle<Texture>, GenericBatch<Handle<Mesh>>>,
	pub(crate) textured_gradient:     VecMap<Handle<Texture>, GenericBatch<Handle<Mesh>>>,
	pub(crate) sdf:                   VecMap<Handle<Texture>, GenericBatch<Handle<Mesh>>>,
	pub(crate) dyn_basic:             UiDynBatch,
	pub(crate) dyn_gradient:          UiDynBatch,
	pub(crate) dyn_textured:          UiDynTexBatch,
	pub(crate) dyn_textured_gradient: UiDynTexBatch,
	pub(crate) dyn_sdf:               UiDynTexBatch,
}

impl SceneContext {
	pub(crate) fn create(
		device: &gpgpu::DeviceRootContext<Arc<World>>,
		_scene:  &Arc<World>
	) -> Result<Self, VkResult> {
		let alloc = AllocatorWithLayout::new(device.dynamic_buffer_alloc.clone())
			.with_min_align(size_of::<VkDrawIndirectCommand>());

		Ok(Self {
			pending:               BTreeMap::new(),
			colored:               CmdBatch::new(alloc.clone()),
			colored_gradient:      CmdBatch::new(alloc.clone()),
			curves:                CmdBatch::new(alloc.clone()),
			curves_gradient:       CmdBatch::new(alloc.clone()),
			textured:              VecMap::new(),
			textured_gradient:     VecMap::new(),
			sdf:                   VecMap::new(),
			dyn_basic:             CmdBatch::new(alloc.clone()),
			dyn_gradient:          CmdBatch::new(alloc.clone()),
			dyn_textured:          CmdBatch::new(alloc.clone()),
			dyn_textured_gradient: CmdBatch::new(alloc.clone()),
			dyn_sdf:               CmdBatch::new(alloc)
		})
	}

	pub(crate) fn update(
		&self,
		_device: &gpgpu::DeviceRootContext<Arc<World>>,
		_world:  &Arc<World>,
	) -> Result<UpdateResult, VkResult> {
		let result = UpdateResult::Reuse;

		// TODO

		Ok(result)
	}

	fn update_add<'a, A: Allocator + Clone>(
		&mut self,
		device:    &gpgpu::DeviceRootContext<Arc<World>>,
		entity:    Entity,
		transform: Option<&GlobalTransform>,
		color:     Option<&Color>,
		mesh:      Option<&Handle<Mesh>>,
		texture:   Option<&Handle<Texture>>,
		_:         Added<&UiRender>,
		meshes:    &'a mut CachedQuery<&'a ResourceState<MeshData>, A>,
		textures:  &'a mut CachedQuery<&'a ResourceState<TextureData>, A>,
		result:    &mut UpdateResult
	) {
		let data = Instance {
			model: transform.map_or(Mat4::default(), |v| v.0),
			color: color.map_or(Vec4::from(1f32), |v| v.0)
		};

		let new_batch = || Batch::new(AllocatorWithLayout::new(device.dynamic_buffer_alloc.clone())
			.with_min_align(size_of::<Instance>()));

		//added += 1;
		match (
			mesh.and_then(|h| Some((h, meshes.get(h.0)?))),
			texture.and_then(|h| Some((h, textures.get(h.0)?)))
		) {
			(Some((mesh, &ResourceState::Loaded(MeshData { pipeline, count, offset, .. }))), texture) => {
				let (stride, batch) = match pipeline {
					UiPipeline::Colored                 => (size_of::<BasicVertex>(), &mut self.colored),
					UiPipeline::ColoredGradient         => (size_of::<GradientVertex>(), &mut self.colored_gradient),
					UiPipeline::Curves                  => (size_of::<BasicVertex>(), &mut self.curves),
					UiPipeline::CurvesGradient          => (size_of::<GradientVertex>(), &mut self.curves_gradient),
					UiPipeline::ColoredTextured         => (size_of::<TexturedVertex>(), &mut self.textured[texture.unwrap().0]),
					UiPipeline::ColoredTexturedGradient => (size_of::<TexturedGradientVertex>(), &mut self.textured_gradient[texture.unwrap().0]),
				};

				// TODO texture init

				if batch.insert_instance(
					*mesh,
					count,
					offset as u32 / stride as u32,
					entity,
					(),
					data,
					new_batch,
					|instances: &Batch<_, _, _, _, _>| device
						.dynamic_buffer_alloc
						.get_index(instances.batch_data.as_ptr())
				) {
					*result = UpdateResult::Redraw;
				}
			}
			(Some((&mesh, ResourceState::Unloaded)), texture) => self.pending.entry(mesh)
				.or_default()
				.entry(texture.map(|(&tex, _)| tex))
				.or_insert_with(new_batch)
				.insert(entity, (), data)
				.expect("instance already present"),
			(None, Some(_)) => log::error!("entity #{:x} cannot be rendered: no mesh attached", entity),
			(None, None)    => log::error!("entity #{:x} cannot be rendered: neither mesh, nor texture attached", entity)
		}

		log::info!("entity #{:x} added", entity)
	}

	fn update_add_pending(
		&mut self,
		device:    &gpgpu::DeviceRootContext<Arc<World>>,
		entity:    Entity,
		mesh:      Mutated<&ResourceState<MeshData>>,
		result:    &mut UpdateResult,
	) {
		let mesh = match *mesh {
			ResourceState::Loaded(v) => v,
			_ => return
		};

		let instances = match self.pending.remove(&Handle::new(entity)) {
			Some(v) => v,
			_ => return
		};

		for (texture, instances) in instances {
			let (stride, batch) = match mesh.pipeline {
				UiPipeline::Colored                 => (size_of::<BasicVertex>(), &mut self.colored),
				UiPipeline::ColoredGradient         => (size_of::<GradientVertex>(), &mut self.colored_gradient),
				UiPipeline::Curves                  => (size_of::<BasicVertex>(), &mut self.curves),
				UiPipeline::CurvesGradient          => (size_of::<GradientVertex>(), &mut self.curves_gradient),
				UiPipeline::ColoredTextured         => (size_of::<TexturedVertex>(), &mut self.textured[&texture.unwrap()]),
				UiPipeline::ColoredTexturedGradient => (size_of::<TexturedGradientVertex>(), &mut self.textured_gradient[&texture.unwrap()]),
			};

			batch.insert_batch(
				Handle::new(entity),
				mesh.count,
				mesh.offset as u32 / stride as u32,
				instances,
				(),
				|instances| device
					.dynamic_buffer_alloc
					.get_index(instances.batch_data.as_ptr())
			);
		}

		*result = UpdateResult::Redraw;
	}

	fn update_add_text<'a, A: Allocator + Clone>(
		&mut self,
		device:    &gpgpu::DeviceRootContext<Arc<World>>,
		entity:    Entity,
		transform: &GlobalTransform,
		color:     &Color,
		glyphs:    &mut GlyphsInstanceData,
		font:      &Handle<Font>,
		text:      Added<&UiText>,
		result:    &mut UpdateResult,
		fonts:     &'a mut CachedQuery<(&'a ResourceState<FontData>, &'a mut GlyphsBatch), A>
	) {
		let (font_data, batch) = fonts.get(**font).expect("invalid font handle");

		// add instance

		batch.instances.push(entity);
		glyphs.index = batch.instances.len() - 1;

		// generate characters

		glyphs.glyphs = Vec::new_in(AllocatorWithLayout::new(device.dynamic_buffer_alloc.clone())
			.with_min_align(size_of::<GlyphVertex>()));

		if let ResourceState::Loaded(data) = font_data {
			Self::gen_chars(text.0.0.as_str(), text.1, &mut glyphs.glyphs, data);
		}

		// add command

		let cmd_ptr = batch.commands.as_ptr();
		batch.commands.reserve_exact(1);
		batch.commands.push((VkDrawIndirectCommand {
			vertexCount:   glyphs.glyphs.len() as _,
			instanceCount: 1,
			firstVertex:   glyphs.glyphs.is_empty().then(|| device
				.dynamic_buffer_alloc
				.get_index(glyphs.glyphs.as_ptr()))
				.unwrap_or_default(),
			firstInstance: device.dynamic_buffer_alloc
				.get_index(batch.commands.as_ptr())
				+ batch.commands.len() as u32
		}, LocalInstanceData { model: transform.0, color: color.0 }));

		// update offset if vec was reallocated

		if batch.commands.as_ptr() != cmd_ptr {
			batch.update_commands(device.dynamic_buffer_alloc
				.get_index(batch.commands.as_ptr()));
		}

		*result = UpdateResult::Redraw;
		log::trace!("entity #{:x} added", entity);
	}

	fn update_add_text_pending<'a, A: Allocator + Clone>(
		&mut self,
		device:    &gpgpu::DeviceRootContext<Arc<World>>,
		font:      Mutated<&ResourceState<FontData>>,
		batch:     &mut GlyphsBatch,
		result:    &mut UpdateResult,
		instances: &'a mut CachedQuery<(&'a UiText, &'a mut GlyphsInstanceData), A>
	) {
		let font = match *font {
			ResourceState::Loaded(v) => v,
			_ => return
		};

		*result = UpdateResult::Redraw;

		let GlyphsBatch { instances: instances2, commands } = batch;

		instances2.iter().for_each(|entity| {
			let (UiText(text, align), GlyphsInstanceData { index, glyphs }) = instances.get(*entity)
				.expect("invalid instance entity id");

			Self::gen_chars(text.as_str(), *align, glyphs, font);
			commands[*index].0.vertexCount = glyphs.len() as _;
			commands[*index].0.firstVertex = match &**glyphs {
				[_, ..] => device.dynamic_buffer_alloc.get_index(glyphs.as_ptr()) as _,
				[]      => 0
			};

			log::trace!("entity #{:x} ready", entity);
		});
	}

	fn update_remove(
		&mut self,
		device: &gpgpu::DeviceRootContext<Arc<World>>,
		entity: Entity,
		_: Removed<&UiRender>
	) {
		if self.colored.remove_instance(entity, |instances| device
			.dynamic_buffer_alloc
			.get_index(instances.batch_data.as_ptr()))
			.is_err()
		{
			log::error!("{}: failed to remove, does not exist", entity);
		}

		log::trace!("{} removed", entity);
	}

	fn update_remove_text<'a, A: Allocator + Clone>(
		&mut self,
		device:    &gpgpu::DeviceRootContext<Arc<World>>,
		entity:    Entity,
		Removed((_, instance, font)): Removed<(&UiText, &GlyphsInstanceData, &Handle<Font>)>,
		fonts:     &'a mut CachedQuery<&'a mut GlyphsBatch, A>,
		instances: &'a mut CachedQuery<&'a mut GlyphsInstanceData, A>,
		result:    &mut UpdateResult
	) {
		let batch = fonts.get(font.0).expect("invalid font handle");
		let cmd_ptr = batch.commands.as_ptr();

		batch.instances.swap_remove(instance.index);
		batch.commands.swap_remove(instance.index);

		// update moved instance with outdated index
		while instance.index != batch.instances.len() {
			if let Some(moved) = instances.get(batch.instances[instance.index]) {
				moved.index = instance.index;
				break;
			}

			log::warn!("failed to update moved instance (entity #{:x}): not found, removing instance",
					   batch.instances[instance.index]);

			batch.instances.swap_remove(instance.index);
			batch.commands.swap_remove(instance.index);
		}

		// update commands offset
		if cmd_ptr != batch.commands.as_ptr() {
			batch.update_commands(device.dynamic_buffer_alloc
				.get_index(batch.commands.as_ptr()));
		}

		*result = UpdateResult::Redraw;
		log::trace!("entity #{:x} removed", entity);
	}

	fn update_update<'a, A: Allocator + Clone>(
		&mut self,
		entity:    Entity,
		transform: &GlobalTransform,
		color:     &Color,
		mesh:      Option<&Handle<Mesh>>,
		texture:   Option<&Handle<Texture>>,
		mut_trans: Option<Mutated<&GlobalTransform>>,
		mut_color: Option<Mutated<&Color>>,
		meshes:    &'a mut CachedQuery<&'a ResourceState<MeshData>, A>
	) {
		use {ResourceState::*, UiPipeline::*};

		//updated += 1;

		let instance = match (mesh.and_then(|h| Some((h, meshes.get(h.0)?))), texture) {
			(Some((mesh, Loaded(MeshData { pipeline: Colored, .. }))), None) =>
				&mut self.colored.batch[mesh][entity],
			(Some((mesh, Loaded(MeshData { pipeline: ColoredGradient, .. }))), None) =>
				&mut self.colored_gradient.batch[mesh][entity],
			(Some((mesh, Loaded(MeshData { pipeline: ColoredTextured, .. }))), Some(tex)) =>
				&mut self.textured[tex].batch[mesh][entity],
			(Some((mesh, Loaded(MeshData { pipeline: ColoredTexturedGradient, .. }))), Some(tex)) =>
				&mut self.textured_gradient[tex].batch[mesh][entity],
			(None, Some(_tex))       => unimplemented!(),
			(Some((_, Unloaded)), _) => return,
			_                        => unreachable!()
		};

		if mut_trans.is_some() { instance.model = transform.0; }
		if mut_color.is_some() { instance.color = color.0; }
	}

	fn update_update_text<'a, A: Allocator + Clone>(
		&mut self,
		device:    &gpgpu::DeviceRootContext<Arc<World>>,
		entity:    Entity,
		transform: &GlobalTransform,
		color:     &Color,
		text:      &UiText,
		data:      &mut GlyphsInstanceData,
		font:      &Handle<Font>,
		mut_trans: Option<Mutated<&GlobalTransform>>,
		mut_color: Option<Mutated<&Color>>,
		mut_text:  Option<Mutated<&UiText>>,
		fonts:     &'a mut CachedQuery<&'a mut GlyphsBatch, A>,
		world:     &Arc<World>
	) {
		let batch = fonts.get(font.0).expect("invalid font handle");
		let (command, instance) = &mut batch.commands[data.index];

		if mut_trans.is_some() { instance.model = transform.0; }
		if mut_color.is_some() { instance.color = color.0; }
		if let (Some(_), ResourceState::Loaded(font)) = (mut_text, &*world.entry(font.0)
			.expect("invalid font handle")
			.get::<ResourceState<FontData>>()
			.expect("font has no resource data"))
		{
			Self::gen_chars(text.0.as_str(), text.1, &mut data.glyphs, font);
			command.vertexCount = data.glyphs.len() as _;
			command.firstVertex = match &*data.glyphs {
				[_, ..] => device.dynamic_buffer_alloc.get_index(data.glyphs.as_ptr()) as _,
				[]      => 0
			};

			log::trace!("entity #{:x} updated", entity)
		}
	}

	fn gen_chars<A: std::alloc::Allocator>(
		text:   &str,
		align:  UiAlign,
		buffer: &mut Vec<GlyphVertex, A>,
		font:   &FontData
	) {
		buffer.clear();
		if text.is_empty() { return; }
		let mut pos = Vec2(0f32, 0f32);

		for line in text.lines() {
			let len = buffer.len();
			let mut chars = line.chars().peekable();

			while let Some(ch) = chars.next() {
				match font.glyphs_host.get(&ch) {
					Some(v) => {
						if v.idx != -1 {
							buffer.push(GlyphVertex { index: v.idx as _, pos });
						}

						pos.0 += v.advance.0 + *chars.peek()
							.and_then(|ch| v.kern_advance.get(ch))
							.unwrap_or(&0f32);
					}
					None => {
						unimplemented!("gen on use")
					}
				}
			}

			let __tmp__ = buffer.len();
			match align.horz {
				UiHorzAlign::Left => (),
				UiHorzAlign::Right => buffer[len..__tmp__].iter_mut()
					.for_each(|vert| vert.pos.0 -= pos.0),
				UiHorzAlign::Center => buffer[len..__tmp__].iter_mut()
					.for_each(|vert| vert.pos.0 -= pos.0 * 0.5f32)
			}

			pos.0 = 0f32;
			pos.1 += font.ascender - font.descender + font.line_gap
		}

		match align.vert {
			UiVertAlign::Top => (),
			UiVertAlign::Bottom => buffer.iter_mut().for_each(|v| v.pos.1 -= pos.1),
			UiVertAlign::Center => buffer.iter_mut().for_each(|v| v.pos.1 -= pos.1 * 0.5f32)
		}
	}
}

#[derive(Debug)]
enum Command {
	Colored(GenericBatch<Handle<Mesh>>),
	ColoredGradient(GenericBatch<Handle<Mesh>>),
	Curves(GenericBatch<Handle<Mesh>>),
	CurvesGradient(GenericBatch<Handle<Mesh>>),
	Textured(GenericBatch<(Handle<Texture>, Handle<Mesh>)>),
	TexturedGradient(GenericBatch<(Handle<Texture>, Handle<Mesh>)>),
	Sdf(GenericBatch<Handle<Texture>>),
	Glyphs(GlyphsBatch)
}

#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct Instance {
	model: Mat4<f32>,
	color: Vec4<f32>
}

#[derive(Debug)]
struct TexData {
	desc_set: VkDescriptorSet
}

#[derive(Debug)]
pub(crate) struct DynData {
	mesh:       Handle<Box<Mesh>>,
	animations: Vec<(Handle<AnimationSrc>, VkDescriptorSet)>
}

#[derive(Debug)]
pub(crate) struct DynTexData {
	mesh:       Handle<Box<Mesh>>,
	texture:    Handle<Box<Texture>>,
	desc_set:   VkDescriptorSet,
	animations: Vec<(Handle<AnimationSrc>, VkDescriptorSet)>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum UiPipeline {
	Colored,
	ColoredGradient,
	Curves,
	CurvesGradient,
	ColoredTextured,
	ColoredTexturedGradient
}

impl UiPipeline {
	pub fn vertex_size(&self) -> usize {
		match self {
			Self::Colored                 => size_of::<BasicVertex>(),
			Self::ColoredGradient         => size_of::<GradientVertex>(),
			Self::Curves                  => size_of::<BasicVertex>(),
			Self::CurvesGradient          => size_of::<GradientVertex>(),
			Self::ColoredTextured         => size_of::<TexturedVertex>(),
			Self::ColoredTexturedGradient => size_of::<TexturedGradientVertex>(),
		}
	}
}

#[derive(Debug)]
pub(crate) struct MeshData {
	pub offset:   VkDeviceSize,
	pub count:    u32,
	pub pipeline: UiPipeline
}

impl MeshData {
	pub fn len(&self) -> usize {
		self.count as usize * self.pipeline.vertex_size()
	}
}

#[derive(Debug)]
pub(crate) struct TextureData {
	pub image:    VkImage,
	pub view:     VkImageView,
	pub desc_set: VkDescriptorSet
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct BasicVertex {
	pub pos: Vec2<f32>
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct GradientVertex {
	pub pos:   Vec2<f32>,
	pub color: Vec4<f32>
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TexturedVertex {
	pub pos: Vec2<f32>,
	pub tex: Vec2<f32>
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TexturedGradientVertex {
	pub pos:   Vec2<f32>,
	pub color: Vec4<f32>,
	pub tex:   Vec2<f32>,
}

#[derive(Debug)]
struct GlyphsBatch {
	// before the font is loaded, this is what `pending` was previously
	instances: Vec<Entity>,
	commands:  Vec<(VkDrawIndirectCommand, LocalInstanceData), AllocatorWithLayout<MappedSubAlloc>>,
}

impl GlyphsBatch {
	pub fn update_commands(&mut self, offset: u32) {
		for (i, command) in self.commands.iter_mut().enumerate() {
			command.0.firstInstance = offset + i as u32;
		}
	}
}

#[derive(Debug)]
struct GlyphsInstanceData {
	index:  usize,
	glyphs: Vec<GlyphVertex, AllocatorWithLayout<MappedSubAlloc>>
}

/// Data required to generate an sdf from glyph paths
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct LocalSdfGenData {
	pub offset: Vec2<u32>,
	pub extent: Vec2<u32>,
	pub scale:  f32,
	pub count:  u32
}

impl LocalSdfGenData {
	pub(crate) fn to_bytes(self) -> [u8; 24] {
		let mut buf = [0u8; 24];
		buf[0 .. 4].copy_from_slice(&self.offset.0.to_le_bytes());
		buf[4 .. 8].copy_from_slice(&self.offset.1.to_le_bytes());
		buf[8 ..12].copy_from_slice(&self.extent.0.to_le_bytes());
		buf[12..16].copy_from_slice(&self.extent.1.to_le_bytes());
		buf[16..20].copy_from_slice(&self.scale.to_le_bytes());
		buf[20..24].copy_from_slice(&self.count.to_le_bytes());
		buf
	}
}

#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct LocalSdfGenVertex {
	pub r#type: u32,
	pub _pad:   u32,
	pub p0:     Vec2<f32>,
	pub p1:     Vec2<f32>,
	pub p2:     Vec2<f32>,
}

/// Per-glyph data required on the cpu
#[derive(Debug, Clone)]
pub(crate) struct HostGlyphData {
	pub idx:          isize,
	pub advance:      Vec2<f32>,
	pub kern_advance: VecMap<char, f32>
}

/// A single char of a text in host visible memory
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
struct GlyphVertex {
	pos:   Vec2<f32>,
	index: GlyphIndex
}

#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
struct LocalInstanceData {
	model: Mat4<f32>,
	color: Vec4<f32>
}

#[derive(Debug)]
pub(crate) struct FontData {
	pub sdf_image:           VkImage,
	pub sdf_view:            VkImageView,
	pub desc_set:            VkDescriptorSet,
	pub glyphs_host:         VecMap<char, HostGlyphData>,
	pub glyphs_local_offset: VkDeviceSize,
	pub glyphs_local_len:    usize,
	pub ascender:            f32,
	pub descender:           f32,
	pub line_gap:            f32
}