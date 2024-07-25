mod vector3;
mod cube;
mod camera;
mod transform;

use camera::Camera;
use cube::Cube;
use vector3::Vector3;

fn main() {
    let cube: Cube = Cube::new(Vector3::default(), Vector3::one());
    let camera: Camera = Camera::new();

    camera.render();
}