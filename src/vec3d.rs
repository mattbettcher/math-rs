use core::ops::{Add, Sub, Mul, Div};
use core::cmp::{PartialEq};

// 3d vector
#[derive(Debug, Clone, Copy)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// linear interpolation
pub fn lerp(a: Vec3d, b: Vec3d, t: f32) -> Vec3d {
    (1.0 - t) * a + t * b
}

// cross product
pub fn cross(a: Vec3d, b: Vec3d) -> Vec3d {
    Vec3d { x: a.y * b.z - a.z * b.y, 
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x }
}

pub fn dot(a: Vec3d, b: Vec3d) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

impl PartialEq for Vec3d {
    fn eq(&self, other: &Vec3d) -> bool {
           self.x == other.x
        && self.y == other.y
        && self.z == other.z
    }
}

impl Add<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Vec3d) -> Vec3d {
        Vec3d { x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z}
    }
}

impl Sub<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: Vec3d) -> Vec3d {
        Vec3d { x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z}
    }
}

impl Mul<f32> for Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: f32) -> Vec3d {
        Vec3d { x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs}
    }
}

impl Mul<Vec3d> for f32 {
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Vec3d {
        Vec3d { x: self * rhs.x,
                y: self * rhs.y,
                z: self * rhs.z}
    }
}

impl Div<f32> for Vec3d {
    type Output = Vec3d;

    fn div(self, rhs: f32) -> Vec3d {
        Vec3d { x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs}
    }
}
