use cgmath::*;
use tecs::*;
use std::sync::Arc;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(Arc<World>);

impl Benchmark {
	pub fn new() -> Self {
		let world = Arc::new(World::new());
		world.add_entities((0..10_000).map(|_| (
			Transform(Matrix4::from_scale(1.0)),
			Position(Vector3::unit_x()),
			Rotation(Vector3::unit_x()),
			Velocity(Vector3::unit_x()),
		)));
		
		Self(world)
	}
	
	pub fn run(&mut self) {
		for (velocity, position) in self.0.query::<(&Velocity, &mut Position)>().iter() {
			position.0 += velocity.0;
		}
	}
}
