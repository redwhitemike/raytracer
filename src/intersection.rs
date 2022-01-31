use crate::sphere::Sphere;
use num::Float;

pub struct Intersection<T, O>
where
    T: Float,
    O: IntersectionObject,
{
    pub value: T,
    pub object: O,
}

pub trait IntersectionObject {}

impl<T, O> Intersection<T, O>
where
    T: Float,
    O: IntersectionObject,
{
    pub fn new(value: T, object: O) -> Self {
        Self { value, object }
    }
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
