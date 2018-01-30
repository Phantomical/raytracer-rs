extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::colours;

use std::env;
use std::fs::File;

use std::sync::Arc;

mod add_objects {
    use raytracer::Scene;
    use raytracer::builder::*;
    use raytracer::colours;

    use raytracer::{Mat3d, normalize};

    pub fn add_objects(scene: &mut Scene) {
        scene.add_object(
            sphere([0.0, -10001.0, 0.0], 10000.0),
            solid_colour(colours::WHITE),
        );

        let rotation = Mat3d::from_axis_angle(
            normalize(vec3d([-1.0, 0.0, 1.0])),
			deg2rad(52.5),
        );

        scene.add_object(
            rotate(sierpinski(10, 2.0), rotation),
            solid_colour(colours::RED),
        );
    }

    pub fn add_lights(scene: &mut Scene) {
        scene.add_light(fuzzy_directional([0.0, -1.0, 1.0], 0.0872665, 40));
        scene.add_light(ambient([0.1, 0.1, 0.1]));
    }
}

fn create_scene() -> Scene {
    let camera = CameraBuilder::new()
        .position(vec3(2.0, 0.5, -5.0))
        .forward(vec3(-0.4, 0.0, 1.0))
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
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: raytracer <output-file>");
        return;
    }

    let desc = ImageDesc {
        width: 1200,
        height: 800,
    };
    let opts = ImageOptions { samples: 5 };
    let scene = Arc::new(create_scene());

    let image_val = trace_image(desc, opts, scene);

    let ref mut file = File::create(args[1].clone()).unwrap();

    image::ImageRgb8(image_val).save(file, image::PNG).unwrap();
}
