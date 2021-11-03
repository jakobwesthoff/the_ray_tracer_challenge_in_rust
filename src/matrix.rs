use crate::F;
use std::convert::From;
use std::ops::{Index, IndexMut, Mul};

use crate::fuzzy_eq::*;
use crate::tuple::*;

// @TODO: Maybe refactor to utilize one Matrix struct in the future.
//        Are const template parameters an option?
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<const D: usize> {
  data: [[F; D]; D],
}

impl<const D: usize> From<[[F; D]; D]> for Matrix<D> {
  fn from(data: [[F; D]; D]) -> Self {
    Matrix { data }
  }
}

impl<const D: usize> Default for Matrix<D> {
  fn default() -> Self {
    Self::new()
  }
}

impl<const D: usize> Matrix<D> {
  pub fn new() -> Matrix<D> {
    Matrix::from([[0.0; D]; D])
  }

  pub fn diagonal(value: F) -> Matrix<D> {
    let mut matrix = Matrix::new();
    for i in 0..D {
      matrix[i][i] = value;
    }
    matrix
  }

  pub fn identity() -> Matrix<D> {
    Matrix::diagonal(1.0)
  }

  pub fn transpose(&self) -> Matrix<D> {
    let mut matrix = Matrix::new();
    for row in 0..D {
      for column in 0..D {
        matrix[column][row] = self.data[row][column];
      }
    }
    matrix
  }
}

impl<const D: usize> Index<usize> for Matrix<D> {
  type Output = [F; D];

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.data[index]
  }
}

impl<const D: usize> FuzzyEq<Self> for Matrix<D> {
  fn fuzzy_eq(&self, other: Self) -> bool {
    for row in 0..D {
      for column in 0..D {
        if self[row][column].fuzzy_ne(other[row][column]) {
          return false;
        }
      }
    }

    true
  }
}

impl<const D: usize> Mul<Matrix<D>> for Matrix<D> {
  type Output = Matrix<D>;

  fn mul(self, other: Matrix<D>) -> Self::Output {
    let mut matrix = Matrix::new();

    for row in 0..D {
      for column in 0..D {
        for i in 0..D {
          matrix[row][column] += self[row][i] * other[i][column];
        }
      }
    }
    matrix
  }
}

impl Matrix<2> {
  pub fn determinant(&self) -> F {
    self[0][0] * self[1][1] - self[0][1] * self[1][0]
  }
}

impl Matrix<3> {
  // @FIXME: Find a nicer way to do this.
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix<2> {
    let mut matrix: Matrix<2> = Matrix::new();
    let mut source_row: usize = 0;
    let mut source_column: usize = 0;
    let mut target_row: usize = 0;
    let mut target_column: usize = 0;

    while target_row < 2 {
      if source_row == row {
        // Skip row to be removed
        source_row += 1;
      }
      while target_column < 2 {
        if source_column == column {
          // Skip column to be removed
          source_column += 1;
        }
        matrix[target_row][target_column] = self[source_row][source_column];

        source_column += 1;
        target_column += 1;
      }
      source_row += 1;
      source_column = 0;
      target_row += 1;
      target_column = 0;
    }

    matrix
  }

  pub fn minor(&self, row: usize, column: usize) -> F {
    self.submatrix(row, column).determinant()
  }

  pub fn cofactor(&self, row: usize, column: usize) -> F {
    let minor = self.minor(row, column);
    if (row + column) % 2 == 0 {
      // Even value
      minor
    } else {
      -minor
    }
  }

  pub fn determinant(&self) -> F {
    let mut determinant: F = 0.0;
    for column in 0..3 {
      determinant += self.cofactor(0, column) * self[0][column];
    }

    determinant
  }
}

impl Mul<Tuple> for Matrix<4> {
  type Output = Tuple;

  fn mul(self, other: Tuple) -> Self::Output {
    Tuple::new(
      self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z + self[0][3] * other.w,
      self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z + self[1][3] * other.w,
      self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z + self[2][3] * other.w,
      self[3][0] * other.x + self[3][1] * other.y + self[3][2] * other.z + self[3][3] * other.w,
    )
  }
}

impl Matrix<4> {
  // @FIXME: Find a nicer way to do this.
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix<3> {
    let mut matrix = Matrix::new();
    let mut source_row: usize = 0;
    let mut source_column: usize = 0;
    let mut target_row: usize = 0;
    let mut target_column: usize = 0;

    while target_row < 3 {
      if source_row == row {
        // Skip row to be removed
        source_row += 1;
      }
      while target_column < 3 {
        if source_column == column {
          // Skip column to be removed
          source_column += 1;
        }
        matrix[target_row][target_column] = self[source_row][source_column];

        source_column += 1;
        target_column += 1;
      }
      source_row += 1;
      source_column = 0;
      target_row += 1;
      target_column = 0;
    }

    matrix
  }

  pub fn minor(&self, row: usize, column: usize) -> F {
    self.submatrix(row, column).determinant()
  }

  pub fn cofactor(&self, row: usize, column: usize) -> F {
    let minor = self.minor(row, column);
    if (row + column) % 2 == 0 {
      // Even value
      minor
    } else {
      -minor
    }
  }

  pub fn determinant(&self) -> F {
    let mut determinant: F = 0.0;
    for column in 0..4 {
      determinant += self.cofactor(0, column) * self[0][column];
    }

    determinant
  }

  pub fn is_invertible(&self) -> bool {
    self.determinant().fuzzy_ne(0.0)
  }

  pub fn inverse(&self) -> Matrix<4> {
    if !self.is_invertible() {
      panic!("Matrix is not invertible, but inverse was called!");
    }

    let mut matrix = Matrix::new();
    let determinant = self.determinant();

    for row in 0..4 {
      for column in 0..4 {
        let cofactor = self.cofactor(row, column);
        // transposed storage
        matrix[column][row] = cofactor / determinant;
      }
    }

    matrix
  }

  #[rustfmt::skip]
  pub fn translation(x: F, y: F, z: F) -> Matrix<4>
  {
    Matrix::from([
      [1.0, 0.0, 0.0,   x],
      [0.0, 1.0, 0.0,   y],
      [0.0, 0.0, 1.0,   z],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  #[rustfmt::skip]
  pub fn scaling(x: F, y: F, z: F) -> Matrix<4>
  {
    Matrix::from([
      [x,   0.0, 0.0, 0.0],
      [0.0,   y, 0.0, 0.0],
      [0.0, 0.0,   z, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  #[rustfmt::skip]
  pub fn rotation_x(r: F) -> Matrix<4>
  {
    Matrix::from([
      [1.0,     0.0,      0.0, 0.0],
      [0.0, r.cos(), -r.sin(), 0.0],
      [0.0, r.sin(),  r.cos(), 0.0],
      [0.0,     0.0,      0.0, 1.0],
    ])
  }

  #[rustfmt::skip]
  pub fn rotation_y(r: F) -> Matrix<4>
  {
    Matrix::from([
      [ r.cos(), 0.0, r.sin(), 0.0],
      [     0.0, 1.0,     0.0, 0.0],
      [-r.sin(), 0.0, r.cos(), 0.0],
      [     0.0, 0.0,     0.0, 1.0],
    ])
  }

  #[rustfmt::skip]
  pub fn rotation_z(r: F) -> Matrix<4>
  {
    Matrix::from([
      [r.cos(), -r.sin(), 0.0, 0.0],
      [r.sin(),  r.cos(), 0.0, 0.0],
      [    0.0,      0.0, 1.0, 0.0],
      [    0.0,      0.0, 0.0, 1.0],
    ])
  }

  #[rustfmt::skip]
  pub fn shearing(xy: F, xz: F, yx: F, yz: F, zx: F, zy: F) -> Matrix<4>
  {
    Matrix::from([
      [1.0,  xy,  xz, 0.0],
      [ yx, 1.0,  yz, 0.0],
      [ zx,  zy, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Self {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    #[rustfmt::skip]
    let orientation_transform = Matrix::from([
      [ left.x,     left.y,     left.z,    0.0],
      [ true_up.x,  true_up.y,  true_up.z, 0.0],
      [-forward.x, -forward.y, -forward.z, 0.0],
      [0.0,         0.0,        0.0,       1.0]
    ]);

    let translation_transform = Matrix::translation(-from.x, -from.y, -from.z);

    orientation_transform * translation_transform
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f64::consts::PI;

  #[test]
  fn constructing_and_inspecting_a_4x4_matrix() {
    let m = Matrix::from([
      [1.0, 2.0, 3.0, 4.0],
      [5.5, 6.5, 7.5, 8.5],
      [9.0, 10.0, 11.0, 12.0],
      [13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][1], 2.0);
    assert_eq!(m[0][2], 3.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][1], 6.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[1][3], 8.5);
    assert_eq!(m[2][0], 9.0);
    assert_eq!(m[2][1], 10.0);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[2][3], 12.0);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][1], 14.5);
    assert_eq!(m[3][2], 15.5);
    assert_eq!(m[3][3], 16.5);
  }

  #[test]
  fn a_2x2_matrix_ought_to_be_representable() {
    let m = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
  }

  #[test]
  fn a_3x3_matrix_ought_to_be_representable() {
    let m = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[0][2], 0.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
    assert_eq!(m[1][2], -7.0);
    assert_eq!(m[2][0], 0.0);
    assert_eq!(m[2][1], 1.0);
    assert_eq!(m[2][2], 1.0);
  }

  #[test]
  fn matrix_equality_with_identical_2x2_matrices() {
    let m1 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);
    let m2 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_almost_identical_2x2_matrices() {
    let m1 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);
    let m2 = Matrix::from([[0.123456780, 1.0], [2.0, 3.0]]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_identical_3x3_matrices() {
    let m1 = Matrix::from([
      [0.123456789, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777777],
    ]);

    let m2 = Matrix::from([
      [0.123456789, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777777],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_almost_identical_3x3_matrices() {
    let m1 = Matrix::from([
      [0.123456789, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777777],
    ]);
    let m2 = Matrix::from([
      [0.123456780, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_identical_4x4_matrices() {
    let m1 = Matrix::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);
    let m2 = Matrix::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_almost_identical_4x4_matrices() {
    let m1 = Matrix::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0000000000001],
    ]);
    let m2 = Matrix::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_inequality_with_non_identical_4x4_matrices() {
    let m1 = Matrix::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);
    let m2 = Matrix::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 2.0],
    ]);

    assert_fuzzy_ne!(m1, m2);
  }

  #[test]
  fn multiplying_two_4x4_matrices() {
    let m1 = Matrix::from([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0],
    ]);
    let m2 = Matrix::from([
      [-2.0, 1.0, 2.0, 3.0],
      [3.0, 2.0, 1.0, -1.0],
      [4.0, 3.0, 6.0, 5.0],
      [1.0, 2.0, 7.0, 8.0],
    ]);

    let expected_result = Matrix::from([
      [20.0, 22.0, 50.0, 48.0],
      [44.0, 54.0, 114.0, 108.0],
      [40.0, 58.0, 110.0, 102.0],
      [16.0, 26.0, 46.0, 42.0],
    ]);

    let actual_result = m1 * m2;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn a_4x4_matrix_multiplied_by_a_point() {
    let m = Matrix::from([
      [1.0, 2.0, 3.0, 4.0],
      [2.0, 4.0, 4.0, 2.0],
      [8.0, 6.0, 4.0, 1.0],
      [0.0, 0.0, 0.0, 1.0],
    ]);
    let p = Tuple::point(1.0, 2.0, 3.0);

    let expected_result = Tuple::point(18.0, 24.0, 33.0);

    let actual_result = m * p;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_a_4x4_matrix_by_the_identity_matrix() {
    let m1 = Matrix::from([
      [0.0, 1.0, 2.0, 4.0],
      [1.0, 2.0, 4.0, 8.0],
      [2.0, 4.0, 8.0, 16.0],
      [4.0, 8.0, 16.0, 32.0],
    ]);
    let m2 = Matrix::identity();

    let expected_result = m1;
    let actual_result = m1 * m2;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn transposing_a_4x4_matrix() {
    let m = Matrix::from([
      [0.0, 9.0, 3.0, 0.0],
      [9.0, 8.0, 0.0, 8.0],
      [1.0, 8.0, 5.0, 3.0],
      [0.0, 0.0, 5.0, 8.0],
    ]);

    let expected_result = Matrix::from([
      [0.0, 9.0, 1.0, 0.0],
      [9.0, 8.0, 8.0, 0.0],
      [3.0, 0.0, 5.0, 5.0],
      [0.0, 8.0, 3.0, 8.0],
    ]);

    let actual_result = m.transpose();

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn calculate_the_determinant_of_a_2x2_matrix() {
    let m = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);

    let expected_result = 17.0;

    let actual_result = m.determinant();

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    let m = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]]);

    let expected_result = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);

    let actual_result = m.submatrix(0, 2);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    let m = Matrix::from([
      [-6.0, 1.0, 1.0, 6.0],
      [-8.0, 5.0, 8.0, 6.0],
      [-1.0, 0.0, 8.0, 2.0],
      [-7.0, 1.0, -1.0, 1.0],
    ]);

    let expected_result = Matrix::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

    let actual_result = m.submatrix(2, 1);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn calculate_the_minor_of_a_3x3_matrix() {
    let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

    let sub = m.submatrix(1, 0);
    let determinant = sub.determinant();
    let minor = m.minor(1, 0);

    assert_fuzzy_eq!(25.0, determinant);
    assert_fuzzy_eq!(25.0, minor);
  }

  #[test]
  fn calculating_a_cofactor_of_a_3x3_matrix() {
    let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

    let minor1 = m.minor(0, 0);
    let minor2 = m.minor(1, 0);

    let cofactor1 = m.cofactor(0, 0);
    let cofactor2 = m.cofactor(1, 0);

    assert_fuzzy_eq!(-12.0, minor1);
    assert_fuzzy_eq!(-12.0, cofactor1);
    assert_fuzzy_eq!(25.0, minor2);
    assert_fuzzy_eq!(-25.0, cofactor2);
  }

  #[test]
  fn calculate_the_determinant_of_a_3x3_matrix() {
    let m = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

    let cofactor00 = m.cofactor(0, 0);
    let cofactor01 = m.cofactor(0, 1);
    let cofactor02 = m.cofactor(0, 2);

    let determinant = m.determinant();

    assert_fuzzy_eq!(56.0, cofactor00);
    assert_fuzzy_eq!(12.0, cofactor01);
    assert_fuzzy_eq!(-46.0, cofactor02);

    assert_fuzzy_eq!(-196.0, determinant);
  }

  #[test]
  fn calculating_the_determinant_of_a_4x4_matrix() {
    let m = Matrix::from([
      [-2.0, -8.0, 3.0, 5.0],
      [-3.0, 1.0, 7.0, 3.0],
      [1.0, 2.0, -9.0, 6.0],
      [-6.0, 7.0, 7.0, -9.0],
    ]);

    let cofactor00 = m.cofactor(0, 0);
    let cofactor01 = m.cofactor(0, 1);
    let cofactor02 = m.cofactor(0, 2);
    let cofactor03 = m.cofactor(0, 3);

    let determinant = m.determinant();

    assert_fuzzy_eq!(690.0, cofactor00);
    assert_fuzzy_eq!(447.0, cofactor01);
    assert_fuzzy_eq!(210.0, cofactor02);
    assert_fuzzy_eq!(51.0, cofactor03);

    assert_fuzzy_eq!(-4071.0, determinant);
  }

  #[test]
  fn testing_an_invertible_matrix_for_invertibility() {
    let m = Matrix::from([
      [6.0, 4.0, 4.0, 4.0],
      [5.0, 5.0, 7.0, 6.0],
      [4.0, -9.0, 3.0, -7.0],
      [9.0, 1.0, 7.0, -6.0],
    ]);

    let determinant = m.determinant();

    assert_fuzzy_eq!(-2120.0, determinant);
    assert!(m.is_invertible());
  }

  #[test]
  fn testing_an_noninvertible_matrix_for_invertibility() {
    let m = Matrix::from([
      [-4.0, 2.0, -2.0, -3.0],
      [9.0, 6.0, 2.0, 6.0],
      [0.0, -5.0, 1.0, -5.0],
      [0.0, 0.0, 0.0, 0.0],
    ]);

    let determinant = m.determinant();

    assert_fuzzy_eq!(0.0, determinant);
    assert!(!m.is_invertible());
  }

  #[test]
  fn calculating_the_inverse_of_a_4x4_matrix() {
    let m = Matrix::from([
      [-5.0, 2.0, 6.0, -8.0],
      [1.0, -5.0, 1.0, 8.0],
      [7.0, 7.0, -6.0, -7.0],
      [1.0, -3.0, 7.0, 4.0],
    ]);

    let determinant = m.determinant();
    let cofactor23 = m.cofactor(2, 3);
    let cofactor32 = m.cofactor(3, 2);

    let expected_result = Matrix::from([
      [0.21805, 0.45113, 0.24060, -0.04511],
      [-0.80827, -1.45677, -0.44361, 0.52068],
      [-0.07895, -0.22368, -0.05263, 0.19737],
      [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    let actual_result = m.inverse();

    assert_fuzzy_eq!(532.0, determinant);
    assert_fuzzy_eq!(-160.0, cofactor23);
    assert_fuzzy_eq!(-160.0 / 532.0, actual_result[3][2]);
    assert_fuzzy_eq!(105.0, cofactor32);
    assert_fuzzy_eq!(105.0 / 532.0, actual_result[2][3]);
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn calculating_the_inverse_of_another_4x4_matrix() {
    let m = Matrix::from([
      [8.0, -5.0, 9.0, 2.0],
      [7.0, 5.0, 6.0, 1.0],
      [-6.0, 0.0, 9.0, 6.0],
      [-3.0, 0.0, -9.0, -4.0],
    ]);

    let expected_result = Matrix::from([
      [-0.15385, -0.15385, -0.28205, -0.53846],
      [-0.07692, 0.12308, 0.02564, 0.03077],
      [0.35897, 0.35897, 0.43590, 0.92308],
      [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);

    let actual_result = m.inverse();

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn calculating_the_inverse_of_a_third_4x4_matrix() {
    let m = Matrix::from([
      [9.0, 3.0, 0.0, 9.0],
      [-5.0, -2.0, -6.0, -3.0],
      [-4.0, 9.0, 6.0, 4.0],
      [-7.0, 6.0, 6.0, 2.0],
    ]);

    let expected_result = Matrix::from([
      [-0.04074, -0.07778, 0.14444, -0.22222],
      [-0.07778, 0.03333, 0.36667, -0.33333],
      [-0.02901, -0.14630, -0.10926, 0.12963],
      [0.17778, 0.06667, -0.26667, 0.33333],
    ]);

    let actual_result = m.inverse();

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_a_product_by_its_inverse() {
    let m1 = Matrix::from([
      [3.0, -9.0, 7.0, 3.0],
      [3.0, -8.0, 2.0, -9.0],
      [-4.0, 4.0, 4.0, 1.0],
      [-6.0, 5.0, -1.0, 1.0],
    ]);

    let m2 = Matrix::from([
      [8.0, 2.0, 2.0, 2.0],
      [3.0, -1.0, 7.0, 0.0],
      [7.0, 0.0, 5.0, 4.0],
      [6.0, -2.0, 0.0, 5.0],
    ]);

    let m3 = m1 * m2;

    let actual_result = m3 * m2.inverse();

    assert_fuzzy_eq!(actual_result, m1);
  }

  #[test]
  fn multiplying_by_a_translation_matrix() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let expected_result = Tuple::point(2.0, 1.0, 7.0);

    let actual_result = transform * p;
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_by_the_inverse_of_a_translation_matrix() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let inverse_transform = transform.inverse();
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let expected_result = Tuple::point(-8.0, 7.0, 3.0);

    let actual_result = inverse_transform * p;
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn translation_does_not_affect_vectors() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let v = Tuple::vector(-3.0, 4.0, 5.0);
    let expected_result = v;

    let actual_result = transform * v;
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn a_scaling_matrix_applied_to_a_point() {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let p = Tuple::point(-4.0, 6.0, 8.0);
    let expected_result = Tuple::point(-8.0, 18.0, 32.0);

    let actual_result = transform * p;
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn a_scaling_matrix_applied_to_a_vector() {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    let expected_result = Tuple::vector(-8.0, 18.0, 32.0);

    let actual_result = transform * v;
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_by_the_inverse_of_a_scaling_matrix() {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let inverse_transform = transform.inverse();
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    let expected_result = Tuple::vector(-2.0, 2.0, 2.0);

    let actual_result = inverse_transform * v;
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn reflection_is_scaling_by_a_negative_value() {
    let transform = Matrix::scaling(-1.0, 1.0, 1.0);
    let p = Tuple::point(2.0, 3.0, 4.0);
    let expected_result = Tuple::point(-2.0, 3.0, 4.0);

    let actual_result = transform * p;
    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn rotating_a_point_around_the_x_axis() {
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let full_quarter = Matrix::rotation_x(PI / 2.0);
    let p = Tuple::point(0.0, 1.0, 0.0);

    assert_fuzzy_eq!(
      half_quarter * p,
      Tuple::point(0.0, (2.0 as F).sqrt() / 2.0, (2.0 as F).sqrt() / 2.0)
    );

    assert_fuzzy_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
  }

  #[test]
  fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let full_quarter = Matrix::rotation_x(PI / 2.0);
    let inverse_half_quarter = half_quarter.inverse();
    let inverse_full_quarter = full_quarter.inverse();

    let p = Tuple::point(0.0, 1.0, 0.0);

    assert_fuzzy_eq!(
      inverse_half_quarter * p,
      Tuple::point(0.0, (2.0 as F).sqrt() / 2.0, -(2.0 as F).sqrt() / 2.0)
    );

    assert_fuzzy_eq!(inverse_full_quarter * p, Tuple::point(0.0, 0.0, -1.0));
  }

  #[test]
  fn rotating_a_point_around_the_y_axis() {
    let half_quarter = Matrix::rotation_y(PI / 4.0);
    let full_quarter = Matrix::rotation_y(PI / 2.0);
    let p = Tuple::point(0.0, 0.0, 1.0);

    assert_fuzzy_eq!(
      half_quarter * p,
      Tuple::point((2.0 as F).sqrt() / 2.0, 0.0, (2.0 as F).sqrt() / 2.0)
    );

    assert_fuzzy_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
  }

  #[test]
  fn rotating_a_point_around_the_z_axis() {
    let half_quarter = Matrix::rotation_z(PI / 4.0);
    let full_quarter = Matrix::rotation_z(PI / 2.0);
    let p = Tuple::point(0.0, 1.0, 0.0);

    assert_fuzzy_eq!(
      half_quarter * p,
      Tuple::point(-(2.0 as F).sqrt() / 2.0, (2.0 as F).sqrt() / 2.0, 0.0)
    );

    assert_fuzzy_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
  }

  #[test]
  fn a_shearing_transformation_moves_x_in_proportion_to_y() {
    let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = Tuple::point(2.0, 3.0, 4.0);

    assert_fuzzy_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
  }

  #[test]
  fn a_shearing_transformation_moves_x_in_proportion_to_z() {
    let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = Tuple::point(2.0, 3.0, 4.0);

    assert_fuzzy_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
  }

  #[test]
  fn a_shearing_transformation_moves_y_in_proportion_to_x() {
    let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = Tuple::point(2.0, 3.0, 4.0);

    assert_fuzzy_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
  }

  #[test]
  fn a_shearing_transformation_moves_y_in_proportion_to_z() {
    let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = Tuple::point(2.0, 3.0, 4.0);

    assert_fuzzy_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
  }

  #[test]
  fn a_shearing_transformation_moves_z_in_proportion_to_x() {
    let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = Tuple::point(2.0, 3.0, 4.0);

    assert_fuzzy_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
  }

  #[test]
  fn a_shearing_transformation_moves_z_in_proportion_to_y() {
    let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = Tuple::point(2.0, 3.0, 4.0);

    assert_fuzzy_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
  }

  #[test]
  fn individual_transformation_are_applied_in_sequence() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let a = Matrix::rotation_x(PI / 2.0);
    let b = Matrix::scaling(5.0, 5.0, 5.0);
    let c = Matrix::translation(10.0, 5.0, 7.0);

    let p2 = a * p;
    assert_fuzzy_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

    let p3 = b * p2;
    assert_fuzzy_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

    let p4 = c * p3;
    assert_fuzzy_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
  }

  #[test]
  fn chained_transformations_must_be_applied_in_reverse_order() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let a = Matrix::rotation_x(PI / 2.0);
    let b = Matrix::scaling(5.0, 5.0, 5.0);
    let c = Matrix::translation(10.0, 5.0, 7.0);

    let transform = c * b * a;
    assert_fuzzy_eq!(transform * p, Tuple::point(15.0, 0.0, 7.0));
  }

  #[test]
  fn view_transform_for_the_default_orientation() {
    let from = Tuple::point(0.0, 0.0, 0.0);
    let to = Tuple::point(0.0, 0.0, -1.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    let matrix = Matrix::view_transform(from, to, up);
    assert_fuzzy_eq!(matrix, Matrix::identity());
  }

  #[test]
  fn view_transformation_looking_into_positive_z_direction() {
    let from = Tuple::point(0.0, 0.0, 0.0);
    let to = Tuple::point(0.0, 0.0, 1.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    let matrix = Matrix::view_transform(from, to, up);
    assert_fuzzy_eq!(matrix, Matrix::scaling(-1.0, 1.0, -1.0));
  }

  #[test]
  fn view_transformation_moves_the_world() {
    let from = Tuple::point(0.0, 0.0, 8.0);
    let to = Tuple::point(0.0, 0.0, 0.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    let matrix = Matrix::view_transform(from, to, up);
    assert_fuzzy_eq!(matrix, Matrix::translation(0.0, 0.0, -8.0));
  }

  #[test]
  fn an_arbitrary_view_transformation() {
    let from = Tuple::point(1.0, 3.0, 2.0);
    let to = Tuple::point(4.0, -2.0, 8.0);
    let up = Tuple::vector(1.0, 1.0, 0.0);
    let matrix = Matrix::view_transform(from, to, up);
    assert_fuzzy_eq!(
      matrix,
      Matrix::from([
        [-0.50709, 0.50709, 0.67612, -2.36643],
        [0.76772, 0.60609, 0.12122, -2.82843],
        [-0.35857, 0.59761, -0.71714, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ])
    );
  }
}
