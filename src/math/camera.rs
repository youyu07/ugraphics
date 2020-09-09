use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    projection: glam::Mat4,
    transform: glam::Mat4,
}

unsafe impl Zeroable for Camera {}
unsafe impl Pod for Camera {}

pub fn perspective(aspect: f32, fov: f32, near: f32, far: f32) -> Camera {
    Camera {
        projection: glam::Mat4::perspective_rh(fov.to_radians(), aspect, near, far),
        transform: glam::Mat4::look_at_rh(glam::vec3(3.0,3.0,3.0), glam::Vec3::zero(), glam::Vec3::unit_y()),
    }
}

impl Camera {
    pub fn mouse_move(&mut self, x: f32, y: f32) {
        let m = glam::Quat::from_rotation_y(x) * glam::Quat::from_rotation_x(y);
        self.transform = glam::Mat4::from_quat(m) * self.transform;
    }

    pub fn mouse_wheel(&mut self, delta: f32) {
        let q = glam::Quat::from_rotation_mat4(&self.transform);
        let v = delta * super::get_forward_vector(&q);

        self.transform = self.transform * glam::Mat4::from_translation(v);
    }

    // pub fn translate(&mut self, v: &TVec3<T>) {
    //     self.transform = nalgebra_glm::translate(&self.transform, &v);
    // }
}

// #[allow(unused)]
// pub struct Camera<T: glm::RealField> {
//     aspect: T,
//     fov: T,
//     near: T,
//     far: T,
//     position: glm::TVec3<T>,
//     front: glm::TVec3<T>,
//     right: glm::TVec3<T>,
//     up: glm::TVec3<T>,
//     world_up: glm::TVec3<T>,
//     yaw: T,
//     pitch: T,
// }

// pub fn create_perspective<T>(aspect: T, fovy: T, near: T, far: T) -> Camera<T> 
//     where T:glm::RealField
// {
//     Camera {
//         aspect,
//         fov: fovy * glm::pi() / glm::convert(180.0),
//         near,
//         far,
//         position: glm::zero(),
//         front: glm::vec3(glm::convert(0.0), glm::convert(0.0), glm::convert(-1.0)),
//         right: glm::vec3(glm::convert(1.0), glm::convert(0.0), glm::convert(-1.0)),
//         up: glm::vec3(glm::convert(0.0), glm::convert(1.0), glm::convert(-1.0)),
//         world_up: glm::vec3(glm::convert(0.0), glm::convert(1.0), glm::convert(-1.0)),
//         yaw: glm::pi::<T>() / glm::convert(-0.5),
//         pitch: glm::convert(0.0),
//     }
// }

// impl<T> Camera<T> 
//     where T:glm::RealField 
// {
//     pub fn perspective(&self) -> glm::TMat4<T> {
//         glm::perspective(self.aspect, self.fov, self.near, self.far)
//     }

//     pub fn view(&self) -> glm::TMat4<T> {
//         glm::look_at(&self.position, &(self.position + self.front), &self.up)
//     }

//     pub fn mouse_move(&mut self, x: T, y: T) {
//         let speed:T = glm::convert(1.0);
//         self.yaw += x * speed;
//         self.pitch += y * speed;
//         if self.pitch > glm::convert(89.0) {
//             self.pitch = glm::convert(89.0);
//         }
//         if self.pitch < glm::convert(-89.0) {
//             self.pitch = glm::convert(-89.0);
//         }
//         self.update();
//     }

//     pub fn translate(&mut self, delta: glm::TVec3<T>) {
//         self.position += delta;
//     }

//     fn update(&mut self) {
//         let front = glm::TVec3::<T>::new(
//             self.yaw.cos() * self.pitch.cos(),
//             self.pitch.sin(),
//             self.yaw.sin() * self.pitch.cos()
//         );

//         self.front = front.normalize();

//         self.right = self.front.cross(&self.world_up).normalize();
//         self.up    = self.right.cross(&self.front).normalize();
//     }
// }