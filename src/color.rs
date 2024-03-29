use crate::float_service::compare_floats;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

/**
    Author: Maciek Mika
    This file contains the color struct and methods. It is used to represent colors on the canvas.
    Also contains tests methods
*/

#[derive(Debug, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

// implement '+' operator overload for Colors
impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        compare_floats(self.red, other.red)
            && compare_floats(self.green, other.green)
            && compare_floats(self.blue, other.blue)
    }
}

// implement '-' operator overload for Colors
impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

// return the r g b values of the pixel. filter out
// all values that are below 0 or above 255 to be 0 or 255
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let red = match (self.red * 255.0).ceil() as i32 {
            n if n > 255 => 255,
            n if n < 0 => 0,
            n => n,
        };
        let green = match (self.green * 255.0).ceil() as i32 {
            n if n > 255 => 255,
            n if n < 0 => 0,
            n => n,
        };
        let blue = match (self.blue * 255.0).ceil() as i32 {
            n if n > 255 => 255,
            n if n < 0 => 0,
            n => n,
        };
        write!(f, "{} {} {}", red, green, blue)
    }
}

// implement '*' operator overload for Colors with another Color
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

// implement '*' operator overload for Colors with a f64
impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

// Color struct functions
impl Color {
    // create new color
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    // return a vec of all the color values
    pub fn to_vec_string(&self) -> Vec<String> {
        let red = match (self.red * 255.0).ceil() as i32 {
            n if n > 255 => 255,
            n if n < 0 => 0,
            n => n,
        };
        let green = match (self.green * 255.0).ceil() as i32 {
            n if n > 255 => 255,
            n if n < 0 => 0,
            n => n,
        };
        let blue = match (self.blue * 255.0).ceil() as i32 {
            n if n > 255 => 255,
            n if n < 0 => 0,
            n => n,
        };
        vec![red.to_string(), green.to_string(), blue.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn creating_color() {
        let color = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(color.red, -0.5);
        assert_eq!(color.green, 0.4);
        assert_eq!(color.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        let color_correct = Color::new(1.6, 0.7, 1.0);

        assert_eq!(color1 + color2, color_correct);
    }

    #[test]
    fn subtracting_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        let color_correct = Color::new(0.2, 0.5, 0.5);

        assert_eq!(color1 - color2, color_correct);
    }

    #[test]
    fn multiplying_colors_by_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        let color_correct = Color::new(0.4, 0.6, 0.8);

        assert_eq!(color * 2.0, color_correct);
    }

    #[test]
    fn multiplying_colors() {
        let color1 = Color::new(1.0, 0.2, 0.4);
        let color2 = Color::new(0.9, 1.0, 0.1);
        let color_correct = Color::new(0.9, 0.2, 0.04);

        assert_eq!(color1 * color2, color_correct);
    }
}
