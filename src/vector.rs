//! Simple three coordinate vector implementation.
//!
//! Operator overloads handled by the extremely convenient
//! [auto_ops](https://docs.rs/auto_ops/0.1.0/auto_ops/index.html) crate.
use std::fmt::{Display, Formatter, Result};

use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use rand::Rng;

/// Axis enumeration.
pub enum Axis {
    X,
    Y,
    Z,
}

/// Simple three coordinate vector.
///
/// Originally generalized to work with any primitive numeric field type,
/// but cut down to just `f64` to be more practical.
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Construct a new vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Construct a zero vector.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Construct a one vector.
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Vector dot product with another vector.
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Element-wise (Hadamard) product with another vector.
    pub fn hadamard_product(&self, rhs: &Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

    /// Cross product operation with another vector.
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// The magnitude of the vector squared.
    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    /// The magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// Convert the vector to a unit vector (normalization).
    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }

    /// Reflect the vector via a surface normal vector.
    ///
    /// ([source](https://raytracing.github.io/books/RayTracingInOneWeekend.html#metal/mirroredlightreflection))
    pub fn reflect(self, n: Self) -> Self {
        self - n * 2.0 * self.dot(&n)
    }

    /// Refract the vector via a surface normal vector and refractive index quotient.
    ///
    /// ([source](https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/snell'slaw))
    pub fn refract(self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(&n);
        let r_out_parallel = (self + n * cos_theta) * etai_over_etat;
        let r_out_perpendicular = n * -(1.0 - r_out_parallel.magnitude_squared()).sqrt();
        r_out_parallel + r_out_perpendicular
    }

    /// Randomized vector with components in the range [min, max).
    pub fn random_clamped(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    /// Randomized vector with components in the range [0.0, 1.0).
    pub fn random() -> Self {
        Self::random_clamped(0.0, 1.0)
    }

    /// Randomized vector with components within a unit disk.
    ///
    /// * `axis` - Denotes the axis normal to unit disk.
    /// (e.g. X normal to the Y-Z plane, components will be in Y-Z unit disk).
    pub fn random_in_unit_disk(axis_normal: Axis) -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let a = rng.gen_range(-1.0, 1.0);
            let b = rng.gen_range(-1.0, 1.0);
            if a * a + b * b <= 1.0 {
                return match axis_normal {
                    Axis::X => Self::new(0.0, a, b),
                    Axis::Y => Self::new(a, 0.0, b),
                    Axis::Z => Self::new(a, b, 0.0),
                };
            }
        }
    }

    /// Randomized vector with components within the unit sphere.
    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Self::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );
            if p.magnitude_squared() <= 1.0 {
                return p;
            }
        }
    }

    /// Randomized vector with components on the unit sphere.
    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    /// Randomized vector with components in the same hemisphere as some normal vector.
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

// Additive inverse (unary negation operator).
impl_op_ex!(- |a: &Vec3| -> Vec3 {
    Vec3 {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
});

// Vector addition
impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});


// Scalar addition
impl_op_ex_commutative!(+ |a: &Vec3, b: &f64| -> Vec3 { a + Vec3::new(*b, *b, *b) });

// Scalar multiplication
impl_op_ex_commutative!(*|a: &Vec3, b: &f64| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

// Vector subtraction
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

// Scalar subtraction
impl_op_ex_commutative!(-|a: &Vec3, b: &f64| -> Vec3 { a - Vec3::new(*b, *b, *b) });

// Scalar division.
impl_op_ex_commutative!(/ |a: &Vec3, b: &f64| -> Vec3 { a * (1.0 / b) });

// Assignment operators.
impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| { *a = *a + b; });
impl_op_ex!(+= |a: &mut Vec3, b: &f64 | { *a = *a + b; });
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| { *a = *a - b; });
impl_op_ex!(-= |a: &mut Vec3, b: &f64 | { *a = *a - b; });
impl_op_ex!(*= |a: &mut Vec3, b: &f64 | { *a = *a * b; });
impl_op_ex!(/= |a: &mut Vec3, b: &f64 | { *a = *a / b; });

// Formatted display.
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[ {} {} {} ]", self.x, self.y, self.z)
    }
}
