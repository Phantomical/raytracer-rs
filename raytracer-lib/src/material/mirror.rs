
use vec::*;
use material::Material;

pub struct Mirror {

}

impl Material for Mirror {
	fn base_colour(&self, _point : Vec3d) -> Colour {
		return Colour::new(0.0, 0.0, 0.0);
	}
	fn roughness(&self, _point : Vec3d) -> f32 {
		return 0.0;
	}
	fn reflectivity(&self, _point : Vec3d) -> f32 {
		return 1.0;
	}
}
