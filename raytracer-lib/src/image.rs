
extern crate image;

use lib::*;
use std::vec::Vec;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;

use threadpool::*;
use rand::{random, Closed01};
use self::image::{Rgb, ImageBuffer};

#[derive(Clone, Copy)]
pub struct ImageDesc {
	pub width  : u32,
	pub height : u32
}

#[derive(Clone, Copy)]
pub struct ImageOptions {
	pub samples : u32,
}

fn randval() -> f64 {
	let Closed01(val) = random::<Closed01<f64>>();
	return val;
}

pub fn render_pixel(
	pixel : &Vec2u,
	desc  : &ImageDesc,
	opts  : &ImageOptions,
	scene : &Scene) -> Colour
{
	let mut result = Colour::zero();

	let point = Vec2d {
		x: ((pixel.x as f64) / (desc.width  as f64)) * 2.0 - 1.0,
		y: ((pixel.y as f64) / (desc.height as f64)) * 2.0 - 1.0
	};

	for _ in 0..opts.samples {
		let jitter = Vec2d {
			x: randval() / (desc.width  as f64),
			y: randval() / (desc.height as f64)
		};

		result += scene.trace_point(point + jitter);
	}

	return result / (opts.samples as f32);
}

pub fn trace_image(
	desc  : ImageDesc,
	opts  : ImageOptions,
	scene : Arc<Scene>) -> ImageBuffer<Rgb<u8>, Vec<u8>>
{
	let imagebuf = Arc::new(Mutex::new(ImageBuffer::new(desc.width, desc.height)));
	let (tx, rx) = channel();
	let pool = ThreadPool::new(num_cpus::get());

	for y in 0..desc.height {
		pool.execute({
			let imagebuf = Arc::clone(&imagebuf);
			let scene = Arc::clone(&scene);
			let tx = tx.clone();
			move || {
				for x in 0..desc.width {
					let colour = render_pixel(
						&Vec2u{ x: x, y: y },
						&desc,
						&opts,
						&*scene);

					imagebuf.lock()
						.unwrap()
						.put_pixel(x, y, Rgb{ 
							data: [
								(colour.x * 255.0) as u8,
								(colour.y * 255.0) as u8, 
								(colour.z * 255.0) as u8] 
						});
				}

				tx.send(y).expect("Channel is present");
			}
		});
	}

	for _ in 0..desc.height {
		let v = rx.recv();
		println!("Line {} complete.", 
			v.expect("Error occurred while tracing image"));
	}

	{ let _ = imagebuf.lock(); }

	return Arc::try_unwrap(imagebuf)
		.expect("Lock stull has multiple owners")
		.into_inner()
		.expect("Mutex cannot be locked");
}
