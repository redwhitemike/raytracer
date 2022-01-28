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
    origin: Tuple<T>,
    direction: Tuple<T>,
}

impl<T> Ray<T>
where
    T: Float,
{
    pub fn new(origin: Tuple<T>, direction: Tuple<T>) -> Self {
        Self { origin, direction }
    }

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
    fn position() {}
}
