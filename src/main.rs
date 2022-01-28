#![warn(rust_2018_idioms)]

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::projectile::{Environment, Projectile};
use crate::tuple::Tuple;
use num::ToPrimitive;
use std::f64::consts::PI;

mod canvas;
mod color;
mod float_service;
mod matrix;
mod projectile;
mod ray;
mod tuple;

/**
   Author: Maciek Mika
   This is the main file that runs the ray tracer
*/

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 800;
const FILE_NAME: &str = "clock.ppm";

fn main() {
    write_clock();
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
        let mut x = 0;
        let mut y = 0;

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

    canvas.write_ppm(FILE_NAME)
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
