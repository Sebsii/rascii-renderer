mod vector3;
mod cube;

use cube::Cube;
use vector3::Vector3;

fn main() {
    let cube: Cube = Cube::new(Vector3::default(), Vector3::one());

    cube.do_stuff();
}

fn print() {
    for _ in 0..10 {
        for __ in 0..10 {
            print!("x");
        }
        print!("\n");
    }
}