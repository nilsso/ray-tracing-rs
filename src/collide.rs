//! Collision traits and simple shape implementations.
use std::rc::Rc;
use std::slice::Iter;

use crate::{Material, Ray, Vec3};

/// Collision record.
pub struct Collision {
    /// Distance along ray at which collision occurred.
    pub t: f64,

    /// Point in space at which collision occurred.
    pub point: Vec3,

    /// Normal from collided object.
    pub normal: Vec3,

    /// Material of collided object.
    pub material: Rc<dyn Material>,

    /// If collision occurred on the front face of the collided object.
    pub front_face: bool,
}

impl Collision {
    /// Construct a collision record.
    ///
    /// * `ray` - Colliding ray.
    /// * `t` - Distance along which at which collision occurred.
    /// * `p` - Point in space at which collision occurred.
    /// * `outward_normal` - Unit vector at the point of collision normal to the surface of the collided object.
    /// * `material` - Material of the collided object.
    fn new(
        ray: &Ray,
        t: f64,
        point: Vec3,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            point,
            normal,
            material,
            front_face,
        }
    }
}

/// A struct with which a ray can collide.
pub trait Collidable {
    /// Find collision between `self` and `ray`.
    ///
    /// * `t_min` - Lower bound on the distance at which collisions are considered.
    /// * `t_max` - Upper bound on the distance at which collisions are considered.
    fn collision(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

/// Iterator addaptor to find
pub trait FindCollision {
    fn find_closest_collision(self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

impl FindCollision for Iter<'_, Box<dyn Collidable>>
{
    fn find_closest_collision(self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        self
            .filter_map(|collidable| collidable.collision(ray, t_min, t_max))
            .min_by(|x, y| (x.t).partial_cmp(&y.t).unwrap_or(std::cmp::Ordering::Greater))
    }
}

/// Collidable sphere.
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Collidable for Sphere {
    fn collision(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let oc = ray.point - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for &t in &[(-half_b - root) / a, (-half_b + root) / a] {
                if t < t_max && t > t_min {
                    let p = ray.at(t);
                    let outward_normal = (p - self.center) / self.radius;
                    return Some(Collision::new(
                        &ray,
                        t,
                        p,
                        outward_normal,
                        self.material.clone(),
                    ));
                }
            }
        }
        None
    }
}
//
// pub struct Cube {
//     center: Vec3,
//     transform: Vec3,
// }
//
// impl Cube {
//     pub fn new(center: Vec3, transform: Vec3) -> Self {
//         Self { center, transform }
//     }
// }
//
// impl Collidable for Cube {
//     fn collision(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
//         None
//     }
// }
