
use vec::*;
use scene::*;

use std::rand::Rng;

pub struct ImageDesc {
	pub width  : u32,
	pub height : u32
}

pub struct ImageOptions {
	pub samples : u32,
	pub seed    : u32
}

pub struct Image {
	pub pixels : [Colour],
	pub desc   : ImageDesc
}

pub fn render_pixel(
	desc : ImageDesc,
	opts : ImageOptions,
	scene : Scene) -> Colour
{
	let mut result = Colour::zero();


}
