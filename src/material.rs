use rand::Rng;

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

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

fn schlick_approx(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3<f64>)> {
        let mut rng = rand::thread_rng();

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if collision.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.normalized();
        let cos_theta = (-unit_direction).dot(&collision.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflected_prob = schlick_approx(cos_theta, etai_over_etat);

        if etai_over_etat * sin_theta > 1.0 || rng.gen::<f64>() < reflected_prob {
            let reflected = unit_direction.reflect(collision.normal);
            Some((Ray::new(collision.p, reflected), attenuation))
        } else {
            let refracted = unit_direction.refract(collision.normal, etai_over_etat);
            Some((Ray::new(collision.p, refracted), attenuation))
        }
    }
}
