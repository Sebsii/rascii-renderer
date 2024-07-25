mod vector3;
mod cube;
mod camera;
mod transform;
mod matrix4;
mod vector2;

use camera::Camera;
use cube::Cube;
use vector3::Vector3;
use matrix4::Matrix4;
use vector2::Vector2;

fn main() {
    let cube = Cube::new(Vector3::zero(), Vector3::one());
    let camera = Camera::new();

    camera.render();
}

fn project(point: &Vector3, projection: &Matrix4) -> (i32, i32) {
    let projected: Vector3 = projection * point;
    let x = (projected.x * 10.0) as i32 + 40;
    let y = (projected.y * 10.0) as i32 + 12;
    (x, y)
}