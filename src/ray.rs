use crate::Tuple;
use num::Float;

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
{
    pub fn new(origin: Tuple<T>, direction: Tuple<T>) -> Self {
        Self { origin, direction }
    }

    // return the position of a ray at a certain distance
    pub fn position(&self, distance: T) -> Tuple<T> {
        self.origin + self.direction * distance
    }
}

mod tests {
    use crate::ray::Ray;
    use crate::Tuple;

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
}
