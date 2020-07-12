//! Material traits and simple material implementations.
use rand::Rng;

use crate::{Collision, Ray, Vec3};

/// A material which interacts with rays by reflecting or absorbing them.
pub trait Material {
    /// Scatter an incoming ray.
    fn scatter(&self, ray_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3)>;
}

/// Simple lambertian material.
pub struct Lambert {
    albedo: Vec3,
}

impl Lambert {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambert {
    fn scatter(&self, _ray_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3)> {
        let scatter_direction = collision.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(collision.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}

/// Simple metallic material with surface scattering fuzz factor.
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3)> {
        let reflected = r_in.direction.normalized().reflect(collision.normal);
        let scattered = Ray::new(
            collision.point,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );
        if scattered.direction.dot(&collision.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

/// Simple dielectric material with refraction index.
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

/// Helper to calculate probability of ray reflection versus refraction.
fn schlick_approx(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, collision: &Collision) -> Option<(Ray, Vec3)> {
        let mut rng = rand::thread_rng();

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_quotient = if collision.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.normalized();
        let cos_theta = (-unit_direction).dot(&collision.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflected_prob = schlick_approx(cos_theta, refraction_quotient);

        if refraction_quotient * sin_theta > 1.0 || rng.gen::<f64>() < reflected_prob {
            let reflected = unit_direction.reflect(collision.normal);
            Some((Ray::new(collision.point, reflected), attenuation))
        } else {
            let refracted = unit_direction.refract(collision.normal, refraction_quotient);
            Some((Ray::new(collision.point, refracted), attenuation))
        }
    }
}
