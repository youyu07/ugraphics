
pub trait Renderer {
    fn device(&self) -> &wgpu::Device;
    fn queue(&self) -> &wgpu::Queue;
    fn resize(&mut self, size:(u32,u32));
    fn swap_chain(&mut self) -> Option<&mut wgpu::SwapChain>;
    fn color_format(&self) -> wgpu::TextureFormat;
}
pub struct ScreenRenderer
{
    _instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: winit::window::Window,
    surface: wgpu::Surface,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
}

pub async fn create_from_window(window: winit::window::Window) -> impl Renderer {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let surface = unsafe{instance.create_surface(&window)};
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }).await.expect("Unable to find a GPU!");
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor{
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        shader_validation: true,
    }, None).await.unwrap();
    let swap_chain_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        width: window.inner_size().width,
        height: window.inner_size().height,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
    ScreenRenderer {
        _instance: instance,
        device,
        queue,
        window,
        surface,
        swap_chain_desc,
        swap_chain,
    }
}

impl Renderer for ScreenRenderer {
    fn device(&self) -> &wgpu::Device {
        &self.device
    }

    fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    fn resize(&mut self, _size:(u32,u32)) {
        self.swap_chain_desc.width = self.window.inner_size().width;
        self.swap_chain_desc.height = self.window.inner_size().height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.swap_chain_desc);
    }

    fn swap_chain(&mut self) -> Option<&mut wgpu::SwapChain> {
        Some(&mut self.swap_chain)
    }

    fn color_format(&self) -> wgpu::TextureFormat {
        self.swap_chain_desc.format
    }
}