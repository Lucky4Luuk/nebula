use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

pub struct Engine {
    pub window: Window,
}

impl Engine {
    pub fn from_window_builder(el: &EventLoop<()>, wb: WindowBuilder) -> Self {
        Self {
            window: wb.build(&el).expect("Failed to open window!"),
        }
    }

    pub fn render(&mut self) {
        
    }
}
