//! Creation and manipulation of a 2D f32 vector.
//!
//! # Vector2
//! Contains a simple definition of a 2D vector, similar to
//! the already existing Vector2 crate, as its recreation was used
//! as learning opportunity.
//!
//! ## Operations
//! The vector implements basic operations
//! such as addition, multiplication and division, along with the
//! respective traits to allow for easier use
//!
//! # Examples
//! Vector2 supports most elementwise operations with other vectors.
//! ```
//! let v1: Vector2 = Vector2::ZERO;
//! let v2: Vector2 = Vector2::new(1.0, 2.0);
//!
//! v1 += v2
//! assert_eq!(v1.x, v2.x);
//! assert_eq!(v1.y, v2.y);
//!
//! v1 = v1 * v2;
//! assert_eq!(v1.x, v1.x * v2.x);
//! assert_eq!(v1.y, v1.y * v2.y);
//! ```
//!
//! Besides operations with other vectors, also scalar division and
//! multiplication are supported
//! ```
//! let v1: Vector2 = Vector2::new(1.0, 2.0);
//!
//! v1 *= 3f32;
//! v1 = v1 / 2f32;
//!
//! assert_eq!(v1.x, 3f32 / 2f32);
//! assert_eq!(v1.y, 2f32 * 3f32 / 2f32);
//! ```

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Constant defining the zero vector
    pub const ZERO: Vector2 = Vector2 { x: 0f32, y: 0f32 };

    /// Creates a new [`Vector2`].
    #[inline]
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    /// Magnitude of vector
    #[inline]
    #[allow(dead_code)]
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    /// Square magnitude of vector
    #[inline]
    #[allow(dead_code)]
    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Dot product of vector with other
    #[inline]
    #[allow(dead_code)]
    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Vector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
