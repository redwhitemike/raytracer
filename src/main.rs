/**
Author: Maciek Mika
This is the main file that runs the ray tracer
*/

struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    // creates a new tuple that is a point
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    // creates a new tuple that is a vector
    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    // compares 2 tuples with each other
    pub fn is_equal(&self, tuple: &Tuple) -> bool {
        Self::compare_floats(self.x, tuple.x)
            && Self::compare_floats(self.y, tuple.y)
            && Self::compare_floats(self.z, tuple.z)
            && Self::compare_floats(self.w, tuple.w)
    }

    // compare floating numbers
    fn compare_floats(x: f32, y: f32) -> bool {
        if f32::abs(x - y) < EPSILON {
            return true;
        }
        false
    }

    // add 2 tuples together and return a new one
    pub fn add_tuples(&self, tuple: &Tuple) -> Tuple {
        Tuple {
            x: self.x + tuple.x,
            y: self.y + tuple.y,
            z: self.z + tuple.z,
            w: self.w + tuple.w,
        }
    }

    // add 2 tuples together and return a new one
    pub fn sub_tuples(&self, tuple: &Tuple) -> Tuple {
        Tuple {
            x: self.x - tuple.x,
            y: self.y - tuple.y,
            z: self.z - tuple.z,
            w: self.w - tuple.w,
        }
    }
}

const EPSILON: f32 = 0.00001;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Tuple;

    #[test]
    fn test_new_point() {
        let point = Tuple::new_point(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn test_new_vector() {
        let point = Tuple::new_vector(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 0.0);
    }

    #[test]
    fn test_tuple_addition() {
        let point = Tuple::new_point(3.0, -2.0, 5.0);
        let vector = Tuple::new_vector(-2.0, 3.0, 1.0);

        let new_tuple = point.add_tuples(&vector);

        assert_eq!(new_tuple.x, 1.0);
        assert_eq!(new_tuple.y, 1.0);
        assert_eq!(new_tuple.z, 6.0);
        assert_eq!(new_tuple.w, 1.0);
    }

    #[test]
    fn test_points_subtraction() {
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point2 = Tuple::new_point(5.0, 6.0, 7.0);

        let new_tuple = point1.sub_tuples(&point2);

        assert_eq!(new_tuple.x, -2.0);
        assert_eq!(new_tuple.y, -4.0);
        assert_eq!(new_tuple.z, -6.0);
        assert_eq!(new_tuple.w, 0.0);
    }

    #[test]
    fn test_vector_point_subtraction() {
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let new_tuple = point1.sub_tuples(&point2);

        assert_eq!(new_tuple.x, -2.0);
        assert_eq!(new_tuple.y, -4.0);
        assert_eq!(new_tuple.z, -6.0);
        assert_eq!(new_tuple.w, 1.0);
    }

    #[test]
    fn test_vectors_subtraction() {
        let point1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let point2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let new_tuple = point1.sub_tuples(&point2);

        assert_eq!(new_tuple.x, -2.0);
        assert_eq!(new_tuple.y, -4.0);
        assert_eq!(new_tuple.z, -6.0);
        assert_eq!(new_tuple.w, 0.0);
    }
}
