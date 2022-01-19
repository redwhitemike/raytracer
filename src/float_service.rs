use num::Float;

const EPSILON: f64 = 0.00001;

// compare floating numbers
pub fn compare_floats<T>(x: T, y: T) -> bool
where
    T: Float,
{
    match T::from(EPSILON) {
        None => false,
        Some(eps) => T::abs(x - y) < eps,
    }
}
