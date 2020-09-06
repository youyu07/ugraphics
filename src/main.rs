use winit::{
    event::{Event, WindowEvent},
    event_loop:: {ControlFlow, EventLoop},
    window::{WindowBuilder,Window},
};

mod renderer;
mod scene_gltf;
mod shader;
mod pipeline;
mod camera;

use renderer::Renderer;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut rd = renderer::create_from_window(window).await;
    let _scene = scene_gltf::Scene::from_file("", rd.device());
    let pipeline = pipeline::create_select_pipeline(rd.device(), rd.color_format());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {event,..} => {
                match event {
                    WindowEvent::Resized(size) => {
                        //recreate swapchain
                        rd.resize((size.width, size.height));
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            },
            Event::RedrawRequested(_) => {
                // encode render pass
                if let Some(swap_chain) = rd.swap_chain() {
                    let frame = swap_chain.get_current_frame().unwrap().output;
                    
                    let mut encoder = rd.device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                    store: true,
                                },
                            }],
                            depth_stencil_attachment: None,
                        });
                    }

                    rd.queue().submit(Some(encoder.finish()));
                }
            },
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    futures::executor::block_on(run(event_loop, window));
}
