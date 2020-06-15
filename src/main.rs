#![allow(unused_imports, unused_variables)]

use std::convert::From;
use std::fmt::Display;
use std::fs::File;
use std::ops::{Add, Mul};
use std::path::Path;
use std::{io, io::Write};

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// trait AsColor {
//     fn as_color(&self) -> String;
// }
//
// impl<T: Scalar + Display> AsColor for Vector3<T> {
//     fn as_color(&self) -> String {
//         format!("{} {} {}", self.data[0], self.data[1], self.data[2])
//     }
// }
//
// struct Ray {
//     pub origin: Point3<f64>,
//     pub direction: Vector3<f64>,
// }
//
// impl Ray {
//     fn new(origin: [f64; 3], direction: [f64; 3]) -> Self {
//         Self {
//             origin: Point3::from(origin),
//             direction: Vector3::from(direction),
//         }
//     }
//
//     fn at(&self, t: f64) -> Point3<f64> {
//         &self.origin + &self.direction * t
//     }
//
//     fn color(&self) -> Color {
//         let t = 0.5 * (self.direction.normalize().data[1] + 1.0);
//         (1.0 - t) * Vector3::identity() + t * Vector3::new(0.5, 0.7, 1.0)
//     }
// }

// fn write_ppm(path: &Path) -> io::Result<()> {
//     let mut bar = progress::Bar::new();
//     bar.set_job_title("Writing PPM image...");
//     let mut file = File::create(path)?;
//     let mut v = Vector3::<f32>::default();
//
//     file.write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_W, IMAGE_H))?;
//     for j in 0..IMAGE_H {
//         for i in 0..IMAGE_W {
//             v.data[0] = i as f32 / (IMAGE_W - 1) as f32;
//             v.data[1] = j as f32 / (IMAGE_H - 1) as f32;
//             v.data[2] = 0.25;
//             file.write_fmt(format_args!("{}\n", v.as_color()))?;
//         }
//         bar.reach_percent((100.0 * j as f32 / IMAGE_H as f32) as i32);
//     }
//     Ok(())
// }

fn main() -> std::io::Result<()> {
    // const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_W: u32 = 384;
    // const IMAGE_H: u32 = (IMAGE_W as f64 / ASPECT_RATIO) as u32;
    //
    // let viewport_height = 2.0;
    // let viewport_width = ASPECT_RATIO * viewport_height;
    // let focal_length = 1.0;
    //
    // let origin = Point3::new(0.0, 0.0, 0.0);
    // let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    // let vertical = Vector3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner =
    //     origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);
    //
    // // write_ppm(Path::new("test.ppm"))?;
    // let mut ray = Ray::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    // ray.color().as_color();
    // // let v = Unit::new_normalize(nalgebra::Vector2::new(1.0, 2.0)) ;
    // // println!("{:?}", v);

    Ok(())
}
