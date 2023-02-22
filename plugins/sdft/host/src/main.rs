
use math::*;

pub type f16 = i16;

pub struct Context {
	sdfs:                   Vec<Sdf>,
	materials:              Vec<Material>,
	instances:              Vec<Instance>,
	surface_radiance_cache: (),
	scene_radiance_cache:   ClipMap<SceneProbe<32>>,
	scene_sdf:              ClipMap<(f16, u16)>,
	camera:                 Camera,
	params:                 TraceParams
}

pub struct Camera {
	pos:               Vec3<f32>,
	lower_left_corner: Vec3<f32>,
	horizontal:        Vec3<f32>,
	vertical:          Vec3<f32>
}

pub struct TraceParams {
	extent:            Vec2<f32>,
	min_step_size:     f32,
	min_dist:          f32,
	min_dist_global:   f32,
	max_dist:          f32,
	min_steps:         u32,
	max_steps:         u32,
	normal_polls:      u32,
	normal_smoothness: f32,
	features:          u32,
	no_hit_color:      Vec3<f32>
}

pub struct Sdf {
	pub data: Box<[f16; 2]>,
	pub x:    usize,
	pub y:    usize,
	pub z:    usize
}

pub struct Material {
	normal_smoothness: f32
}

pub struct Instance {
	pub transform: Mat4<f32>,
	pub mat:       u16,
	pub sdf:       u16
}

pub struct SurfaceProbe<const R: usize>(pub [[SurfaceProbeTexel; R]; R]);

pub struct SurfaceProbeTexel {
	pub radiance:  [u16; 3],
	pub depth_min: f16,
}

pub struct SceneProbe<const R: usize>(pub [[SceneProbeTexel; R]; R]);

pub struct SceneProbeTexel {
	pub radiance:   [u16; 3],
	pub depth_mean: f16,
	pub depth_std:  f16
}

pub struct ClipMap<T> {
	pub res:    usize,
	pub levels: usize,
	pub pos_x:  usize,
	pub pos_y:  usize,
	pub pos_z:  usize,
	pub data:   Box<[T]>
}

fn sdf_lookup(sdf: &Sdf, pos: Vec3<f32>) -> f32 {
	unimplemented!()
}

/// Calculates the normal at the given point in an sdf.
fn sdf_normal(
	sdf:        &Sdf,
	pos:        Vec3<f32>,
	smoothness: f32,
) -> Vec3<f32> {
	let eps = Vec3(smoothness, 0.0, 0.0);
	Vec3(
		sdf_lookup(sdf, pos - eps.xyz()) - sdf_lookup(sdf, pos + eps.xyz()),
		sdf_lookup(sdf, pos - eps.zxy()) - sdf_lookup(sdf, pos + eps.zxy()),
		sdf_lookup(sdf, pos - eps.zyx()) - sdf_lookup(sdf, pos + eps.zyx())
	)
}

fn trace(context: &Context, pos: Vec3<f32>, dir: Vec3<f32>) -> Option<(Vec3<f32>, u32)> {
	unimplemented!()
}

fn get_probes(pos: Vec3<f32>) -> Vec4<u32> {
	unimplemented!()
}

fn lookup_probe(probe: u32, uv: Vec2<u32>) -> Vec4<f32> {
	unimplemented!()
}

fn sample_probes(probes: Vec4<u32>, uv: Vec2<u32>) -> Vec3<f32> {
	// TODO sample all probes, do bilinear interpolation
	unimplemented!()
}

fn render(context: &Context, id: Vec2<u32>) -> Vec3<f32> {
	let uv  = Vec2(id.0 as f32, id.1 as f32) / context.params.extent;
	let pos = context.camera.pos;
	let dir = context.camera.lower_left_corner
		+ context.camera.horizontal * uv.x()
		+ context.camera.vertical * uv.y()
		- context.camera.pos;
	
	let (pos, id) = match trace(context, pos, dir) {
		Some(v) => v,
		None    => return context.params.no_hit_color
	};
	
	let instance = &context.instances[id as usize];
	
	let normal = sdf_normal(
		&context.sdfs[instance.sdf as usize],
		instance.transform.transform_pos(pos),
		context.materials[instance.mat as usize].normal_smoothness
	);
	
	let albedo = ();
	let (metalness, roughness) = ();
	let probes = get_probes(pos);
	let mut irradiance = Vec3::from(0.0);
	
	for x in 0..8 {
		for y in 0..8 {
			let dir_out = ();
			let radiance = sample_probes(probes, Vec2(x, y));
			let brdf = brdf(
				dir,
				dir_out,
				normal,
				albedo,
				metalness,
				roughness
			);
			
			irradiance += radiance * brdf;
		}
	}
	
	irradiance
}

fn brdf(
	dir_in:    Vec3<f32>,
	dir_out:   Vec3<f32>,
	normal:    Vec3<f32>,
	albedo:    Vec4<f32>,
	metalness: f32,
	roughness: f32,
) -> Vec3<f32> {
	let cos_theta_out = dir_out.2.abs(); // normal.dot(out)
	let cos_theta_in = dir_in.2.abs(); // normal.dot(r#in)
	let half = dir_in + dir_out;
	
	if cos_theta_in == 0.0 || cos_theta_out == 0.0 || half == Vec3::from(0.0) {
		return Vec3::from(0.0);
	}
	
	let half = half.normalize(1.0);
	let tan2_theta = tan2_theta(half);
	if tan2_theta.is_infinite() { return 0.0; }
	let cos4_theta = half.2.powi(4);
	let e = tan2_theta * (cos_phi(half).powi(2) / alpha.0.powi(2)
		+ sin_phi(half).powi(2) / alpha.1.powi(2));
	
	let normal_distribution = (-e).exp() / (core::f32::consts::PI * alpha.0 * alpha.1 * cos4_theta);
	
	let geometric = 1.0 / (1.0 + lambda(r#in) + lambda(out));
	
	let cos_theta_in = r#in.dot(half).clamp(-1.0, 1.0);
	
	if cos_theta_in > 0.0 {
		core::mem::swap(&mut ior_in, &mut ior_tr);
		cos_theta_in = cos_theta_in.abs();
	}
	
	let sin_theta_in = 0f32.max(1.0 - cos_theta_in * cos_theta_in).sqrt();
	let sin_theta_tr = ior_in / ior_tr * sin_theta_in;
	if sin_theta_tr >= 1.0 { return 1.0; } // total internal reflection
	
	let cos_theta_tr = 0f32.max(1.0 - sin_theta_tr * sin_theta_tr).sqrt();
	let r_par_l = ((ior_tr * cos_theta_in) - (ior_in * cos_theta_tr))
		/ ((ior_tr * cos_theta_in) - (ior_in * cos_theta_tr));
	let r_per_p = ((ior_in * cos_theta_in) - (ior_tr * cos_theta_tr))
		/ ((ior_in * cos_theta_in) - (ior_tr * cos_theta_tr));
	
	let fresnel = (r_par_l * r_par_l + r_per_p * r_per_p) * 0.5;
	
	normal_distribution
		* geometric_shadowing(r#in, out)
		* fresnel(r#in.dot(half))
		/ (4.0 * cos_theta_in * cos_theta_out)
}

fn main() {
	println!("Hello, world!");
}
