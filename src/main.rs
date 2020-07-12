#![feature(trait_alias, const_int_pow)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_doc_comments)]
// #![allow(unused_assignments)]

use std::fs::File;
use std::io::{Result, Write as IoWrite};
use std::rc::Rc;

use itertools::iproduct;

pub mod camera;
pub mod collide;
pub mod color;
pub mod material;
pub mod ray;
pub mod vector;
pub mod world;

use camera::Camera;
use collide::{Collidable, Collision, FindCollision, Sphere};
use color::Color;
use material::*;
use ray::Ray;
use vector::Vec3;
use world::World;

#[doc(hidden)]
fn main() -> Result<()> {
    const SKY: Vec3 = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = {
        // 225
        450
        // 900
    };
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const IMAGE_AREA: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

    let mut rng = rand::thread_rng();

    let material_lambert_huge = Rc::new(Lambert::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_lambert = Rc::new(Lambert::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_metal = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    let material_dielectric = Rc::new(Dielectric::new(1.5));

    let collidables: Vec<Box<dyn Collidable>> = vec![
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

    let world = World::new(SKY, collidables, T_MIN, T_MAX, MAX_DEPTH, SAMPLES_PER_PIXEL);

    const LOOK_FROM: Vec3 = Vec3 {
        x: -3.0,
        y: 1.0,
        z: 2.0,
    };
    const LOOK_AT: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    const UP: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    const VERTICAL_FOV: f64 = 20.0;
    const APERTURE: f64 = 0.7;

    const T_MIN: f64 = 0.001;
    const T_MAX: f64 = f64::INFINITY;
    const MAX_DEPTH: usize = 2usize.pow(6);
    const SAMPLES_PER_PIXEL: usize = 2usize.pow(6);

    let dist_to_focus: f64 = (LOOK_FROM - LOOK_AT).magnitude();

    let mut camera = Camera::new();
    camera
        .look_from(LOOK_FROM)
        .look_at(LOOK_AT)
        .set_up(UP)
        .set_vertical_fov(VERTICAL_FOV)
        .set_aspect_ratio(ASPECT_RATIO)
        .set_aperture(APERTURE)
        .set_focus_distance(dist_to_focus)
        .update();

    let mut bar = progress::Bar::new();

    let pixel_colors: Vec<Color> = iproduct!((0..IMAGE_HEIGHT).rev(), 0..IMAGE_WIDTH)
        .enumerate()
        .map(|(i, (y, x))| {
            bar.reach_percent((100.0 * i as f64 / IMAGE_AREA as f64) as i32);
            world.pixel_color(x, y, IMAGE_WIDTH, IMAGE_HEIGHT, &camera, &mut rng)
        })
        .collect();

    bar.reach_percent(100);
    bar.jobs_done();

    let mut file = File::create("test.ppm")?;
    writeln!(file, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for color in pixel_colors {
        writeln!(file, "{}", color)?;
    }
    drop(file);

    Ok(())
}
