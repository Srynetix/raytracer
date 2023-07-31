//! Raytracer core module.
//!
//! Contains everything needed for the raytracer to work.

#![forbid(missing_docs)]

mod camera;
mod collider;
mod color;
mod context;
mod hit_record;
mod image;
mod material;
pub mod primitives;
mod random;
mod ray;
mod ray_shader;
mod renderer;
mod scene;
mod vec3;
mod world;

pub use camera::Camera;
pub use collider::Collider;
pub use color::Color;
pub use context::Context;
pub use hit_record::HitRecord;
pub use image::Image;
pub use material::{Material, ScatterResult};
pub use random::{RngWrapper, SeedType};
pub use ray::Ray;
pub use ray_shader::RayShader;
pub use renderer::Renderer;
pub use scene::{Scene, SceneBuilder, SceneSize};
pub use vec3::Vec3;
pub use world::World;

pub use rand;
pub use rand_chacha;
