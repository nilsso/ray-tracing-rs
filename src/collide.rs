use std::rc::Rc;

use crate::{Ray, Vec3, Material};

pub struct Collision {
    pub t: f64,
    pub p: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl Collision {
    fn new(ray: &Ray, t: f64, p: Vec3<f64>, outward_normal: Vec3<f64>, material: Rc<dyn Material>) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            p,
            normal,
            front_face,
            material,
        }
    }
}

pub trait Collidable {
    fn collision(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

pub trait FindCollision {
    fn find_collision(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

impl FindCollision for Vec<Box<dyn Collidable>> {

    fn find_collision(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        self.iter()
            .filter_map(|shape| shape.collision(ray, t_min, t_max))
            .min_by(|x, y| (x.t).total_cmp(&y.t))
    }
}

pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64, material: Rc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Collidable for Sphere {
    fn collision(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for &t in &[(-half_b - root) / a, (-half_b + root) / a] {
                if t < t_max && t > t_min {
                    let p = ray.at(t);
                    let outward_normal = (p - self.center) / self.radius;
                    return Some(Collision::new(&ray, t, p, outward_normal, self.material.clone()));
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
