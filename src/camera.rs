extern crate nalgebra_glm as glm;


struct Camera<T: glm::RealField> {
    aspect: T,
    fov: T,
    near: T,
    far: T,
    position: glm::TVec3<T>,
    front: glm::TVec3<T>,
    right: glm::TVec3<T>,
    up: glm::TVec3<T>,
    world_up: glm::TVec3<T>,
    yaw: T,
    pitch: T,
}

fn create_perspective<T>(aspect: T, fovy: T, near: T, far: T) -> Camera<T> 
    where T:glm::RealField
{
    Camera {
        aspect,
        fov: fovy * glm::pi() / glm::convert(180.0),
        near,
        far,
        position: glm::zero(),
        front: glm::vec3(glm::convert(0.0), glm::convert(0.0), glm::convert(-1.0)),
        right: glm::vec3(glm::convert(1.0), glm::convert(0.0), glm::convert(-1.0)),
        up: glm::vec3(glm::convert(0.0), glm::convert(1.0), glm::convert(-1.0)),
        world_up: glm::vec3(glm::convert(0.0), glm::convert(1.0), glm::convert(-1.0)),
        yaw: glm::pi::<T>() / glm::convert(-0.5),
        pitch: glm::convert(0.0),
    }
}

impl<T> Camera<T> 
    where T:glm::RealField 
{
    fn perspective(&self) -> glm::TMat4<T> {
        glm::perspective(self.aspect, self.fov, self.near, self.far)
    }

    fn view(&self) -> glm::TMat4<T> {
        unimplemented!()
    }

    fn update(&mut self) {
        let front = glm::TVec3::<T>::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos()
        );

        self.front = front.normalize();

        self.right = self.front.cross(&self.world_up).normalize();
        self.up    = self.right.cross(&self.front).normalize();
    }
}