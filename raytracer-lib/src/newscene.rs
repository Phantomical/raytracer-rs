use lib::light::Light;
use lib::*;
use raymarcher::raymarch;

use camera::Camera;

use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::io::Write;
use std::fs::File;

use pbr::ProgressBar;

use rand::{random, Closed01};
use image::{ImageBuffer, Rgb};

use futures::Future;
use futures::future::{join_all, lazy};
use futures_cpupool::*;

#[derive(Copy, Clone)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
}

pub struct ImageDesc {
    pub scene: Arc<Scene>,
    pub camera: Camera,

    pub size: ImageSize,
    pub opts: RaymarchOptions,
}

fn isect_illumination(
    light: &Arc<Light>,
    isect: &Intersection,
    scene: &Scene,
    opts: &RaymarchOptions,
) -> Colour {
    let mut count: usize = 0;
    let mut visible: usize = 0;

    for (ray, maxdist) in light.shadow_rays(isect) {
        let mut opts_new = *opts;
        opts_new.min_distance = opts.intersect_distance * 4.0;
        opts_new.max_distance = maxdist;

        let test = raymarch(ray, &scene.objects, &opts_new);

        count += 1;
        visible += match test {
            Some(_) => 0,
            None => 1,
        };
    }

    if count == 0 {
        count = 1;
        visible = 1;
    }

    let mult = (visible as f32) / (count as f32);

    return light.illumination(isect) * mult;
}

fn isect_colour(scene: &Scene, isect: &Intersection, opts: &RaymarchOptions) -> Colour {
    let base_colour = isect.object.material.base_colour(&isect);
    let mut illum = Colour::zero();

    for ref light in scene.lights.iter() {
        illum = max(isect_illumination(light, isect, scene, opts), illum);
    }

    return base_colour * clamp(illum, 0.0, 1.0);
}

fn trace_ray(scene: &Scene, ray: Ray, opts: RaymarchOptions) -> Colour {
    let isect = raymarch(ray, &scene.objects, &opts);

    match isect {
        None => scene.background,
        Some(ref isect) => isect_colour(scene, isect, &opts),
    }
}

pub fn render_pixel(
    pixel: Vec2u,
    desc: ImageSize,
    opts: RaymarchOptions,
    camera: Camera,
    scene: &Scene,
) -> Colour {
    fn randval() -> f64 {
        let Closed01(val) = random::<Closed01<f64>>();
        return val;
    }

    let mut result = Colour::zero();

    let point = Vec2d {
        x: ((pixel.x as f64) / (desc.width as f64)) * 2.0 - 1.0,
        y: ((pixel.y as f64) / (desc.height as f64)) * 2.0 - 1.0,
    };

    for _ in 0..desc.samples {
        let jitter = Vec2d {
            x: randval() / (desc.width as f64),
            y: randval() / (desc.height as f64),
        };

        result += trace_ray(scene, camera.screen_ray(point + jitter), opts);
    }

    return result / (desc.samples as f32);
}

type ImageBufferType = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn trace_image_future<T>(
    _image_idx: usize,
    desc: &ImageDesc,
    pool: &CpuPool,
    pb: Arc<Mutex<ProgressBar<T>>>,
) -> impl Future<Item = ImageBufferType>
where
    T: Write + Send + 'static,
{
    let mut futures = Vec::new();

    for y in 0..desc.size.height {
        futures.push(pool.spawn({
            let cached_scene = Arc::clone(&desc.scene);
            let opts = desc.opts;
            let size = desc.size;
            let camera = desc.camera;
            let pb = Arc::clone(&pb);
            lazy(move || {
                let scene = cached_scene;
                let mut values = Vec::new();
                for x in 0..size.width {
                    let colour = render_pixel(Vec2u { x: x, y: y }, size, opts, camera, &scene);

                    values.push(Rgb {
                        data: [
                            (colour.x * 255.0) as u8,
                            (colour.y * 255.0) as u8,
                            (colour.z * 255.0) as u8,
                        ],
                    });
                }

                pb.lock().expect("Unlock Error").inc();

                let res: Result<_, ()> = Ok(values);
                res
            })
        }));
    }

    let new_fut = join_all(futures.into_iter()).then({
        let size = desc.size;
        //let pb = Arc::clone(&pb);
        move |rows| {
            let mut imagebuf = ImageBufferType::new(size.width, size.height);

            for (y, row) in rows.unwrap().iter().enumerate() {
                for (x, pixel) in row.iter().enumerate() {
                    imagebuf.put_pixel(x as u32, y as u32, *pixel);
                }
            }

            //pb.lock()
            //	.expect("Unable to unlock progress bar!")
            //	.finish_print(&format!("done image {}", image_idx));

            let res: Result<_, ()> = Ok(imagebuf);
            res
        }
    });

    new_fut
}

pub fn trace_to_disk<I>(descs: I)
where
    I: Iterator<Item = (ImageDesc, String)>,
{
    let pool = CpuPool::new_num_cpus();
    let pb = Arc::new(Mutex::new(ProgressBar::new(0)));

    let mut futures = Vec::new();

    for (i, (desc, name)) in descs.enumerate() {
        let pb = Arc::clone(&pb);
        pb.lock().unwrap().total += desc.size.height as u64;
        futures.push(trace_image_future(i, &desc, &pool, pb).then(move |result| {
            let imagebuf = result.ok().unwrap();
            let ref mut file = File::create(&name).expect("Could not open file");

            image::ImageRgb8(imagebuf).save(file, image::PNG).unwrap();

            let res: Result<(), ()> = Ok(());
            res
        }));
    }

    //let handle = thread::spawn({
    //	let pb = Arc::clone(&pb);
    //	move || {
    //		pb.listen();
    //	}
    //});

    join_all(futures)
        .then(move |_| {
            pb.lock().unwrap().finish();
            let res: Result<(), ()> = Ok(());
            res
        })
        .wait()
        .unwrap();
    //handle.join().unwrap();
}
