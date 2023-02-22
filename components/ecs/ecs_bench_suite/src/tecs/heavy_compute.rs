use cgmath::*;
use tecs::*;
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(Arc<World>);

impl Benchmark {
	pub fn new() -> Self {
		let world = Arc::new(World::default());
		
		world.add_entities((0..1000).map(|_| {
			(
				Matrix4::<f32>::from_angle_x(Rad(1.2)),
				Position(Vector3::unit_x()),
				Rotation(Vector3::unit_x()),
				Velocity(Vector3::unit_x()),
			)
		}));
		
		Self(world)
	}
	
	pub fn run(&mut self) {
		self.0.query::<(&mut Position, &mut Matrix4<f32>)>()
			.iter_batched(64)
			.par_bridge()
			.for_each(|batch| {
				for (mut pos, mat) in batch {
					for _ in 0..100 {
						*mat = mat.invert().unwrap();
					}
					
					pos.0 = mat.transform_vector(pos.0);
				}
			});
	}
}
