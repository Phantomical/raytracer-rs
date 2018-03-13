extern crate image;
extern crate raytracer;
#[macro_use]
extern crate serde;

use raytracer::*;
use raytracer::colours;

use std::env;

use std::sync::Arc;

mod custom {
    use raytracer::Vec3d;
    use raytracer::object::Raymarchable;
    use builder::deg2rad;

    #[derive(Copy, Clone, Serialize, Deserialize)]
    pub struct IFSElement {}

    fn rotate1(_angle: &mut f64, x: &mut f64, y: &mut f64) {
        let (im, re) = deg2rad(45.0).sin_cos();

        let a = re * *x - im * *y;
        *y = re * *y + im * *x;
        *x = a;
    }
    fn rotate2(mut x: &mut f64, mut angle: &mut f64, mut y: &mut f64) {
        rotate1(&mut angle, &mut x, &mut y);
    }

    impl Raymarchable for IFSElement {
        fn distance(&self, point: Vec3d) -> f64 {
            let mut x = point.x;
            let mut y = point.y;
            let mut z = point.z;
            let scale: f64 = 2.0;

            let mut r = x * x + y * y + z * z;
            for _ in 0..10 {
                rotate1(&mut x, &mut y, &mut z);
                //Folding... These are some of the symmetry planes of the tetrahedron
                if x + y < 0.0 {
                    let x1 = -y;
                    y = -x;
                    x = x1;
                }
                if x + z < 0.0 {
                    let x1 = -z;
                    z = -x;
                    x = x1;
                }
                if y + z < 0.0 {
                    let y1 = -z;
                    z = -y;
                    y = y1;
                }

                rotate2(&mut x, &mut y, &mut z);

                //Stretche about the point [1,1,1]*(scale-1)/scale; The "(scale-1)/scale" is here in order to keep the size of the fractal constant wrt scale
                x = scale * x - (scale - 1.0); //equivalent to: x=scale*(x-cx); where cx=(scale-1)/scale;
                y = scale * y - (scale - 1.0);
                z = scale * z - (scale - 1.0);
                r = x * x + y * y + z * z;
            }
            return (r.sqrt() - 2.0) * scale.powi(-10); //the estimated distance
        }
    }
}

mod add_objects {
    use custom;
    use raytracer::SceneBuilder;
    use raytracer::builder::*;
    use raytracer::colours;

    pub fn add_objects(scene: SceneBuilder) -> SceneBuilder {
        scene
            .add_object(
                sphere([0.0, -10001.0, 0.0], 10000.0),
                solid_colour(colours::WHITE),
            )
            .add_object(custom::IFSElement {}, normal())
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene.add_light(directional([0.0, -1.0, 5.0])) //, 0.0872665, 10));
			.add_light(ambient([0.2; 3]))
    }
}

fn create_scene(size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(2.0, 0.0, -4.0))
        .forward(vec3(-0.5, 0.0, 1.0))
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
        width: 1200,
        height: 800,
        samples: 2,
    };
    let desc = create_scene(size);

    trace_to_disk(vec![(desc, args[1].to_string())].into_iter());
}
