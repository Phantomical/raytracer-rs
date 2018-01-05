
use colour::Colour;

macro_rules! hex {
	($e:expr) => (Colour::new(
		 (((($e as u32) & 0xFF0000) >> 16) as f32) / 255.0,
		 (((($e as u32) & 0x00FF00) >> 08) as f32) / 255.0,
		 (((($e as u32) & 0x0000FF) >> 00) as f32) / 255.0
	));
}

pub fn hex(colour : u32) -> Colour<f32> {
	return Colour::new(
		(((colour & 0xFF0000) >> 16) as f32) / 255.0,
		(((colour & 0x00FF00) >> 08) as f32) / 255.0,
		(((colour & 0x0000FF) >> 00) as f32) / 255.0
	);
}
// Some predefined colours. Goal: Have all HTML colours
pub const BLACK   : Colour<f32> = hex!(0x000000);
pub const WHITE   : Colour<f32> = hex!(0xFFFFFF);
pub const RED     : Colour<f32> = hex!(0xFF0000);
pub const GREEN   : Colour<f32> = hex!(0x00FF00);
pub const BLUE    : Colour<f32> = hex!(0x0000FF);
pub const YELLOW  : Colour<f32> = hex!(0xFFFF00);
pub const ORANGE  : Colour<f32> = hex!(0xFFA500);
pub const MAGENTA : Colour<f32> = hex!(0xFF00FF);
pub const GRAY    : Colour<f32> = hex!(0x808080);
