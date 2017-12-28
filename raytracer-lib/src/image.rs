
extern crate rand;
extern crate image;

use lib::*;
use std::vec::Vec;

use self::rand::{random, Closed01};
use self::image::{Rgb, ImageBuffer};

pub struct ImageDesc {
	pub width  : u32,
	pub height : u32
}

pub struct ImageOptions {
	pub samples : u32,
}

impl Clone for ImageDesc {
	fn clone(&self) -> Self {
		return ImageDesc {
			width  : self.width,
			height : self.height,
		};
	} 
}
impl Clone for ImageOptions {
	fn clone(&self) -> Self {
		return ImageOptions {
			samples : self.samples,
		};
	}
}

impl Copy for ImageDesc {} 
impl Copy for ImageOptions {}

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
	scene : &Scene) -> ImageBuffer<Rgb<u8>, Vec<u8>>
{
	let mut imagebuf = ImageBuffer::new(desc.width, desc.height);

	for (x, y, pixel) in imagebuf.enumerate_pixels_mut() {
		let colour = render_pixel(
			&Vec2u{ x: x, y: y },
			&desc,
			&opts,
			scene);

		*pixel = Rgb{ 
			data: [
				(colour.x * 255.0) as u8,
				(colour.y * 255.0) as u8, 
				(colour.z * 255.0) as u8] 
		};
	}

	return imagebuf;
}
