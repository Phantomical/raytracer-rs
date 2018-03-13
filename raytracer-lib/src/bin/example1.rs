extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::math::*;

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
            .add_object(sphere([0.0, 0.0, 0.0], 1.0), solid_colour(colours::RED))
            .add_object(
                translate(
                    transform(torus(1.0, 0.5), rotate_xyz(deg2rad(-45.0), 0.0, 0.0)),
                    [1.0, 0.0, 0.0],
                ),
                solid_colour(colours::ORANGE),
            )
            .add_object(
                translate(
                    transform(torus(1.0, 0.5), rotate_xyz(deg2rad(45.0), 0.0, 0.0)),
                    [-1.0, 0.0, 0.0],
                ),
                solid_colour(colours::ORANGE),
            )
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene.add_light(fuzzy_directional([0.0, -1.0, 1.0], 0.0872665, 20))
    }
}

fn create_desc(size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(0.0, 1.0, -10.0))
        .orthonormalize()
        .unwrap();

    let opts = RaymarchOptions {
        max_distance: 1.0e8,
        ..Default::default()
    };

    let mut scene = SceneBuilder::new();

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
        samples: 50,
    };
    let desc = create_desc(size);

    trace_to_disk(vec![(desc, args[1].to_string())].into_iter());
}
