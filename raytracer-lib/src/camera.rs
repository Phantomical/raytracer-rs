use lib::*;
use std::f64::consts::PI;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3d,

    pub forward: Vec3d,
    pub up: Vec3d,
    pub right: Vec3d,

    pub fov_y: f64,
    pub fov_x: f64,
}

impl Camera {
    /// The aspect ratio of the camera
    pub fn aspect(&self) -> f64 {
        return self.fov_x / self.fov_y;
    }

    /// Calculate a ray direction given screen coordinates
    pub fn screen_ray(&self, point: Vec2d) -> Ray {
        return Ray::new(
            self.position,
            rotate(self.forward, self.right, point.y * self.fov_y * 0.5)
                + rotate(self.forward, self.up, point.x * self.fov_x * 0.5),
        );
    }
}

pub struct CameraBuilder {
    cam: Camera,
}

const DEG2RAD: f64 = PI / 180.0;

impl CameraBuilder {
    /// Initializes the camera with default values. These
    /// default values are:
    ///
    /// *`position` = Vec3d::zero()
    /// *`forward`  = Vec3d::unit_z()
    /// *`up`       = Vec3d::unit_y()
    /// *`right`    = Vec3d::unit_x()
    /// *`fov_y`    = 60 degrees (in radians)
    /// *`fov_x`    = 80 degrees (in radians)
    ///
    /// Note: fov_y and fov_x are expressed in radians
    /// in the camera.
    pub fn new() -> CameraBuilder {
        return CameraBuilder {
            cam: Camera {
                position: Vec3d::zero(),
                forward: Vec3d::unit_z(),
                up: Vec3d::unit_y(),
                right: Vec3d::unit_x(),
                fov_y: 60.0 * DEG2RAD,
                fov_x: 80.0 * DEG2RAD,
            },
        };
    }

    /// Set the position of the camera under construction
    ///
    /// # Arguments
    /// *`position` - The position of the camera under construction
    pub fn position(mut self, position: Vec3d) -> Self {
        self.cam.position = position;
        return self;
    }

    /// Set the forward vector of the camera under construction.
    /// This is the direction that the camera is pointing.
    ///
    /// # Arguments
    /// *`forward` - The direction the camera will be pointing
    pub fn forward(mut self, forward: Vec3d) -> Self {
        self.cam.forward = forward.normalize();
        return self;
    }

    /// Set the upward vector of the camera under construction.
    ///
    /// # Arguments
    /// *`up` - The up vector of the camera
    pub fn up(mut self, up: Vec3d) -> Self {
        self.cam.up = up.normalize();
        return self;
    }

    /// Set the right vector of the camera under construction.
    ///
    /// # Arguments
    /// *`right` - The right vector of the camera.
    pub fn right(mut self, right: Vec3d) -> Self {
        self.cam.right = right.normalize();
        return self;
    }

    /// Set the vertical field of view for the camera.
    ///
    /// # Arguments
    /// *`fovy` - The vertical field of view (in radians)
    pub fn fov_y(mut self, fovy: f64) -> Self {
        self.cam.fov_y = fovy;
        return self;
    }

    /// Set the horizontal field of view for the camera.
    ///
    /// # Arguments
    /// *`fovx` - The horizontal field of view (in radians)
    pub fn fov_x(mut self, fovx: f64) -> Self {
        self.cam.fov_x = fovx;
        return self;
    }

    /// Set the fov_y and fov_x of the camera using
    /// the given FovY and aspect ration.
    ///
    /// # Arguments
    /// *`fovy`   - The vertical field of view (in radians)
    /// *`aspect` - The ration between the horizontal and vertical
    ///             field of view. (fov_x/fov_y)
    pub fn aspect_y(mut self, fovy: f64, aspect: f64) -> Self {
        self.cam.fov_y = fovy;
        self.cam.fov_x = fovy * aspect;
        return self;
    }
    /// Set the fov_y and fov_x of the camera using
    /// the given FovX and aspect ration.
    ///
    /// # Arguments
    /// *`fovy`   - The horizontal field of view (in radians)
    /// *`aspect` - The ration between the horizontal and vertical
    ///             field of view. (fov_x/fov_y)
    pub fn aspect_x(mut self, fovx: f64, aspect: f64) -> Self {
        self.cam.fov_x = fovx;
        self.cam.fov_y = fovx / aspect;
        return self;
    }

    /// Changes the right and up camera vectors so that
    /// they are at right angles to each other and the
    /// forward vector. It is recomended to do this when
    /// creating the camera as otherwise pixel determination
    /// will not be as expected.
    pub fn orthonormalize(mut self) -> Self {
        self.cam.right = cross(self.cam.up, self.cam.forward);
        self.cam.up = cross(self.cam.forward, self.cam.right);
        return self;
    }

    /// Creates the camera struct
    pub fn unwrap(self) -> Camera {
        self.cam
    }
}
