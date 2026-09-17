use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 2D floating-point vector with sub-cell precision.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const UP: Self = Self { x: 0.0, y: -1.0 };
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 1e-6 {
            self / len
        } else {
            Self::ZERO
        }
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn from_angle(radians: f32) -> Self {
        Self {
            x: radians.cos(),
            y: radians.sin(),
        }
    }

    #[inline]
    pub fn to_angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn rotate(self, radians: f32) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    #[inline]
    pub fn lerp(self, target: Self, t: f32) -> Self {
        self + (target - self) * t.clamp(0.0, 1.0)
    }

    #[inline]
    pub fn clamp_length(self, max_len: f32) -> Self {
        let len_sq = self.length_squared();
        if len_sq > max_len * max_len && len_sq > 0.0 {
            self.normalize() * max_len
        } else {
            self
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

/// A 2D rectangle in continuous space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    #[inline]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    #[inline]
    pub fn min(self) -> Vec2 {
        self.position
    }

    #[inline]
    pub fn max(self) -> Vec2 {
        self.position + self.size
    }

    #[inline]
    pub fn center(self) -> Vec2 {
        self.position + self.size * 0.5
    }

    #[inline]
    pub fn contains_point(self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.x < self.position.x + self.size.x
            && point.y >= self.position.y
            && point.y < self.position.y + self.size.y
    }

    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        self.position.x < other.position.x + other.size.x
            && self.position.x + self.size.x > other.position.x
            && self.position.y < other.position.y + other.size.y
            && self.position.y + self.size.y > other.position.y
    }
}

/// 2D Transform component: position, rotation angle (radians), scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform {
    pub fn from_pos(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_operations() {
        let v1 = Vec2::new(3.0, 4.0);
        assert_eq!(v1.length(), 5.0);

        let v2 = Vec2::new(1.0, 2.0);
        assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_eq!(v1 - v2, Vec2::new(2.0, 2.0));
        assert_eq!(v1 * 2.0, Vec2::new(6.0, 8.0));
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let r2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        let r3 = Rect::new(20.0, 20.0, 5.0, 5.0);

        assert!(r1.intersects(r2));
        assert!(!r1.intersects(r3));
    }
}
