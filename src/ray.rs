use crate::Vec3;
use crate::Collision;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(center: Vec3, direction: Vec3) -> Self {
        Self {
            origin: center,
            direction,
        }
    }

    pub fn zero() -> Self {
        Self {
            origin: Vec3::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::new(
            (self.origin).x + t * (self.direction).x,
            (self.origin).y + t * (self.direction).y,
            (self.origin).z + t * (self.direction).z,
        )
    }

    pub fn color(&self, collision: Option<Collision>) -> Vec3 {
        const SPHERE_CENTER: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };

        const SKY_COLOR: Vec3 = Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };

        if let Some(intersection) = collision {
            (intersection.normal + 1.0) * 0.5
        } else {
            let t = (self.direction.normalized().y + 1.0) * 0.5;
            Vec3::identity() * (1.0 - t) + SKY_COLOR * t
        }
    }
}
