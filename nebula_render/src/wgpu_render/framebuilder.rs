use wgpu::Surface;
use wgpu::Queue;

pub struct FrameBuilder<'wgpu> {
    //WGPU stuff
    queue: &'wgpu Queue,
    surface: &'wgpu Surface,

    //Frame information
    clear_color: (f32, f32, f32, f32)
}

impl<'wgpu> FrameBuilder<'wgpu> {
    pub(crate) fn new(surface: &'wgpu Surface, queue: &'wgpu Queue, rgba: (f32, f32, f32, f32)) -> Self {
        Self {
            queue: queue,
            surface: surface,

            clear_color: rgba,
        }
    }

    /// Finish rendering this frame
    pub fn finish(self) {
        if let Ok(frame) = self.surface.get_current_texture() {
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            trace!("Swapchain texture acquired!");
            self.queue.submit(None);
            frame.present();
        } else {
            error!("Timeout on acquiring swapchain texture!");
            panic!("^");
        }
    }
}
