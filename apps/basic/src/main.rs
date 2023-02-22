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

#![feature(allocator_api)]

use {
	std::{sync::{*, atomic::*}, time::Duration},
	wayland::*,
	pipewire::*,
	vk::*,
	engine_core::*,
	math::*,
	ecs::*,
	scene::*,
	gpgpu::*,
	plugin_ui::UiRender,
	plugin_sdft::SceneOptions
};
use gpgpu::render_graph::RenderGraph;
use gpgpu::misc::spawn_transfer_thread;
use gpgpu::plugins::Plugins;

fn main() {
	engine_core::run(None, run);
}

struct RegistryListener(Registry);

impl WlRegistryListener for RegistryListener {
	fn global(&self, proxy: &mut WlRegistry, name: u32, interface: &str, version: u32) {
		let r = (move || Ok::<_, ()>(match interface {
			"wl_compositor" => self.0.insert(proxy.bind::<WlCompositor>(name, &WL_COMPOSITOR_INTERFACE, version)?),
			"wl_shell"      => self.0.insert(proxy.bind::<WlShell>(name, &WL_SHELL_INTERFACE, version)?),
			"xdg_wm_base"   => self.0.insert(proxy.bind::<XdgWmBase>(name, &XDG_WM_BASE_INTERFACE, version)?),
			"wl_seat"       => self.0.insert(proxy.bind::<WlSeat>(name, &WL_SEAT_INTERFACE, version)?),
			"wl_output"     => self.0.insert(proxy.bind::<WlOutput>(name, &WL_OUTPUT_INTERFACE, version)?),
			_ => !0
		}))();
		
		const INV_ID: usize = !0;
		
		match r {
			Ok(INV_ID) => (),
			Ok(_)  => log::info!("[WAYLAND] bound global `{}` with interface `{}` version {}", name, interface, version),
			Err(_) => log::error!("[WAYLAND] failed to bind global `{}` with interface `{}` version {}", name, interface, version)
		}
	}
	
	fn global_remove(&self, proxy: &mut WlRegistry, name: u32) {
		todo!()
	}
}

struct WmBaseListener;

impl XdgWmBaseListener for WmBaseListener {
	fn ping(&self, proxy: &mut XdgWmBase, serial: u32) {
		proxy.pong(serial);
	}
}

struct TopLevelListener(Registry, Arc<AtomicBool>);

impl XdgToplevelListener for TopLevelListener  {
	fn configure(&self, proxy: &mut XdgToplevel, width: i32, height: i32, states: &WlArray) {
		log::info!("configure {}x{}", width, height);
		//todo!()
	}
	
	fn close(&self, proxy: &mut XdgToplevel) {
		self.1.store(true, std::sync::atomic::Ordering::Relaxed);
	}
}

#[derive(Clone)]
struct FrameListener(Arc<Box<WlSurface, WlAlloc>>, Arc<Mutex<TargetRootContext<Arc<World>>>>);

impl WlCallbackListener for FrameListener {
	fn done(&self, proxy: &mut WlCallback, callback_data: u32) {
		self.0.frame().unwrap().set_listener(self.clone()).unwrap();
		self.1.lock().unwrap().submit().unwrap();
	}
}

async fn run() {
	log::debug!("[WAYLAND] initializing ...");
	let display         = WlDisplay::connect(None).expect("failed to init wayland");
	let mut wl_registry = display.get_registry().expect("failed to get wayland registry");
	let registry        = Registry::new();
	let shutdown        = Arc::new(AtomicBool::new(false));
	
	wl_registry.set_listener(RegistryListener(registry.clone())).unwrap();
	display.dispatch().expect("dispatch failed");
	display.roundtrip().expect("roundtrip failed");
	
	log::debug!("[WAYLAND] initializing ...");
	pipewire::init();
	
	
	let (_, compositor)  = registry.get_first::<Box<WlCompositor, WlAlloc>>();
	let (_, xdg_wm_base) = registry.get_first::<Box<XdgWmBase, WlAlloc>>();
	let wl_surface       = Arc::new(compositor.read().unwrap().create_surface().expect("failed to create surface"));
	let xdg_surface      = xdg_wm_base.read().unwrap().get_xdg_surface(&wl_surface).expect("failed to get xdg surface");
	let mut xdg_toplevel = xdg_surface.get_toplevel().expect("failed to get xdg toplevel");
	let mut vk_surface   = [VK_NULL_HANDLE];
	
	xdg_wm_base.write().unwrap().set_listener(WmBaseListener).unwrap();
	xdg_toplevel.set_listener(TopLevelListener(registry.clone(), shutdown.clone())).unwrap();
	xdg_toplevel.set_app_id("Arcturos Game Engine\0");
	wl_surface.commit();
	display.flush().expect("flush failed");
	
	log::debug!("[GPGPU] initializing ...");
	
	let plugins = Plugins::new()
		.add(plugin_ui::RootContext)
		.add(plugin_sdft::RootContext);
	
	let device_ctx = DeviceRootContext::create(
		&gpgpu::cfg::DEFAULT_CFG,
		&[&(&*display, &**wl_surface)],
		&mut vk_surface,
		&[VK_KHR_SURFACE_EXTENSION_NAME, VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME],
		&[Arc::new(plugin_ui::RootContext), Arc::new(plugin_sdft::RootContext)]
	).expect("failed to create gpgpu device context");
	
	let [vk_surface] = vk_surface;
	spawn_transfer_thread(&device_ctx)
		.expect("failed to spawn transfer sched thread");
	
	log::debug!("[SCENE] initializing ...");
	
	let scene = Arc::new(World::new());
	let _singleton_entity = scene.add_entity((
		UiDimensions { surface_size: Vec2(1920, 1080), display_size: Vec2(0, 0) },
		SceneOptions {
			sdf_extent: Vec3::from(128),
			ids_extent: Vec3::from(128),
			..SceneOptions::default()
		}
	));
	
	spawn(init_scene(scene.clone(), device_ctx.clone()));
	
	let scene_ctx = Arc::new(SceneRootContext::new(
		device_ctx.clone(),
		scene.clone(),
		&plugins
	).expect("failed to create gpgpu scene context"));
	
	log::debug!("[SURFACE] initializing ...");
	
	let target_ctx = Arc::new(Mutex::new(TargetRootContext::new(
		device_ctx.clone(),
		scene_ctx.clone(),
		RenderGraph::build()
			.stage(|b| b
				.name("main")
				.node(|b| b
					.name("update_ui")
					.host_job(()))
				.node(|b| b
					.name("update_sdft_host")
					.host_job(()))
				.node(|b| b
					.name("update_sdft_local")
					.dependency(1)
					.local_job(()))
				.node(|b| b
					.name("render_sdft")
					.dependency(2)
					.local_job(()))
				.pass(|b| b
					.name("render_ui")
					.subpass(|b| b
						.name("render_ui")
						.node(|b| b
							.name("render_ui")
							.dependencies([0, 3])
							.local_job(())))))
			.finish(&device_ctx),
		(),
		Some(vk_surface),
		VkExtent2D { width: 1920, height: 1080 },
		&plugins
	).expect("failed to create gpgpu surface context")));
	
	log::debug!("initialization sequence completed, starting main loop");
	
	// RENDER LOOP
	
	target_ctx.lock().unwrap().submit().unwrap();
	wl_surface.frame().unwrap().set_listener(FrameListener(
		wl_surface.clone(), target_ctx.clone())).unwrap();
	
	while !shutdown.load(Ordering::Relaxed) {
		display.dispatch().expect("dispatch failed");
	}
	
	log::debug!("shutting down");
	
	//<UiLoader as Loader<(MeshDescriptor, Box<dyn MeshReader>), _>>::unload(&UiLoader, mesh, (&*device_ctx, &*storages)).await.unwrap();
	//<UiLoader as Loader<(FontDescriptor, Box<dyn FontReader>), _>>::unload(&UiLoader, font, (&*device_ctx, &*storages)).await.unwrap();
	
	std::mem::drop(target_ctx);
	device_ctx.write_pipeline_cache("./tmp/pipeline_cache");
	stop();
}

async fn init_scene(world: Arc<World>, device_ctx: Arc<DeviceRootContext<Arc<World>>>) {
	use Val::*;
	
	let mesh        = world.add_entity(());
	let font        = world.add_entity(());
	/*let scene2      = storages.clone();
	let device_ctx2 = device_ctx.clone();
	let mesh        = storages.build_entity().finish();
	
	app::spawn(async move { UiLoader.load(
		mesh,
		&*StaticMeshSource::new(MeshDescriptor::new(4, &[
			MeshAttribute::Pos(vk::VkFormat::VK_FORMAT_R32G32_SFLOAT)
		]), &[
			ui::BasicVertex { pos: Vec2(-0.5f32, -0.5f32) },
			ui::BasicVertex { pos: Vec2(-0.5f32, 0.5f32) },
			ui::BasicVertex { pos: Vec2(0.5f32, -0.5f32) },
			ui::BasicVertex { pos: Vec2(0.5f32, 0.5f32) }
		]),
		(&device_ctx2, &*scene2)
	).await.unwrap() });
	
	let scene2      = storages.clone();
	let device_ctx2 = device_ctx.clone();
	let font        = storages.build_entity().finish();
	
	app::spawn(async move { UiLoader.load(
		font,
		&PathSource("/usr/share/fonts/JetBrainsMono-1.0.2/ttf/JetBrainsMono-Regular.ttf") as &dyn Source<(FontDescriptor, Box<dyn FontReader>)>,
		(&device_ctx2, _, &*scene2)
	).await.unwrap() });*/
	
	world.add_entity((
		UiRender,
		Handle::<Box<Mesh>>::new(mesh),
		Color(Vec4(1f32, 1f32, 0f32, 1f32)),
		UiTransform::new()
			.scaling(Norm(0.4f32), Norm(0.4f32))
			.z(1.0),
		LocalTransform::default(),
		GlobalTransform::default(),
		Interactable::default()
	));
	
	world.add_entity((
		UiRender,
		Handle::<Box<Mesh>>::new(mesh),
		Color(Vec4(1f32, 0f32, 1f32, 1f32)),
		UiTransform::new()
			.scaling(Norm(0.4f32), Norm(0.4f32))
			.z(0.75),
		LocalTransform::default(),
		GlobalTransform::default(),
		Interactable::default()
	));
	
	world.add_entity((
		UiRender,
		Handle::<Box<Mesh>>::new(mesh),
		Color(Vec4(0f32, 1f32, 1f32, 1f32)),
		UiTransform::new()
			.scaling(Norm(0.1f32), Norm(0.1f32))
			.z(0.5),
		LocalTransform::default(),
		GlobalTransform::default(),
		Interactable::default()
	));
	
	let _tri = world.add_entity((
		UiRender,
		Handle::<Box<Mesh>>::new(mesh),
		Color(Vec4(1f32, 0f32, 0f32, 1f32)),
		UiTransform::new()
			.scaling(DMax(0.1f32), DMax(0.1f32)),
		LocalTransform::default(),
		GlobalTransform::default()
	));
	
	let _text = world.add_entity((
		UiText::from("0123456789 -+.:,;$%&~*#<=>/\\()[]{}\nabcdefghijklmnopqrstuvwxyz\nABCDEFGHIJKLMNOPQRSTUVWXYZ"),
		Handle::<Box<Font>>::new(font),
		Color(Vec4(1f32, 1f32, 1f32, 1f32)),
		UiTransform::new()
			.translation(Norm(0f32), Norm(0f32))
			.scaling(Norm(0.05f32), Norm(0.05f32)),
		LocalTransform::default(),
		GlobalTransform::default(),
	));
	
	let _text_fps = world.add_entity((
		UiText::from("- FPS"),
		Handle::<Box<Font>>::new(font),
		UiTransform::new()
			.translation(Norm(0.775f32), Norm(-0.95f32))
			.scaling(Norm(0.05f32), Norm(0.05f32)),
		LocalTransform::default(),
		GlobalTransform::default()
	));
	
	let _text_input = world.add_entity((
		UiText::default(),
		Handle::<Box<Font>>::new(font),
		Color(Vec4(1f32, 1f32, 1f32, 1f32)),
		UiTransform::new()
			.translation(Norm(-0.9f32), Norm(0.9f32))
			.scaling(Norm(0.05f32), Norm(0.05f32)),
		LocalTransform::default(),
		GlobalTransform::default()
	));
	
	log::debug!("[SCENE] loaded");
	std::mem::drop(device_ctx);
}

/*async fn recv_wsi_events(
	display:        wsi::Display,
	surface:        Arc<Surface>,
	storages:       Arc<Storages>,
	mut scene_ctx:  SceneContext,
	surface_ctx:    Arc<Mutex<SurfaceContext>>,
	systems:        Mutex<SceneSystems>,
	time:           Mutex<Instant>,
	tri:            Entity,
	text:           Entity,
	text_fps:       Entity,
	text_input:     Entity
) {
	use {wsi::Event, std::ops::Try};
	
	loop {
		let event = display.poll_event_async().await.unwrap();
		
		match event.clone() {
			Event::Ping { serial } => display.pong(serial),
			Event::SurfaceFrame { surface } => {
				let mut time = time.lock().unwrap();
				
				let fps = 1f64 / time.elapsed().as_secs_f64();
				storages.write::<Storage<UiText>>().get_mut(text_fps)
					.unwrap().0 = format!("{:.2} FPS", fps);
				
				*time = std::time::Instant::now();
				surface.frame();
				systems.lock().unwrap().run(&storages);
				let mut ctx = surface_ctx.lock().unwrap();
				
				if scene_ctx.update().unwrap() == UpdateResult::Redraw {
					ctx.record(&storages, &scene_ctx.contexts).into_result().unwrap();
				}
				
				match ctx.draw() {
					Err(vk::VK_ERROR_OUT_OF_DATE_KHR | vk::VK_SUBOPTIMAL_KHR) => {
						ctx.update(surface.get_extent()).into_result().unwrap();
						ctx.record(&storages, &scene_ctx.contexts).into_result().unwrap();
						ctx.draw()
					},
					err => err
				}.expect("failed to draw");
			}
			Event::SurfaceConfigure { width, height, .. } => {
				storages.write::<UiDimensions>().surface_size = Vec2(width as _, height as _);
				let mut ctx = surface_ctx.lock().unwrap();
				ctx.update(vk::VkExtent2D { width: width as _, height: height as _ }).into_result().unwrap();
				ctx.record(&storages, &scene_ctx.contexts).into_result().unwrap();
			}
			Event::SurfaceEnter { output, .. } => {
				let info = output.get_info();
				storages.write::<UiDimensions>().display_size = Vec2(info.width as _, info.height as _);
			}
			Event::SurfaceClose { .. } | Event::KeyboardKey { key: 27, .. } => return,
			
			Event::PointerMotion { x, y, .. } => {
				let size = storages.read::<UiDimensions>().surface_size;
				storages.write::<Storage<UiTransform>>()
					.get_mut(tri).unwrap().translation = Vec2(
					UiUnit::Norm(x as f32 / size.0 as f32 * 2f32 - 1f32),
					UiUnit::Norm(y as f32 / size.1 as f32 * 2f32 - 1f32)
				);
			}
			
			Event::PointerButton { button: PointerButton::Right, state: ButtonState::Pressed, serial, pointer, .. } =>
				surface.interactive_move(&pointer.input, serial),
			
			Event::KeyboardKey { key: key @ 0x20..0x7F, state: ButtonState::Pressed, .. } => storages.write::<Storage<UiText>>()
				.get_mut(text_input).unwrap().0.push(std::char::from_u32(key).unwrap_or('?')),
			Event::KeyboardKey { key: 8, state: ButtonState::Pressed, .. } => { storages.write::<Storage<UiText>>()
				.get_mut(text_input).unwrap().0.pop(); },
			Event::KeyboardKey { key: 13, state: ButtonState::Pressed, .. } => {
				let mut lock = storages.write::<Storage<UiText>>();
				let text_ = lock.get_mut(text_input).unwrap();
				let mut iter = text_.0.split(' ');
				
				match iter.next() {
					Some("set-text") => {
						let __tmp__ = iter.next()
							.unwrap_or("").to_string();
						text_.0.clear();
						lock.get_mut(text).unwrap().0 = __tmp__;
						continue;
					},
					Some("exit") => return,
					_ => log::error!("unrecognized command")
				}
				
				text_.0.clear();
			},
			
			_ => ()
		}
		
		storages.write::<EventChannel<Event>>().send(event);
		systems.lock().unwrap().run_events(&storages);
	}
}*/

#[allow(dead_code)]
fn render_scene() {
	use math::raytracing::*;
	
	let mut scene = SdfScene::new_default(Vec3::from(512), Vec3::from(512));
	
	scene.instances.extend_from_slice(&[
		SdfSceneInstance { sdf: 0, mat: 0, trans: Mat4::from_translation(Vec3(0.0, 0.25, 0.25)) },
		SdfSceneInstance { sdf: 0, mat: 1, trans: Mat4::from_translation(Vec3(0.0, 0.0, 0.25)) },
		SdfSceneInstance { sdf: 0, mat: 2, trans: Mat4::from_translation(Vec3(0.0, -0.25, 0.25)) },
		SdfSceneInstance { sdf: 1, mat: 3, trans: Mat4::from_translation(Vec3(0.0, 0.0, -0.1)).rotate(Vec3(0.0, 1.0, 0.0), 55f32.to_radians()) },
		SdfSceneInstance { sdf: 1, mat: 3, trans: Mat4::from_translation(Vec3(-0.1, 0.0, -0.19)).rotate(Vec3(0.0, 1.0, 0.0), 45f32.to_radians()) },
		SdfSceneInstance { sdf: 1, mat: 3, trans: Mat4::from_translation(Vec3(-0.2, 0.0, -0.31)).rotate(Vec3(0.0, 1.0, 0.0), 35f32.to_radians()) },
		SdfSceneInstance { sdf: 1, mat: 3, trans: Mat4::from_translation(Vec3(-0.3, 0.0, -0.4)).rotate(Vec3(0.0, 1.0, 0.0), 25f32.to_radians()) },
	]);
	
	scene.sdfs.push(Box::new(Sphere { radius: 0.1 }));
	scene.sdfs.push(Box::new(Cuboid(Vec3(0.05, 0.5, 0.01))));
	scene.materials.extend_from_slice(&[
		(BasicMaterial::new(Vec3(0.75, 0.0, 0.0), Vec3::from(0.0), 0.5, 0.5), 1.0),
		(BasicMaterial::new(Vec3(0.0, 0.75, 0.0), Vec3::from(0.75), 0.5, 0.5), 1.0),
		(BasicMaterial::new(Vec3(0.0, 0.0, 0.75), Vec3::from(0.0), 0.5, 0.5), 1.0),
		(BasicMaterial::new(Vec3(0.5, 0.5, 0.5), Vec3::from(0.0), 0.5, 0.5), 1.0)
	]);
	
	scene.update_all();
	
	let cam = BasicCamera::new(
		Vec3(0.75, 0.0, 0.0),
		Vec3::from(0.0),
		Vec3(0.0, 0.0, 1.0),
		90f32.to_radians(),
		1.0
	);
	
	let extent = Vec2::from(1080);
	let mut output = BasicOutput::new(extent, Vec3::from(0.0));
	
	let t = std::time::Instant::now();
	render(&cam, &scene, &mut output, extent, 4, 4, None);
	
	println!("metrics: total: {}ms, trace: {}ms, update: {}ms",
			 t.elapsed().as_millis(),
			 scene.metrics_trace_ns.load(std::sync::atomic::Ordering::SeqCst) / 1_000_000,
			 scene.metrics_update_ns.load(std::sync::atomic::Ordering::SeqCst) / 1_000_000);
	
	output.write_ppm(&mut std::fs::OpenOptions::new()
		.create(true)
		.write(true)
		.open("./out.ppm").unwrap()).unwrap();
}