use crate::ray::Ray;
use crate::Tuple;
use num::Float;
use std::clone::Clone;
use std::ops::{Index, IndexMut};

// this trait is used to identify shape structs
pub trait IntersectionObject<T>
where
    T: Float,
{
    type Object;

    // return an array of 2 intersections where the ray has intersected
    // the given object.
    fn intersect(&self, ray: Ray<T>) -> Result<Intersections<T, Self::Object, 2>, &'static str>
    where
        <Self as IntersectionObject<T>>::Object: IntersectionObject<T> + Clone;

    fn normal_at(&self, vector: Tuple<T>) -> Tuple<T>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<T, O>
where
    T: Float,
    O: IntersectionObject<T> + Clone,
{
    pub value: T,
    pub object: O,
}

impl<T, O> Intersection<T, O>
where
    T: Float,
    O: IntersectionObject<T> + Clone,
{
    pub fn new(value: T, object: O) -> Self {
        Self { value, object }
    }
}

// array primitive for holding all the intersection
pub struct Intersections<T, O, const N: usize>
where
    T: Float,
    O: IntersectionObject<T> + Clone,
{
    data: [Intersection<T, O>; N],
}

impl<T, O, const N: usize> Intersections<T, O, N>
where
    T: Float,
    O: IntersectionObject<T> + Clone,
{
    pub fn new(data: [Intersection<T, O>; N]) -> Self {
        Self { data }
    }

    // return the intersection with closest to 0.0
    // value
    pub fn hit(&self) -> Option<Intersection<T, O>> {
        let mut closest_intersection = None;

        self.data.iter().cloned().for_each(|inter| {
            if inter.value > T::zero() {
                match closest_intersection.is_none() {
                    true => {
                        closest_intersection = Some(inter);
                    }
                    false => {
                        if closest_intersection.as_ref().unwrap().value > inter.value {
                            closest_intersection = Some(inter)
                        }
                    }
                }
            }
        });
        closest_intersection
    }
}

impl<T, O, const N: usize> Index<usize> for Intersections<T, O, N>
where
    T: Float,
    O: IntersectionObject<T> + Clone,
{
    type Output = Intersection<T, O>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, O, const N: usize> IndexMut<usize> for Intersections<T, O, N>
where
    T: Float,
    O: IntersectionObject<T> + Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

mod tests {
    use crate::intersection::{Intersection, IntersectionObject, Intersections};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::Tuple;

    #[test]
    fn create_intersection() {
        let sphere = Sphere::<f64>::new(1);
        let intersection = Intersection::<f64, Sphere<f64>>::new(1.0, sphere.clone());

        assert_eq!(intersection.value, 1.0);
        assert_eq!(intersection.object, sphere)
    }

    #[test]
    fn create_intersections_check_values() {
        let sphere = Sphere::<f64>::new(1);
        let intersection_1 = Intersection::<f64, Sphere<f64>>::new(1.0, sphere.clone());
        let intersection_2 = Intersection::<f64, Sphere<f64>>::new(2.0, sphere.clone());
        let intersections =
            Intersections::<f64, Sphere<f64>, 2>::new([intersection_1, intersection_2]);

        assert_eq!(intersections[0].value, 1.0);
        assert_eq!(intersections[1].value, 2.0)
    }

    #[test]
    fn create_intersection_check_objects() {
        let sphere = Sphere::<f64>::new(1);
        let ray = Ray::<f64>::new(
            Tuple::<f64>::new_point(0.0, 0.0, -5.0),
            Tuple::<f64>::new_vector(0.0, 0.0, 1.0),
        );
        match sphere.intersect(ray) {
            Ok(intersection) => {
                assert_eq!(intersection[0].object, sphere);
                assert_eq!(intersection[1].object, sphere)
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn return_hit() {
        let sphere = Sphere::<f64>::new(1);
        let intersection_1 = Intersection::<f64, Sphere<f64>>::new(1.0, sphere.clone());
        let intersection_2 = Intersection::<f64, Sphere<f64>>::new(2.0, sphere.clone());

        let intersections =
            Intersections::<f64, Sphere<f64>, 2>::new([intersection_1.clone(), intersection_2]);

        match intersections.hit() {
            None => {
                assert_eq!(true, false)
            }
            Some(inter) => {
                assert_eq!(inter, intersection_1)
            }
        }
    }

    #[test]
    fn return_hit_some_negative() {
        let sphere = Sphere::<f64>::new(1);
        let intersection_1 = Intersection::<f64, Sphere<f64>>::new(-1.0, sphere.clone());
        let intersection_2 = Intersection::<f64, Sphere<f64>>::new(1.0, sphere.clone());
        let intersections =
            Intersections::<f64, Sphere<f64>, 2>::new([intersection_1, intersection_2.clone()]);

        match intersections.hit() {
            None => {
                assert_eq!(true, false)
            }
            Some(inter) => {
                assert_eq!(inter, intersection_2)
            }
        }
    }

    #[test]
    fn return_no_hit() {
        let sphere = Sphere::<f64>::new(1);
        let intersection_1 = Intersection::<f64, Sphere<f64>>::new(-2.0, sphere.clone());
        let intersection_2 = Intersection::<f64, Sphere<f64>>::new(-1.0, sphere);
        let intersections =
            Intersections::<f64, Sphere<f64>, 2>::new([intersection_1, intersection_2]);

        match intersections.hit() {
            None => {
                assert_eq!(true, true)
            }
            Some(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn return_lowest_multiple_intersections() {
        let sphere = Sphere::<f64>::new(1);
        let intersection_1 = Intersection::<f64, Sphere<f64>>::new(5.0, sphere.clone());
        let intersection_2 = Intersection::<f64, Sphere<f64>>::new(7.0, sphere.clone());
        let intersection_3 = Intersection::<f64, Sphere<f64>>::new(-3.0, sphere.clone());
        let intersection_4 = Intersection::<f64, Sphere<f64>>::new(2.0, sphere.clone());
        let intersections = Intersections::<f64, Sphere<f64>, 4>::new([
            intersection_1,
            intersection_2,
            intersection_3,
            intersection_4.clone(),
        ]);

        match intersections.hit() {
            None => {
                assert_eq!(true, false)
            }
            Some(inter) => {
                assert_eq!(intersection_4, inter)
            }
        }
    }
}
