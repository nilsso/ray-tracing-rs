use crate::{Ray, Vec3};

pub struct Camera {
    origin: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3<f64>,
        look_at: Vec3<f64>,
        up: Vec3<f64>,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        const FOCAL_LENGTH: f64 = 1.0;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u  = up.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner =
            origin - horizontal * 0.5 - vertical * 0.5 - w * focus_distance;
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = u * rd.x + v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
