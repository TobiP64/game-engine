use tecs::*;
use std::sync::Arc;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            $world.add_entities((0..20).map(|_| ($variants(0.0), Data(1.0))));
        )*
    };
}

struct Data(f32);

pub struct Benchmark(Arc<World>);

impl Benchmark {
	pub fn new() -> Self {
		let world = Arc::new(World::default());
		
		create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
		
		Self(world)
	}
	
	pub fn run(&mut self) {
		for data in self.0.query::<&mut Data>().iter() {
			data.0 *= 2.0;
		}
	}
}
