use crate::Ray;
use crate::Vec3;

pub struct Collision {
    pub front_face: bool,
    pub p: Vec3,
    pub t: f64,
    pub normal: Vec3,
}

impl Collision {
    fn new(ray: &Ray, p: Vec3, t: f64, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            front_face,
            p,
            t,
            normal,
        }
    }
}

pub trait Collidable {
    fn intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

pub trait FindCollision {
    fn find_intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

impl FindCollision for Vec<Box<dyn Collidable>> {
    fn find_intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        self
            .iter()
            .find_map(|shape| shape.intersection(ray, t_min, t_max))
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Collidable for Sphere {
    fn intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        let mut res = None;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let r1 = (-half_b - root) / a;
            let r2 = (-half_b + root) / a;
            if r1 < t_max && r1 > t_min {
                let p = ray.at(r1);
                let outward_normal = (p - self.center) / self.radius;
                res = Some(Collision::new(&ray, p, r1, outward_normal))
            } else if r2 < t_max && r2 > r2 {
                let p = ray.at(r2);
                let outward_normal = (p - self.center) / self.radius;
                res = Some(Collision::new(&ray, p, r2, outward_normal))
            }
        }
        res
    }
}

pub struct Cube {
    center: Vec3,
    transform: Vec3,
}

impl Cube {
    pub fn new(center: Vec3, transform: Vec3) -> Self {
        Self { center, transform }
    }
}

impl Collidable for Cube {
    fn intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        None
    }
}
