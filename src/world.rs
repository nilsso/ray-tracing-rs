//! Ray tracing world encapsulation.
use crate::{Camera, Collidable, Color, Vec3};

use rand::{Rng, RngCore};

/// Ray tracing world encapsulation.
pub struct World {
    sky: Vec3,
    collidables: Vec<Box<dyn Collidable>>,
    t_min: f64,
    t_max: f64,
    max_depth: usize,
    samples: usize,
}

impl World {
    pub fn new(
        sky: Vec3,
        collidables: Vec<Box<dyn Collidable>>,
        t_min: f64,
        t_max: f64,
        max_depth: usize,
        samples: usize,
    ) -> Self {
        Self {
            sky,
            collidables,
            t_min,
            t_max,
            max_depth,
            samples,
        }
    }

    /// Calculate pixel color given a camera.
    pub fn pixel_color<RNG: RngCore>(
        &self,
        x: usize,
        y: usize,
        window_width: usize,
        window_height: usize,
        camera: &Camera,
        rng: &mut RNG,
    ) -> Color {
        let World {
            sky,
            collidables,
            t_min,
            t_max,
            max_depth,
            samples,
        } = self;

        let mut pixel_color = Vec3::zero();
        for _ in 0..(*samples) {
            let u = (x as f64 + rng.gen::<f64>()) / (window_width - 1) as f64;
            let v = (y as f64 + rng.gen::<f64>()) / (window_height - 1) as f64;
            let ray = camera.get_ray(u, v);
            pixel_color += ray.color(&sky, &collidables, *t_min, *t_max, *max_depth)
        }
        let scale = 1.0 / (*samples) as f64;
        let r = (scale * pixel_color.x).sqrt();
        let g = (scale * pixel_color.y).sqrt();
        let b = (scale * pixel_color.z).sqrt();

        Color::new(r, g, b)
    }
}
