#![feature(trait_alias, const_int_pow, total_cmp)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

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

    const LOOK_FROM: Vec3<f64> = Vec3 { x: -3.0, y: 1.0, z: 2.0 };
    const LOOK_AT: Vec3<f64> = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
    const UP: Vec3<f64> = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    const APERTURE: f64 = 0.7;

    const SAMPLES_PER_PIXEL: u32 = 2u32.pow(8);
    const MAX_DEPTH: u32 = 2u32.pow(8);

    let dist_to_focus: f64 = (LOOK_FROM - LOOK_AT).length();

    let mut rng = rand::thread_rng();

    let material_lambert_huge = Rc::new(Lambert::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_lambert = Rc::new(Lambert::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_metal = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    let material_dielectric = Rc::new(Dielectric::new(1.5));

    let world: Vec<Box<dyn Collidable>> = vec![
        // Huge sphere
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            material_lambert_huge.clone(),
        )),
        // Middle lambert sphere
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            material_lambert.clone(),
        )),
        // Right metal sphere
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            material_metal.clone(),
        )),
        // Left glass sphere
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            material_dielectric.clone(),
        )),
        // Left glass sphere (interior)
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            material_dielectric.clone(),
        )),
    ];

    let camera = Camera::new(
        LOOK_FROM,
        LOOK_AT,
        UP,
        20.0,
        ASPECT_RATIO,
        APERTURE,
        dist_to_focus,
    );

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
