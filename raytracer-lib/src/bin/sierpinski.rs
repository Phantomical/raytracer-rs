
extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::colours;

use std::env;
use std::fs::File;

use std::sync::Arc;

mod add_objects {
    use raytracer::SceneBuilder;
    use raytracer::builder::*;
    use raytracer::colours;

    use raytracer::{Mat3d, normalize};

    pub fn add_objects(scene: SceneBuilder) -> SceneBuilder {
        let rotation = Mat3d::from_axis_angle(
            normalize(vec3d([-1.0, 0.0, 1.0])),
			deg2rad(52.5),
        );

        scene.add_object(
				sphere([0.0, -10001.0, 0.0], 10000.0),
				solid_colour(colours::WHITE))
			.add_object(
				rotate(sierpinski(10, 2.0), rotation),
				solid_colour(colours::RED))
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene.add_light(fuzzy_directional([0.0, -1.0, 1.0], 0.0872665, 40))
			.add_light(ambient([0.1, 0.1, 0.1]))
    }
}

fn create_scene(size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(2.0, 0.5, -5.0))
        .forward(vec3(-0.4, 0.0, 1.0))
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
		samples: 5
    };
    let desc = create_scene(size);

    let image_val = trace_image(&desc);

    let ref mut file = File::create(args[1].clone()).unwrap();

    image::ImageRgb8(image_val).save(file, image::PNG).unwrap();
}
