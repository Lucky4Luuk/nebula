#[macro_use] extern crate log;

use winit_input_helper::WinitInputHelper;

use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;
use winit::event_loop::EventLoop;
use winit::event_loop::ControlFlow;
use winit::event::Event;

pub mod engine;
use engine::Engine;

fn main() {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::max()).init();
    debug!("Hello, world!");

    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let mut engine = {
        let wb = WindowBuilder::new().with_title("Nebula").with_inner_size(LogicalSize::new(1280, 720));
        Engine::from_window_builder(&event_loop, wb)
    };

    event_loop.run(move |event, _event_loop, control_flow| {
        if let Event::RedrawRequested(_id) = event {
            engine.render();
        }

        if input.update(&event) {
            if input.quit() {
                *control_flow = ControlFlow::Exit;
            }

            if let Some(size) = input.window_resized() {
                engine.window.set_inner_size(size);
                engine.renderer.on_resize(size.into());
            }

            engine.window.request_redraw();
        }
    })
}
