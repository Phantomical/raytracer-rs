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

    pub fn add_objects(scene: SceneBuilder) -> SceneBuilder {
        scene
            .add_object(
                plane([0.0, 1.0, 0.0], [0.0, -1.0, 0.0]),
                solid_colour(colours::WHITE),
            )
            .add_object(
                repeat(sphere([0.0, 0.0, 0.0], 1.0), [8.0, 0.0, 8.0]),
                solid_colour(colours::RED),
            )
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene.add_light(directional([0.0, -1.0, 1.0])) //, 0.0872665))
    }
}

fn create_scene(size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(0.0, 10.0, -10.0))
        .forward(vec3(1.0, 0.0, 1.0))
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
        width: 1080,
        height: 720,
        samples: 5,
    };
    let desc = create_scene(size);

    trace_to_disk(vec![(desc, args[1].to_string())].into_iter());
}
