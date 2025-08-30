use std::{
    array,
    fmt::Display,
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
};

use rand::{
    distr::{Distribution, StandardUniform},
    random_range,
};

use crate::color::Color;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3([f64; 3]);

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(array::from_fn(|i| -self.0[i]))
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(array::from_fn(|i| self[i] * rhs))
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self[i] * rhs[i]))
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(array::from_fn(|i| self[i] / rhs))
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::default(), |result, current| result + current)
    }
}

impl Distribution<Vec3> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(array::from_fn(|_| rng.random()))
    }
}

impl Vec3 {
    pub const fn new(values: [f64; 3]) -> Self {
        Self(values)
    }

    pub fn random(range: Range<f64>) -> Self {
        Self(array::from_fn(|_| random_range(range.clone())))
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let point = Vec3::random(-1.0..1.0);
            let length_squared = point.length_squared();
            if 1e-160 < length_squared && length_squared <= 1.0 {
                return point / length_squared.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let point = Vec3::new([random_range(-1.0..1.0), random_range(-1.0..1.0), 0.0]);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();

        // In the same hemisphere as the normal if greater than 0
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub const fn x(&self) -> f64 {
        self.0[0]
    }

    pub const fn y(&self) -> f64 {
        self.0[1]
    }

    pub const fn z(&self) -> f64 {
        self.0[2]
    }

    pub const fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub const fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    /// Returns true if the vector is close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.iter().all(|value| value.abs() < s)
    }

    pub fn reflect(&self, other: &Self) -> Self {
        *self - 2.0 * self.dot(other) * *other
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(&n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub const fn cross(&self, other: &Self) -> Vec3 {
        Self([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        ])
    }
}

pub type Point3 = Vec3;
