use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VertexData {
    position: glam::Vec3,
    normal: glam::Vec3,
}

impl Default for VertexData {
    fn default() -> Self {
        Self {
            position: glam::Vec3::zero(),
            normal: glam::Vec3::zero(),
        }
    }
}

unsafe impl Zeroable for VertexData {}
unsafe impl Pod for VertexData {}

pub struct SubMesh {
    pub count: usize,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub mode: wgpu::PrimitiveTopology,
}
pub struct Mesh {
    pub name: String,
    pub subs: Vec<SubMesh>,
}

pub struct Scene {
    pub meshes: Vec<Mesh>,
}

pub async fn from_gltf(device: &wgpu::Device, path: &str) -> Scene {
    let (document, buffers, _images) = gltf::import(path).unwrap();

    let meshes: Vec<Mesh> = document.meshes().map(|gm| {
        let subs: Vec<SubMesh> = gm.primitives().map(|gp|{
            let reader = gp.reader(|bf|Some(&buffers[bf.index()]));
            let positions: Vec<_> = reader
                    .read_positions()
                    .map(|iter| iter.collect())
                    .ok_or(format!("mesh primitive is missing positions"))
                    .unwrap();

            let mut vertices: Vec<VertexData> = positions
                .iter()
                .map(|pos| VertexData {
                    position: glam::Vec3::from(pos.clone()),
                    ..Default::default()
                })
                .collect();

            if let Some(normals) = reader.read_normals() {
                for (i, normal) in normals.enumerate() {
                    vertices[i].normal = glam::Vec3::from(normal.clone());
                }
            }


            let vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor{
                label: Some("vertex buffer"),
                contents: &bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
        
            let indices: Vec<u32> = if let Some(i) = reader.read_indices() {
                i.into_u32().collect()
            } else {
                panic!("model doesn't have indices");
            };

            let index_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor{
                    label: Some("index buffer"),
                    contents: &bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsage::INDEX,
                }
            );

            SubMesh {
                count: indices.len(), 
                vertex_buffer, 
                index_buffer,
                mode: get_primitive_mode(gp.mode())
            }
        }).collect();

        Mesh {name: gm.name().unwrap().to_string(), subs}
    }).collect();

    Scene{ meshes }
}

fn get_primitive_mode(mode: gltf::mesh::Mode) -> wgpu::PrimitiveTopology {
    match mode {
        gltf::mesh::Mode::Points => wgpu::PrimitiveTopology::PointList,
        gltf::mesh::Mode::Lines => wgpu::PrimitiveTopology::LineList,
        gltf::mesh::Mode::LineStrip => wgpu::PrimitiveTopology::LineStrip,
        gltf::mesh::Mode::Triangles => wgpu::PrimitiveTopology::TriangleList,
        gltf::mesh::Mode::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        _ => panic!("Error loading mesh topology isn't supported!"),
    }
}