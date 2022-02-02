use crate::intersection::{Intersection, IntersectionObject};
use crate::ray::Ray;
use crate::{Matrix, Tuple};
use num::Float;
use std::ops::AddAssign;

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere<T>
where
    T: Float,
{
    id: i32,
    transformation: Matrix<T, 4>,
}

impl<T> Sphere<T>
where
    T: Float,
    T: AddAssign,
{
    pub fn new(id: i32) -> Self {
        Self {
            id,
            transformation: Matrix::<T, 4>::identity_matrix(),
        }
    }

    // made it a function for now, maybe
    // more functionality is added in the future
    pub fn set_translation(&mut self, transformation: Matrix<T, 4>) {
        self.transformation = transformation
    }
}

// IntersectionObject is used as generic trait
// for all the objects that intersect with rays
impl<T> IntersectionObject<T> for Sphere<T>
where
    T: Float,
    T: AddAssign,
{
    type Object = Sphere<T>;

    fn intersect(&self, ray: Ray<T>) -> Result<[Intersection<T, Self::Object>; 2], &'static str> {
        let mut inverse = self.transformation.clone();
        match inverse.inverse() {
            Ok(inv) => inverse = inv,
            Err(_) => return Err("inverse failed"),
        }
        let ray = ray.transform(inverse);
        let sphere_to_ray = ray.origin - Tuple::<T>::new_point(T::zero(), T::zero(), T::zero());
        let a = ray.direction.dot_product(&ray.direction);
        let b: T = ray.direction.dot_product(&sphere_to_ray) * T::from(2.0).unwrap();
        let c: T = sphere_to_ray.dot_product(&sphere_to_ray) - T::from(1.0).unwrap();
        let discriminant: T = b.powi(2) - T::from(4.0).unwrap() * a * c;

        match discriminant < T::zero() {
            true => Err("Discriminant is lower then 0"),
            false => {
                let intersection_1 = Intersection::<T, Sphere<T>>::new(
                    (-b - discriminant.sqrt()) / (T::from(2.0).unwrap() * a),
                    self.clone(),
                );
                let intersection_2 = Intersection::<T, Sphere<T>>::new(
                    (-b + discriminant.sqrt()) / (T::from(2.0).unwrap() * a),
                    self.clone(),
                );

                Ok([intersection_1, intersection_2])
            }
        }
    }
}

mod tests {
    use crate::intersection::{Intersection, IntersectionObject};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::{Matrix, Tuple};

    #[test]
    fn ray_intersection_2_points() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 0.0, -5.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::<f64>::new(1);

        match sphere.intersect(ray) {
            Ok(inter) => {
                assert_eq!(inter[0].value, 4.0_f64);
                assert_eq!(inter[1].value, 6.0_f64)
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn ray_intersect_tangent() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 1.0, -5.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::<f64>::new(1);

        match sphere.intersect(ray) {
            Ok(inter) => {
                assert_eq!(inter[0].value, 5.0_f64);
                assert_eq!(inter[1].value, 5.0_f64)
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 2.0, -5.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::<f64>::new(1);

        match sphere.intersect(ray) {
            Ok(_) => {
                assert_eq!(true, false)
            }
            Err(_) => {
                assert_eq!(true, true)
            }
        }
    }

    #[test]
    fn ray_originates_in_sphere() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 0.0, 0.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::<f64>::new(1);

        match sphere.intersect(ray) {
            Ok(inter) => {
                assert_eq!(inter[0].value, -1.0_f64);
                assert_eq!(inter[1].value, 1.0_f64)
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 0.0, 5.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::<f64>::new(1);

        match sphere.intersect(ray) {
            Ok(inter) => {
                assert_eq!(inter[0].value, -6.0_f64);
                assert_eq!(inter[1].value, -4.0_f64)
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn sphere_default_transformation() {
        let sphere = Sphere::<f64>::new(1);
        let identity_matrix = Matrix::<f64, 4>::identity_matrix();

        assert_eq!(sphere.transformation, identity_matrix)
    }

    #[test]
    fn change_sphere_transformation() {
        let mut sphere = Sphere::<f64>::new(1);
        let translation = Matrix::<f64, 4>::translation(2.0, 3.0, 4.0);
        sphere.set_translation(translation.clone());

        assert_eq!(sphere.transformation, translation)
    }

    #[test]
    fn intersect_scaled_sphere_ray() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 0.0, -5.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        let mut sphere = Sphere::<f64>::new(1);
        sphere.set_translation(Matrix::scaling(2.0, 2.0, 2.0));
        match sphere.intersect(ray) {
            Ok(inter) => {
                assert_eq!(inter[0].value, 3.0);
                assert_eq!(inter[1].value, 7.0)
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }
}
