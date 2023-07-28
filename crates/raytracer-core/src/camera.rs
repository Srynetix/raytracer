use crate::{Ray, Vec3};

pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        Self {
            viewport_height,
            viewport_width,
            focal_length,
        }
    }
}

impl Camera {
    pub fn from_aspect_ratio(aspect_ratio: f64) -> Self {
        let mut camera = Camera::default();
        camera.set_aspect_ratio(aspect_ratio);
        camera
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.viewport_width = self.viewport_height * aspect_ratio;
    }

    pub fn set_viewport_height(&mut self, height: f64) {
        self.viewport_height = height;
    }

    pub fn cast_ray(&self, origin: Vec3, horizontal_factor: f64, vertical_factor: f64) -> Ray {
        let horizontal = Vec3::from_xyz(self.viewport_width, 0.0, 0.0);
        let vertical = Vec3::from_xyz(0.0, self.viewport_height, 0.0);
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3::from_xyz(0.0, 0.0, self.focal_length);

        Ray::from_points(
            origin,
            lower_left_corner + horizontal_factor * horizontal + vertical_factor * vertical
                - origin,
        )
    }
}
