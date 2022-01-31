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

mod tests {
    use crate::intersection::Intersection;
    use crate::sphere::Sphere;

    #[test]
    fn create_intersection() {
        let sphere = Sphere::new(1);
        let intersection = Intersection::<f64, Sphere>::new(1.0, sphere.clone());

        assert_eq!(intersection.value, 1.0);
        assert_eq!(intersection.object, sphere)
    }
}
