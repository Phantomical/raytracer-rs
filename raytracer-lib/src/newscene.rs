use lib::*;

use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::io::Write;
use std::fs::File;

use pbr::ProgressBar;

use image::{ImageBuffer, Rgb};

use futures::Future;
use futures::future::{join_all, lazy};
use futures_cpupool::*;

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
            let desc = (*desc).clone();
            let pb = Arc::clone(&pb);
            lazy(move || {
                let mut values = Vec::new();
                for x in 0..desc.size.width {
                    let colour = trace_pixel(
						&desc,
						Vec2u { x: x, y: y });

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
        move |rows| {
            let mut imagebuf = ImageBufferType::new(size.width, size.height);

            for (y, row) in rows.unwrap().iter().enumerate() {
                for (x, pixel) in row.iter().enumerate() {
                    imagebuf.put_pixel(x as u32, y as u32, *pixel);
                }
            }

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

    join_all(futures)
        .then(move |_| {
            pb.lock().unwrap().finish();
            let res: Result<(), ()> = Ok(());
            res
        })
        .wait()
        .unwrap();
}
