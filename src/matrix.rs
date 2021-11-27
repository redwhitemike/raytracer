use num::Float;

struct Matrix<T>
where
    T: Float,
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T>
where
    T: Float,
{
    // create a new Matrix of 2x2 dimensions
    pub fn new_2x2() -> Self {
        Self {
            data: vec![T::zero(); 2 * 2],
            rows: 2,
            cols: 2,
        }
    }

    // create a new Matrix of 3x3 dimensions
    pub fn new_3x3() -> Self {
        Self {
            data: vec![T::zero(); 3 * 3],
            rows: 3,
            cols: 3,
        }
    }

    // create a new Matrix of 4x4 dimensions
    pub fn new_4x4() -> Self {
        Self {
            data: vec![T::zero(); 4 * 4],
            rows: 4,
            cols: 4,
        }
    }

    // get from the matrix
    pub fn get(&self, i1: usize, i2: usize) -> Option<&T> {
        self.data.get((i1 * self.rows) + i2)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn create_matrix_4x4() {
        let mut matrix: Matrix<f64> = Matrix::new_4x4();
        let matrix_values: Vec<f64> = vec![
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ];
        matrix.data = matrix_values;

        assert_eq!(*matrix.get(0, 0).unwrap(), 1.0);
        assert_eq!(*matrix.get(0, 3).unwrap(), 4.0);
        assert_eq!(*matrix.get(1, 0).unwrap(), 5.5);
        assert_eq!(*matrix.get(1, 2).unwrap(), 7.5);
        assert_eq!(*matrix.get(2, 2).unwrap(), 11.0);
        assert_eq!(*matrix.get(3, 0).unwrap(), 13.5);
        assert_eq!(*matrix.get(3, 2).unwrap(), 15.5);
    }

    #[test]
    fn create_matrix_3x3() {
        let mut matrix: Matrix<f64> = Matrix::new_3x3();
        let matrix_values: Vec<f64> = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        matrix.data = matrix_values;

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
        assert_eq!(*matrix.get(2, 2).unwrap(), 1.0);
    }

    #[test]
    fn create_matrix_2x2() {
        let mut matrix: Matrix<f64> = Matrix::new_2x2();
        let matrix_values: Vec<f64> = vec![-3.0, 5.0, 1.0, -2.0];
        matrix.data = matrix_values;

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(0, 1).unwrap(), 5.0);
        assert_eq!(*matrix.get(1, 0).unwrap(), 1.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
    }
}
