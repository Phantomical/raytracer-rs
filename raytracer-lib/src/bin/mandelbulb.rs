
extern crate gradient;
extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::colours;

use std::env;
use std::sync::Arc;

mod add_objects {
    use raytracer::SceneBuilder;
    use raytracer::builder::*;
    use raytracer::colours;
    use raytracer::object::Mandelbulb;
    use raytracer::material::OriginTrap;

    use gradient::Gradient;

    pub fn add_objects(scene: SceneBuilder) -> SceneBuilder {
        let mandelbulb = Mandelbulb::new(3, 8);

        scene.add_object(
				sphere([0.0, -10001.0, 0.0], 10000.0),
				solid_colour(colours::WHITE))
			.add_object(
				mandelbulb,
				OriginTrap::new(
				    Gradient::new(&[
				        (colour(colours::ORANGE) * 0.8, 1.0),
				        (colour(colours::GREEN) * 0.4, 0.85),
				        (colour(colours::BLUE), 0.7),
				    ]),
				    mandelbulb,
				))
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene.add_light(directional([0.0, -1.0, 5.0])) //, 0.0872665, 10))
			.add_light(ambient([0.2; 3]))
    }
}

use builder::deg2rad;

fn create_scene(size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(2.0, 0.0, -4.0))
        .forward(vec3(-0.5, 0.0, 1.0))
        .aspect_y(deg2rad(60.0), (size.width as f64) / (size.height as f64))
        .orthonormalize()
        .unwrap();

    let opts = RaymarchOptions {
        max_distance: 1.0e8,
        ..Default::default()
    };

    let mut scene = SceneBuilder::new()
		.background(builder::colour(colours::BLACK));

    scene = add_objects::add_objects(scene);
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

    if args.len() < 2 {
        println!("Usage: raytracer <output-file>");
        return;
    }

    let size = ImageSize {
        width: 1200,
        height: 800,
		samples: 2,
    };
    let desc = create_scene(size);

	trace_to_disk(vec![(desc, args[1].to_string())].into_iter());
}
