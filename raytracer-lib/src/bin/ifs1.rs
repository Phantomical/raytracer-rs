extern crate cgmath;
extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::colours;

use std::env;
use std::fs::File;

use std::sync::Arc;

mod custom {
    use raytracer::Vec3d;
    use raytracer::object::Raymarchable;
    use builder::deg2rad;

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
    use std::sync::Arc;
    use raytracer::Scene;
    use raytracer::builder::*;
    use raytracer::colours;

    pub fn add_objects(scene: &mut Scene) {
        scene.add_object(
            sphere([0.0, -10001.0, 0.0], 10000.0),
            solid_colour(colours::WHITE),
        );

        scene.add_object(Arc::new(custom::IFSElement {}), normal());
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
