use crate::{Ray, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    width: f64,
    height: f64,
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
}

impl Default for Camera {
    fn default() -> Self {
        let width = 256.0;
        let aspect_ratio = 16.0 / 9.0;
        let height = width / aspect_ratio;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        Self {
            aspect_ratio,
            width,
            height,
            viewport_height,
            viewport_width,
            focal_length,
        }
    }
}

impl Camera {
    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
        self.height = self.width / self.aspect_ratio;
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
        self.height = self.width / self.aspect_ratio;
        self.viewport_width = self.viewport_height * self.aspect_ratio;
    }

    pub fn set_viewport_height(&mut self, height: f64) {
        self.viewport_height = height;
        self.viewport_width = self.viewport_height * self.aspect_ratio;
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
