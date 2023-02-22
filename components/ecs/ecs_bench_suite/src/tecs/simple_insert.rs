use cgmath::*;
use tecs::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
	pub fn new() -> Self {
		Self
	}
	
	pub fn run(&mut self) {
		let world = World::new();
		
		/*world.add_entities_soa(10_000, (
			std::iter::repeat(Transform(Matrix4::from_scale(1.0))).take(10_000),
			std::iter::repeat(Position(Vector3::unit_x())).take(10_000),
			std::iter::repeat(Rotation(Vector3::unit_x())).take(10_000),
			std::iter::repeat(Velocity(Vector3::unit_x())).take(10_000),
		));*/
		
		world.add_entities((0..10_000).map(|_| (
			Transform(Matrix4::from_scale(1.0)),
			Position(Vector3::unit_x()),
			Rotation(Vector3::unit_x()),
			Velocity(Vector3::unit_x()),
		)));
	}
}
