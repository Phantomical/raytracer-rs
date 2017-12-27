
extern crate raytracer;
extern crate image;

use raytracer::*;
use raytracer::colours;
use raytracer::material::*;
use raytracer::object::*;

use std::env;
use std::fs::File;

use std::sync::Arc;

fn make_red_sphere() -> (Arc<Raymarchable>, Arc<Material>) {
	return (
		Arc::new(Sphere::new(Vec3d::zero(), 1.0)),
		Arc::new(SolidColour::new(colours::RED))
	);
}

fn create_scene() -> Scene {
	let camera = CameraBuilder::new()
		.position(Vec3d::new(0.0, 0.0, -10.0))
		.orthonormalize()
		.unwrap();
	let opts = RaymarchOptions {
		max_distance: 1.0e8,
		..Default::default()
	};
	let colour = colours::RED;

	let mut scene = Scene::new(camera, opts, colour);

	scene.add_object(make_red_sphere());

	return scene;
}

fn main() {
	let args : Vec<_> = env::args().collect();

	if args.len() < 2 {
		println!("Usage: raytracer <output-file>");
		return;
	}

	let desc = ImageDesc {
		width:  4,
		height: 3
	};
	let opts = ImageOptions {
		samples: 50
	};
    let scene = create_scene();

	let image_val = trace_image(desc, opts, &scene);

	let ref mut file = File::create(args[1].clone()).unwrap();

	image::ImageRgb8(image_val)
		.save(file, image::PNG)
		.unwrap();
}
