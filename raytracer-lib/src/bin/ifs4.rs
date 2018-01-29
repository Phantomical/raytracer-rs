extern crate cgmath;
extern crate gradient;
extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::colours;

use std::env;
use std::fs::File;

use std::sync::Arc;

mod custom {
    use raytracer::{Vec3d, vec3, abs};
    use raytracer::object::{Raymarchable, IFS};
    use std::vec::Vec;

	#[derive(Copy, Clone)]
    pub struct IFSElement {
        pub angle: f64,
		pub scale: f64,
		pub c: Vec3d
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

				x=abs(x);y=abs(y);z=abs(z);
				if x-y<0.0 { let x1=y;y=x;x=x1; }
				if x-z<0.0 { let x1=z;z=x;x=x1; }
				if y-z<0.0 { let y1=z;z=y;y=y1; }

				rotate2(self.angle, &mut x, &mut y, &mut z);
    
				x=scale*x-cx*(scale-1.0);
				y=scale*y-cy*(scale-1.0);
				z=scale*z;
				if z>0.5*cz*(scale-1.0) { z-=cz*(scale-1.0) }
				
				r=x*x+y*y+z*z;
            }
            return r.sqrt() * scale.powi(-10); //the estimated distance
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

				x=abs(x);y=abs(y);z=abs(z);
				if x-y<0.0 { let x1=y;y=x;x=x1; }
				if x-z<0.0 { let x1=z;z=x;x=x1; }
				if y-z<0.0 { let y1=z;z=y;y=y1; }

				rotate2(self.angle, &mut x, &mut y, &mut z);
    
				x=scale*x-cx*(scale-1.0);
				y=scale*y-cy*(scale-1.0);
				z=scale*z;
				if z>0.5*cz*(scale-1.0) { z-=cz*(scale-1.0) }

                points.push(vec3(x, y, z));
            }

            return Box::new(points.into_iter());
        }
    }
}

mod add_objects {
    use custom;
    use std::sync::Arc;
    use raytracer::{Scene, vec3};
    use raytracer::builder::*;
    use raytracer::colours;
    use raytracer::material::OriginTrap;

    use gradient::Gradient;

    pub fn add_objects(scene: &mut Scene, angle: f64) {
        scene.add_object(
            sphere([0.0, -10002.0, 0.0], 10000.0),
            solid_colour(colours::WHITE),
        );

		let elem = custom::IFSElement { angle, scale: 3.0, c: vec3(1.0, 1.0, 1.0) };
        scene.add_object(
            Arc::new(elem),
            Arc::new(OriginTrap::new(
                Gradient::new(&[
                    (colour(colours::ORANGE), 0.75),
                    (colour(colours::GRAY), 0.4),
                    (colour(colours::BLUE), 0.0),
                ]),
                elem
            )),
        );
    }

    pub fn add_lights(scene: &mut Scene) {
        scene.add_light(directional([0.0, -1.0, 2.0]));//, 0.0872665, 10));
        scene.add_light(ambient([0.2; 3]));
    }
}

use raytracer::builder::deg2rad;

fn create_scene(angle: f64, desc: &ImageDesc) -> Scene {
    let camera = CameraBuilder::new()
        .position(vec3(4.0, 1.0, -8.0))
        .forward(vec3(-4.0, -1.0, 8.0))
		.aspect_y(deg2rad(60.0), (desc.width as f64) / (desc.height as f64))
        .orthonormalize()
        .unwrap();

    let opts = RaymarchOptions {
        max_distance: 1.0e8,
        ..Default::default()
    };

    let mut scene = Scene::new(camera, opts, builder::colour(colours::BLACK));

    add_objects::add_objects(&mut scene, deg2rad(angle));
    add_objects::add_lights(&mut scene);

    return scene;
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: raytracer <output-file> <angle>");
        return;
    }

    let angle = args[2].parse().expect("Error: Angle was not a number");

    let desc = ImageDesc {
		//width: 3840,
		//height: 2160,

		width: 1200,
		height: 800
    };
    let opts = ImageOptions { samples: 20 };
    let scene = Arc::new(create_scene(angle, &desc));

    let image_val = trace_image(desc, opts, scene);

    let ref mut file = File::create(args[1].clone()).unwrap();

    image::ImageRgb8(image_val).save(file, image::PNG).unwrap();
}
