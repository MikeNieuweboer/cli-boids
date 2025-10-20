use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0f32, y: 0f32 };

    // Magnitude of vector
    pub fn magnitude(&self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y);
    }

    // Square magnitude of vector
    pub fn sqr_magnitude(&self) -> f32 {
        return self.x * self.x + self.y + self.y;
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
