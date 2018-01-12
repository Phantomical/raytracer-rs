
use lib::Colour;

macro_rules! hex {
	($e:expr) => ([
		(((($e as u32) & 0xFF0000) >> 16) as f32) / 255.0,
		(((($e as u32) & 0x00FF00) >> 08) as f32) / 255.0,
		(((($e as u32) & 0x0000FF) >> 00) as f32) / 255.0,
	]);
}

pub fn hex(colour : u32) -> Colour {
	return Colour {
		x: (((colour & 0xFF0000) >> 16) as f32) / 255.0,
		y: (((colour & 0x00FF00) >> 08) as f32) / 255.0,
		z: (((colour & 0x0000FF) >> 00) as f32) / 255.0,
	};
}
// Some predefined colours. Goal: Have all HTML colours
pub const BLACK   : [f32; 3] = hex!(0x000000);
pub const WHITE   : [f32; 3] = hex!(0xFFFFFF);
pub const RED     : [f32; 3] = hex!(0xFF0000);
pub const GREEN   : [f32; 3] = hex!(0x00FF00);
pub const BLUE    : [f32; 3] = hex!(0x0000FF);
pub const YELLOW  : [f32; 3] = hex!(0xFFFF00);
pub const ORANGE  : [f32; 3] = hex!(0xFFA500);
pub const MAGENTA : [f32; 3] = hex!(0xFF00FF);
pub const GRAY    : [f32; 3] = hex!(0x808080);
