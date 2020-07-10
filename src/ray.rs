use crate::{Vec3, Collision, Collidable, FindCollision};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3<f64>,
    pub direction: Vec3<f64>,
}

fn diffuse_method(collision: &Collision) -> Vec3<f64> {
    // Method 1.
    // collision.p + collision.normal + Vec3::random_in_unit_sphere()
    // Method 2.
    // collision.p + collision.normal + Vec3::random_normalized()
    // Method 3.
    collision.p + collision.normal + Vec3::random_normalized()
}

impl Ray {
    pub fn new(center: Vec3<f64>, direction: Vec3<f64>) -> Self {
        Self {
            origin: center,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3<f64> {
        Vec3::new(
            self.origin.x + t * self.direction.x,
            self.origin.y + t * self.direction.y,
            self.origin.z + t * self.direction.z,
        )
    }

    pub fn color(
        &self,
        sky: &Vec3<f64>,
        world: &Vec<Box<dyn Collidable>>,
        depth: u32,
    ) -> Vec3<f64> {
        if depth == 0 {
            Vec3::zero()
        } else if let Some(collision) = world.find_collision(self, 0.001, f64::INFINITY) {
            // Simple
            // (collision.normal + Vec3::one()) * 0.5

            // Diffuse
            // let target = diffuse_method(&collision);
            // let refraction = Self::new(collision.p, target - collision.p);
            // refraction.color(&sky, &world, depth - 1) * 0.5

            // Material
            if let Some((scattered, attenuation)) = collision.material.scatter(self, &collision) {
                attenuation.hadamard(&scattered.color(&sky, &world, depth - 1))
            } else {
                Vec3::zero()
            }
        } else {
            let t = (self.direction.normalized().y + 1.0) * 0.5;
            Vec3::one() * (1.0 - t) + (*sky) * t
        }
    }
}
