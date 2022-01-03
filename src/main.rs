#![warn(rust_2018_idioms)]

use crate::canvas::Canvas;
use crate::color::Color;
use crate::projectile::{Environment, Projectile};
use crate::tuple::Tuple;
use std::fs;

mod canvas;
mod color;
mod matrix;
mod projectile;
mod tuple;

/**
   Author: Maciek Mika
   This is the main file that runs the ray tracer
*/

fn main() {
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

    let ppm = canvas.to_ppm();
    match fs::write("first_unit.ppm", ppm) {
        Ok(_) => {
            println!("printed ray tracing file!")
        }
        Err(_) => {
            println!("An error occurred when saving the ray tracing file")
        }
    }
}
