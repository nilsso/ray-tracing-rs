use num::traits::{Inv, real::Real, One, Zero};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Field<T> = One + Zero + Inv<Output = T> + Real;
pub trait PrimitiveField<T> = Copy + Clone + Field<T>;

#[derive(Copy, Clone)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: PrimitiveField<T>> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

/// Additive inverse (unary negation operator)
impl<T: PrimitiveField<T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Vector addition
impl<T: PrimitiveField<T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Scalar addition
impl<T: PrimitiveField<T>> Add<T> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self + Self::new(rhs, rhs, rhs)
    }
}

/// Scalar multiplication
impl<T: PrimitiveField<T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Subtraction
/// Covers both vector and scalar subtraction, since both T and Vec3 implement the unary negation
/// operator.
impl<T: PrimitiveField<T>> Sub<T> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self + (-rhs)
    }
}

/// Scalar division
impl<T: PrimitiveField<T>> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self * rhs.inv()
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[ {} {} {} ]", self.x, self.y, self.z)
    }
}
