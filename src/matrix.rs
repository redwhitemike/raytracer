use num::Float;
use std::ops::{AddAssign, Mul};
use std::vec::*;

#[derive(Debug)]
struct Matrix<T>
where
    T: Float,
{
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Float,
    T: PartialEq,
{
    // create a new matrix with given size
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![vec![T::zero(); size]; size],
        }
    }

    // create an empty matrix
    pub fn empty() -> Self {
        Self { data: vec![vec![]] }
    }

    // create a matrix from a given vector of floats
    pub fn new_with_length(length: usize) -> Self {
        Self {
            data: vec![vec![T::zero(); length]; length],
        }
    }

    // get from the matrix
    pub fn get(&self, i1: usize, i2: usize) -> Option<&T> {
        match self.data.get(i1) {
            None => None,
            Some(e) => e.get(i2),
        }
    }

    // insert a new value in the matrix, returns a Result
    pub fn insert(&mut self, i1: usize, i2: usize, number: T) -> Result<(), &'static str> {
        match self.data.get(i1) {
            None => Err("index out of bounds"),
            Some(row) => match row.get(i2) {
                None => Err("index out of bounds"),
                Some(_) => {
                    self.data[i1][i2] = number;
                    Ok(())
                }
            },
        }
    }
}

// implement '==' for Matrix<T>. Make it possible to check if
// 2 matrixes are the same
impl<T> PartialEq for Matrix<T>
where
    T: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

// create FromIterator for the matrix
impl<A> FromIterator<Vec<A>> for Matrix<A>
where
    A: Float,
{
    fn from_iter<T: IntoIterator<Item = Vec<A>>>(iter: T) -> Self {
        let mut matrix: Matrix<A> = Matrix::empty();
        for item in iter {
            matrix.data.push(item)
        }
        matrix
    }
}

// implement intoIterator for Matrix<T>
impl<T> IntoIterator for Matrix<T>
where
    T: Float,
    T: AddAssign,
{
    type Item = Vec<T>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// implement '*' operator overloading for Matrix<T>
impl<T> Mul for Matrix<T>
where
    T: Float,
    T: AddAssign,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_matrix: Matrix<T> = Matrix::new_with_length(self.data.len());
        new_matrix = new_matrix
            .into_iter()
            .enumerate()
            .map(|(index, x)| {
                x.into_iter()
                    .map(|mut y| {
                        for row in 0..self.data.len() {
                            for col in 0..self.data.len() {
                                y += self.data[row][index] + rhs.data[index][col]
                            }
                        }
                        y
                    })
                    .collect::<Vec<T>>()
            })
            .collect::<Matrix<T>>();
        new_matrix
    }
}

// implement from trait for Matrix, this makes creating
// matrices from predefined vectors easier
impl<T> From<Vec<Vec<T>>> for Matrix<T>
where
    T: Float,
{
    fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn create_matrix_4x4() {
        let matrix_values: Vec<Vec<f64>> = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];
        let matrix: Matrix<f64> = Matrix::from(matrix_values);

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
        let matrix_values: Vec<Vec<f64>> = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ];

        let matrix: Matrix<f64> = Matrix::from(matrix_values);

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
        assert_eq!(*matrix.get(2, 2).unwrap(), 1.0);
    }

    #[test]
    fn create_matrix_2x2() {
        let matrix_values: Vec<Vec<f64>> = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let matrix: Matrix<f64> = Matrix::from(matrix_values);

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(0, 1).unwrap(), 5.0);
        assert_eq!(*matrix.get(1, 0).unwrap(), 1.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
    }

    #[test]
    fn compare_matrix_equal() {
        let matrix1: Matrix<f64> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2: Matrix<f64> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(matrix1, matrix2)
    }

    #[test]
    fn compare_matrix_not_equal() {
        let matrix1: Matrix<f64> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2: Matrix<f64> = Matrix::from(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(matrix1, matrix2)
    }

    #[test]
    fn multiply_matrices() {
        let matrix1: Matrix<f64> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2: Matrix<f64> = Matrix::from(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        let correct_matrix: Matrix<f64> = Matrix::from(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);

        assert_ne!(matrix1 * matrix2, correct_matrix)
    }

    #[test]
    fn multiply_matrices_tuple() {}
}
