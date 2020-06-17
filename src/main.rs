#![feature(trait_alias)]
// #![allow(unused_imports)]
#![allow(unused_variables, dead_code)]
// use std::fs::File;

pub mod vector;
use vector::Vec3;

// pub mod color;
// use color::Color;

// pub mod ray;
// use ray::Ray;

// pub mod collide;
// use collide::{Collidable, Collision, Cube, FindCollision, Sphere};

// pub mod camera;
// use camera::Camera;

fn main() -> std::io::Result<()> {
    let a = Vec3::new(1.0, 2.0, 3.0) / 2.0;
    println!("{}", a);
    println!("{}", a.length());

    // let world: Vec<Box<dyn Collidable>> = vec![
    //     Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
    //     Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    //     // Box::new(Cube::new(Vec3::identity(), Vec3::zero())),
    // ];

    // const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_W: u32 = {
    //     225
    //     // 450
    //     // 900
    // };
    // const IMAGE_H: u32 = (IMAGE_W as f64 / ASPECT_RATIO) as u32;

    // let origin = Vec3::new(0.0, 0.0, 0.0);
    // let viewport_height = 2.0;
    // let viewport_width = ASPECT_RATIO * viewport_height;
    // let focal_length = 1.0;
    // let samples_per_pixel = 4e1 as u32;
    //
    // let camera = Camera::new(origin, viewport_width, viewport_height, focal_length);
    //
    // let mut file = File::create("test.ppm")?;
    // file.write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_W, IMAGE_H))?;
    // let mut bar = progress::Bar::new();
    // for (i, y) in (0..IMAGE_H).rev().enumerate() {
    //     for x in 0..IMAGE_W {
    //         let mut pixel_color = Vec3::zero();
    //         for _ in 0..samples_per_pixel {
    //             let u = (x as f64 + rand::random::<f64>()) / (IMAGE_W - 1) as f64;
    //             let v = (y as f64 + rand::random::<f64>()) / (IMAGE_H - 1) as f64;
    //             let ray = camera.get_ray(u, v);
    //             pixel_color += ray.color(world.find_intersection(&ray, 0.0, f64::INFINITY))
    //         }
    //         file.write_fmt(format_args!(
    //             "{}\n",
    //             Color::from(pixel_color / samples_per_pixel as f64)
    //         ))?;
    //     }
    //     bar.reach_percent((100.0 * i as f64 / IMAGE_H as f64) as i32);
    // }
    // bar.reach_percent(100);
    // bar.jobs_done();
    Ok(())
}
