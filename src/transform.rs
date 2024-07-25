use crate::vector3::Vector3;

pub struct Transform {
    position: Vector3,
    scale: Vector3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vector3::zero(),
            scale: Vector3::one(),
        }
    }
}