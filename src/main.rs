use winit::{
    event::WindowEvent,
};


mod shader;
mod pipeline;
mod app;
mod assets;
mod math;

struct Example {
    scene: Option<assets::Scene>,
    camera: math::Camera,
    uniform_buffer: wgpu::Buffer,
    _color_buffer: wgpu::Buffer,
    pipeline: pipeline::PipelineResource,
    bind_group: wgpu::BindGroup
}

impl app::App for Example {
    fn init(device: &wgpu::Device, queue: &wgpu::Queue, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let scene = {
            if let Some(path) = std::env::args().nth(1) {
                let path = std::path::Path::new(&path);
                if path.is_file() {
                    Some(futures::executor::block_on(assets::from_gltf(device, &path)))
                } else {None}
            } else {None}
        };

        let pipeline = pipeline::create_select_pipeline(device, sc_desc.format).unwrap();

        let camera = {
            let aspect = sc_desc.width as f32 / sc_desc.height as f32;
            let camera = math::perspective(aspect, 45.0, 1.0, 1000.0);
            camera
        };

        let uniform_buffer = {
            device.create_buffer(&wgpu::BufferDescriptor{
                label: Some("camera uniform"),
                size: std::mem::size_of::<math::Camera>() as _,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            })
        };

        let color_buffer = {
            device.create_buffer(&wgpu::BufferDescriptor{
                label: Some("object color"),
                size: 4 * 4,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            })
        };
        {
            let color:[f32; 4] = [0.5,0.5,0.0,1.0];
            queue.write_buffer(&color_buffer, 0, bytemuck::cast_slice(&[color]));
        }

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &pipeline.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(color_buffer.slice(..)),
                }
            ],
            label: None,
        });

        Example {
            scene,
            camera,
            uniform_buffer,
            _color_buffer: color_buffer,
            pipeline,
            bind_group
        }
    }

    fn update(&mut self, _event: WindowEvent) {
        
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, frame: &wgpu::SwapChainTexture, _spawner: &impl futures::task::LocalSpawn) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.camera]));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            if let Some(s) = &self.scene {
                rpass.set_pipeline(&self.pipeline.pipeline);
                rpass.set_bind_group(0, &self.bind_group, &[0,0]);
    
                for mesh in &s.meshes {
                    for sub in &mesh.subs {
                        rpass.set_index_buffer(sub.index_buffer.slice(..));
                        rpass.set_vertex_buffer(0,sub.vertex_buffer.slice(..));
                        
                        let range = 0..(sub.count as u32);
                        rpass.draw_indexed(range, 0, 0..1);
                    }
                }
            }
        }
        queue.submit(Some(encoder.finish()));
    }

    fn mouse_wheel(&mut self, delta: f32) {
        self.camera.mouse_wheel(delta * 0.1);
    }

    fn mouse_move(&mut self, x: f32, y: f32) {
        self.camera.mouse_move(x * 0.01, y * 0.01);
    }
}
fn main() {
    app::run::<Example>("example");
}