/*
extern crate gradient;
extern crate image;
extern crate raytracer;

extern crate pbr;
extern crate futures;

#[macro_use]
extern crate runtime_fmt;

use raytracer::*;
use raytracer::colours;

use std::env;

use std::sync::Arc;
use std::vec::Vec;

mod add_objects {
    use raytracer::SceneBuilder;
    use raytracer::builder::*;
    use raytracer::material::OriginTrap;
	use raytracer::colours;

    use gradient::Gradient;

    pub fn add_objects(scene: SceneBuilder, script: String) -> SceneBuilder {
        scene.add_object(
			scripted_object(script),
			OriginTrap::new(
				    Gradient::new(&[
				        (colour(colours::ORANGE), 0.75),
				        (colour(colours::GRAY), 0.4),
				        (colour(colours::BLUE), 0.0),
				    ]),
				    scripted_object(script)))
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene.add_light(fuzzy_directional([0.0, -1.0, 2.0], 0.0872665, 10))
			.add_light(ambient([0.2; 3]))
    }
}

use raytracer::builder::deg2rad;

fn create_scene(script: String, size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(4.0, 1.0, -20.0))
        .forward(vec3(-4.0, -1.0, 20.0))
        .aspect_y(deg2rad(60.0), (size.width as f64) / (size.height as f64))
        .orthonormalize()
        .unwrap();

    let opts = RaymarchOptions {
        max_distance: 1.0e8,
        ..Default::default()
    };

    let mut scene = SceneBuilder::new()
		.background(builder::colour(colours::BLACK));

    scene = add_objects::add_objects(scene, script);
    scene = add_objects::add_lights(scene);

    return ImageDesc {
		scene: Arc::new(scene.unwrap()),
		camera,
		size,
		opts,
	};
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: raytracer <output-file> <angle>");
        return;
    }

	let mut descriptors = Vec::new();
    let size = ImageSize {
        //width: 3840,
        //height: 2160,
        width: 1200/2,
        height: 800/2,

		samples: 10,
    };

	for i in 2..args.len() {
		let angle = args[i].parse().expect("Error: Angle was not a number");
		let name = rt_format!(&args[1], i).expect("Could not format string");

		descriptors.push((create_scene(angle, size), name));
	}

	trace_to_disk(descriptors.into_iter());
}
*/
fn main() {}