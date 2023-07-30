use std::fmt::Display;

use indicatif::{ProgressIterator, ProgressStyle};
use rand::Rng;
use tracing::info;

use crate::{Camera, Color, Context, Image, RayShader, World};

#[derive(Debug, Clone)]
pub struct SceneSize {
    width: u32,
    height: u32,
}

impl Display for SceneSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}x{}", self.width, self.height))
    }
}

impl SceneSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl From<(u32, u32)> for SceneSize {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
    }
}

pub struct Scene {
    size: SceneSize,
    world: World,
    camera: Camera,
    samples_per_pixel: u32,
    max_depth: u32,
}

pub struct SceneBuilder {
    size: SceneSize,
    samples_per_pixel: Option<u32>,
    world: Option<World>,
    camera: Option<Camera>,
    max_depth: Option<u32>,
}

impl SceneBuilder {
    pub fn new(size: SceneSize) -> Self {
        Self {
            size,
            samples_per_pixel: None,
            world: None,
            camera: None,
            max_depth: None,
        }
    }

    pub fn with_size(mut self, size: SceneSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_antialias(mut self, mut value: u32) -> Self {
        if value == 0 {
            value = 1;
        }

        self.samples_per_pixel = Some(value);
        self
    }

    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn with_max_depth(mut self, mut max_depth: u32) -> Self {
        if max_depth == 0 {
            max_depth = 1;
        }

        self.max_depth = Some(max_depth);
        self
    }

    pub fn with_world(mut self, world: World) -> Self {
        self.world = Some(world);
        self
    }

    pub fn build(self) -> Scene {
        Scene {
            size: self.size,
            samples_per_pixel: self.samples_per_pixel.unwrap_or(1),
            camera: self.camera.unwrap_or_else(|| Camera::builder().build()),
            world: self.world.unwrap_or_default(),
            max_depth: self.max_depth.unwrap_or(1),
        }
    }
}

impl Scene {
    pub fn builder(size: SceneSize) -> SceneBuilder {
        SceneBuilder::new(size)
    }

    fn gen_random_f64<R: Rng>(rng: &mut R) -> f64 {
        rng.gen_range(0.0..=1.0)
    }

    fn gen_jitter<R: Rng>(&self, rng: &mut R) -> f64 {
        if self.samples_per_pixel > 1 {
            Self::gen_random_f64(rng)
        } else {
            0.0
        }
    }

    pub fn set_antialias(&mut self, value: u32) {
        self.samples_per_pixel = value;
    }

    pub fn set_max_depth(&mut self, value: u32) {
        self.max_depth = value;
    }

    pub fn set_size(&mut self, value: SceneSize) {
        self.size = value;
    }

    pub fn render(&self, ctx: &mut Context, ray_color: impl RayShader) -> Image {
        let mut pixels = vec![];

        info!(
            message = "Rendering image",
            size = %self.size,
            antialias = self.samples_per_pixel,
            max_depth = self.max_depth
        );

        let progress_style = ProgressStyle::default_bar()
            .template("Rendering image ... {bar:40} {pos}/{len} ({elapsed_precise})")
            .unwrap();

        for y in (1..self.size.height + 1)
            .rev()
            .progress_with_style(progress_style)
        {
            for x in 0..self.size.width {
                let mut color = Color::black();

                for _ in 0..self.samples_per_pixel {
                    let u =
                        (x as f64 + self.gen_jitter(&mut ctx.rng)) / (self.size.width - 1) as f64;
                    let v =
                        (y as f64 + self.gen_jitter(&mut ctx.rng)) / (self.size.height - 1) as f64;

                    let ray = self.camera.cast_ray(ctx, u, v);
                    color += ray_color.ray_color(ctx, &ray, &self.world, self.max_depth);
                }

                // Divide color
                color /= self.samples_per_pixel as f64;

                // Gamma correction
                color = color.map(f64::sqrt);

                pixels.push(color);
            }
        }

        Image::from_pixels(self.size.width, pixels)
    }
}
