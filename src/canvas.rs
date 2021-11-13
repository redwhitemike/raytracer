/**
    Author: Maciek Mika
    This file contains the canvas struct, its methods and its tests
*/
use crate::color::*;

// the canvas struct, used as a rectangular grid of pixels
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl PartialEq for Canvas {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.pixels == other.pixels
    }
}

impl Canvas {
    // create a new Canvas struct and initialize its pixels to default color
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width - 1]; height - 1],
        }
    }

    // change a pixel at the given position
    pub fn write_pixel(&mut self, width: usize, height: usize, color: Color) {
        self.pixels[width][height] = color
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(8, 8);
        let check_canvas = Canvas {
            width: 8,
            height: 8,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); 7]; 7],
        };

        assert_eq!(canvas, check_canvas)
    }

    #[test]
    fn insert_pixel() {
        let mut canvas = Canvas::new(8, 8);
        let color = Color::new(1.0, 0.0, 0.0);
        let test_color = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, color);

        assert_eq!(canvas.pixels[2][3], test_color)
    }
}
