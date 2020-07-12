//! Camera encapsulation.
use crate::ray::Ray;
use crate::vector::{Axis, Vec3};

/// Camera encapsulation.
pub struct Camera {
    // User set state:
    origin: Vec3,
    direction: Vec3,
    up: Vec3,
    vertical_fov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_distance: f64,
    // Calculated state:
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    /// Construct a new camera.
    pub fn new() -> Self {
        Self {
            origin: Vec3::zero(),
            direction: Vec3::zero(),
            up: Vec3::zero(),
            vertical_fov: 0.0,
            aspect_ratio: 0.0,
            aperture: 0.0,
            focus_distance: 0.0,
            horizontal: Vec3::zero(),
            vertical: Vec3::zero(),
            lower_left_corner: Vec3::zero(),
        }
    }

    /// Update computed camera state.
    pub fn update(&mut self) -> &mut Self {
        let Camera {
            ref origin,
            ref direction,
            ref up,
            ref vertical_fov,
            ref aspect_ratio,
            ref focus_distance,
            horizontal,
            vertical,
            lower_left_corner,
            ..
        } = self;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let u = up.cross(&direction).normalized();
        let v = direction.cross(&u);

        *horizontal = u * viewport_width * focus_distance;
        *vertical = v * viewport_height * focus_distance;
        *lower_left_corner =
            origin - (*horizontal) * 0.5 - (*vertical) * 0.5 - direction * focus_distance;
        self
    }

    pub fn look_from(&mut self, point: Vec3) -> &mut Self {
        self.origin = point;
        self
    }

    pub fn look_at(&mut self, point: Vec3) -> &mut Self {
        self.direction = (self.origin - point).normalized();
        self
    }

    pub fn set_up(&mut self, up: Vec3) -> &mut Self {
        self.up = up;
        self
    }

    pub fn set_vertical_fov(&mut self, vertical_fov: f64) -> &mut Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn set_aperture(&mut self, aperture: f64) -> &mut Self {
        self.aperture = aperture;
        self
    }

    pub fn set_focus_distance(&mut self, focus_distance: f64) -> &mut Self {
        self.focus_distance = focus_distance;
        self
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let lens_radius = self.aperture / 2.0;
        let rd = Vec3::random_in_unit_disk(Axis::Z) * lens_radius;
        let offset = u * rd.x + v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
