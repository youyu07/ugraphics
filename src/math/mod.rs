
mod camera;
pub use camera::{
    Camera,perspective,
};

pub fn get_forward_vector(rotation: &glam::Quat) -> glam::Vec3 {
    let q = glam::vec3(rotation.x(), rotation.y(), rotation.z());
    let t = q.cross(glam::Vec3::unit_z()) * 2.0;
    glam::Vec3::unit_z() + t * rotation.w() + q.cross(t)
}