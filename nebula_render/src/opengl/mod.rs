use std::ops::Deref;

use raw_window_handle::HasRawWindowHandle;

use raw_gl_context::{GlContext, GlConfig, Profile};

use gl_bindings::Gl as RawGl;

mod framebuilder;
use framebuilder::FrameBuilder;

pub struct Gl {
    gl: RawGl,
    context: GlContext,
}

impl Gl {
    pub fn new(gl: RawGl, context: GlContext) -> Self {
        Self {
            gl: gl,
            context: context,
        }
    }

    pub fn make_current(&self) {
        self.context.make_current();
    }

    pub fn make_not_current(&self) {
        self.context.make_not_current();
    }

    pub fn swap_buffers(&self) {
        self.context.swap_buffers();
    }
}

impl Deref for Gl {
    type Target = RawGl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

fn create_context(handle: &impl HasRawWindowHandle, version: (u8, u8)) -> Gl {
    let mut config = GlConfig::default();
    config.version = version;
    config.alpha_bits = 0;
    config.profile = Profile::Core;
    let context = GlContext::create(handle, config).expect("Failed to create opengl context!");
    context.make_current();
    let gl = RawGl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    // context.make_not_current(); //Throws an invalid operation

    trace!("Opengl has been loaded!");
    Gl::new(gl, context)
}

pub struct Renderer {
    gl: Gl,
}

impl Renderer {
    pub fn from_handle(handle: &impl HasRawWindowHandle) -> Self {
        let gl = create_context(handle, (4,5));
        Self {
            gl: gl,
        }
    }

    pub fn render(&self) -> FrameBuilder {
        FrameBuilder::new(&self.gl)
    }
}
