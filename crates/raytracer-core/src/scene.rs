use std::{fmt::Display, time::Instant};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use tracing::info;

use crate::{Camera, Color, Context, Image, RayShader, World};

/// A scene size: width and height.
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
    /// Create a scene size from a width and a height value.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Get the width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height.
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl From<(u32, u32)> for SceneSize {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
    }
}

/// A scene to render.
pub struct Scene {
    size: SceneSize,
    world: World,
    camera: Camera,
    samples_per_pixel: u32,
    max_depth: u32,
}

/// Scene builder.
pub struct SceneBuilder {
    size: SceneSize,
    samples_per_pixel: Option<u32>,
    world: Option<World>,
    camera: Option<Camera>,
    max_depth: Option<u32>,
}

impl SceneBuilder {
    /// Create a scene builder.
    pub fn new(size: SceneSize) -> Self {
        Self {
            size,
            samples_per_pixel: None,
            world: None,
            camera: None,
            max_depth: None,
        }
    }

    /// Set the scene size.
    pub fn with_size(mut self, size: SceneSize) -> Self {
        self.size = size;
        self
    }

    /// Set the scene antialiasing (samples per pixel).
    pub fn with_antialias(mut self, mut value: u32) -> Self {
        if value == 0 {
            value = 1;
        }

        self.samples_per_pixel = Some(value);
        self
    }

    /// Set the scene camera.
    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    /// Set the scene max rendering depth.
    pub fn with_max_depth(mut self, mut max_depth: u32) -> Self {
        if max_depth == 0 {
            max_depth = 1;
        }

        self.max_depth = Some(max_depth);
        self
    }

    /// Set the scene world.
    pub fn with_world(mut self, world: World) -> Self {
        self.world = Some(world);
        self
    }

    /// Build the scene.
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
    /// Create a new scene builder.
    pub fn builder(size: SceneSize) -> SceneBuilder {
        SceneBuilder::new(size)
    }

    /// Set the scene antialiasing (samples per pixel).
    pub fn set_antialias(&mut self, value: u32) {
        self.samples_per_pixel = value;
    }

    /// Set the scene max rendering depth.
    pub fn set_max_depth(&mut self, value: u32) {
        self.max_depth = value;
    }

    /// Set the scene scale.
    pub fn set_scale(&mut self, value: f64) {
        self.size.width = (self.size.width as f64 * value) as u32;
        self.size.height = (self.size.height as f64 * value) as u32;
    }

    /// Set the scene size.
    pub fn set_size(&mut self, value: SceneSize) {
        self.size = value;
    }

    /// Render the scene.
    pub fn render<S: RayShader>(&self, ctx: Context, shader: S) -> Image {
        self.render_parallel(ctx, shader, 1)
    }

    /// Render the scene on multiple threads.
    pub fn render_parallel<S: RayShader>(
        &self,
        ctx: Context,
        shader: S,
        thread_count: u32,
    ) -> Image {
        let mut pixels = vec![Color::BLACK; (self.size.width * self.size.height) as usize];

        info!(
            message = "Rendering image",
            size = %self.size,
            antialias = self.samples_per_pixel,
            max_depth = self.max_depth,
            thread_count = thread_count,
        );

        let pixels_per_thread = (self.size.width * self.size.height / thread_count) as usize;

        let m = MultiProgress::new();
        let sty = ProgressStyle::with_template("{bar:40.cyan/blue}  {msg:<20} {pos:>7}/{len:7}")
            .unwrap()
            .progress_chars("##-");

        let shader = &shader;
        let before_time = Instant::now();

        crossbeam::scope(|scope| {
            pixels
                .chunks_mut(pixels_per_thread)
                .enumerate()
                .for_each(|(chunk_id, slice)| {
                    let mut ctx = ctx.clone();

                    let pb = m.add(ProgressBar::new(slice.len() as u64));
                    pb.set_style(sty.clone());

                    if thread_count == 1 {
                        pb.set_message("Rendering image...");
                    } else {
                        pb.set_message(format!("Rendering chunk #{chunk_id}..."));
                    }

                    scope.spawn(move |_| {
                        slice.iter_mut().enumerate().for_each(|(idx, elem)| {
                            let pixel_idx = chunk_id * pixels_per_thread + idx;
                            *elem = self.render_pixel(pixel_idx, &mut ctx, shader, &pb);
                        });

                        pb.finish_with_message("Done");
                    });
                });
        })
        .unwrap();

        let after_time = Instant::now();
        let duration = (after_time - before_time).as_secs_f64();
        info!(message = "Rendering done", duration = duration);

        Image::from_pixels(self.size.width, pixels)
    }

    fn render_pixel(
        &self,
        pixel_idx: usize,
        ctx: &mut Context,
        shader: &dyn RayShader,
        pb: &ProgressBar,
    ) -> Color {
        // Compute x and y
        let x = pixel_idx as u32 % self.size.width;
        let y = pixel_idx as u32 / self.size.width;

        let mut color = Color::BLACK;

        for _ in 0..self.samples_per_pixel {
            let u = (x as f64 + Self::gen_jitter(&mut ctx.rng, self.samples_per_pixel))
                / (self.size.width - 1) as f64;
            let v = (self.size.height as f64 - y as f64
                + Self::gen_jitter(&mut ctx.rng, self.samples_per_pixel))
                / (self.size.height - 1) as f64;

            let ray = self.camera.cast_ray(ctx, u, v);
            color += shader.ray_color(ctx, &ray, &self.world, self.max_depth);
        }

        color /= self.samples_per_pixel as f64;

        // Gamma correction
        color = color.map(f64::sqrt);

        pb.inc(1);

        color
    }

    fn gen_random_f64<R: Rng>(rng: &mut R) -> f64 {
        rng.gen_range(0.0..=1.0)
    }

    fn gen_jitter<R: Rng>(rng: &mut R, samples_per_pixel: u32) -> f64 {
        if samples_per_pixel > 1 {
            Self::gen_random_f64(rng)
        } else {
            0.0
        }
    }
}
