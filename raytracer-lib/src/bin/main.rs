
extern crate raytracer;
extern crate image;

use raytracer::*;
use raytracer::colours;
use raytracer::material::*;
use raytracer::object::*;
use raytracer::light::*;
use raytracer::builder;

use std::env;
use std::fs::File;

use std::sync::Arc;

fn make_sphere(colour : [f32; 3], pos : [f64; 3], radius : f64) -> (Arc<Raymarchable>, Arc<Material>) {
	return (
		builder::sphere(pos, radius),
		builder::solid_colour(colour)
	);
}
fn make_directional_light() -> Arc<Light> {
	return Arc::new(FuzzyDirectionalLight::new(Vec3d::new(0.0, -1.0, 1.0), 0.0872665));
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
	let colour = colours::BLACK;

	let mut scene = Scene::new(camera, opts, builder::colour(colour));

	scene.add_object(make_sphere(colours::BLUE,  [1.0, 0.0, 0.0], 1.0));
	scene.add_object(make_sphere(colours::GREEN, [-1.0, 0.0, 0.0], 0.7));
	scene.add_object(make_sphere(colours::WHITE, [0.0, -10001.5, 0.0], 10000.0));
	scene.add_object((
		builder::translate(builder::torus(1.0, 0.5), [0.0, 1.0, 0.0]),
		builder::solid_colour(colours::ORANGE)
	));

	scene.add_light(make_directional_light());
	scene.add_light(Arc::new(AmbientLight::new(Colour::new(1.0, 1.0, 1.0) * 0.1)));

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
		samples: 50
	};
    let scene = Arc::new(create_scene());

	let image_val = trace_image(desc, opts, scene);

	let ref mut file = File::create(args[1].clone()).unwrap();

	image::ImageRgb8(image_val)
		.save(file, image::PNG)
		.unwrap();
}
