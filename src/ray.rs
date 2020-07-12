//! Geometric ray which accrues color from collidable objects.
use crate::{Collidable, FindCollision, Vec3};

/// Geometric ray which accrues color from collidable objects.
///
/// Portion of a line passing through a point and along a direction.
#[derive(Debug)]
pub struct Ray {
    pub point: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Construct a new ray.
    pub fn new(point: Vec3, direction: Vec3) -> Self {
        Self { point, direction }
    }

    /// Point on ray for parameter t.
    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::new(
            self.point.x + t * self.direction.x,
            self.point.y + t * self.direction.y,
            self.point.z + t * self.direction.z,
        )
    }

    /// Ray color from the materials of collidables.
    ///
    /// By reflecting off of collidable objects the color becomes a composite of the multiple
    /// materials it observes.
    ///
    /// * `t_min` - Lower bound on the distance at which collisions are considered.
    /// * `t_max` - Upper bound on the distance at which collisions are considered.
    /// * `max_depth` - Bound on the number of recursive reflections.
    pub fn color(
        &self,
        sky: &Vec3,
        world: &Vec<Box<dyn Collidable>>,
        t_min: f64,
        t_max: f64,
        max_depth: usize,
    ) -> Vec3 {
        if max_depth == 0 {
            Vec3::zero()
        } else {
            if let Some(coll) = world.iter().find_closest_collision(self, t_min, t_max) {
                if let Some((scattered, att)) = coll.material.scatter(self, &coll)
                {
                    let color = scattered.color(&sky, &world, t_min, t_max, max_depth - 1);
                    att.hadamard_product(&color)
                } else {
                    Vec3::zero()
                }
            } else {
                let t = (self.direction.normalized().y + 1.0) * 0.5;
                Vec3::one() * (1.0 - t) + (*sky) * t
            }
        }
    }
}
