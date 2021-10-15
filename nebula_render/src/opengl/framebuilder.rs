use crate::opengl::Gl;

pub struct FrameBuilder<'gl> {
    gl: &'gl Gl,
}

impl<'gl> FrameBuilder<'gl> {
    pub fn new(gl: &'gl Gl) -> Self {
        gl.make_current();
        Self {
            gl: gl,
        }
    }

    pub fn clear(&self, rgba: (f32, f32, f32, f32)) {
        unsafe {
            self.gl.ClearColor(rgba.0, rgba.1, rgba.2, rgba.3);
            self.gl.Clear(gl_bindings::COLOR_BUFFER_BIT);
        }
    }

    pub fn finish(self) {
        self.gl.make_not_current();
        self.gl.swap_buffers();
    }
}
