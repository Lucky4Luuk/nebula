use super::FrameBuilder;

use nebula_scene::Scene;

pub struct SceneRenderer {

}

impl SceneRenderer {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn render_scene(&self, frame: &FrameBuilder, scene: &Scene) {
        unsafe {
            // frame.gl.do_stuff()
        }
    }
}
