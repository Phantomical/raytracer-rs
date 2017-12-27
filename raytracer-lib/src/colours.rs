
use vec::*;

macro_rules! hex {
	($e:expr) => (Colour {
		x: (((($e as u32) & 0xFF0000) >> 16) as f32) / 256.0,
		y: (((($e as u32) & 0x00FF00) >> 08) as f32) / 256.0,
		z: (((($e as u32) & 0x0000FF) >> 00) as f32) / 256.0,
	});
}

pub fn hex(colour : u32) -> Colour {
	return Colour {
		x: (((colour & 0xFF0000) >> 16) as f32) / 256.0,
		y: (((colour & 0x00FF00) >> 08) as f32) / 256.0,
		z: (((colour & 0x0000FF) >> 00) as f32) / 256.0,
	};
}

pub const BLACK   : Colour = hex!(0x000000);
pub const WHITE   : Colour = hex!(0xFFFFFF);
pub const RED     : Colour = hex!(0xFF0000);
pub const GREEN   : Colour = hex!(0x00FF00);
pub const BLUE    : Colour = hex!(0x0000FF);
pub const YELLOW  : Colour = hex!(0xFFFF00);
pub const ORANGE  : Colour = hex!(0xFFA500);
pub const MAGENTA : Colour = hex!(0xFF00FF);
pub const GRAY    : Colour = hex!(0x808080);
