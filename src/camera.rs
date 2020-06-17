use crate::Vec3;
use crate::Ray;

pub struct Camera {
    origin: Vec3,
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,

    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        viewport_width: f64,
        viewport_height: f64,
        focal_length: f64,
    ) -> Self {
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal * 0.5 - vertical * 0.5 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            origin,
            viewport_height,
            viewport_width,
            focal_length,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
        )
    }
}
