#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y}
    }

    pub fn one() -> Self {
        Vector2::new(1.0, 1.0)
    }

    pub fn zero() -> Self {
        Vector2::new(0.0, 0.0)
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Vector2 {
        let magnitude = self.magnitude();
        *self / magnitude
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Vector2::zero()
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
