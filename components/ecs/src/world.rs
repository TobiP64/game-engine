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
	super::*,
	core::{
		any::TypeId,
		sync::atomic::*,
		alloc::*,
		hash::{Hash, Hasher},
		marker::PhantomData,
		ptr,
		mem,
		cmp,
		fmt,
		iter
	}
};

#[cfg(feature = "std")]
use std::{sync::{Mutex, MutexGuard}, collections};
#[cfg(not(feature = "std"))]
use {spin::{Mutex, MutexGuard}, ::alloc::collections};

const DEFAULT_LAYOUT: Layout = unsafe { Layout::from_size_align_unchecked(0x4000, 0x1000) };
const HASH_PRIME:     u64    = 31;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Entity {
	// This pointer is guaranteed to always be valid
	location:   *const EntityLocation,
	generation: u32
}

unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}

impl Entity {
	fn is_dirty(&self) -> bool {
		self.generation != unsafe { &*self.location }.generation.load(Ordering::SeqCst)
	}

	pub(crate) unsafe fn location<'a, A: Allocator>(&self) -> (&'a Archetype<A>, u32) {
		(
			&*((*self.location).archetype.load(Ordering::Relaxed) as *const Archetype<A>),
			(*self.location).index.load(Ordering::Relaxed)
		)
	}
}

impl cmp::PartialOrd for Entity {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(cmp::Ord::cmp(self, other))
	}
}

impl cmp::Ord for Entity {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		cmp::Ord::cmp(&self.location, &other.location)
			.then(cmp::Ord::cmp(&self.generation, &other.generation))
	}
}

impl fmt::Debug for Entity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut hasher = xxhash_rust::xxh3::Xxh3::new();
		self.hash(&mut hasher);
		let id = hasher.finish() & 0xFFFF;

		if self.is_dirty() {
			write!(f, "Entity#{:04X}[DELETED]", id)
		} else {
			write!(f, "Entity#{:04X}", id)
		}
	}
}

impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[cfg(feature = "entity-uid")]
pub type EntityUid = u128;

pub(crate) struct EntityComponent {
	// This pointer is guaranteed to always be valid
	location: *const EntityLocation,
	#[cfg(feature = "entity-uid")]
	uid:      EntityUid
}

impl EntityComponent {
	pub const INFO: TypeInfo = TypeInfo::of::<Self>();

	pub fn entity(&self) -> Entity {
		Entity {
			location:   self.location,
			generation: unsafe { &*self.location }.generation.load(Ordering::Relaxed)
		}
	}

	#[cfg(feature = "entity-uid")]
	pub fn uid(&self) -> EntityUid {
		self.uid
	}
}

unsafe impl Send for EntityComponent {}
unsafe impl Sync for EntityComponent {}

#[derive(Debug)]
struct EntityLocation {
	archetype:  AtomicPtr<()>,
	index:      AtomicU32,
	generation: AtomicU32,
}

impl EntityLocation {
	unsafe fn init<A: Allocator>(self: *const Self, archetype: *const Archetype<A>, idx: u32) -> Entity {
		(*self).archetype.store(archetype as _, Ordering::Relaxed);
		(*self).index.store(idx, Ordering::Relaxed);
		Entity { location: self, generation: (*self).generation.load(Ordering::Relaxed) }
	}

	unsafe fn load<'a, A: Allocator>(self: *const Self) -> Option<&'a Archetype<A>> {
		((*self).archetype.load(Ordering::Acquire) as *const Archetype<A>).as_ref()
	}

	unsafe fn update<A: Allocator>(
		self:         *const Self,
		current_type: &mut Option<&Archetype<A>>,
		current_idx:  &mut u32,
		new_type:     *const Archetype<A>,
		new_idx:      u32
	) -> bool {
		let __current_type__ = current_type.map_or(ptr::null_mut(), |v| v as *const _ as _);

		if let Err(v) = (*self).archetype.compare_exchange(__current_type__, new_type as _, Ordering::Release, Ordering::Relaxed) {
			*current_type = (v as *mut Archetype<A>).as_ref();
			false
		} else {
			*current_idx = (*self).index.swap(new_idx, Ordering::AcqRel);
			true
		}
	}

	unsafe fn delete(self: *const Self) {
		(*self).generation.fetch_add(1, Ordering::SeqCst);
	}
}

impl cmp::PartialEq for EntityLocation {
	fn eq(&self, other: &Self) -> bool {
		core::ptr::eq(self, other)
	}
}

impl cmp::Eq for EntityLocation {}

impl cmp::PartialOrd for EntityLocation {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		cmp::PartialOrd::partial_cmp(&(self as *const Self), &(other as *const Self))
	}
}

impl cmp::Ord for EntityLocation {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		cmp::Ord::cmp(&(self as *const Self), &(other as *const Self))
	}
}

impl fmt::Display for EntityLocation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut hasher = xxhash_rust::xxh3::Xxh3::new();
		Entity { location: self, generation: self.generation.load(Ordering::Relaxed) }.hash(&mut hasher);
		let hash = hasher.finish();

		if !self.archetype.load(Ordering::Relaxed).is_null() {
			write!(f, "#{:016X}[...] @ #{:016X}#{:04}",
				   hash,
				   unsafe { &*(self.archetype.load(Ordering::Relaxed) as *mut Archetype<GlobalChunkAlloc>) }.get_hash(),
				   self.index.load(Ordering::Relaxed))
		} else {
			write!(f, "#{:016X}[DELETED]", hash)
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Subscriber {
	data:       *const SubscriberData,
	generation: u32
}

#[derive(Debug)]
struct SubscriberData {
	closure:    *const dyn Fn(),
	generation: AtomicU32
}

impl SubscriberData {
	unsafe fn call<A: Allocator + Clone>(self: *const Self, world: &World<A>, event: Event<A>) {
		(&*((*self).closure as *const dyn Fn(&World<A>, Event<A>)))(world, event)
	}
}

impl fmt::Display for SubscriberData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut hasher = xxhash_rust::xxh3::Xxh3::new();
		Subscriber { data: self, generation: self.generation.load(Ordering::Relaxed) }.hash(&mut hasher);
		let hash = hasher.finish();

		if !self.closure.is_null() {
			write!(f, "#{:016X}[{:?}]", hash, self.closure)
		} else {
			write!(f, "#{:016X}[DELETED]", hash)
		}
	}
}

pub enum Event<'a, A: Allocator> {
	ArchetypeAdded(&'a Archetype<A>),
	ArchetypeRemoved(&'a Archetype<A>),
	EntityAdded(Entity, &'a Archetype<A>),
	EntityRemoved(Entity, &'a Archetype<A>),
	EntityMoved(Entity, &'a Archetype<A>, &'a Archetype<A>)
}

impl<'a, A: Allocator> Clone for Event<'a, A> {
	fn clone(&self) -> Self {
		match self {
			Self::ArchetypeAdded(a)    => Self::ArchetypeAdded(*a),
			Self::ArchetypeRemoved(a)  => Self::ArchetypeAdded(*a),
			Self::EntityAdded(e, a)    => Self::EntityAdded(*e, *a),
			Self::EntityRemoved(e, a)  => Self::EntityRemoved(*e, *a),
			Self::EntityMoved(e, s, d) => Self::EntityMoved(*e, *s, *d)
		}
	}
}

impl<'a, A: Allocator> Copy for Event<'a, A> {}

impl<'a, A: Allocator> fmt::Debug for Event<'a, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::ArchetypeAdded(a)    => write!(f, "ArchetypeAdded({:016X})", a.get_hash()),
			Self::ArchetypeRemoved(a)  => write!(f, "ArchetypeRemoved({:016X})", a.get_hash()),
			Self::EntityAdded(e, a)    => write!(f, "EntityAdded({}, {:016X})", e, a.get_hash()),
			Self::EntityRemoved(e, a)  => write!(f, "EntityRemoved({}, {:016X})", e, a.get_hash()),
			Self::EntityMoved(e, s, d) => write!(f, "EntityMoved({}, {:016X} -> {:016X})", e, s.get_hash(), d.get_hash()),
		}
	}
}

pub trait Subscriber_<A: Allocator> {
	fn archetype_added(&self, archetype: &Archetype<A>);

	fn archetype_removed(&self, archetype: &Archetype<A>);

	fn entity_added(&self, entity: Entity, archetype: &Archetype<A>);

	fn entity_removed(&self, entity: Entity, archetype: &Archetype<A>);

	fn entity_moved(&self, entity: Entity, src: &Archetype<A>, dst: &Archetype<A>);
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct QueryCacheEntry<A: Allocator> {
	data:       *const QueryData<A>,
	generation: u32,
	iteration:  u32
}

struct QueryData<A: Allocator> {
	generation: AtomicU32,
	lock:       AtomicU32,
	iteration:  u32,
	types:      Vec<(*const Archetype<A>, !)>
}

impl<A: Allocator> QueryData<A> {
	const UNIQUE_BIT: usize = !(!0 >> 1);

	fn lock_read(&self) -> bool {
		let value = self.lock.fetch_add(1, Ordering::Acquire).wrapping_add(1);
		if value == 0 {
			panic!("borrow counter wrapped")
		} else if value & Self::UNIQUE_BIT != 0 {
			self.lock.fetch_sub(1, Ordering::Release);
			false
		} else {
			true
		}
	}

	fn lock_write(&self) -> bool {
		self.lock
			.compare_exchange(0, Self::UNIQUE_BIT, Ordering::Acquire, Ordering::Relaxed)
			.is_ok()
	}

	fn unlock_read(&self) {
		let value = self.lock.fetch_sub(1, Ordering::Release);
		debug_assert_ne!(value, 0, "unbalanced release");
		debug_assert_eq!(value & Self::UNIQUE_BIT, 0, "shared release of unique borrow");
	}

	fn unlock_write(&self) {
		let value = self.lock.fetch_and(!Self::UNIQUE_BIT, Ordering::Release);
		debug_assert_ne!(value & Self::UNIQUE_BIT, 0, "unique release of shared borrow");
	}
}

/// ## Synchronization
///
/// ### Add
///
/// ```no-run
/// LOCK DST TYPE
/// ACQUIRE ENTITY
/// STORE DST TYPE
/// STORE DST IDX
/// COPY COMPONENTS
/// UNLOCK DST TYPE
/// ```
///
/// ### Update
///
/// ```no-run
/// LOAD SRC TYPE
/// LOCK SRC TYPE    <--+
/// LOCK DST TYPE       | IF SRC TYPE INVALID
/// CMP XCG SRC TYPE ---+
/// SWAP IDX
/// COPY COMPONENTS
/// UNLOCK DST TYPE
/// IF SRC TYPE LEN == 0
/// 	DROP SRC TYPE
/// 	RELEASE SRC TYPE
/// ELSE
/// 	UNLOCK SRC TYPE
/// ```
///
/// ### Remove
///
/// ```no-run
/// LOAD SRC TYPE
/// LOCK SRC TYPE    <--+ IF SRC TYPE INVALID
/// CMP XCG SRC TYPE ---+
/// SWAP IDX
/// DROP COMPONENTS
/// ADD ENTITY GENERATION + 1
/// RELEASE ENTITY
/// IF SRC TYPE LEN == 0
/// 	DROP SRC TYPE
/// 	RELEASE SRC TYPE
/// ELSE
/// 	UNLOCK SRC TYPE
/// ```
#[allow(clippy::tabs_in_doc_comments)]
#[derive(Debug)]
pub struct World<A: Allocator + Clone = GlobalChunkAlloc> {
	entities:    PoolAlloc<EntityLocation, A>,
	archetypes:  PoolAlloc<Archetype<A>, A>,
	subscribers: PoolAlloc<SubscriberData, A>,
	queries:     PoolAlloc<QueryData<A>, A>,
	// convert to atomic binary trees with chunks as leaves
	types_cache: Mutex<collections::BTreeMap<u64, *const Archetype<A>>>,
	query_cache: Mutex<collections::BTreeMap<TypeId, *const QueryData<A>>>,
	iteration:   AtomicUsize,
	sub_count:   AtomicUsize,
	layout:      Layout,
	alloc:       A
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl Default for World {
	fn default() -> Self {
		Self::new()
	}
}

impl World {
	pub fn new() -> Self {
		Self::with_layout_in(DEFAULT_LAYOUT, GlobalChunkAlloc)
	}
}

impl<A: Allocator + Clone> World<A> {
	/// Returns the archetype with the given types, or allocates a new one if none matches.
	pub(crate) unsafe fn archetype(&self, types: impl IntoIterator<Item: AsRef<TypeInfo>> + Clone) -> &Archetype<A> {
		let hash = types.clone().into_iter()
			.fold(0u64, |hash, ty| hash.wrapping_mul(HASH_PRIME)
				.wrapping_add(mem::transmute::<_, u64>(ty.as_ref().id)));

		let cache = self.types_cache.lock()
			.expect("failed to lock types cache");

		match cache.entry(hash) {
			collections::btree_map::Entry::Occupied(entry) => &**entry.get(),
			collections::btree_map::Entry::Vacant(entry) => {
				let (hash, types) = prepare_types(types);
				let archetype = self.archetypes.acquire();
				archetype.write(Archetype::new(types, hash, self.alloc.clone(), self.layout));
				entry.insert(archetype);
				cache.insert(hash, archetype);
				self.increment_iteration();
				self.emit_event(Event::ArchetypeAdded(&*archetype));
				&*archetype
			}
		}
	}

	/// Returns all archetypes
	pub(crate) fn archetypes(&self) -> ArchetypesIter<A> {
		ArchetypesIter { iter: self.archetypes.iter(), guard: self.types_cache.lock()
			.expect("failed to lock types cache") }
	}

	/// Returns all archetypes that contain the given types.
	pub(crate) fn archetypes_filtered<
		'a,
		I: IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator> + Copy + 'a,
		E: IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator> + Copy + 'a
	>(&'a self, include: I, exclude: E) -> FilteredArchetypesIter<'a, A, I, E> {
		FilteredArchetypesIter { iter: self.archetypes(), include, exclude }
	}

	pub(crate) unsafe fn query_cache<'a, T: QueryItem<'a>>(&self, info: &QueryInfo) -> QueryCacheEntry<A> {
		use collections::btree_map::Entry;
		let iteration = self.get_iteration() as u32;

		match self.query_cache.lock()
			.expect("failed to lock query cache")
			.entry(TypeId::of::<T>())
		{
			Entry::Occupied(entry) => QueryCacheEntry {
				data:       *entry.get(),
				generation: (**entry.get()).generation.load(Ordering::Relaxed),
				iteration
			},
			Entry::Vacant(entry) => {
				let ty = self.queries.acquire();
				ptr::write(ty, QueryData {
					generation: AtomicU32::new(*(*ty).generation.get_mut()),
					lock:       AtomicU32::new(0),
					iteration,
					types:      mem::transmute(self.archetypes_filtered(info.includes(), info.excludes())
						.filter(|v| T::Fetch::filter(v))
						.map(|v| (v as *const Archetype<A>, T::Fetch::new(self, v)))
						.collect::<Vec<_>>())
				});

				entry.insert(ty);

				QueryCacheEntry {
					data:       ty,
					generation: (*ty).generation.load(Ordering::Relaxed),
					iteration
				}
			}
		}
	}

	pub(crate) unsafe fn query_cache_lock<'a, T: QueryItem<'a>>(&self, info: &QueryInfo, mut entry: QueryCacheEntry<A>) -> QueryCacheEntry<A> {
		let data = &*entry.data;

		if entry.generation != data.generation.load(Ordering::Relaxed) {
			return self.query_cache::<T>(info);
		}

		loop {
			if !data.lock_read() { // another thread is already updating the cache
				while !data.lock_read() {}

				if data.iteration != self.get_iteration() {
					log::warn!("query cache out of sync!");
				}

				entry.iteration = data.iteration;
				return entry;
			}

			if entry.iteration == data.iteration {
				data.unlock_read();
				return entry;
			}

			data.unlock_read();

			if !data.lock_write() { // another thread is already updating the cache
				continue;
			}

			data.iteration = self.get_iteration();
			let types = mem::transmute::<_, &mut Vec<(*const Archetype<A>, )>>(&mut data.types);
			types.clear();
			types.extend(self.archetypes_filtered(info.includes(), info.excludes())
				.filter(|v| T::Fetch::filter(v))
				.map(|v| (v as *const Archetype<A>, T::Fetch::new(self, v))));

			data.unlock_write();
			while !data.lock_read() {}

			entry.iteration = data.iteration;
			return entry;
		}
	}

	pub fn new_in(alloc: A) -> Self {
		Self::with_layout_in(DEFAULT_LAYOUT, alloc)
	}

	pub fn with_layout_in(layout: Layout, alloc: A) -> Self {
		Self {
			entities:    PoolAlloc::with_capacity_in(1, layout, alloc.clone()),
			archetypes:  PoolAlloc::with_capacity_in(1, layout, alloc.clone()),
			subscribers: PoolAlloc::with_capacity_in(1, layout, alloc.clone()),
			queries:     PoolAlloc::with_capacity_in(1, layout, alloc.clone()),
			types_cache: Mutex::new(collections::BTreeMap::new()),
			query_cache: Mutex::new(collections::BTreeMap::new()),
			iteration:   AtomicUsize::new(1),
			sub_count:   AtomicUsize::new(0),
			layout,
			alloc
		}
	}

	pub fn query<'a, T: QueryItem<'a>>(&'a self) -> DirectQuery<'a, T, A> {
		DirectQuery::new(self)
	}

	pub fn entry(&self, entity: Entity) -> Option<Entry<A>> {
		self.contains(entity).then(|| Entry::new(self, entity))
	}

	/// Returns the world's current iteration.
	pub fn get_iteration(&self) -> usize {
		self.iteration.load(Ordering::Relaxed)
	}

	fn increment_iteration(&self) {
		self.iteration.fetch_add(1, Ordering::SeqCst);
	}

	pub fn contains(&self, entity: Entity) -> bool {
		!entity.is_dirty()
	}

	pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
		self.has_bundle::<(T,), 1>(entity)
	}

	pub fn has_bundle<T: ComponentBundle<N>, const N: usize>(&self, entity: Entity) -> bool {
		self.has_types_iter(entity, T::TYPES.iter().map(|v| v.id))
	}

	pub fn has_types_iter(&self, entity: Entity, types: impl IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator>) -> bool {
		if !self.contains(entity) {
			return false;
		}

		let mut types = types.into_iter().collect::<Vec<_>>();
		types.sort_unstable();

		unsafe { &*((*entity.location).archetype
			.load(Ordering::Relaxed)
			.cast::<Archetype<A>>()) }
			.has_types(types)
	}

	pub fn create_entity(&self) -> Entry<A> {
		let entity = self.add_entity(());
		self.entry(entity).unwrap()
	}

	pub fn create_entities(&self) -> impl Iterator<Item = Entry<A>> {
		iter::repeat_with(move || self.create_entity())
	}

	pub fn add_entity<T: ComponentBundle<N>, const N: usize>(&self, bundle: T) -> Entity {
		self.add_entities(iter::once(bundle)).next().unwrap()
	}

	pub fn add_entities<'a, I: IntoIterator<Item: ComponentBundle<N>> + 'a, const N: usize>(&'a self, bundles: I) -> impl Iterator<Item = Entity> + 'a {
		unsafe {
			let archetype = self.archetype(iter::once(EntityComponent::INFO)
				.chain(I::Item::TYPES.iter().copied()));

			iter::from_fn(move || Some(self.entities.acquire()))
				.zip(bundles)
				.zip(archetype.add_aos())
				.map(move |((location, bundle), (idx, mut iter))| {
					let entity = location.init(archetype, idx);
					self.emit_event(Event::EntityAdded(entity, archetype));

					(*iter.next()
						.unwrap_unchecked().1
						.cast::<EntityComponent>())
						.location = location;

					iter.zip(bundle.into_iter())
						.for_each(|((ty, dst), (_, src))| dst.copy_from_nonoverlapping(src, ty.layout.size()));

					entity
				})
		}
	}

	pub fn add_entities_soa<'a, C: 'a + Components<N>, const N: usize>(&'a self, components: C) -> impl ExactSizeIterator<Item = Entity> + 'a {
		unsafe {
			let archetype = self.archetype(iter::once(EntityComponent::INFO)
				.chain(C::TYPES.iter().copied()));
			let (dst_idx, mut dst_iter) = archetype.add_soa(components.len());

			#[allow(clippy::needless_collect)]
			let entities = dst_iter.next()
				.unwrap_unchecked().1
				.zip(dst_idx)
				.map(move |(ptr, idx)| {
					let entity = self.entities.acquire().init(archetype, idx);
					(*(ptr as *mut EntityComponent)).location = entity.location;
					self.emit_event(Event::EntityAdded(entity, archetype));
					entity
				})
				.collect::<Vec<_>>();

			dst_iter.zip(components.into_iter())
				.for_each(|((ty, dst), (_, src))| src.zip(dst)
					.for_each(|(src, dst)| dst.copy_from_nonoverlapping(src, ty.layout.size())));

			entities.into_iter()
		}
	}

	pub fn add_entities_packed<'a, C: 'a + PackedComponents<N>, const N: usize>(&'a self, components: C) -> impl ExactSizeIterator<Item = Entity> + 'a {
		unsafe {
			let archetype = self.archetype(iter::once(EntityComponent::INFO)
				.chain(C::TYPES.iter().copied()));
			let (mut dst_idx, mut dst_iter) = archetype.add_packed(components.len());
			let mut rem = components.len();

			/*let entities = dst_iter.next()
				.unwrap_unchecked().1
				.flat_map(move |chunk| {
					let mut chunk = chunk.as_mut_ptr() as *mut EntityComponent;
					iter::from_fn(|| {
						if rem == 0 {
							return None;
						}

						let entity = self.entities.acquire()
							.new(archetype, (&mut*dst_idx.get()).next().unwrap_unchecked());

						(*chunk).location = entity.location;
						chunk = chunk.add(1);
						rem -= 1;
						Some(entity)
					})
				})
				.collect::<Vec<_>>();*/

			let mut entities = Vec::with_capacity(components.len());

			for chunk in dst_iter.next().unwrap_unchecked().1 {
				let mut chunk = chunk.as_mut_ptr() as *mut EntityComponent;

				while rem > 0 {
					let entity = self.entities.acquire()
						.init(archetype, dst_idx.next().unwrap_unchecked());

					(*chunk).location = entity.location;
					chunk = chunk.add(1);
					rem -= 1;
					self.emit_event(Event::EntityAdded(entity, archetype));
					entities.push(entity)
				}
			}

			dst_iter.zip(components.to_ptrs())
				.for_each(|((_ty, dst), src)| {
					let mut ptr = src.as_mut_ptr();
					let end = ptr.add(src.len());

					for chunk in dst {
						let len = chunk.len().min(end.offset_from(ptr) as _);
						ptr::copy_nonoverlapping(ptr, chunk.as_mut_ptr(), len);
						ptr = ptr.add(len);
					}
				});

			entities.into_iter()
		}
	}

	pub fn add_component<T: Component>(&self, entity: Entity, component: T) {
		self.add_bundle(entity, (component,));
	}

	pub fn add_bundle<T: ComponentBundle<N>, const N: usize>(&self, entity: Entity, bundle: T) -> bool {
		unsafe {
			let mut bundle_iter = bundle.into_iter();
			let mut bundle_next = bundle_iter.next();
			let mut src_type    = entity.location.load();

			loop {
				let src_type_ = match (src_type, entity.is_dirty()) {
					(Some(v), false) => v,
					_ => return false
				};

				let mut src_idx         = !0;
				let src_idx_ptr         = &mut src_idx as *mut u32;
				let mut src_guard       = src_type_.move_aos(iter::once(&src_idx).copied());
				let dst_type            = self.archetype(src_type_.get_types()
					.chain(T::TYPES.iter().copied()));
				let mut dst_guard       = dst_type.add_aos();
				let (dst_idx, dst_iter) = dst_guard.next().unwrap_unchecked();

				if !entity.location.update(&mut src_type, &mut *src_idx_ptr, dst_type, dst_idx) {
					continue;
				}

				let mut src_iter = src_guard.next().unwrap_unchecked();
				let mut src_next = src_iter.next();

				for (dst_ty, dst_ptr) in dst_iter {
					match (bundle_next, src_next) {
						(_, Some((src_ty, src_ptr))) if src_ty.id == dst_ty.id => {
							ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_ty.layout.size());
							src_next = src_iter.next();
						}
						(Some((src_ty, src_ptr)), _) if src_ty.id == dst_ty.id => {
							ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_ty.layout.size());
							bundle_next = bundle_iter.next();
						}
						_ => unreachable!()
					}
				}

				self.emit_event(Event::EntityMoved(entity, src_type.unwrap_unchecked(), dst_type));
				return true;
			}
		}
	}

	pub fn add_bundles<I: IntoIterator<Item = (Entity, T)>, T: ComponentBundle<N>, const N: usize>(&self, bundles: I) -> bool {
		bundles.into_iter()
			.map(|(entity, bundle)| self.add_bundle(entity, bundle))
			.all(core::convert::identity)
	}

	pub fn remove_entity(&self, entity: Entity) -> bool {
		unsafe {
			let mut archetype = entity.location.load::<A>();

			loop {
				let archetype_ = match (archetype, entity.is_dirty()) {
					(Some(v), false) => v,
					_ => return false
				};

				let mut idx    = [!0];
				let idx_ptr    = &mut idx[0] as *mut u32;
				let mut remove = archetype_.remove_soa_deferred(&idx);

				if !entity.location.update(&mut archetype, &mut *idx_ptr, ptr::null_mut(), !0) {
					continue;
				}

				remove.remove();
				entity.location.delete();
				self.entities.release(entity.location);
				self.emit_event(Event::EntityRemoved(entity, archetype_));
				return true;
			}
		}
	}

	pub fn remove_entities(&self, entities: impl IntoIterator<Item = Entity>) -> bool {
		entities.into_iter()
			.map(|entity| self.remove_entity(entity))
			.all(core::convert::identity)
	}

	pub fn remove_component<T: Component>(&self, entity: Entity) -> Option<T> {
		self.remove_bundle(entity).map(|(v,)| v)
	}

	pub fn remove_bundle<T: ComponentBundle<N>, const N: usize>(&self, entity: Entity) -> Option<T> {
		unsafe {
			let mut bundle_iter = T::from_iter();
			let mut bundle_next = bundle_iter.next();
			let mut src_type    = entity.location.load::<A>();

			loop {
				let src_type_ = match (src_type, entity.is_dirty()) {
					(Some(v), false) => v,
					_ => return None
				};

				let mut src_idx             = !0;
				let src_idx_ptr             = &mut src_idx as *mut u32;
				let mut src_guard           = src_type_.move_aos(iter::once(&src_idx).copied());
				let dst_type                = self.archetype(src_type_.get_types()
					.filter(|t| !T::TYPES.contains(t)));
				let mut dst_guard           = dst_type.add_aos();
				let (dst_idx, mut dst_iter) = dst_guard.next().unwrap_unchecked();

				if !entity.location.update(&mut src_type, &mut *src_idx_ptr, dst_type, dst_idx) {
					continue;
				}

				let src_iter     = src_guard.next().unwrap_unchecked();
				let mut dst_next = dst_iter.next();

				for (src_ty, src_ptr) in src_iter {
					match (dst_next, bundle_next) {
						(Some((dst_ty, dst_ptr)), _) if src_ty.id == dst_ty.id => {
							ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_ty.layout.size());
							dst_next = dst_iter.next();
						}
						(_, Some((dst_ty, dst_ptr))) if src_ty.id == dst_ty.id => {
							ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_ty.layout.size());
							bundle_next = bundle_iter.next();
						}
						_ => unreachable!()
					}
				}

				self.emit_event(Event::EntityMoved(entity, src_type_, dst_type));
				return Some(bundle_iter.get());
			}
		}
	}

	pub fn remove_bundles<'a, T: ComponentBundle<N>, const N: usize, I: IntoIterator<Item = Entity, IntoIter: 'a>>(&'a self, entities: I) -> impl Iterator<Item = Option<T>> + 'a {
		entities.into_iter().map(move |entity| self.remove_bundle(entity))
	}

	pub fn modify_entity<ADD: ComponentBundle<AN>, REM: ComponentBundle<RN>, const AN: usize, const RN: usize>(&self, entity: Entity, bundle: ADD) -> Result<REM, ADD> {
		unsafe {
			let mut add_iter = bundle.into_iter();
			let mut add_next = add_iter.next();
			let mut rem_iter = REM::from_iter();
			let mut rem_next = rem_iter.next();
			let mut src_type = entity.location.load();

			loop {
				let src_type_ = match (src_type, entity.is_dirty()) {
					(Some(v), false) => v,
					_ => return Err(add_iter.get())
				};

				let mut src_idx             = !0;
				let src_idx_ptr             = &mut src_idx as *mut u32;
				let mut src_guard           = src_type_.move_aos(iter::once(&src_idx).copied());
				let dst_type                = self.archetype(src_type_.get_types()
					.filter(|t| !REM::TYPES.contains(t))
					.chain(ADD::TYPES.iter().copied()));
				let mut dst_guard           = dst_type.add_aos();
				let (dst_idx, mut dst_iter) = dst_guard.next().unwrap_unchecked();

				if !entity.location.update(&mut src_type, &mut *src_idx_ptr, dst_type, dst_idx) {
					continue;
				}

				let mut src_iter = src_guard.next().unwrap_unchecked();
				let mut src_next = src_iter.next();
				let mut dst_next = dst_iter.next();

				loop {
					match (src_next, add_next, dst_next, rem_next) {
						(Some((src_ty, src_ptr)), _, Some((dst_ty, dst_ptr)), _) if src_ty.id == dst_ty.id => {
							ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_ty.layout.size());
							src_next = src_iter.next();
							dst_next = dst_iter.next();
						}
						(Some((src_ty, src_ptr)), _, _, Some((dst_ty, dst_ptr))) if src_ty.id == dst_ty.id => {
							ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_ty.layout.size());
							src_next = src_iter.next();
							rem_next = rem_iter.next();
						}
						(_, Some((src_ty, src_ptr)), Some((dst_ty, dst_ptr)), _) if src_ty.id == dst_ty.id => {
							ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_ty.layout.size());
							add_next = add_iter.next();
							dst_next = dst_iter.next();
						}
						(None, None, None, None) => break,
						_ => unreachable!()
					}
				}

				self.emit_event(Event::EntityMoved(entity, src_type_, dst_type));
				return Ok(rem_iter.get());
			}
		}
	}

	pub fn modify_entities<
		'a,
		ADD: ComponentBundle<AN>,
		REM: ComponentBundle<RN>,
		I:   IntoIterator<Item = (Entity, ADD), IntoIter: 'a>,
		const AN: usize,
		const RN: usize
	>(&'a self, iter: I) -> impl Iterator<Item = Result<REM, ADD>> + 'a {
		iter.into_iter().map(move |(entity, bundle)| self.modify_entity(entity, bundle))
	}

	/// Deletes all entities and deallocates all archetypes
	pub fn clear(&self) {
		let mut types_cache = self.types_cache.lock().expect("failed to lock types cache");
		let mut query_cache = self.types_cache.lock().expect("failed to lock query cache");

		unsafe {
			for entity in self.entities.iter() {
				entity.delete();
				self.emit_event(Event::EntityRemoved(Entity {
					location: entity, generation: (*entity).generation.load(Ordering::Relaxed)
				}, entity.load().unwrap_unchecked()));
			}

			self.entities.clear();

			for archetype in self.archetypes.iter() {
				mem::forget((*archetype).add_aos());
				self.emit_event(Event::ArchetypeRemoved(&*archetype));
				ptr::drop_in_place(archetype as *mut Archetype<A>);
			}

			self.archetypes.clear();
		}

		types_cache.clear();
		query_cache.clear();
		self.increment_iteration();
	}

	pub fn repack(&self, mut limit: usize) {
		let mut cache = self.types_cache.lock().expect("failed to lock types cache");

		for archetype in self.archetypes.iter() {
			unsafe {
				let archetype = &*archetype;
				let (moved, _, chunks) = archetype.repack(limit);
				limit -= moved;

				if chunks > 0 {
					continue;
				}

				cache.retain(|_, v| !ptr::eq(*v, archetype));
				self.emit_event(Event::ArchetypeRemoved(archetype));
				//mem::forget(guard); // don't unlock the archetype
				ptr::drop_in_place(archetype as *const _ as *mut Archetype<A>);
				self.archetypes.release(archetype);
			}
		}

		self.increment_iteration();
	}

	fn emit_event(&self, event: Event<A>) {
		if self.sub_count.load(Ordering::Relaxed) > 0 {
			self.subscribers.iter()
				.for_each(|v| unsafe { v.call(self, event) });
		}
	}

	pub fn subscribe(&self, f: impl Fn(&Self, Event<A>) + 'static) -> Subscriber {
		unsafe {
			self.sub_count.fetch_add(1, Ordering::Relaxed);
			let data = self.subscribers.acquire();
			(*data).closure = Box::leak(Box::new(f) as Box<dyn Fn(&Self, Event<A>)>)
				as *mut dyn Fn(&Self, Event<A>) as _;
			Subscriber { data, generation: (*data).generation.load(Ordering::Relaxed) }
		}
	}

	pub fn unsubscribe(&self, subscriber: Subscriber) -> bool {
		unsafe {
			if (*subscriber.data).generation.compare_exchange(
				subscriber.generation, subscriber.generation + 1, Ordering::Relaxed, Ordering::Relaxed).is_err() {
				return false;
			}

			let closure = mem::replace(
				&mut (*(subscriber.data as *mut SubscriberData)).closure, mem::transmute([0usize; 2]));
			mem::drop(Box::from_raw(closure as *mut dyn Fn(&Self, Event<A>)));
			self.subscribers.release(subscriber.data);
			self.sub_count.fetch_sub(1, Ordering::Relaxed);
			true
		}
	}
}

impl<A: Allocator + Clone> fmt::Display for World<A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "ITERATION: {}, LAYOUT SIZE: {}, LAYOUT ALIGN: {}, ALLOC: {}",
			self.get_iteration(),
			self.layout.size(),
			self.layout.align(),
			core::any::type_name::<A>())?;

		writeln!(f, "\nENTITIES:")?;

		for location in self.entities.iter() {
			writeln!(f, "{}", unsafe { &*location })?;
		}

		writeln!(f, "\nARCHETYPES:")?;

		for archetype in self.archetypes.iter() {
			writeln!(f, "{}", unsafe { &*archetype })?;
		}

		writeln!(f, "\nSUBSCRIBERS:")?;

		for subscriber in self.subscribers.iter() {
			writeln!(f, "{}", unsafe { &*subscriber })?;
		}

		Ok(())
	}
}

pub(crate) struct ArchetypesIter<'a, A: Allocator> {
	iter:  PoolAllocIter<'a, Archetype<A>, A>,
	guard: MutexGuard<'a, collections::BTreeMap<u64, *const Archetype<A>>>
}

impl<'a, A: Allocator> Iterator for ArchetypesIter<'a, A> {
	type Item = &'a Archetype<A>;

	fn next(&mut self) -> Option<Self::Item> {
		// SAFE: `types_cache` is locked
		self.iter.next().map(|ptr| unsafe { &*ptr })
	}
}

pub(crate) struct FilteredArchetypesIter<
	'a,
	A: Allocator,
	I: IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator> + Copy,
	E: IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator> + Copy
> {
	iter:    ArchetypesIter<'a, A>,
	include: I,
	exclude: E
}

impl<
	'a,
	A: Allocator,
	I: IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator> + Copy,
	E: IntoIterator<Item = TypeId, IntoIter: ExactSizeIterator> + Copy
> Iterator for FilteredArchetypesIter<'a, A, I, E> {
	type Item = &'a Archetype<A>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.iter.next() {
				Some(v) if v.filter(self.include, self.exclude) => return Some(v),
				Some(_) => continue,
				None    => return None
			}
		}
	}
}

struct DropIter<'a, I: ExactSizeIterator + 'a, F: FnOnce() + 'a>(I, Option<F>, PhantomData<&'a ()>);

impl<'a, I: ExactSizeIterator + 'a, F: FnOnce() + 'a> Iterator for DropIter<'a, I, F> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl<'a, I: ExactSizeIterator + 'a, F: FnOnce() + 'a> ExactSizeIterator for DropIter<'a, I, F> {}

impl<'a, I: ExactSizeIterator + 'a, F: FnOnce() + 'a> Drop for DropIter<'a, I, F> {
	fn drop(&mut self) {
		(self.1.take().unwrap())();
	}
}

fn prepare_types(types: impl IntoIterator<Item: AsRef<TypeInfo>>) -> (u64, impl Iterator<Item = TypeInfo>) {
	let mut hasher = xxhash_rust::xxh3::Xxh3::new();
	let mut buf    = [TypeInfo::of::<()>(); 64];
	let mut idx    = 0;

	for ty in types {
		buf[idx] = *ty.as_ref();
		idx += 1;
	}

	buf[1..idx].sort_unstable();
	buf[..idx].iter()
		.for_each(|ty| core::hash::Hash::hash(&ty.id, &mut hasher));

	(hasher.finish(), <[TypeInfo; 64] as IntoIterator>::into_iter(buf).take(idx))
}