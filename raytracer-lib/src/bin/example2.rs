
extern crate raytracer;
extern crate image;

use raytracer::*;
use raytracer::colours;

use std::env;
use std::fs::File;

use std::sync::Arc;

mod add_objects {
	use raytracer::Scene;
	use raytracer::builder::*;
	use raytracer::colours;

	pub fn add_objects(scene : &mut Scene) {
		scene.add_object(
			sphere([0.0, -100001.0, 0.0], 100000.0),
			solid_colour(colours::WHITE));

		scene.add_object(
			sphere([2.0, 0.0, 2.0], 1.0),
			solid_colour(colours::GREEN));

		scene.add_object(
			sphere([-2.0, 0.0, 2.0], 1.0),
			solid_colour(colours::GREEN));
	}

	pub fn add_lights(scene : &mut Scene) {
		scene.add_light(tint(
			fuzzy_directional([0.0, -1.0, 1.0], deg2rad(5.0)),
			[0.3; 3]));

		scene.add_light(point_light([0.0, 1.0, 0.0], 3.0));
	}
}

fn create_scene() -> Scene {
	let camera = CameraBuilder::new()
		.position(Vec3d::new(0.0, 1.0, -10.0))
		.orthonormalize()
		.unwrap();

	let opts = RaymarchOptions {
		max_distance: 1.0e8,
		..Default::default()
	};

	let mut scene = Scene::new(camera, opts, builder::colour(colours::BLACK));

	add_objects::add_objects(&mut scene);
	add_objects::add_lights(&mut scene);
	
	return scene;
}

fn main() {
	let args : Vec<_> = env::args().collect();

	if args.len() < 2 {
		println!("Usage: raytracer <output-file>");
		return;
	}

	let desc = ImageDesc {
		width:  1000,
		height: 750
	};
	let opts = ImageOptions {
		samples: 5
	};
    let scene = Arc::new(create_scene());

	let image_val = trace_image(desc, opts, scene);

	let ref mut file = File::create(args[1].clone()).unwrap();

	image::ImageRgb8(image_val)
		.save(file, image::PNG)
		.unwrap();
}
