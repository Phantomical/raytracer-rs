
use vec;
use vec::Vec3d;
use material::Material;

pub struct Colour {
	pub colour : vec::Colour
}

impl Colour {
	pub fn new(colour : vec::Colour) -> Colour {
		return Colour {
			colour: colour
		};
	}
}

impl Material for Colour {
	fn base_colour(&self, _point : Vec3d) -> vec::Colour {
		return self.colour;
	}

	fn roughness(&self, _point : Vec3d) -> f32 {
		return 0.0;
	}
	fn metallicity(&self, _point : Vec3d) -> f32 {
		return 0.0;
	}
	fn reflectivity(&self, _point : Vec3d) -> f32 {
		return 0.0;
	}
}
