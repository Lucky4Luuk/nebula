use raw_window_handle::HasRawWindowHandle;

mod framebuilder;
use framebuilder::FrameBuilder;

pub mod scene_renderer;

use wgpu::Adapter;
use wgpu::SurfaceConfiguration;
use wgpu::Device;
use wgpu::Queue;
use wgpu::Instance;
use wgpu::Surface;

async fn get_device(instance: &Instance, surface: &Surface) -> (Adapter, Device, Queue) {
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(surface),
    }).await.expect("Failed to find an appropriate adapter!");

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
    }, None).await.expect("Failed to create device!");

    (adapter, device, queue)
}

pub struct Renderer {
    instance: Instance,
    surface: Surface,
    config: SurfaceConfiguration,
    device: Device,
    main_queue: Queue,
}

impl Renderer {
    pub fn from_handle(handle: &impl HasRawWindowHandle, size: (u32, u32)) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(handle) };
        let (adapter, device, queue) = pollster::block_on(get_device(&instance, &surface));

        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        Self {
            instance: instance,
            surface: surface,
            config: config,
            device: device,
            main_queue: queue,
        }
    }

    pub fn on_resize(&mut self, size: (u32, u32)) {
        self.config.width = size.0;
        self.config.height = size.1;
    }

    pub fn render(&self, rgba: (f32, f32, f32, f32)) -> FrameBuilder {
        FrameBuilder::new(&self.surface, &self.main_queue, rgba)
    }
}
