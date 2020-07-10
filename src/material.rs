use crate::{Collision, Ray, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3<f64>)>;
}

pub struct Dummy;

impl Material for Dummy {
    fn scatter(&self, r_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3<f64>)> {
        None
    }
}

pub struct Lambert {
    albedo: Vec3<f64>,
}

impl Lambert {
    pub fn new(albedo: Vec3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambert {
    fn scatter(&self, r_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3<f64>)> {
        let scatter_direction = collision.normal + Vec3::random_normalized();
        let scattered = Ray::new(collision.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3<f64>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3<f64>)> {
        let reflected = r_in.direction.normalized().reflect(collision.normal);
        let scattered = Ray::new(
            collision.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );
        if scattered.direction.dot(&collision.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
