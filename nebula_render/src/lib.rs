#[macro_use] extern crate log;

pub mod distance_field;
pub mod camera;

//TODO: Pick based on feature flag
mod wgpu_render;
pub use wgpu_render::Renderer;
pub use wgpu_render::scene_renderer;
