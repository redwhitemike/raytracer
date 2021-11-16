use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
/**
   Author: Maciek Mika
   This is the tuple file. It contains the tuple struct information and methods.
   Also contains tests
*/

#[derive(Debug, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{} y:{} z:{} w:{}", self.x, self.y, self.z, self.w)
    }
}

// implement '-' operator overloading
impl Neg for Tuple {
    type Output = ();

    fn neg(mut self) -> Self::Output {
        self.x = self.x.neg();
        self.y = self.y.neg();
        self.z = self.z.neg();
        self.w = self.w.neg();
    }
}

// implement '+' operator overloading
impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

// implement '-' operator overloading
impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

// implement '==' operator overload for comparing tuples
impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        Tuple::compare_floats(self.x, other.x)
            && Tuple::compare_floats(self.y, other.y)
            && Tuple::compare_floats(self.z, other.z)
    }
}

// implement operator overloading for multiplying tuple
impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

// implement operator overloading for dividing tuple
impl Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Tuple {
    // create new tuple
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    // creates a new tuple that is a point
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    // creates a new tuple that is a vector
    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    // compare floating numbers
    pub fn compare_floats(x: f32, y: f32) -> bool {
        if f32::abs(x - y) < EPSILON {
            return true;
        }
        false
    }

    // return the magnitude of the vector
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
    }

    // convert vector into unit vector
    pub fn normalize(&self) -> Tuple {
        Tuple {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    // create dot product of 2 vectors
    pub fn dot_product(t1: &Tuple, t2: &Tuple) -> f32 {
        t1.x * t2.x + t1.y * t2.y + t1.z * t2.z + t1.w * t1.w
    }

    // create cross product of 2 vectors
    pub fn cross_product(t1: &Tuple, t2: &Tuple) -> Tuple {
        Tuple::new_vector(
            t1.y * t2.z - t1.z * t2.y,
            t1.z * t2.x - t1.x * t2.z,
            t1.x * t2.y - t1.y * t2.x,
        )
    }
}

const EPSILON: f32 = 0.00001;

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn new_point() {
        let point = Tuple::new_point(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn new_vector() {
        let point = Tuple::new_vector(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 0.0);
    }

    #[test]
    fn tuple_addition() {
        let point = Tuple::new_point(3.0, -2.0, 5.0);
        let vector = Tuple::new_vector(-2.0, 3.0, 1.0);

        let new_tuple = point + vector;

        assert_eq!(new_tuple.x, 1.0);
        assert_eq!(new_tuple.y, 1.0);
        assert_eq!(new_tuple.z, 6.0);
        assert_eq!(new_tuple.w, 1.0);
    }

    #[test]
    fn points_subtraction() {
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point2 = Tuple::new_point(5.0, 6.0, 7.0);

        let new_tuple = point1 - point2;

        assert_eq!(new_tuple.x, -2.0);
        assert_eq!(new_tuple.y, -4.0);
        assert_eq!(new_tuple.z, -6.0);
        assert_eq!(new_tuple.w, 0.0);
    }

    #[test]
    fn vector_point_subtraction() {
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let new_tuple = point1.sub_tuples(&point2);

        assert_eq!(new_tuple.x, -2.0);
        assert_eq!(new_tuple.y, -4.0);
        assert_eq!(new_tuple.z, -6.0);
        assert_eq!(new_tuple.w, 1.0);
    }

    #[test]
    fn vectors_subtraction() {
        let point1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let point2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let new_tuple = point1.sub_tuples(&point2);

        assert_eq!(new_tuple.x, -2.0);
        assert_eq!(new_tuple.y, -4.0);
        assert_eq!(new_tuple.z, -6.0);
        assert_eq!(new_tuple.w, 0.0);
    }

    #[test]
    fn negate_tuple() {
        let point = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-point.x, -1.0);
        assert_eq!(-point.y, 2.0);
        assert_eq!(-point.z, -3.0);
        assert_eq!(-point.w, 4.0);
    }

    #[test]
    fn multiply_tuple() {
        let point1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let point2 = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let point1 = point1 * 3.5;
        let point2 = point2 * 0.5;
        assert_eq!(point1.x, 3.5);
        assert_eq!(point1.y, -7.0);
        assert_eq!(point1.z, 10.5);
        assert_eq!(point1.w, -14.0);

        assert_eq!(point2.x, 0.5);
        assert_eq!(point2.y, -1.0);
        assert_eq!(point2.z, 1.5);
        assert_eq!(point2.w, -2.0);
    }

    #[test]
    fn division_tuple() {
        let point = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let point = point / 2.0;
        assert_eq!(point.x, 0.5);
        assert_eq!(point.y, -1.0);
        assert_eq!(point.z, 1.5);
        assert_eq!(point.w, -2.0);
    }

    #[test]
    fn magnitude_vector() {
        let vector1 = Tuple::new_vector(1.0, 0.0, 0.0);
        let vector2 = Tuple::new_vector(0.0, 1.0, 0.0);
        let vector3 = Tuple::new_vector(0.0, 0.0, 1.0);
        let vector4 = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector5 = Tuple::new_vector(-1.0, -2.0, -3.0);

        assert_eq!(vector1.magnitude(), 1.0);
        assert_eq!(vector2.magnitude(), 1.0);
        assert_eq!(vector3.magnitude(), 1.0);
        assert_eq!(vector4.magnitude(), f32::sqrt(14.0));
        assert_eq!(vector5.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn normalization_vector() {
        let vector1 = Tuple::new_vector(4.0, 0.0, 0.0);
        let vector2 = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector3 = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(vector1.normalize(), Tuple::new_vector(1.0, 0.0, 0.0));
        assert_eq!(
            vector2.normalize(),
            Tuple::new_vector(
                1.0 / f32::sqrt(14.0),
                2.0 / f32::sqrt(14.0),
                3.0 / f32::sqrt(14.0)
            )
        );
        assert!(Tuple::compare_floats(vector3.normalize().magnitude(), 1.0));
    }

    #[test]
    fn dot_product() {
        let vector1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector2 = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::dot_product(&vector1, &vector2), 20.0)
    }

    #[test]
    fn cross_product() {
        let vector1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let vector2 = Tuple::new_vector(2.0, 3.0, 4.0);
        let right_vector1 = Tuple::new_vector(-1.0, 2.0, -1.0);
        let right_vector2 = Tuple::new_vector(1.0, -2.0, 1.0);

        assert_eq!(Tuple::cross_product(&vector1, &vector2), right_vector1);
        assert_eq!(Tuple::cross_product(&vector2, &vector1), right_vector2);
    }
}
