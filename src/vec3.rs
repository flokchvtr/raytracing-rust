use std::ops::{Add, AddAssign, Mul, Neg, Sub};

pub const ORIGINE: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        let l = 1. / self.length();
        self * l
    }

    pub fn reflechi(self, n: Vec3) -> Vec3 {
        self - 2. * (self.dot(n)) * n
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        iter.fold(ORIGINE, |acc, v| acc + v)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scal: f64) -> Vec3 {
        Self::new(self.x * scal, self.y * scal, self.z * scal)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflection() {
        let v = Vec3::new(1., -1., 0.);
        let n = Vec3::new(0., 1., 0.);
        let res = Vec3::new(1., 1., 0.);
        assert_eq!(v.reflechi(n), res);
        let bas = Vec3::new(0., -1., 0.);
        assert_eq!(bas.reflechi(n), n);
    }

    #[test]
    fn sum() {
        let v = Vec3::new(1., -1., 0.);
        let n = Vec3::new(0., 1., 0.);

        let vec = vec![v, n];
        let s: Vec3 = vec.into_iter().sum();
        assert_eq!(s, Vec3::new(1., 0., 0.));
    }
}
