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

use {super::*, std::{time::Instant, collections::BTreeMap}};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PointerButton {
	Left,
	Middle,
	Right,
	
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PointerButtonState {
	Pressed,
	Released
}

#[derive(Debug, Clone)]
pub enum PointerEvent {
	PointerMotion     { time: Instant, x: f64, y: f64 },
	PointerButton     { time: Instant, button: PointerButton, state: PointerButtonState },
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Time {
	Frames(usize),
	Duration(std::time::Duration)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Val {
	Norm(f32),
	VWidth(f32),
	VHeight(f32),
	VMax(f32),
	VMin(f32),
	DWidth(f32),
	DHeight(f32),
	DMax(f32),
	DMin(f32),
	Pt(f32)
}

impl Default for Val {
	fn default() -> Self {
		Self::Norm(0f32)
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UiTransform {
	pub translation: Vec2<Val>,
	pub rotation:    Quat32,
	pub scaling:     Vec2<Val>,
	pub align:       UiAlign,
	pub z:           f32
}

impl UiTransform {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub fn translation(mut self, x: Val, y: Val) -> Self {
		self.translation = Vec2(x, y);
		self
	}
	
	pub fn rotation(mut self, t: Quat32) -> Self {
		self.rotation = t;
		self
	}
	
	pub fn scaling(mut self, x: Val, y: Val) -> Self {
		self.scaling = Vec2(x, y);
		self
	}
	
	pub fn align(mut self, a: UiAlign) -> Self {
		self.align = a;
		self
	}
	
	pub fn z(mut self, z: f32) -> Self {
		self.z = z;
		self
	}
}

impl Default for UiTransform {
	fn default() -> Self {
		Self {
			translation: Vec2::from(Val::Norm(0.0)),
			rotation:    Quat32::default(),
			scaling:     Vec2::from(Val::Norm(1.0)),
			align:       UiAlign::default(),
			z:           0.0
		}
	}
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct UiDimensions {
	pub surface_size: Vec2<usize>,
	pub display_size: Vec2<usize>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UiHorzAlign {
	Right,
	Center,
	Left
}

impl Default for UiHorzAlign {
	fn default() -> Self {
		Self::Center
	}
}

impl Into<f32> for UiHorzAlign {
	fn into(self) -> f32 {
		match self {
			Self::Right  => -1f32,
			Self::Center => 0f32,
			Self::Left   => 1f32,
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UiVertAlign {
	Top,
	Center,
	Bottom
}

impl Default for UiVertAlign {
	fn default() -> Self {
		Self::Center
	}
}

impl Into<f32> for UiVertAlign {
	fn into(self) -> f32 {
		match self {
			Self::Top    => 1f32,
			Self::Center => 0f32,
			Self::Bottom => -1f32,
		}
	}
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct UiAlign {
	pub horz: UiHorzAlign,
	pub vert: UiVertAlign,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiText(pub String, pub UiAlign);

impl From<String> for UiText {
	fn from(s: String) -> Self {
		Self(s, UiAlign { horz: UiHorzAlign::Left, vert: UiVertAlign::Top })
	}
}

impl<'a> From<&'a str> for UiText {
	fn from(s: &'a str) -> Self {
		Self::from(s.to_string())
	}
}

impl Default for UiText {
	fn default() -> Self {
		Self(String::new(), UiAlign { horz: UiHorzAlign::Left, vert: UiVertAlign::Top })
	}
}

#[derive(Clone, Debug, Default)]
pub struct FmtText(Vec<(TextCtrl, String)>);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TextCtrl {
	reset:    bool,
	effects:  usize,
	color_fg: Vec4<f64>,
	color_bg: Vec4<f64>,
	alt_font: usize
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClearMode {
	Line,
	LineEnd,
	LineBeginning,
	Display,
	DisplayEnd,
	DisplayBeginning,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Interactable;

#[derive(Debug, Clone)]
pub struct UiEvent {
	pub entity: Entity,
	pub event:  UiEventType
}

#[derive(Debug, Clone)]
pub enum UiEventType {
	/// A pointer has entered this entity.
	Enter,
	/// A pointer has left this entity.
	Leave,
	/// A pointer has entered an entity that is covered by a different entity.
	EnterShadow,
	/// A pointer has left an entity that is covered by a different entity.
	LeaveShadow,
	Other(PointerEvent)
}

#[derive(Debug, Clone)]
pub struct AlphaAccelStorage(BTreeMap<f32, Entity>);

/// Struct for caching converion ratios for every `Unit`.
#[derive(Debug, Default)]
pub struct UiTransformCache {
	viewport_width:  f32,
	viewport_height: f32,
	viewport_max:    f32,
	viewport_min:    f32,
	display_width:   f32,
	display_height:  f32,
	display_max:     f32,
	display_min:     f32,
	points:          f32
}

impl UiTransformCache {
	fn update(&mut self, range: f32, dim: UiDimensions) {
		self.viewport_width  = range / dim.surface_size.0 as f32;
		self.viewport_height = range / dim.surface_size.1 as f32;
		self.viewport_max    = range / dim.surface_size.0.max(dim.surface_size.1) as f32;
		self.viewport_min    = range / dim.surface_size.0.min(dim.surface_size.1) as f32;
		self.display_width   = range / dim.display_size.0 as f32;
		self.display_height  = range / dim.display_size.1 as f32;
		self.display_max     = range / dim.display_size.0.max(dim.display_size.1) as f32;
		self.display_min     = range / dim.display_size.0.min(dim.display_size.1) as f32;
		self.points          = 0f32;
	}
	
	fn to_norm(&self, val: Val) -> f32 {
		match val {
			Val::Norm(v)    => v,
			Val::VWidth(v)  => v * self.viewport_width,
			Val::VHeight(v) => v * self.viewport_height,
			Val::VMax(v)    => v * self.viewport_max,
			Val::VMin(v)    => v * self.viewport_min,
			Val::DWidth(v)  => v * self.display_width,
			Val::DHeight(v) => v * self.display_height,
			Val::DMax(v)    => v * self.display_max,
			Val::DMin(v)    => v * self.display_min,
			Val::Pt(v)      => v * self.points
		}
	}
}

pub fn update_cache(dimensions: Mutated<&UiDimensions>, cache: &mut Vec2<UiTransformCache>) {
	cache.0.update(dimensions.surface_size.0 as f32, **dimensions);
	cache.1.update(dimensions.surface_size.1 as f32, **dimensions);
}

pub fn update_ui_transform(
	transform:       Mutated<&UiTransform>,
	local_transform: &mut LocalTransform,
	cache:           Mutated<&Vec2<UiTransformCache>>
) {
	let (transform, cache) = (*transform, *cache);
	
	let scaling = Vec3(
		cache.1.to_norm(transform.scaling.1),
		cache.0.to_norm(transform.scaling.0),
		1f32
	);
	
	let translation = Vec3(
		cache.0.to_norm(transform.translation.0) + scaling.0
			* (Into::<f32>::into(transform.align.horz) * 0.5),
		cache.1.to_norm(transform.translation.1) + scaling.1
			* (Into::<f32>::into(transform.align.horz) * 0.5),
		transform.z
	);
	
	local_transform.0 = Mat4::from_transform(translation, transform.rotation, scaling);
}

pub struct InteractionFlags(pub u32);

impl InteractionFlags {
	pub const HOVERED:  u32 = 0x1;
	pub const CLICKED:  u32 = 0x4;
	pub const COVERED:  u32 = 0x8;
	pub const FOCUSED:  u32 = 0x10;
	pub const SELECTED: u32 = 0x20;
	
	pub fn hovered(&self) -> bool {
		self.0 & Self::HOVERED != 0
	}
	
	pub fn clicked(&self) -> bool {
		self.0 & Self::CLICKED != 0
	}
	
	pub fn covered(&self) -> bool {
		self.0 & Self::COVERED != 0
	}
	
	pub fn focused(&self) -> bool {
		self.0 & Self::FOCUSED != 0
	}
	
	pub fn selected(&self) -> bool {
		self.0 & Self::SELECTED != 0
	}
	
	pub fn set_hovered(&mut self, v: bool) {
		match v {
			true =>  self.0 |= Self::HOVERED,
			false => self.0 &= !Self::HOVERED
		}
	}
	
	pub fn set_clicked(&mut self, v: bool) {
		match v {
			true =>  self.0 |= Self::CLICKED,
			false => self.0 &= !Self::CLICKED
		}
	}
	
	pub fn set_covered(&mut self, v: bool) {
		match v {
			true =>  self.0 |= Self::COVERED,
			false => self.0 &= !Self::COVERED
		}
	}
	
	pub fn set_focused(&mut self, v: bool) {
		match v {
			true =>  self.0 |= Self::FOCUSED,
			false => self.0 &= !Self::FOCUSED
		}
	}
	
	pub fn set_selected(&mut self, v: bool) {
		match v {
			true =>  self.0 |= Self::SELECTED,
			false => self.0 &= !Self::SELECTED
		}
	}
}

pub struct UiEventHandler(Box<dyn Fn(UiEvent) -> BoxedFuture<()>>);
/*
/// This system takes window system events and emits ui events.
#[derive(Debug)]
pub struct UiEventSystem {
	receiver_events: EventReceiver<PointerEvent>
}

impl UiEventSystem {
	pub fn setup(World: &mut World) {
		<Self as System>::Data::setup(World)
	}
	
	pub fn new(World: &World) -> Self {
		Self {
			receiver_events: Write::<EventChannel<PointerEvent>>::fetch(World)
				.register_receiver()
		}
	}
	
	fn handle_event(
		pos:            Vec3<f32>,
		entity:         Entity,
		transform:      &GlobalTransform,
		hovered:        &mut Storage<Hovered>,
		hovered_shadow: &mut Storage<HoveredShadow>,
		shadow:         &mut bool,
		ui_events:      &mut EventChannel<UiEvent>
	) {
		let pos = (!transform.0).transform_pos(pos);
		
		match (
			pos.0 <= 0.5 && pos.0 >= -0.5 && pos.1 <= 0.5 && pos.1 >= -0.5,
			hovered.get_mut(entity).is_some(),
			hovered_shadow.get_mut(entity).is_some(),
		) {
			(true, true,  false) if !*shadow => *shadow = true,
			(true, _,     true)  if !*shadow => {
				hovered_shadow.remove(entity);
				ui_events.send(UiEvent { entity, event: UiEventType::LeaveShadow });
			}
			(true, hov, hov_shadow) => {
				if !hov_shadow && *shadow {
					hovered_shadow.insert(entity, HoveredShadow);
					ui_events.send(UiEvent { entity, event: UiEventType::EnterShadow });
				}
				
				if !hov && !*shadow {
					*shadow = true;
					hovered.insert(entity, Hovered);
					ui_events.send(UiEvent { entity, event: UiEventType::Enter });
				}
			}
			(false, hov, hov_shadow) => {
				if hov {
					hovered.remove(entity);
					ui_events.send(UiEvent { entity, event: UiEventType::Leave });
				}
				
				if hov_shadow {
					hovered_shadow.remove(entity);
					ui_events.send(UiEvent { entity, event: UiEventType::LeaveShadow });
				}
			}
		}
	}
}

pub struct UiFlags {
	hovered:        bool,
	hovered_shadow: bool,
	active_buttons: usize,
	focused:        bool
}

pub fn update_ui_depth<'a, A: Allocator + Clone>(
	query: &'a mut Query<(Entity, &'a GlobalTransform, &'a Interactable, Mutated<&'a GlobalTransform>), A>,
	depth: &mut AlphaAccelStorage,
) {
	depth.0 = query.iter()
		.map(|(entity, transforms, ..)| (transforms.0.get_translation().2, entity))
		.collect::<BTreeMap<_, _>>()
}

pub fn dispatch_ui_events(
	event:      Events<PointerEvent>,
	dimensions: &UiDimensions,
	depth:      &mut AlphaAccelStorage,
) {
	let (x, y) = match *event {
		PointerEvent::PointerMotion { x, y, .. } => (x, y),
		event => {
			(&entities, &focused).join().for_each(|(entity, _)| ui_events.send(
				UiEvent { entity, event: UiEventType::Other(event.clone()) }));
			return;
		}
	};
	
	let pos = Vec3(
		*x as f32 / dimensions.surface_size.0 as f32 * 2f32 - 1f32,
		*y as f32 / dimensions.surface_size.1 as f32 * 2f32 - 1f32,
		0f32
	);
	
	let mut shadow = false;
	vec.iter().copied().for_each(|(_, entity)| Self::handle_event(
		pos,
		entity,
		transforms.get(entity).unwrap(),
		&mut*hovered,
		&mut*hovered_shadow,
		&mut shadow,
		&mut ui_events
	));
}*/