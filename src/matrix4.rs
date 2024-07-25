use std::ops::Mul;
use crate::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new() -> Self {
        Matrix4 {
            m: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        let mut mat = Matrix4::new();
        for i in 0..4 {
            mat.m[i][i] = 1.0;
        }
        mat
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let nf = 1.0 / (near - far);

        let mut m = [[0.0; 4]; 4];
        m[0][0] = f / aspect;
        m[1][1] = f;
        m[2][2] = (far + near) * nf;
        m[2][3] = (2.0 * far * near) * nf;
        m[3][2] = -1.0;
        m
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        let x = self.m[0][0] * rhs.x + self.m[0][1] * rhs.y + self.m[0][2] * rhs.z + self.m[0][3];
        let y = self.m[1][0] * rhs.x + self.m[1][1] * rhs.y + self.m[1][2] * rhs.z + self.m[1][3];
        let z = self.m[2][0] * rhs.x + self.m[2][1] * rhs.y + self.m[2][2] * rhs.z + self.m[2][3];
        let w = self.m[3][0] * rhs.x + self.m[3][1] * rhs.y + self.m[3][2] * rhs.z + self.m[3][3];

        Vector3::new(x / w, y / w, z / w)
    }
}