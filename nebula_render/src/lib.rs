#[macro_use] extern crate log;

pub mod distance_field;
pub mod camera;

//TODO: Pick based on feature flag
mod opengl;
pub use opengl::Renderer;
pub use opengl::scene_renderer;
