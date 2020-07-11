// use std::convert::{From, Into};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use num::traits::{real::Real, Inv, NumCast, One, Zero};
use rand::{distributions::uniform::SampleUniform, Rng};

// Conceptual trait alias
pub trait Field<T> = One + Zero + Inv<Output = T> + Real;

// Practical trait alias
pub trait PrimitiveField<T> = Copy + Clone + NumCast + SampleUniform + Field<T>;

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: PrimitiveField<T>> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    pub fn one() -> Self {
        Self::new(T::one(), T::one(), T::one())
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn hadamard(&self, rhs: &Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }

    pub fn reflect(self, n: Self) -> Self {
        let d = self.dot(&n);
        self - n * (d + d)
    }

    pub fn refract(self, n: Vec3<T>, etai_over_etat: T) -> Self {
        let cos_theta = (-self).dot(&n);
        let r_out_parallel = (self + n * cos_theta) * etai_over_etat;
        let r_out_perpendicular = n * (-Real::sqrt(T::one() - r_out_parallel.length_squared()));
        r_out_parallel + r_out_perpendicular
    }

    pub fn random_clamped(min: T, max: T) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min, max) as T,
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn random() -> Self {
        Self::random_clamped(T::zero(), T::one())
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let p = Self::new(
                T::from(rng.gen_range(-1.0, 1.0)).unwrap(),
                T::from(rng.gen_range(-1.0, 1.0)).unwrap(),
                T::zero());
            if p.length_squared() <= T::one() {
                return p;
            }
        }

        // let u = rng.gen::<f64>();
        // let v = rng.gen::<f64>();
        // let r = Real::sqrt(u);
        // let theta = 2.0 * std::f64::consts::PI * v;
        // Self {
        //     x: T::from(r * Real::cos(theta)).unwrap(),
        //     y: T::from(r * Real::sin(theta)).unwrap(),
        //     z: T::zero(),
        // }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();

        let u = 2.0 * rng.gen::<f64>() - 1.0;
        let phi = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
        let r = Real::powf(rng.gen::<f64>(), 1.0 / 3.0);
        let x = T::from(r * Real::cos(phi) * Real::sqrt(1.0 - u * u)).unwrap();
        let y = T::from(r * Real::sin(phi) * Real::sqrt(1.0 - u * u)).unwrap();
        let z = T::from(r * u).unwrap();
        Self { x, y, z }
    }

    pub fn random_normalized() -> Self {
        let mut rng = rand::thread_rng();
        let a = T::from(rng.gen_range(0.0, 2.0 * std::f64::consts::PI)).unwrap();
        let z = T::from(rng.gen_range(-1.0, 1.0)).unwrap();
        let r = (T::one() - z * z).sqrt();
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3<T>) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > T::zero() {
            in_unit_sphere
        } else {
            -in_unit_sphere
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

/// Vector subtraction
impl<T: PrimitiveField<T>> Sub<Self> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

/// Scalar subtraction
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

/// Addition assignment
impl<T: PrimitiveField<T>> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Addition assignment
impl<T: PrimitiveField<T>> AddAssign<T> for Vec3<T> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[ {} {} {} ]", self.x, self.y, self.z)
    }
}
