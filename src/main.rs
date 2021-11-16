use crate::canvas::Canvas;
use crate::color::Color;
use crate::projectile::{Environment, Projectile};
use crate::tuple::Tuple;

mod canvas;
mod color;
mod projectile;
mod tuple;

/**
   Author: Maciek Mika
   This is the main file that runs the ray tracer
*/

fn main() {
    let mut proj = Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    );

    let env = Environment::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(-0.01, 0.0, 0.0),
    );

    let mut count = 0;
    while proj.position.y > 0.0 {
        count += 1;
        proj.tick(env.clone());
        println!("count: {} | projectile: {}", count, proj);
    }
}
