
use lib::object::{Raymarchable, Analytical};
use lib::material::Material;
use lib::light::Light;
use lib::*;
use raymarcher::raymarch;

use cacheable::Cacheable;
use camera::Camera;
use scenedata::ObjectData;

use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::ops::Deref;
use std::rc::Rc;
use std::io::Write;

use pbr;
use pbr::ProgressBar;

use num_cpus;
use rand::{random, Closed01};
use threadpool::ThreadPool;
use image::{Rgb, ImageBuffer};
use std::thread;

pub struct CachedObjectData {
	pub object: Arc<Cacheable<Rc<Raymarchable>>>,
	pub bounds: Option<Arc<Cacheable<Rc<Analytical>>>>,
	pub material: Arc<Cacheable<Rc<Material>>>
}

pub struct CachedScene {
	pub objects: Vec<CachedObjectData>,
	pub lights:  Vec<Arc<Cacheable<Rc<Light>>>>,
	pub background: Colour
}

#[derive(Copy, Clone)]
pub struct ImageSize {
	pub width: u32,
	pub height: u32,
	pub samples: u32
}

pub struct ImageDesc {
	pub scene: Arc<CachedScene>,
	pub camera: Camera,

	pub size: ImageSize,
	pub opts: RaymarchOptions,
}

impl Cacheable<ObjectData> for CachedObjectData {
	fn cached(&self) -> ObjectData {
		ObjectData {
			object: self.object.deref().cached(),
			material: self.material.deref().cached(),
			bound: match self.bounds {
				Some(ref p) => Some(p.deref().cached()),
				None => None
			}
		}
	}
}

impl Cacheable<Scene> for CachedScene {
	fn cached(&self) -> Scene {
		let mut scene = Scene::new(self.background);

		for ref obj in &self.objects {
			scene.objects.push(obj.deref().cached());
		}
		for ref light in &self.lights {
			scene.lights.push((***light).cached());
		}

		scene
	}
}

fn isect_illumination(
	light: &Rc<Light>, 
	isect: &Intersection,
	scene: &Scene,
	opts: &RaymarchOptions) -> Colour
{
	let mut count: usize = 0;
	let mut visible: usize = 0;

	for (ray, maxdist) in light.shadow_rays(isect) {
		let mut opts_new = *opts;
		opts_new.min_distance = opts.intersect_distance * 4.0;
		opts_new.max_distance = maxdist;

		let test = raymarch(ray, &scene.objects, &opts_new);

		count += 1;
		visible += match test {
			Some(_) => 0,
			None => 1
		};
	}

	if count == 0 {
		count = 1;
		visible = 1;
	}

	let mult = (visible as f32) / (count as f32);

	return light.illumination(isect) * mult;
}

fn isect_colour(scene: &Scene, isect: &Intersection, opts: &RaymarchOptions) -> Colour {
	let base_colour = isect.object.material.base_colour(&isect);
	let mut illum = Colour::zero();

	for ref light in scene.lights.iter() {
		illum = max(isect_illumination(light, isect, scene, opts), illum);
	}

	return base_colour * clamp(illum, 0.0, 1.0);
}

fn trace_ray(
	scene: &Scene,
	ray: Ray,
	opts: RaymarchOptions) -> Colour 
{
	let isect = raymarch(ray, &scene.objects, &opts);

	match isect {
		None => scene.background,
		Some(ref isect) => isect_colour(scene, isect, &opts)
	}
}

pub fn render_pixel(
	pixel: Vec2u,
	desc: ImageSize,
	opts: RaymarchOptions,
	camera: Camera,
	scene: &Scene) -> Colour
{
	fn randval() -> f64 {
	    let Closed01(val) = random::<Closed01<f64>>();
	    return val;
	}

    let mut result = Colour::zero();

    let point = Vec2d {
        x: ((pixel.x as f64) / (desc.width as f64)) * 2.0 - 1.0,
        y: ((pixel.y as f64) / (desc.height as f64)) * 2.0 - 1.0,
    };

    for _ in 0..desc.samples {
        let jitter = Vec2d {
            x: randval() / (desc.width as f64),
            y: randval() / (desc.height as f64),
        };

        result += trace_ray(
			scene,
			camera.screen_ray(point + jitter),
			opts);
    }

    return result / (desc.samples as f32);
}

fn trace_single_image<T>(
	image_idx: usize,
	tx: Sender<(usize, u32, Vec<Rgb<u8>>)>,
	pool: &ThreadPool,
	desc: &ImageDesc,
	pb: Arc<Mutex<ProgressBar<T>>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>
	where T: Write + Send + 'static
{
	let count = Arc::new(Mutex::new(0u32));

	for y in 0..desc.size.height {
		pool.execute({
            let cached_scene = Arc::clone(&desc.scene);
			let opts = desc.opts;
            let tx = tx.clone();
			let size = desc.size;
			let camera = desc.camera;
			let pb = Arc::clone(&pb);
			let count = Arc::clone(&count);
            move || {
				let scene = cached_scene.deref().cached();
				let mut values = Vec::new();
                for x in 0..size.width {
                    let colour = render_pixel(
						Vec2u { x: x, y: y }, 
						size, 
						opts,
						camera,
						&scene);

					values.push(Rgb { 
						data: [
							(colour.x * 255.0) as u8,
							(colour.y * 255.0) as u8,
							(colour.z * 255.0) as u8,
						]		
					});
                }
				tx.send((
					image_idx,
					y,
					values
				)).expect("Channel is present");

				{
					let ref mut bar = pb.lock().expect("Unlock Error");
					let ref mut cnt = count.lock().unwrap();

					bar.inc();
					**cnt += 1;

					if **cnt == size.height {
						bar.finish_print(&format!("done image {}", image_idx));
					}
				}
            }
		});
	}

	ImageBuffer::new(desc.size.width, desc.size.height)    
}

pub fn trace_image(desc: &ImageDesc) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (tx, rx) = channel();
    let pool = ThreadPool::new(num_cpus::get());
	let pb = Arc::new(Mutex::new(ProgressBar::new(desc.size.height as u64)));

	let mut imagebuf = trace_single_image(0, tx, &pool, desc, pb);

    for _ in 0..desc.size.height {
        let (_, y, pixels) = rx.recv().expect("Receive Error");

		for x in 0..desc.size.width {
			imagebuf.put_pixel(x, y, pixels[x as usize]);
		}
    }

    return imagebuf;
}

pub fn multi_trace_image<I>(descriptors: I) -> impl Iterator<Item = ImageBuffer<Rgb<u8>, Vec<u8>>> 
	where I: Iterator<Item=ImageDesc>
{
	let (tx, rx) = channel();
	let pool = ThreadPool::new(num_cpus::get());
	let mut widths = Vec::new();
	let mut images = Vec::new();
	let mut pb = pbr::MultiBar::new();
	let mut count = 0;

	for (i, desc) in descriptors.enumerate() {
		let pb = Arc::new(Mutex::new(pb.create_bar(desc.size.height as u64)));
		widths.push(desc.size.width);
		images.push(trace_single_image(i, tx.clone(), &pool, &desc, pb));
		count += desc.size.height;
	}

	let handle = thread::spawn(move || {
		pb.listen();
	});

	for _ in 0..count {
		let (img, y, pixels) = rx.recv().expect("Receive Error");
		//println!("here");

		for x in 0..widths[img] {
			images[img].put_pixel(x, y, pixels[x as usize]);
		}
	}

	handle.join().unwrap();

	return images.into_iter();
}

