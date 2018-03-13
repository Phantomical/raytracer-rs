extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::math::*;
use raytracer::colours;

use std::env;
use std::sync::Arc;

mod add_objects {
    use raytracer::SceneBuilder;
    use raytracer::builder::*;
    use raytracer::colours;

    pub fn add_objects(scene: SceneBuilder) -> SceneBuilder {
        scene
            .add_object(
                sphere([0.0, -100001.0, 0.0], 100000.0),
                solid_colour(colours::WHITE),
            )
            .add_object(sphere([2.0, 0.0, 2.0], 1.0), solid_colour(colours::GREEN))
            .add_object(sphere([-2.0, 0.0, 2.0], 1.0), solid_colour(colours::GREEN))
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene
            .add_light(tint(
                fuzzy_directional([0.0, -1.0, 1.0], deg2rad(5.0), 20),
                [0.3; 3],
            ))
            .add_light(point_light([0.0, 1.0, 0.0], 3.0))
    }
}

fn create_scene(size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(0.0, 1.0, -10.0))
        .orthonormalize()
        .unwrap();

    let opts = RaymarchOptions {
        max_distance: 1.0e8,
        ..Default::default()
    };

    let mut scene = SceneBuilder::new().background(builder::colour(colours::BLACK));

    scene = add_objects::add_objects(scene);
    scene = add_objects::add_lights(scene);

    ImageDesc {
        scene: Arc::new(scene.unwrap()),
        camera,
        size,
        opts,
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: raytracer <output-file>");
        return;
    }

    let size = ImageSize {
        width: 1000,
        height: 750,
        samples: 5,
    };
    let desc = create_scene(size);

    trace_to_disk(vec![(desc, args[1].to_string())].into_iter());
}
