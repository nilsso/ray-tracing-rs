#![feature(trait_alias, const_int_pow, total_cmp)]
// #![allow(unused_imports)]
#![allow(unused_variables, dead_code)]

use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Result, Write as IoWrite};
use std::rc::Rc;

use rand::Rng;

pub mod vector;
use vector::Vec3;

pub mod color;
use color::Color;

pub mod ray;
use ray::Ray;

pub mod collide;
use collide::{Collidable, Collision, FindCollision, Sphere};

pub mod camera;
use camera::Camera;

pub mod material;
use material::*;

fn main() -> Result<()> {
    const ORIGIN: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    const SKY: Vec3<f64> = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = {
        // 225
        // 450
        900
    };
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;
    const SAMPLES_PER_PIXEL: u32 = 2u32.pow(7);
    const MAX_DEPTH: u32 = 2u32.pow(7);

    let mut rng = rand::thread_rng();

    let lambert_a: Rc<dyn Material> = Rc::new(Lambert::new(Vec3::new(0.7, 0.3, 0.3)));
    let lambert_b: Rc<dyn Material> = Rc::new(Lambert::new(Vec3::new(0.8, 0.8, 0.0)));
    let metal_a: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1));
    let metal_b: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.6));

    let big_sphere = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, lambert_b.clone()));
    let diffuse_sphere = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, lambert_a.clone()));
    let metal_sphere_a = Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, metal_a.clone()));
    let metal_sphere_b = Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, metal_b.clone()));

    let world: Vec<Box<dyn Collidable>> = vec![
        diffuse_sphere,
        big_sphere,
        metal_sphere_a,
        metal_sphere_b,
    ];

    let camera = Camera::new(ORIGIN, VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH);
    let mut out_string = format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).to_string();

    let mut bar = progress::Bar::new();
    for (i, y) in (0..IMAGE_HEIGHT).rev().enumerate() {
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray.color(&SKY, &world, MAX_DEPTH)
            }
            let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
            let r = (scale * pixel_color.x).sqrt();
            let g = (scale * pixel_color.y).sqrt();
            let b = (scale * pixel_color.z).sqrt();
            out_string
                .write_fmt(format_args!("{}\n", Color::new(r, g, b)))
                .ok();
        }
        bar.reach_percent((100.0 * i as f64 / IMAGE_HEIGHT as f64) as i32);
    }
    bar.reach_percent(100);

    let mut file = File::create("test.ppm")?;
    file.write_all(out_string.as_bytes())?;

    bar.jobs_done();
    Ok(())
}
