struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_equal(&self, tuple: &Tuple) -> bool {
        self.x == tuple.x && self.y == tuple.y && self.
    }

    fn compare_floats(x: f32, y: f32) -> bool {
        if (x - y < EPSILON) {

        }
    }
}

const EPSILON: f64 = 0.00001;

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
}
