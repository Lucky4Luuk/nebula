use wgpu::Device;
use wgpu::Surface;
use wgpu::Queue;
use wgpu::Color;

pub struct FrameBuilder<'wgpu> {
    //WGPU stuff
    queue: &'wgpu Queue,
    surface: &'wgpu Surface,
    device: &'wgpu Device,

    //Frame information
    clear_color: (f32, f32, f32, f32)
}

impl<'wgpu> FrameBuilder<'wgpu> {
    pub(crate) fn new(surface: &'wgpu Surface, queue: &'wgpu Queue, device: &'wgpu Device, rgba: (f32, f32, f32, f32)) -> Self {
        Self {
            queue: queue,
            surface: surface,
            device: device,

            clear_color: rgba,
        }
    }

    /// Finish rendering this frame
    pub fn finish(self) {
        if let Ok(frame) = self.surface.get_current_texture() {
            trace!("Swapchain texture acquired!");
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(Color { r: self.clear_color.0 as f64, g: self.clear_color.1 as f64, b: self.clear_color.2 as f64, a: self.clear_color.3 as f64 }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
                //rpass.set_pipeline()
                //rpass.draw(0..3, 0..1) //vertex range, instance range
            }

            self.queue.submit(Some(encoder.finish()));

            frame.present();
        } else {
            error!("Timeout on acquiring swapchain texture!");
            panic!("^");
        }
    }
}
