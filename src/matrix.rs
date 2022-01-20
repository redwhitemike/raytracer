use crate::float_service::compare_floats;
use crate::Tuple;
use num::Float;
use std::array::IntoIter;
use std::ops::{AddAssign, Index, IndexMut, Mul};

#[derive(Debug, Clone)]
struct Matrix<T, const N: usize>
where
    T: Float,
    T: Clone,
{
    data: [[T; N]; N],
}

impl<T, const N: usize> Matrix<T, N>
where
    T: Float,
{
    // create a new matrix with given size
    pub fn new() -> Self {
        Self {
            data: [[T::zero(); N]; N],
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
                    self[i1][i2] = number;
                    Ok(())
                }
            },
        }
    }

    // return a identity matrix
    pub fn identity_matrix() -> Matrix<T, N> {
        Matrix::from(vec![
            vec![T::one(), T::zero(), T::zero(), T::zero()],
            vec![T::zero(), T::one(), T::zero(), T::zero()],
            vec![T::zero(), T::zero(), T::one(), T::zero()],
            vec![T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }

    // return a transposed version of the matrix
    pub fn transpose(&self) -> Matrix<T, N> {
        let mut new_matrix = Matrix::<T, N>::new();
        for row in 0..N {
            for col in 0..N {
                new_matrix[col][row] = self[row][col]
            }
        }
        new_matrix
    }
}

// functions for Matrix with a constant size of 4
impl<T> Matrix<T, 4>
where
    T: Float,
    T: AddAssign,
{
    // create a sub matrix of size 3x3, by deleting the given row and col
    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix<T, 3> {
        let mut new_matrix = Matrix::<T, 3>::new();
        let mut matrix_row: usize = 0;
        for x in 0..4 {
            if x != row {
                let mut matrix_col: usize = 0;
                for y in 0..4 {
                    if y != col {
                        new_matrix[matrix_row][matrix_col] = self[x][y];
                        matrix_col += 1
                    }
                }
                matrix_row += 1
            }
        }
        new_matrix
    }

    // a minor of an element at row i and column j is the determinant of the sub
    // matrix at (i,j)
    pub fn minor(&self, row: usize, col: usize) -> T {
        self.sub_matrix(row, col).determinant()
    }

    // return the cofactor at the given row and column
    // cofactor are minors but are maybe negated based on
    // (row, col) position
    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let cofactor = self.minor(row, col);

        match (row + col) % 2 == 0 {
            true => cofactor,
            false => -cofactor,
        }
    }

    // calculate determinant of matrix size 4
    pub fn determinant(&self) -> T {
        let mut det = T::zero();
        for col in 0..4 {
            det += self[0][col] * self.cofactor(0, col)
        }
        det
    }

    // check if the determinant is not equal to 0
    // if that is the case the matrix is invertible
    pub fn invertible(&self) -> bool {
        self.determinant() != T::zero()
    }

    // return the inverse matrix of the current matrix
    pub fn inverse(&self) -> Result<Matrix<T, 4>, &'static str> {
        match self.invertible() {
            true => {
                let mut new_matrix = Matrix::<T, 4>::new();
                self.data.iter().enumerate().for_each(|(row, row_vec)| {
                    row_vec.iter().enumerate().for_each(|(col, _)| {
                        let cofactor = self.cofactor(row, col);
                        new_matrix[col][row] = cofactor / self.determinant()
                    })
                });
                Ok(new_matrix)
            }
            false => Err("index out of bounds"),
        }
    }
}

// functions for Matrix with a constant size of 3x3
impl<T> Matrix<T, 3>
where
    T: Float,
    T: AddAssign,
{
    // create a sub matrix of size 2x2, by deleting the given row and col.
    pub fn sub_matrix(&self, row: usize, col: usize) -> Matrix<T, 2> {
        let mut new_matrix = Matrix::<T, 2>::new();
        let mut matrix_row: usize = 0;
        for x in 0..3 {
            if x != row {
                let mut matrix_col: usize = 0;
                for y in 0..3 {
                    if y != col {
                        new_matrix[matrix_row][matrix_col] = self[x][y];
                        matrix_col += 1
                    }
                }
                matrix_row += 1
            }
        }
        new_matrix
    }

    // a minor of an element at row i and column j is the determinant of the sub
    // matrix at (i,j)
    pub fn minor(&self, row: usize, col: usize) -> T {
        self.sub_matrix(row, col).determinant()
    }

    // return the cofactor at the given row and column
    // cofactor are minors but are maybe negated based on
    // (row, col) position
    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let cofactor = self.minor(row, col);

        match (row + col) % 2 == 0 {
            true => cofactor,
            false => -cofactor,
        }
    }

    // calculate determinant of matrix size 3
    pub fn determinant(&self) -> T {
        let mut det = T::zero();
        for col in 0..3 {
            det += self[0][col] * self.cofactor(0, col)
        }
        det
    }
}

impl<T> Matrix<T, 2>
where
    T: Float,
{
    // return the determinant of a matrix that is 2x2
    pub fn determinant(&self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}
// implement '==' for Matrix<T>. Make it possible to check if
// 2 matrices are the same
impl<T, const N: usize> PartialEq for Matrix<T, N>
where
    T: Float,
{
    fn eq(&self, other: &Self) -> bool {
        for row in 0..N {
            for col in 0..N {
                match compare_floats(self[row][col], other[row][col]) {
                    true => {} // do nothing
                    false => return false,
                }
            }
        }
        true
    }
}

// create FromIterator for the matrix
impl<A, const N: usize> FromIterator<Vec<A>> for Matrix<A, N>
where
    A: Float,
{
    fn from_iter<T: IntoIterator<Item = Vec<A>>>(iter: T) -> Self {
        let mut matrix: Matrix<A, N> = Matrix::new();
        let mut matrix_row = 0;

        for row in iter {
            let mut matrix_col = 0;
            for col in row {
                matrix[matrix_row][matrix_col] = col;
                matrix_col += 1
            }
            matrix_row += 1
        }
        matrix
    }
}

// implement trait Index for Matrix
// makes indexing without matrix.data possible
impl<T, const N: usize> Index<usize> for Matrix<T, N>
where
    T: Float,
{
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// implement trait IndexMut for Matrix
// makes indexing without matrix.data possible
impl<T, const N: usize> IndexMut<usize> for Matrix<T, N>
where
    T: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// implement intoIterator for Matrix<T>
impl<T, const N: usize> IntoIterator for Matrix<T, N>
where
    T: Float,
    T: AddAssign,
{
    type Item = [T; N];
    type IntoIter = IntoIter<Self::Item, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// implement '*' operator overloading for Matrix<T>
impl<T, const N: usize> Mul for Matrix<T, N>
where
    T: Float,
    T: AddAssign,
{
    type Output = Matrix<T, N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_matrix: Matrix<T, N> = Matrix::new();
        for row in 0..N {
            for col in 0..N {
                for index in 0..N {
                    new_matrix[row][col] += self[row][index] * rhs[index][col]
                }
            }
        }
        new_matrix
    }
}

// implement multiplication between a Matrix<T, 4> and Tuple<T>
impl<T, const N: usize> Mul<Tuple<T>> for Matrix<T, N>
where
    T: Float,
    T: AddAssign,
{
    type Output = Tuple<T>;

    fn mul(self, rhs: Tuple<T>) -> Tuple<T> {
        let mut tuple = rhs.clone();
        for i in 0..4 {
            let mut right_number = T::zero();
            for y in 0..4 {
                right_number += self[i][y] * rhs[y]
            }
            tuple[i] = right_number
        }
        tuple
    }
}

// implement from trait for Matrix, this makes creating
// matrices from predefined vectors easier
impl<T, const N: usize> From<Vec<Vec<T>>> for Matrix<T, N>
where
    T: Float,
{
    fn from(data: Vec<Vec<T>>) -> Self {
        match data.len() == N {
            true => {
                let mut new_matrix = Matrix::<T, N>::new();

                for x in 0..N {
                    for y in 0..N {
                        new_matrix[x][y] = data[x][y]
                    }
                }
                new_matrix
            }
            false => {
                panic!(
                    "tried to convert vec with length {} into array of length {}",
                    data.len(),
                    N
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::Tuple;

    #[test]
    fn create_matrix_4x4() {
        let matrix_values: Vec<Vec<f64>> = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];
        let matrix: Matrix<f64, 4> = Matrix::from(matrix_values);

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

        let matrix: Matrix<f64, 3> = Matrix::from(matrix_values);

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
        assert_eq!(*matrix.get(2, 2).unwrap(), 1.0);
    }

    #[test]
    fn create_matrix_2x2() {
        let matrix_values: Vec<Vec<f64>> = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let matrix: Matrix<f64, 2> = Matrix::from(matrix_values);

        assert_eq!(*matrix.get(0, 0).unwrap(), -3.0);
        assert_eq!(*matrix.get(0, 1).unwrap(), 5.0);
        assert_eq!(*matrix.get(1, 0).unwrap(), 1.0);
        assert_eq!(*matrix.get(1, 1).unwrap(), -2.0);
    }

    #[test]
    fn insert_matrix() {
        let mut matrix: Matrix<f64, 4> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        match matrix.insert(0, 0, 0.0) {
            Ok(_) => {
                assert_eq!(matrix.data[0][0], 0.0)
            }
            Err(_) => {
                assert_eq!(false, true)
            }
        }
    }

    #[test]
    fn compare_matrix_equal() {
        let matrix1: Matrix<f64, 4> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2: Matrix<f64, 4> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(matrix1, matrix2)
    }

    #[test]
    fn compare_matrix_not_equal() {
        let matrix1: Matrix<f64, 4> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2: Matrix<f64, 4> = Matrix::from(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(matrix1, matrix2)
    }

    #[test]
    fn multiply_matrices() {
        let matrix1: Matrix<f64, 4> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2: Matrix<f64, 4> = Matrix::from(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        let correct_matrix: Matrix<f64, 4> = Matrix::from(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(matrix1 * matrix2, correct_matrix)
    }

    #[test]
    fn multiply_matrices_tuple() {
        let matrix: Matrix<f64, 4> = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let tuple: Tuple<f64> = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let correct_tuple: Tuple<f64> = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(matrix * tuple, correct_tuple)
    }

    #[test]
    fn identity_matrices() {
        let matrix: Matrix<f64, 4> = Matrix::from(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);

        let check = matrix.clone();
        assert_eq!(matrix * Matrix::<f64, 4>::identity_matrix(), check)
    }

    #[test]
    fn transpose_matrix() {
        let matrix: Matrix<f64, 4> = Matrix::from(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);

        let transposed_matrix: Matrix<f64, 4> = Matrix::from(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(matrix.transpose(), transposed_matrix)
    }

    #[test]
    fn transpose_identity_matrix() {
        let identity_matrix = Matrix::<f64, 4>::identity_matrix();

        assert_eq!(identity_matrix.transpose(), identity_matrix)
    }

    #[test]
    fn calculate_determinant() {
        let matrix = Matrix::<f64, 2>::from(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        assert_eq!(matrix.determinant(), 17.0)
    }

    #[test]
    fn sub_matrix_3x3() {
        let matrix = Matrix::<f64, 3>::from(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let sub_matrix = Matrix::<f64, 2>::from(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);
        assert_eq!(matrix.sub_matrix(0, 2), sub_matrix)
    }

    #[test]
    fn sub_matrix_4x4() {
        let matrix = Matrix::<f64, 4>::from(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);

        let sub_matrix = Matrix::<f64, 3>::from(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);

        assert_eq!(matrix.sub_matrix(2, 1), sub_matrix)
    }

    #[test]
    fn minor_3x3() {
        let matrix = Matrix::<f64, 3>::from(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        let sub_matrix = matrix.sub_matrix(1, 0);

        assert_eq!(matrix.minor(1, 0), 25.0);
        assert_eq!(matrix.minor(1, 0), sub_matrix.determinant())
    }

    #[test]
    fn cofactor_3x3() {
        let matrix = Matrix::<f64, 3>::from(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.minor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
        assert_eq!(matrix.minor(1, 0), 25.0)
    }

    #[test]
    fn determinant_3x3() {
        let matrix = Matrix::<f64, 3>::from(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);

        assert_eq!(matrix.determinant(), -196.0)
    }

    #[test]
    fn determinant_4x4() {
        let matrix = Matrix::<f64, 4>::from(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(matrix.determinant(), -4071.0)
    }

    #[test]
    fn is_invertible() {
        let matrix = Matrix::<f64, 4>::from(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);

        assert_eq!(matrix.determinant(), -2120.0);
        assert_eq!(matrix.invertible(), true)
    }

    #[test]
    fn is_not_invertible() {
        let matrix = Matrix::<f64, 4>::from(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(matrix.determinant(), 0.0);
        assert_eq!(matrix.invertible(), false)
    }

    #[test]
    fn inverse_1() {
        let matrix = Matrix::<f64, 4>::from(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);

        let mut inverse_matrix = Matrix::<f64, 4>::new();
        match matrix.inverse() {
            Ok(e) => inverse_matrix = e,
            Err(_) => {
                assert_eq!(true, false)
            }
        };

        let correct_inverse_matrix = Matrix::<f64, 4>::from(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_eq!(matrix.determinant(), 532.0);
        assert_eq!(matrix.cofactor(2, 3), -160.0);
        assert_eq!(inverse_matrix[3][2], -160.0 / 532.0);
        assert_eq!(matrix.cofactor(3, 2), 105.0);
        assert_eq!(inverse_matrix[2][3], 105.0 / 532.0);
        assert_eq!(inverse_matrix, correct_inverse_matrix);
    }

    #[test]
    fn inverse_2() {
        let matrix = Matrix::<f64, 4>::from(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);

        let mut inverse_matrix = Matrix::<f64, 4>::new();

        match matrix.inverse() {
            Ok(e) => inverse_matrix = e,
            Err(_) => {
                assert_eq!(true, false)
            }
        }

        let correct_inverse_matrix = Matrix::<f64, 4>::from(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(inverse_matrix, correct_inverse_matrix)
    }

    #[test]
    fn inverse_3() {
        let matrix = Matrix::<f64, 4>::from(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);

        let mut inverse_matrix = Matrix::<f64, 4>::new();
        match matrix.inverse() {
            Ok(e) => inverse_matrix = e,
            Err(_) => {
                assert_eq!(true, false)
            }
        }

        let correct_inverse_matrix = Matrix::<f64, 4>::from(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.029011, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(inverse_matrix, correct_inverse_matrix)
    }

    #[test]
    fn multiply_product_by_inverse() {
        let matrix_a = Matrix::<f64, 4>::from(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);

        let matrix_b = Matrix::<f64, 4>::from(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);

        let matrix_c = matrix_a.clone() * matrix_b.clone();

        let mut inverse_matrix_b = Matrix::<f64, 4>::new();

        match matrix_b.inverse() {
            Ok(e) => inverse_matrix_b = e,
            Err(_) => {
                assert_eq!(true, false)
            }
        }

        assert_eq!(matrix_c * inverse_matrix_b, matrix_a)
    }
}