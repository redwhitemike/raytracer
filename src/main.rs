use crate::canvas::Canvas;
use crate::color::Color;

mod canvas;
mod color;
mod tuple;

/**
Author: Maciek Mika
This is the main file that runs the ray tracer
 */

fn main() {
    let mut canvas = Canvas::new(5, 3);
    let color1 = Color::new(1.5, 0.0, 0.0);
    let color2 = Color::new(0.0, 0.5, 0.0);
    let color3 = Color::new(-0.5, 0.0, 1.0);

    canvas.write_pixel(0, 0, color1);
    canvas.write_pixel(2, 1, color2);
    canvas.write_pixel(4, 2, color3);

    let ppm_string = canvas.to_ppm();
    println!("{}", ppm_string);
}
