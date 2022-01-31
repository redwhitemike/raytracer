use crate::intersection::{Intersection, IntersectionObject};
use crate::ray::Ray;
use crate::Tuple;
use num::Float;

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere {
    id: i32,
}

impl Sphere {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

// IntersectionObject is used as generic trait
// for all the objects that intersect with rays
impl IntersectionObject for Sphere {
    type Object = Sphere;

    fn intersect<T: Float>(
        &self,
        ray: Ray<T>,
    ) -> Result<[Intersection<T, Self::Object>; 2], &'static str> {
        let sphere_to_ray = ray.origin - Tuple::<T>::new_point(T::zero(), T::zero(), T::zero());
        let a = ray.direction.dot_product(&ray.direction);
        let b: T = ray.direction.dot_product(&sphere_to_ray) * T::from(2.0).unwrap();
        let c: T = sphere_to_ray.dot_product(&sphere_to_ray) - T::from(1.0).unwrap();
        let discriminant: T = b.powi(2) - T::from(4.0).unwrap() * a * c;

        match discriminant < T::zero() {
            true => Err("Discriminant is lower then 0"),
            false => {
                let intersection_1 = Intersection::<T, Sphere>::new(
                    (-b - discriminant.sqrt()) / (T::from(2.0).unwrap() * a),
                    self.clone(),
                );
                let intersection_2 = Intersection::<T, Sphere>::new(
                    (-b + discriminant.sqrt()) / (T::from(2.0).unwrap() * a),
                    self.clone(),
                );

                Ok([intersection_1, intersection_2])
            }
        }
    }
}

mod tests {
    use crate::intersection::IntersectionObject;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::Tuple;

    #[test]
    fn ray_intersection_2_points() {
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 0.0, -5.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        let sphere = Sphere::new(1);

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
        let sphere = Sphere::new(1);

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
        let sphere = Sphere::new(1);

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
        let sphere = Sphere::new(1);

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
        let sphere = Sphere::new(1);

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
}
