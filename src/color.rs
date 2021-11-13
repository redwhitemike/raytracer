use crate::tuple::Tuple;
use std::ops::{Add, Mul, Sub};
/**
    Author: Maciek Mika
    This file contains the color struct and methods. It is used to represent colors on the canvas.
    Also contains tests methods
*/

#[derive(Debug)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
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
        Tuple::compare_floats(self.red, other.red)
            && Tuple::compare_floats(self.green, other.green)
            && Tuple::compare_floats(self.blue, other.blue)
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

// implement '*' operator overload for Colors with a f32
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

// Color struct functions
impl Color {
    // create new color
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn test_creating_color() {
        let color = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(color.red, -0.5);
        assert_eq!(color.green, 0.4);
        assert_eq!(color.blue, 1.7);
    }

    #[test]
    fn test_adding_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        let color_correct = Color::new(1.6, 0.7, 1.0);

        assert_eq!(color1 + color2, color_correct);
    }

    #[test]
    fn test_substracting_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        let color_correct = Color::new(0.2, 0.5, 0.5);

        assert_eq!(color1 - color2, color_correct);
    }

    #[test]
    fn test_multiplying_colors_by_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        let color_correct = Color::new(0.4, 0.6, 0.8);

        assert_eq!(color * 2.0, color_correct);
    }

    #[test]
    fn test_multiplying_colors() {
        let color1 = Color::new(1.0, 0.2, 0.4);
        let color2 = Color::new(0.9, 1.0, 0.1);
        let color_correct = Color::new(0.9, 0.2, 0.04);

        assert_eq!(color1 * color2, color_correct);
    }
}
