use crate::{hittable::Hittable, Camera, Color, Image, Ray, Vec3, World};

pub struct Scene {
    world: World,
    camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            world: World::default(),
        }
    }

    pub fn add_hittable(&mut self, element: Box<dyn Hittable>) {
        self.world.add_hittable(element);
    }

    pub fn render(&self, ray_color_fn: impl Fn(Ray, &dyn Hittable) -> Color) -> Image {
        let mut pixels = vec![];
        let origin = Vec3::zero();

        for y in (1..self.camera.height() + 1).rev() {
            for x in 0..self.camera.width() {
                let u = x as f64 / (self.camera.width() - 1) as f64;
                let v = y as f64 / (self.camera.height() - 1) as f64;
                pixels.push(ray_color_fn(
                    self.camera.cast_ray(origin, u, v),
                    &self.world,
                ));
            }
        }

        Image::from_pixels(self.camera.width(), pixels)
    }
}
