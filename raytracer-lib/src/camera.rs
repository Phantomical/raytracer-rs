
use vec::*;
use ray::*;

pub struct Camera {
	pub position : Vec3d,

	pub forward  : Vec3d,
	pub up       : Vec3d,
	pub right    : Vec3d,

	pub fov_y    : f64,
	pub fov_x    : f64
}

impl Camera {
	pub fn new(
		position : Vec3d,
		forward  : Vec3d,
		up       : Vec3d,
		right    : Vec3d,
		fovy     : f64,
		fovx     : f64
	) -> Camera {
		return Camera {
			position: position,
			forward:  forward,
			up:       up,
			right:    right,
			fov_y:    fovy,
			fov_x:    fovx,
		};
	}

	/// Set the fov_y and fov_x of the camera using
	/// the given FovY and aspect ration.
	/// 
	/// # Arguments
	/// *`fovy`   - The vertical field of view (in radians)
	/// *`aspect` - The ration between the horizontal and vertical
	///             field of view. (fov_x/fov_y)
	pub fn set_aspect_y(&mut self, fovy : f64, aspect : f64) {
		self.fov_y = fovy;
		self.fov_x = fovy * aspect;
	}
	/// Set the fov_y and fov_x of the camera using
	/// the given FovX and aspect ration.
	/// 
	/// # Arguments
	/// *`fovy`   - The horizontal field of view (in radians)
	/// *`aspect` - The ration between the horizontal and vertical
	///             field of view. (fov_x/fov_y)
	pub fn set_aspect_x(&mut self, fovx : f64, aspect : f64) {
		self.fov_x = fovx;
		self.fov_y = fovx / aspect;
	}

	/// The aspect ratio of the camera
	pub fn aspect(&self) -> f64 {
		return self.fov_x / self.fov_y;
	}

	/// Calculate a ray direction given screen coordinates
	pub fn screen_ray(&self, point : Vec2d) -> Ray {
		return Ray::new(self.position, 
			rotate(self.forward, self.right, point.y * self.fov_y - self.fov_y * 0.5) +
			rotate(self.forward, self.up, point.x * self.fov_x - self.fov_x * 0.5));
	}
}
