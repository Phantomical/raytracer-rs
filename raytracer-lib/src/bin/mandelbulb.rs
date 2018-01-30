extern crate cgmath;
extern crate gradient;
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
    use raytracer::object::Mandelbulb;
    use raytracer::material::OriginTrap;

    use std::sync::Arc;
    use gradient::Gradient;

    pub fn add_objects(scene: &mut Scene) {
        scene.add_object(
            sphere([0.0, -10001.0, 0.0], 10000.0),
            solid_colour(colours::WHITE),
        );

        let mandelbulb = Mandelbulb::new(3, 8);
        scene.add_object(
            Arc::new(mandelbulb),
            Arc::new(OriginTrap::new(
                Gradient::new(&[
                    (colour(colours::ORANGE) * 0.8, 1.0),
                    (colour(colours::GREEN) * 0.4, 0.85),
                    (colour(colours::BLUE), 0.7),
                ]),
                mandelbulb,
            )),
        );
    }

    pub fn add_lights(scene: &mut Scene) {
        scene.add_light(directional([0.0, -1.0, 5.0])); //, 0.0872665, 10));
        scene.add_light(ambient([0.2; 3]));
    }
}

fn create_scene() -> Scene {
    let camera = CameraBuilder::new()
        .position(vec3(2.0, 0.0, -4.0))
        .forward(vec3(-0.5, 0.0, 1.0))
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
    let opts = ImageOptions { samples: 2 };
    let scene = Arc::new(create_scene());

    let image_val = trace_image(desc, opts, scene);

    let ref mut file = File::create(args[1].clone()).unwrap();

    image::ImageRgb8(image_val).save(file, image::PNG).unwrap();
}
