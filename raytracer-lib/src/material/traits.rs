
use vec::*;

pub trait Material: Sync + Send {
	fn base_colour (&self, point : Vec3d) -> Colour;
	fn roughness   (&self, point : Vec3d) -> f32;
	fn reflectivity(&self, point : Vec3d) -> f32;
}
