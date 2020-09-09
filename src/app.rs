use std::time::{Duration, Instant};
use winit::{
    event::{self, Event, WindowEvent, DeviceEvent, MouseScrollDelta},
    event_loop::{ControlFlow, EventLoop},
};

pub trait App: 'static + Sized {
    fn optional_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::default()
    }

    #[allow(unused_variables)]
    fn mouse_move(&mut self, x: f32, y: f32) {}

    #[allow(unused_variables)]
    fn mouse_wheel(&mut self, delta: f32) {}

    fn init(device: &wgpu::Device, queue: &wgpu::Queue, sc_desc: &wgpu::SwapChainDescriptor) -> Self;

    #[allow(unused_variables)]
    fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, sc_desc: &wgpu::SwapChainDescriptor) {}

    fn update(&mut self, event: WindowEvent);

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, frame: &wgpu::SwapChainTexture, spawner: &impl futures::task::LocalSpawn);
}

struct Setup {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn setup<A: App>(title: &str) -> Setup {
    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title(title);
    let window = builder.build(&event_loop).unwrap();

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let (_size, surface) = unsafe {
        let size = window.inner_size();
        let surface = instance.create_surface(&window);
        (size, surface)
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let optional_features = A::optional_features();
    let required_features = A::required_features();
    let adapter_features = adapter.features();

    let needed_limits = A::required_limits();

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: (optional_features & adapter_features) | required_features,
                limits: needed_limits,
                shader_validation: true,
            },
            trace_dir.ok().as_ref().map(std::path::Path::new),
        )
        .await
        .unwrap();

    Setup {
        window,
        event_loop,
        instance,
        surface,
        adapter,
        device,
        queue,
    }
}


fn start<A: App>(Setup {window,event_loop,instance,surface,adapter,device,queue}: Setup) {
    let (mut pool, spawner) = {
        let local_pool = futures::executor::LocalPool::new();
        let spawner = local_pool.spawner();
        (local_pool, spawner)
    };

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        // TODO: Allow srgb unconditionally
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let mut app = A::init(&device, &queue, &sc_desc);

    let mut last_update_inst = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter); // force ownership by the closure
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10));

        match event {
            Event::MainEventsCleared => {
                if last_update_inst.elapsed() > Duration::from_millis(20) {
                    window.request_redraw();
                    last_update_inst = Instant::now();
                }
                pool.run_until_stalled();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    sc_desc.width = if size.width == 0 { 1 } else { size.width };
                    sc_desc.height = if size.height == 0 { 1 } else { size.height };
                    app.resize(&device, &queue,&sc_desc);
                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                }
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    app.update(event);
                }
            },
            Event::RedrawRequested(_) => {
                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(_) => {
                        swap_chain = device.create_swap_chain(&surface, &sc_desc);
                        swap_chain.get_current_frame().expect("Failed to acquire next swap chain texture!")
                    }
                };

                app.render(&device, &queue, &frame.output, &spawner);
            },
            Event::DeviceEvent{event, ..} => {
                match event {
                    DeviceEvent::MouseMotion {delta} => {
                        app.mouse_move(delta.0 as f32, delta.1 as f32);
                    },
                    DeviceEvent::MouseWheel {delta} => {
                        match delta {
                            MouseScrollDelta::LineDelta(_x,y ) => {
                                app.mouse_wheel(y);
                            },
                            MouseScrollDelta::PixelDelta(p) => {
                                app.mouse_wheel(p.y as f32);
                            }
                        };
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    });
}

pub fn run<A:App>(title: &str) {
    let setup = futures::executor::block_on(setup::<A>(title));
    start::<A>(setup);
}