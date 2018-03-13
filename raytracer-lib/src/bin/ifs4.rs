extern crate gradient;
extern crate image;
extern crate raytracer;
#[macro_use]
extern crate serde;

#[macro_use]
extern crate runtime_fmt;

use raytracer::*;
use raytracer::math::*;
use raytracer::colours;

use std::env;

use std::sync::Arc;
use std::vec::Vec;

mod custom {
    use raytracer::math::*;
    use raytracer::object::{Raymarchable, IFS};
    use std::vec::Vec;

    #[derive(Copy, Clone, Serialize, Deserialize)]
    pub struct IFSElement {
        pub angle: f64,
        pub scale: f64,
        pub c: Vec3d,
    }

    fn rotate1(angle: f64, x: &mut f64, y: &mut f64, _: &mut f64) {
        let (im, re) = angle.sin_cos();

        let a = re * *x - im * *y;
        *y = re * *y + im * *x;
        *x = a;
    }
    fn rotate2(angle: f64, mut x: &mut f64, mut y: &mut f64, mut z: &mut f64) {
        rotate1(angle, &mut x, &mut z, &mut y);
    }

    impl Raymarchable for IFSElement {
        fn distance(&self, point: Vec3d) -> f64 {
            let mut x = point.x;
            let mut y = point.y;
            let mut z = point.z;
            let scale: f64 = self.scale;
            let cx = self.c.x;
            let cy = self.c.y;
            let cz = self.c.z;

            let mut r = x * x + y * y + z * z;
            for _ in 0..10 {
                rotate1(self.angle, &mut x, &mut y, &mut z);

                x = abs(x);
                y = abs(y);
                z = abs(z);
                if x - y < 0.0 {
                    let x1 = y;
                    y = x;
                    x = x1;
                }
                if x - z < 0.0 {
                    let x1 = z;
                    z = x;
                    x = x1;
                }
                if y - z < 0.0 {
                    let y1 = z;
                    z = y;
                    y = y1;
                }

                rotate2(self.angle, &mut x, &mut y, &mut z);

                x = scale * x - cx * (scale - 1.0);
                y = scale * y - cy * (scale - 1.0);
                z = scale * z;
                if z > 0.5 * cz * (scale - 1.0) {
                    z -= cz * (scale - 1.0)
                }

                r = x * x + y * y + z * z;
            }
            return (r.sqrt() - 2.0) * scale.powi(-10); //the estimated distance
        }
    }

    impl IFS for IFSElement {
        fn points(&self, point: Vec3d) -> Box<Iterator<Item = Vec3d>> {
            let mut x = point.x;
            let mut y = point.y;
            let mut z = point.z;
            let scale: f64 = 2.0;
            let cx = self.c.x;
            let cy = self.c.y;
            let cz = self.c.z;

            let mut points = Vec::new();

            points.push(point);

            for _ in 0..10 {
                rotate1(self.angle, &mut x, &mut y, &mut z);

                x = abs(x);
                y = abs(y);
                z = abs(z);
                if x - y < 0.0 {
                    let x1 = y;
                    y = x;
                    x = x1;
                }
                if x - z < 0.0 {
                    let x1 = z;
                    z = x;
                    x = x1;
                }
                if y - z < 0.0 {
                    let y1 = z;
                    z = y;
                    y = y1;
                }

                rotate2(self.angle, &mut x, &mut y, &mut z);

                x = scale * x - cx * (scale - 1.0);
                y = scale * y - cy * (scale - 1.0);
                z = scale * z;
                if z > 0.5 * cz * (scale - 1.0) {
                    z -= cz * (scale - 1.0)
                }

                points.push(vec3(x, y, z));
            }

            return Box::new(points.into_iter());
        }
    }
}

mod add_objects {
    use custom;
    use raytracer::SceneBuilder;
	use raytracer::math::vec3;
    use raytracer::builder::*;
    use raytracer::colours;
    use raytracer::material::OriginTrap;

    use gradient::Gradient;

    pub fn add_objects(scene: SceneBuilder, angle: f64) -> SceneBuilder {
        let elem = custom::IFSElement {
            angle,
            scale: 3.0,
            c: vec3(1.0, 1.0, 1.0),
        };

        scene
            .add_object(
                sphere([0.0, -10002.0, 0.0], 10000.0),
                solid_colour(colours::WHITE),
            )
            .add_object(
                bound_sphere(elem, 3.0),
                OriginTrap::new(
                    Gradient::new(&[
                        (colour(colours::ORANGE), 0.75),
                        (colour(colours::GRAY), 0.4),
                        (colour(colours::BLUE), 0.0),
                    ]),
                    elem,
                ),
            )
    }

    pub fn add_lights(scene: SceneBuilder) -> SceneBuilder {
        scene
            .add_light(fuzzy_directional([0.0, -1.0, 2.0], 0.0872665, 10))
            .add_light(ambient([0.2; 3]))
    }
}

use raytracer::builder::deg2rad;

fn create_scene(angle: f64, size: ImageSize) -> ImageDesc {
    let camera = CameraBuilder::new()
        .position(vec3(4.0, 1.0, -8.0))
        .forward(vec3(-4.0, -1.0, 8.0))
        .aspect_y(deg2rad(60.0), (size.width as f64) / (size.height as f64))
        .orthonormalize()
        .unwrap();

    let opts = RaymarchOptions {
        max_distance: 1.0e8,
        ..Default::default()
    };

    let mut scene = SceneBuilder::new().background(builder::colour(colours::BLACK));

    scene = add_objects::add_objects(scene, deg2rad(angle));
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

    if args.len() < 3 {
        println!("Usage: raytracer <output-file> <angle>");
        return;
    }

    let mut descriptors = Vec::new();
    let size = ImageSize {
        //width: 3840,
        //height: 2160,
        width: 1080,
        height: 720,

        samples: 10,
    };

    for i in 2..args.len() {
        let angle = args[i].parse().expect("Error: Angle was not a number");
        let name = rt_format!(&args[1], i - 2).expect("Could not format string");

        descriptors.push((create_scene(angle, size), name));
    }

    trace_to_disk(descriptors.into_iter());
}
