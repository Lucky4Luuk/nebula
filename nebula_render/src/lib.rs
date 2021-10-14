use winit::window::Window;

use wgpu::Instance;
use wgpu::Surface;
use wgpu::Queue;
use wgpu::Device;

pub use nebula_scene::Scene;

async fn wgpu_device(instance: &Instance, surface: &Surface) -> (Device, Queue) {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    (device, queue)
}

pub struct Renderer {
    instance: Instance,
    surface: Surface,
    device: Device,
    queue: Queue,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let (device, queue) = pollster::block_on(wgpu_device(&instance, &surface));

        Self {
            instance: instance,
            surface: surface,
            device: device,
            queue: queue,
        }
    }

    pub fn render_scene(&self, scene: &Scene) {
        
    }
}
