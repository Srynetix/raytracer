use crate::{Context, Ray, Vec3};

/// Camera.
#[derive(Clone)]
pub struct Camera {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

/// Camera builder.
pub struct CameraBuilder {
    position: Vec3,
    look_at: Vec3,
    up_vector: Vec3,
    aspect_ratio: f64,
    aperture: f64,
    focus_distance: f64,
    field_of_view: Option<f64>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            look_at: Vec3::new(0.0, 0.0, -1.0),
            up_vector: Vec3::new(0.0, 1.0, 0.0),
            aspect_ratio: 16.0 / 9.0,
            field_of_view: None,
            aperture: 0.0,
            focus_distance: 1.0,
        }
    }
}

impl CameraBuilder {
    /// Create a new camera builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the camera position.
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    /// Set the camera target.
    pub fn with_look_at(mut self, look_at: Vec3) -> Self {
        self.look_at = look_at;
        self
    }

    /// Set the camera up vector.
    pub fn with_up_vector(mut self, up_vector: Vec3) -> Self {
        self.up_vector = up_vector;
        self
    }

    /// Set the camera aspect ratio.
    pub fn with_aspect_ratio(mut self, value: f64) -> Self {
        self.aspect_ratio = value;
        self
    }

    /// Set the camera lens aperture.
    pub fn with_aperture(mut self, value: f64) -> Self {
        self.aperture = value;
        self
    }

    /// Set the camera focus distance.
    pub fn with_focus_distance(mut self, value: f64) -> Self {
        self.focus_distance = value;
        self
    }

    /// Set the camera field of view.
    pub fn with_field_of_view(mut self, value: f64) -> Self {
        self.field_of_view = Some(value);
        self
    }

    /// Build the camera.
    pub fn build(self) -> Camera {
        Camera::new(
            self.position,
            self.look_at,
            self.up_vector,
            self.aspect_ratio,
            self.field_of_view.unwrap_or(90.0),
            self.aperture,
            self.focus_distance,
        )
    }
}

impl Camera {
    fn new(
        origin: Vec3,
        look_at: Vec3,
        up_vector: Vec3,
        aspect_ratio: f64,
        vertical_field_of_view: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_field_of_view.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (origin - look_at).normalized();
        let u = up_vector.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            u,
            v,
            lower_left_corner,
            lens_radius,
        }
    }

    /// Create a new camera builder.
    pub fn builder() -> CameraBuilder {
        CameraBuilder::new()
    }

    /// Cast a ray from the camera.
    pub fn cast_ray(&self, ctx: &mut Context, horizontal_factor: f64, vertical_factor: f64) -> Ray {
        let rd = self.lens_radius * Vec3::gen_in_unit_disk(&mut ctx.rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner
                + horizontal_factor * self.horizontal
                + vertical_factor * self.vertical
                - self.origin
                - offset,
        )
    }
}
