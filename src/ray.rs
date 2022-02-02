use crate::{Matrix, Tuple};
use num::Float;
use std::ops::AddAssign;

/**
    Author: Maciek Mika
    This file contains the Ray structure and its functions
*/

pub struct Ray<T>
where
    T: Float,
{
    pub origin: Tuple<T>,
    pub direction: Tuple<T>,
}

impl<T> Ray<T>
where
    T: Float,
    T: AddAssign,
{
    pub fn new(origin: Tuple<T>, direction: Tuple<T>) -> Self {
        Self { origin, direction }
    }

    // return the position of a ray at a certain distance
    pub fn position(&self, distance: T) -> Tuple<T> {
        self.origin + self.direction * distance
    }

    // apply matrix transformation to ray and return a new ray
    pub fn transform(&self, translation: Matrix<T, 4>) -> Self {
        Self {
            origin: translation.clone() * self.origin,
            direction: translation.clone() * self.direction,
        }
    }
}

mod tests {
    use crate::ray::Ray;
    use crate::{Matrix, Tuple};

    #[test]
    fn create_ray() {
        let origin = Tuple::<f64>::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::<f64>::new_vector(4.0, 5.0, 6.0);
        let ray = Ray::<f64>::new(origin.clone(), direction.clone());
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction)
    }
    #[test]
    fn position() {
        let ray = Ray::new(
            Tuple::<f64>::new_point(2.0, 3.0, 4.0),
            Tuple::<f64>::new_vector(1.0, 0.0, 0.0),
        );
        let point_1 = Tuple::<f64>::new_point(2.0, 3.0, 4.0);
        let point_2 = Tuple::<f64>::new_point(3.0, 3.0, 4.0);
        let point_3 = Tuple::<f64>::new_point(1.0, 3.0, 4.0);
        let point_4 = Tuple::<f64>::new_point(4.5, 3.0, 4.0);

        assert_eq!(ray.position(0.0), point_1);
        assert_eq!(ray.position(1.0), point_2);
        assert_eq!(ray.position(-1.0), point_3);
        assert_eq!(ray.position(2.5), point_4)
    }

    #[test]
    fn translating_ray() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let translation = Matrix::<f64, 4>::translation(3.0, 4.0, 5.0);
        let transform = ray.transform(translation);
        let correct_origin = Tuple::<f64>::new_point(4.0, 6.0, 8.0);
        let correct_direction = Tuple::<f64>::new_vector(0.0, 1.0, 0.0);

        assert_eq!(transform.origin, correct_origin);
        assert_eq!(transform.direction, correct_direction)
    }

    #[test]
    fn scale_ray() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let scaling = Matrix::<f64, 4>::scaling(2.0, 3.0, 4.0);
        let transform = ray.transform(scaling);
        let correct_origin = Tuple::<f64>::new_point(2.0, 6.0, 12.0);
        let correct_direction = Tuple::<f64>::new_vector(0.0, 3.0, 0.0);

        assert_eq!(transform.origin, correct_origin);
        assert_eq!(transform.direction, correct_direction)
    }
}
