#![warn(rust_2018_idioms)]

use crate::canvas::Canvas;
use crate::color::Color;
use crate::intersection::{Intersection, IntersectionObject, Intersections};
use crate::matrix::Matrix;
use crate::projectile::{Environment, Projectile};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use num::ToPrimitive;
use rayon::iter::*;
use rayon::prelude;
use std::f64::consts::PI;
use std::sync::Mutex;

mod canvas;
mod color;
mod float_service;
mod intersection;
mod matrix;
mod projectile;
mod ray;
mod sphere;
mod tuple;

/**
   Author: Maciek Mika
   This is the main file that runs the ray tracer
*/

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 800;
const FILE_NAME: &str = "intersection.ppm";

fn main() {
    //write_projectile();
    //write_clock();
    cast_rays_at_sphere()
}

fn cast_rays_at_sphere() {
    let origin = Tuple::<f64>::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let canvas_pixel = 100;
    let pixel_size = wall_size / canvas_pixel.to_f64().unwrap();
    let canvas = Mutex::new(Canvas::new(canvas_pixel, canvas_pixel));
    let color = Color::new(1.0, 0.0, 0.0);
    let sphere = Sphere::<f64>::new(1);

    (0..canvas_pixel).par_bridge().for_each(|y| {
        let world_y = half - pixel_size * y.to_f64().unwrap();

        for x in 0..canvas_pixel {
            let world_x = -half + pixel_size * x.to_f64().unwrap();
            let position = Tuple::<f64>::new_point(world_x, world_y, wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());
            match sphere.intersect(ray) {
                Ok(intersections) => match intersections.hit() {
                    None => {}
                    Some(_) => {
                        let mut canvas = canvas.lock().unwrap();
                        match canvas.write_pixel(x, y, color.clone()) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                },
                Err(_) => {}
            }
        }
    });

    canvas.lock().unwrap().write_ppm(FILE_NAME);
}

// we use isize because the point coordinates can become negative but by adding
// the center y and center x pixels raise it back to a positive value
fn write_clock() {
    let mut point = Tuple::new_point(0.0, 0.0, 1.0);
    let mut canvas = Canvas::new_with_color(CANVAS_WIDTH, CANVAS_HEIGHT, Color::new(0.0, 0.0, 0.0));
    let radius: isize = ((CANVAS_WIDTH as f64 / 8.0) * 3.0) as isize;
    let rotation = Matrix::<f64, 4>::rotate_y(3.0 * (PI / 6.0));
    let (center_x, center_y) = (
        (CANVAS_HEIGHT / 2 - 1) as isize,
        (CANVAS_WIDTH / 2 - 1) as isize,
    );

    for _i in 0..4 {
        let x;
        let y;

        match (point.x.to_isize(), point.z.to_isize()) {
            (Some(xi), Some(yi)) => {
                x = xi * radius + center_x;
                y = yi * radius + center_y;
            }
            (_, _) => {
                println!("failed to convert point coordinates");
                continue;
            }
        }

        match canvas.write_pixel(x as usize, y as usize, Color::new(0.0, 1.0, 1.0)) {
            Ok(_) => point = rotation.clone() * point,
            Err(e) => {
                println!("writing pixel to canvas failed : {}", e)
            }
        }
    }

    canvas.write_ppm("clock.ppm")
}

fn write_projectile() {
    let start = Tuple::new_point(0.0, 1.0, 0.0);
    let velocity = Tuple::new_vector(1.0, 1.8, 0.0).normalize() * 11.25;

    let mut proj = Projectile::new(start, velocity);

    let gravity = Tuple::new_vector(0.0, -0.1, 0.0);
    let wind = Tuple::new_vector(-0.01, 0.0, 0.0);
    let env = Environment::new(gravity, wind);

    let mut canvas = Canvas::new_with_color(900, 500, Color::new(0.0, 0.0, 0.0));

    while proj.position.y > 0.0 {
        match canvas.write_pixel(
            proj.position.x as usize,
            canvas.height - proj.position.y as usize,
            Color::new(1.0, 0.0, 0.0),
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e)
            }
        }
        proj.tick(env.clone());
    }

    canvas.write_ppm("projectile.ppm")
}
