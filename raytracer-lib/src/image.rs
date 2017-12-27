
extern crate rand;

use vec::*;
use scene::*;
use std::vec::Vec;

use self::rand::{random, Closed01};

pub struct ImageDesc {
	pub width  : u32,
	pub height : u32
}

pub struct ImageOptions {
	pub samples : u32,
}

pub struct Image {
	pub desc   : ImageDesc,
	pub pixels : Vec<u8>,
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
	scene : &Scene) -> Image 
{
	let mut pixels : Vec<u8> = Vec::new();

	pixels.resize((desc.width * desc.height * 3) as usize, 0);

	for y in 0..desc.height {
		for x in 0..desc.width {
			let colour = render_pixel(
				&Vec2u{ x: x, y: y },
				&desc,
				&opts,
				scene);

			pixels[((y * desc.width + x) * 3 + 0) as usize] 
				= (colour.x * 256.0) as u8;
			pixels[((y * desc.width + x) * 3 + 1) as usize]
				= (colour.y * 256.0) as u8;
			pixels[((y * desc.width + x) * 3 + 2) as usize]
				= (colour.z * 256.0) as u8;
		}
	}

	return Image {
		desc   : desc,
		pixels : pixels
	};
}
