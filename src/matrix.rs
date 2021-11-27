use num::Float;

#[derive(Debug)]
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
    T: PartialEq,
{
    // create a new matrix with given size
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![T::zero(); size * size],
            rows: size,
            cols: size,
        }
    }

    // create a matrix from a given vector of floats
    pub fn new_from(size: usize, data: Vec<T>) -> Self {
        Self {
            data,
            rows: size,
            cols: size,
        }
    }

    // get from the matrix
    pub fn get(&self, i1: usize, i2: usize) -> Option<&T> {
        self.data.get((i1 * self.rows) + i2)
    }
}

// implement '==' for Matrix<T>. Make it possible to check if
// 2 matrixes are the same
impl<T: Float> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.rows == other.rows && self.cols == self.cols
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn create_matrix_4x4() {
        let mut matrix: Matrix<f64> = Matrix::new(4);
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
        let mut matrix: Matrix<f64> = Matrix::new(3);
        let matrix_values: Vec<f64> = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        matrix.data = matrix_values;

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
        assert_eq!(*matrix.get(2, 2).unwrap(), 1.0);
    }

    #[test]
    fn create_matrix_2x2() {
        let mut matrix: Matrix<f64> = Matrix::new(2);
        let matrix_values: Vec<f64> = vec![-3.0, 5.0, 1.0, -2.0];
        matrix.data = matrix_values;

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(0, 1).unwrap(), 5.0);
        assert_eq!(*matrix.get(1, 0).unwrap(), 1.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
    }

    #[test]
    fn compare_matrix_equal() {
        let matrix1: Matrix<f64> = Matrix::new_from(
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let matrix2: Matrix<f64> = Matrix::new_from(
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        assert_eq!(matrix1, matrix2)
    }

    #[test]
    fn compare_matrix_not_equal() {
        let matrix1: Matrix<f64> = Matrix::new_from(
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let matrix2: Matrix<f64> = Matrix::new_from(
            4,
            vec![
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ],
        );

        assert_ne!(matrix1, matrix2)
    }
}
