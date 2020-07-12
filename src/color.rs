//! Basic color printing utility.
use std::fmt::{Display, Formatter};

use crate::vector::Vec3;

pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: (255.999 * r) as u32,
            g: (255.999 * g) as u32,
            b: (255.999 * b) as u32,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl From<Vec3> for Color {
    fn from(other: Vec3) -> Self {
        Self::new(
            other.x,
            other.y,
            other.z,
        )
    }
}
