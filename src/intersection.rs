use crate::sphere::Sphere;
use num::Float;

// this trait is used to identify shape structs
// TODO: will be expanded on later
pub trait IntersectionObject {}

pub struct Intersection<T, O>
where
    T: Float,
    O: IntersectionObject,
{
    pub value: T,
    pub object: O,
}

impl<T, O> Intersection<T, O>
where
    T: Float,
    O: IntersectionObject,
{
    pub fn new(value: T, object: O) -> Self {
        Self { value, object }
    }
}

// array primitive for holding all the intersection
// TODO: will be expanded on later
pub struct Intersections<T, O, const N: usize>
where
    T: Float,
    O: IntersectionObject,
{
    data: [Intersection<T, O>; N],
}

impl<T, O, const N: usize> Intersections<T, O, N>
where
    T: Float,
    O: IntersectionObject,
{
    pub fn new(data: [Intersection<T, O>; N]) -> Self {
        Self { data }
    }
}

impl<T, O, const N: usize> Index<usize> for Intersections<T, O, N>
where
    T: Float,
    O: IntersectionObject,
{
    type Output = Intersection<T, O>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, O, const N: usize> IndexMut<usize> for Intersections<T, O, N>
where
    T: Float,
    O: IntersectionObject,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self[index]
    }
}

mod tests {
    use crate::intersection::{Intersection, Intersections};
    use crate::sphere::Sphere;

    #[test]
    fn create_intersection() {
        let sphere = Sphere::new(1);
        let intersection = Intersection::<f64, Sphere>::new(1.0, sphere.clone());

        assert_eq!(intersection.value, 1.0);
        assert_eq!(intersection.object, sphere)
    }

    #[test]
    fn create_intersections_check_values() {
        let sphere = Sphere::new(1);
        let intersection_1 = Intersection::<f64, Sphere>::new(1.0, sphere.clone());
        let intersection_2 = Intersection::<f64, Sphere>::new(2.0, sphere.clone());
        let intersections = Intersections::<f64, Sphere, 2>::new([intersection_1, intersection_2]);

        assert_eq!(intersections[0].value, 1.0);
        assert_eq!(intersections[1].value, 2.0)
    }

    #[test]
    fn create_intersection_check_objects() {
        let sphere = Sphere::new(1);
        let intersection_1 = Intersection::<f64, Sphere>::new(1.0, sphere.clone());
        let intersection_2 = Intersection::<f64, Sphere>::new(2.0, sphere.clone());
        let intersections = Intersections::<f64, Sphere, 2>::new([intersection_1, intersection_2]);

        assert_eq!(intersections[0].object, sphere);
        assert_eq!(intersections[1].object, sphere)
    }
}
