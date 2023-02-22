use tecs::*;

struct A(f32);
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
	pub fn new() -> Self {
		let world = World::default();
		
		let entities = world
			.add_entities((0..10000).map(|_| (A(0.0),)))
			.collect::<Vec<_>>();
		
		Self(world, entities)
	}
	
	pub fn run(&mut self) {
		for entity in &self.1 {
			self.0.add_component(*entity, B(0.0));
		}
		
		for entity in &self.1 {
			self.0.remove_component::<B>(*entity).unwrap();
		}
	}
}
