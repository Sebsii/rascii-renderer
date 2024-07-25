use crate::vector3::Vector3;

pub struct Cube {
    position: Vector3,
    size: Vector3,
}

impl Cube {
    pub fn new(position: Vector3, size: Vector3) -> Self {
        Cube { position, size }
    }

    pub fn do_stuff(&self) {
        println!("wow, i am a cube and my position is {} and my size is {}", self.position, self.size);
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::new(Vector3::default(), Vector3::one())
    }
}