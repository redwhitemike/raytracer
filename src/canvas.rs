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
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height],
        }
    }

    // create a new canvas with a default color given
    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![color; width]; height],
        }
    }

    // change a pixel at the given position
    pub fn write_pixel(&mut self, width: usize, height: usize, color: Color) {
        self.pixels[height][width] = color
    }

    // helper function to make the retrieval easier for getting a pixel
    pub fn at_pixel(&self, width: usize, height: usize) -> &Color {
        &self.pixels[height][width]
    }

    // create ppm string out of a canvas struct
    pub fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut ppm_vec: Vec<String> = Vec::new();
        ppm_vec.push(header);
        ppm_vec.push(String::new());

        self.pixels.iter().for_each(|x| {
            x.iter().for_each(|y| {
                let string_vec = y.to_vec_string();
                // check if the last string in the ppm vec is longer then 70
                // when you add length of the color string, a space and newline
                // characters.
                string_vec.iter().for_each(|c| {
                    match ppm_vec.last().unwrap().len() + c.chars().count() + 3 <= 70 {
                        true => ppm_vec.last_mut().unwrap().push_str(&*format!("{} ", c)),
                        false => {
                            ppm_vec.last_mut().unwrap().push_str(" \n");
                            ppm_vec.push(format!("{} ", c))
                        }
                    }
                })
            });
            // push empty new string after all pixels
            // in a row have been converted
            ppm_vec.push("\n".to_string())
        });

        ppm_vec.join("")
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
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); 8]; 8],
        };

        assert_eq!(canvas, check_canvas)
    }

    #[test]
    fn insert_pixel() {
        let mut canvas = Canvas::new(8, 8);
        let color = Color::new(1.0, 0.0, 0.0);
        let test_color = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, color);

        assert_eq!(*canvas.at_pixel(2, 3), test_color)
    }

    #[test]
    fn construct_ppm() {
        let mut canvas = Canvas::new(5, 3);
        let color1 = Color::new(1.5, 0.0, 0.0);
        let color2 = Color::new(0.0, 0.5, 0.0);
        let color3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, color1);
        canvas.write_pixel(2, 1, color2);
        canvas.write_pixel(4, 2, color3);

        let ppm_string = canvas.to_ppm();
        let correct_string = String::from(
            "P3\n\
            10 2\n\
        255\n\
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n",
        );
        assert_eq!(ppm_string, correct_string)
    }

    #[test]
    fn check_lines_split_ppm() {
        let color = Color::new(1.0, 0.8, 0.6);
        let canvas = Canvas::new_with_color(10, 2, color);

        let ppm_string = canvas.to_ppm();
        let correct_string = String::from(
            "P3\n\
            5 3\n\
        255\n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n",
        );
        assert_eq!(ppm_string, correct_string)
    }
}
