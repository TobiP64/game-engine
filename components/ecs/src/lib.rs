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

#![feature(
	alloc_layout_extra,
	allocator_api,
	arbitrary_self_types,
	associated_type_bounds,
	associated_type_defaults,
	generic_const_exprs,
	const_type_id,
	const_type_name,
	slice_ptr_get,
	slice_ptr_len,
)]

#![warn(clippy::all)]
#![allow(
	incomplete_features,
	clippy::missing_safety_doc,
	clippy::needless_lifetimes,
	clippy::type_complexity
)]

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod component;
pub mod world;
pub mod archetype;
pub mod entry;
pub mod query;
pub mod resources;
pub mod system;
//pub mod schedule;
pub mod alloc;
mod utils;

pub use self::{
	query::*,
	system::*,
	//schedule::*,
	world::*,
	entry::*,
	archetype::*,
	component::*,
	alloc::*,
	resources::*,
	utils::*
};

#[cfg(test)]
mod tests {
	use {super::*, std::sync::{Arc, atomic::{AtomicUsize, Ordering}}};

	#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
	struct A(usize);

	#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
	struct B(usize);

	#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
	struct C(usize);

	#[test]
	fn get_iteration() {
		let world = World::new();
		let i = world.get_iteration();
		world.create_entity();
		assert!(world.get_iteration() > i);
	}

	#[test]
	fn has_entity_types_1() {
		let world = World::new();
		let e = world.add_entity((A(1), B(2)));
		assert!(world.has_bundle::<(A,), 1>(e));
	}

	#[test]
	fn has_entity_types_2() {
		let world = World::new();
		let e = world.add_entity((A(1), B(2)));
		assert!(world.has_bundle::<(A, B), 2>(e));
	}

	#[test]
	fn has_entity_types_3() {
		let world = World::new();
		let e = world.add_entity((A(1), B(2)));
		assert!(!world.has_bundle::<(A, B, C), 3>(e));
	}

	#[test]
	fn has_entity_types_4() {
		let world = World::new();
		let e = world.add_entity((A(1), B(2)));
		assert!(!world.has_bundle::<(C,), 1>(e));
	}

	#[test]
	fn create_entity() {
		let world = World::new();
		let counters = subscribe(&world);
		let e = world.create_entity();
		assert!(world.contains(e.entity()));
		assert_counters(counters, 1, 0, 1, 0, 0);
	}

	#[test]
	fn create_entities() {
		let world = World::new();
		let counters = subscribe(&world);
		let e = world.create_entities()
			.take(100)
			.collect::<Vec<_>>();
		assert!(e.iter().all(|e| world.contains(e.entity())));
		assert_counters(counters, 1, 0, 100, 0, 0);
	}

	#[test]
	fn add_entity() {
		let world = World::new();
		let counters = subscribe(&world);
		let e = world.add_entity((A(1), B(2)));
		let mut query = world.query::<(&A, &B)>();
		assert!(world.contains(e));
		assert_eq!(query.get(e).as_deref(), Some(&(&A(1), &B(2))));
		assert_counters(counters, 1, 0, 1, 0, 0);
	}

	#[test]
	fn add_entities() {
		let world = World::new();
		let counters = subscribe(&world);
		let e = world.add_entities(std::iter::repeat((A(1), B(2))).take(100))
			.collect::<Vec<_>>();
		let mut query = world.query::<(&A, &B)>();

		for e in e {
			assert!(world.contains(e));
			assert_eq!(query.get(e).as_deref(), Some(&(&A(1), &B(2))));
		}

		assert_counters(counters, 1, 0, 100, 0, 0);
	}

	/*#[test]
	fn add_entities_soa() {
		let world = World::new();
		let counters = subscribe(&world);
		let e     = world.add_entities_soa((
			std::iter::repeat(A(1)).take(100),
			std::iter::repeat(B(2)).take(100)
		)).collect::<Vec<_>>();
		let query = world.query::<(&A, &B)>();

		for e in e {
			assert!(world.contains(e));
			assert_eq!(query.get(e).as_deref(), Some(&(&A(1), &B(2))));
		}

		assert_counters(counters, 1, 0, 100, 0, 0);
	}

	#[test]
	fn add_entities_packed() {
		let world = World::new();
		let counters = subscribe(&world);
		let e     = world.add_entities_packed((
			vec![A(1); 100],
			vec![B(2); 100]
		)).collect::<Vec<_>>();
		let query = world.query::<(&A, &B)>();

		for e in e {
			assert!(world.contains(e));
			assert_eq!(query.get(e).as_deref(), Some(&(&A(1), &B(2))));
		}

		assert_counters(counters, 1, 0, 100, 0, 0);
	}*/

	#[test]
	fn remove_entity() {
		let world = World::new();
		let counters = subscribe(&world);
		let e     = world.create_entity();
		assert!(world.remove_entity(e.entity()));
		assert!(!world.contains(e.entity()));
		assert_counters(counters, 1, 0, 1, 1, 0);
	}

	#[test]
	fn remove_entity_invalid() {
		let world = World::new();
		let counters = subscribe(&world);
		let e     = world.create_entity().entity();
		world.remove_entity(e);
		assert!(!world.remove_entity(e));
		assert_counters(counters, 1, 0, 1, 1, 0);
	}

	#[test]
	fn remove_entities() {
		let world = World::new();
		let counters = subscribe(&world);
		let e = world.create_entities().take(100)
			.collect::<Vec<_>>();
		assert!(world.remove_entities(e.iter().map(|e| e.entity())));
		assert!(e.iter().all(|e| !world.contains(e.entity())));
		assert_counters(counters, 1, 0, 100, 100, 0);
	}

	#[test]
	fn remove_entities_invalid() {
		let world    = World::new();
		let counters = subscribe(&world);
		let e        = world.create_entities().take(100)
			.collect::<Vec<_>>();
		world.remove_entities(e.iter().map(|e| e.entity()));
		assert!(!world.remove_entities(e.iter().map(|e| e.entity())));
		assert_counters(counters, 1, 0, 100, 100, 0);
	}

	#[test]
	fn add_remove_component() {
		let world    = World::new();
		let counters = subscribe(&world);
		let e        = world.add_entity((A(1),));
		world.add_component(e, B(2));
		world.remove_component::<B>(e).unwrap();
		assert_counters(counters, 2, 0, 1, 0, 2);
	}

	#[test]
	fn clear() {
		let world    = World::new();
		let counters = subscribe(&world);
		let e        = world.create_entity().entity();
		world.clear();
		assert!(!world.contains(e));
		assert_counters(counters, 1, 1, 1, 1, 0);
	}

	#[test]
	fn query_basic_1() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let e2        = world.add_entity((A(3), B(4)));
		let e3        = world.add_entity((A(5), B(6)));
		let mut query = world.query::<(Entity, &A, &B)>();
		assert_eq!(query.iter().collect::<Vec<_>>(), vec![
			(e1, &A(1), &B(2)),
			(e2, &A(3), &B(4)),
			(e3, &A(5), &B(6)),
		]);
	}

	#[test]
	fn query_basic_2() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let _e2       = world.add_entity((B(3), C(4)));
		let e3        = world.add_entity((A(5), C(6)));
		let mut query = world.query::<(Entity, &A)>();
		assert_eq!(query.iter().collect::<Vec<_>>(), vec![
			(e3, &A(5)),
			(e1, &A(1)),
		]);
	}

	#[test]
	fn query_batched_basic_1() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let e2        = world.add_entity((A(3), B(4)));
		let e3        = world.add_entity((A(5), B(6)));
		let mut query = world.query::<(Entity, &A, &B)>();
		assert_eq!(query.iter_batched(64).flatten().collect::<Vec<_>>(), vec![
			(e1, &A(1), &B(2)),
			(e2, &A(3), &B(4)),
			(e3, &A(5), &B(6)),
		]);
	}

	#[test]
	fn query_batched_basic_2() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let _e2       = world.add_entity((B(3), C(4)));
		let e3        = world.add_entity((A(5), C(6)));
		let mut query = world.query::<(Entity, &A)>();
		assert_eq!(query.iter_batched(64).flatten().collect::<Vec<_>>(), vec![
			(e3, &A(5)),
			(e1, &A(1)),
		]);
	}

	#[test]
	fn query_cached_basic_1() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let e2        = world.add_entity((A(3), B(4)));
		let e3        = world.add_entity((A(5), B(6)));
		let mut query = world.query::<(Entity, &A, &B)>().cached();
		assert_eq!(query.iter().collect::<Vec<_>>(), vec![
			(e1, &A(1), &B(2)),
			(e2, &A(3), &B(4)),
			(e3, &A(5), &B(6)),
		]);
	}

	#[test]
	fn query_cached_basic_2() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let _e2       = world.add_entity((B(3), C(4)));
		let e3        = world.add_entity((A(5), C(6)));
		let mut query = world.query::<(Entity, &A)>().cached();
		assert_eq!(query.iter().collect::<Vec<_>>(), vec![
			(e3, &A(5)),
			(e1, &A(1)),
		]);
	}

	#[test]
	fn query_cached_batched_basic_1() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let e2        = world.add_entity((A(3), B(4)));
		let e3        = world.add_entity((A(5), B(6)));
		let mut query = world.query::<(Entity, &A, &B)>().cached();
		assert_eq!(query.iter_batched(64).flatten().collect::<Vec<_>>(), vec![
			(e1, &A(1), &B(2)),
			(e2, &A(3), &B(4)),
			(e3, &A(5), &B(6)),
		]);
	}

	#[test]
	fn query_cached_batched_basic_2() {
		let world     = World::new();
		let e1        = world.add_entity((A(1), B(2)));
		let _e2       = world.add_entity((B(3), C(4)));
		let e3        = world.add_entity((A(5), C(6)));
		let mut query = world.query::<(Entity, &A)>().cached();
		assert_eq!(query.iter_batched(64).flatten().collect::<Vec<_>>(), vec![
			(e3, &A(5)),
			(e1, &A(1)),
		]);
	}

	#[test]
	fn multi_threading() {
		let world = Arc::new(World::new());

		for _ in 0..1000 {
			let world = world.clone();
			std::thread::spawn(move || {
				for _ in 0..1000 {
					let e = world.add_entity((A(1),));

					for _ in 0..1000 {
						world.add_component(e, B(2));
						world.remove_component::<A>(e).unwrap();
						world.remove_component::<B>(e).unwrap();
						world.add_component(e, A(1));
					}
				}
			});
		}
	}

	#[test]
	fn world_display() {
		let world = World::new();
		subscribe(&world);
		world.add_entity((A(1), B(2)));
		world.add_entity((A(1), B(2)));
		world.add_entity((A(1), C(3)));
		world.add_entity((B(2), C(3)));
		world.add_entity((B(2), C(3)));
		println!("{}", world);
	}

	fn subscribe(world: &World) -> Arc<[AtomicUsize; 5]> {
		let counters = Arc::new([AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0)]);
		let cloned = counters.clone();
		world.subscribe(move |_, e| { match e {
			Event::ArchetypeAdded(..)   => cloned[0].fetch_add(1, Ordering::Relaxed),
			Event::ArchetypeRemoved(..) => cloned[1].fetch_add(1, Ordering::Relaxed),
			Event::EntityAdded(..)      => cloned[2].fetch_add(1, Ordering::Relaxed),
			Event::EntityRemoved(..)    => cloned[3].fetch_add(1, Ordering::Relaxed),
			Event::EntityMoved(..)      => cloned[4].fetch_add(1, Ordering::Relaxed)
		}; });
		counters
	}

	fn assert_counters(
		counters:       Arc<[AtomicUsize; 5]>,
		type_adds:      usize,
		type_removes:   usize,
		entity_adds:    usize,
		entity_removes: usize,
		entity_moved:   usize
	) {
		assert_eq!(counters[0].load(Ordering::Relaxed), type_adds, "ArchetypeAdded events counter");
		assert_eq!(counters[1].load(Ordering::Relaxed), type_removes, "ArchetypeRemoved events counter");
		assert_eq!(counters[2].load(Ordering::Relaxed), entity_adds, "EntityAdded events counter");
		assert_eq!(counters[3].load(Ordering::Relaxed), entity_removes, "EntityRemoved events counter");
		assert_eq!(counters[4].load(Ordering::Relaxed), entity_moved, "EntityMoved events counter");
	}
}