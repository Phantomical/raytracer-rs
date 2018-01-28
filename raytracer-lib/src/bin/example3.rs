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

    pub fn add_objects(scene: &mut Scene) {
        scene.add_object(
            plane([0.0, 1.0, 0.0], [0.0, -1.0, 0.0]),
            solid_colour(colours::WHITE),
        );

        scene.add_object(
            repeat(sphere([0.0, 0.0, 0.0], 1.0), [8.0, 0.0, 8.0]),
            solid_colour(colours::RED),
        );
    }

    pub fn add_lights(scene: &mut Scene) {
        scene.add_light(directional([0.0, -1.0, 1.0])); //, 0.0872665));
    }
}

fn create_scene() -> Scene {
    let camera = CameraBuilder::new()
        .position(vec3(0.0, 10.0, -10.0))
        .forward(vec3(1.0, 0.0, 1.0))
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
        width: 1080,
        height: 720,
    };
    let opts = ImageOptions { samples: 5 };
    let scene = Arc::new(create_scene());

    let image_val = trace_image(desc, opts, scene);

    let ref mut file = File::create(args[1].clone()).unwrap();

    image::ImageRgb8(image_val).save(file, image::PNG).unwrap();
}
