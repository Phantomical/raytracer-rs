use vec::*;

pub trait Material: Sync + Send {
    fn base_colour(&self, point: Vec3d) -> Colour;

    fn roughness(&self, _point: Vec3d) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _point: Vec3d) -> f32 {
        return 0.0;
    }
}
