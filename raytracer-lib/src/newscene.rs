
use lib::object::{Raymarchable, Analytical};
use lib::material::Material;
use lib::light::Light;
use lib::*;
use raymarcher::raymarch;

use cacheable::Cacheable;
use camera::Camera;
use scenedata::ObjectData;

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::ops::Deref;
use std::rc::Rc;

use pbr;
use num_cpus;
use rand::{random, Closed01};
use threadpool::ThreadPool;
use image::{Rgb, ImageBuffer};

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

pub fn trace_image(desc: &ImageDesc) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let imagebuf = Arc::new(Mutex::new(ImageBuffer::new(desc.size.width, desc.size.height)));
    let (tx, rx) = channel();
    let pool = ThreadPool::new(num_cpus::get());

	for y in 0..desc.size.height {
		pool.execute({
		    let imagebuf = Arc::clone(&imagebuf);
            let cached_scene = Arc::clone(&desc.scene);
			let opts = desc.opts;
            let tx = tx.clone();
			let size = desc.size;
			let camera = desc.camera;
            move || {
				let scene = cached_scene.deref().cached();
                for x in 0..size.width {
                    let colour = render_pixel(
						Vec2u { x: x, y: y }, 
						size, 
						opts,
						camera,
						&scene);

                    imagebuf.lock().unwrap().put_pixel(
                        x,
                        y,
                        Rgb {
                            data: [
                                (colour.x * 255.0) as u8,
                                (colour.y * 255.0) as u8,
                                (colour.z * 255.0) as u8,
                            ],
                        },
                    );
                }
                tx.send(y).expect("Channel is present");
            }
		});
	}

	let mut pb = pbr::ProgressBar::new(desc.size.height as u64);
    for _ in 0..desc.size.height {
        let _ = rx.recv();

        pb.inc();
    }

    {
        let _ = imagebuf.lock();
    }

    return Arc::try_unwrap(imagebuf)
        .expect("Lock still has multiple owners")
        .into_inner()
        .expect("Mutex cannot be locked");
}

