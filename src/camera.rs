use crate::vector3::Vector3;
use crate::transform::Transform;

pub struct Camera {
    transform: Transform,
    fov: f32,
    front: f32,
    back: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            transform: Transform::new(),
            fov: 60.0,
            front: 0.01,
            back: 100.0,
        }
    }

    pub fn render(&self) {
        for _ in 1..10 {
            println!("ssssssss");
        }
    }
}