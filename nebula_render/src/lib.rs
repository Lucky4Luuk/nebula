#[macro_use] extern crate log;

mod opengl;

//TODO: Pick based on feature flag
pub use opengl::Renderer;
