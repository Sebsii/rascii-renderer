use crate::transform::Transform;

pub struct Camera {
    transform: Transform,
    fov: f32,
    near: f32,
    far: f32,
    aspect: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            transform: Transform::new(),
            fov: 60.0,
            near: 0.01,
            far: 100.0,
            aspect: 16.0 / 9.0,
        }
    }

    pub fn render(&self) {
        for _ in 1..10 {
            println!("ssssssss");
        }
    }
}