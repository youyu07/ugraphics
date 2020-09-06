use std::collections::HashMap;
use wgpu::util::DeviceExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum PrimitiveType {
    Position(wgpu::Buffer,usize),
    Normal(wgpu::Buffer,usize),
    Texcoord(wgpu::Buffer,usize),
    Indices(wgpu::Buffer,usize),
}

pub struct Primitive(Vec<PrimitiveType>);

pub struct Mesh(Vec<Primitive>);

pub struct Scene {
    pub document: gltf::Document,
    pub meshes: HashMap<String, Mesh>,
}

impl Scene {
    pub fn from_file(path: &str, device:&wgpu::Device) -> Result<Scene> {
        let (document, buffers, _images) = gltf::import(path)?;
        let mut meshes = HashMap::new();

        for gltf_mesh in document.meshes() {
            let primitives: Vec<Primitive> = gltf_mesh.primitives().map(|p| {
                let mut datas: Vec<PrimitiveType> = vec![];

                let create_buffer = |acc: &gltf::Accessor, usage: wgpu::BufferUsage|{
                    if let Some(view) = acc.view() {
                        let buf = &buffers[view.buffer().index()];
                        let start = acc.offset() + view.offset();
                        let end = acc.size();
                        let device_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                            label: None,
                            contents: &buf[start..end],
                            usage: usage,
                        });
                        (Some(device_buffer), acc.count())
                    } else {
                        (None, 0)
                    }
                };

                if let Some(acc) = p.get(&gltf::Semantic::Positions) {
                    if let (Some(buffer), count) = create_buffer(&acc, wgpu::BufferUsage::VERTEX) {
                        datas.push(PrimitiveType::Position(buffer, count));
                    }
                }

                if let Some(acc) = p.get(&gltf::Semantic::Normals) {
                    if let (Some(buffer), count) = create_buffer(&acc, wgpu::BufferUsage::VERTEX) {
                        datas.push(PrimitiveType::Normal(buffer, count));
                    }
                }

                if let Some(acc) = p.get(&gltf::Semantic::TexCoords(0)) {
                    if let (Some(buffer), count) = create_buffer(&acc, wgpu::BufferUsage::VERTEX) {
                        datas.push(PrimitiveType::Texcoord(buffer, count));
                    }
                }

                if let Some(acc) = p.indices() {
                    if let (Some(buffer), count) = create_buffer(&acc, wgpu::BufferUsage::INDEX) {
                        datas.push(PrimitiveType::Indices(buffer, count));
                    }
                }

                Primitive(datas)
            }).collect();

            meshes.insert("".to_string(), Mesh(primitives));
        }

        Ok(Scene {document, meshes})
    }
}