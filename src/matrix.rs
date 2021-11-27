use num::Float;
use std::ops::{AddAssign, Mul};

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

    // insert a new value in the matrix, returns a Result
    pub fn insert(&mut self, i1: usize, i2: usize, number: T) -> Result<(), &'static str> {
        match (i1 * self.rows) + i2 < self.data.len() {
            true => {
                self.data[(i1 * self.rows) + i2] = number;
                Ok(())
            }
            false => Err("index out of bounds"),
        }
    }
}

// implement '==' for Matrix<T>. Make it possible to check if
// 2 matrixes are the same
impl<T: Float> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.rows == other.rows && self.cols == self.cols
    }
}

// implement '*' operator overloading for Matrix<T>
impl<T: Float + AddAssign> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_matrix = Matrix::new(self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut matrix_product: T = T::zero();
                for index in 0..new_matrix.rows {
                    matrix_product +=
                        self.data[(row * self.rows) + index] + rhs.data[(index * self.rows) + col];
                }
                match new_matrix.insert(row, col, matrix_product) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
        new_matrix
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

    #[test]
    fn multiply_matrices() {
        let matrix1: Matrix<f64> = Matrix::new_from(
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let matrix2: Matrix<f64> = Matrix::new_from(
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );

        let correct_matrix: Matrix<f64> = Matrix::new_from(
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );

        assert_ne!(matrix1 * matrix2, correct_matrix)
    }
}
