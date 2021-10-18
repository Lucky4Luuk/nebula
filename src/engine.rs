use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

use nebula_render::Renderer;

pub struct Engine {
    pub window: Window,
    pub renderer: Renderer,
}

impl Engine {
    pub fn from_window_builder(el: &EventLoop<()>, wb: WindowBuilder) -> Self {
        let window = wb.build(&el).expect("Failed to open window!");
        Self {
            renderer: Renderer::from_handle(&window, window.inner_size().into()),
            window: window,
        }
    }

    pub fn render(&mut self) {
        let frame = self.renderer.render((1.0, 0.0, 0.0, 1.0));
        frame.finish();
    }
}
