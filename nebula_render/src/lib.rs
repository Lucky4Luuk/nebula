use std::collections::HashMap;

use winit::window::Window;

use wgpu::ShaderModule;
use wgpu::Instance;
use wgpu::Surface;
use wgpu::Queue;
use wgpu::Device;

use wgpu::include_spirv;

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

    shaders: HashMap<String, ShaderModule>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let (device, queue) = pollster::block_on(wgpu_device(&instance, &surface));

        let mut obj = Self {
            instance: instance,
            surface: surface,
            device: device,
            queue: queue,

            shaders: HashMap::new(),
        };

        // obj.load_shader_raw("rt_main", include_spirv!("../shaders/rt.spirv"));

        obj
    }

    pub fn load_shader_spirv(&mut self, name: impl AsRef<str>, src: &[u32]) {
        let shader = self.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(std::borrow::Cow::Borrowed(src))
        });
        self.shaders.insert(name.as_ref().to_string(), shader);
    }

    pub fn load_shader_raw(&mut self, name: impl AsRef<str>, src: wgpu::ShaderModuleDescriptor) {
        let shader = self.device.create_shader_module(&src);
        self.shaders.insert(name.as_ref().to_string(), shader);
    }

    pub fn render_scene(&self, scene: &Scene) {

    }
}
