use std::convert::From;
use std::ops::{Index, IndexMut, Mul};

use crate::fuzzy_eq::*;
use crate::tuple::*;

type Matrix2fArrayRow = [f64; 2];
type Matrix2fArray = [Matrix2fArrayRow; 2];
type Matrix3fArrayRow = [f64; 3];
type Matrix3fArray = [Matrix3fArrayRow; 3];
type Matrix4fArrayRow = [f64; 4];
type Matrix4fArray = [Matrix4fArrayRow; 4];

// @TODO: Maybe refactor to utilize one Matrix struct in the future.
//        Are const template parameters an option?
#[derive(Debug, Copy, Clone)]
pub struct Matrix2f {
  data: Matrix2fArray,
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix3f {
  data: Matrix3fArray,
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix4f {
  data: Matrix4fArray,
}

impl From<Matrix2fArray> for Matrix2f {
  fn from(data: Matrix2fArray) -> Self {
    Matrix2f { data }
  }
}

impl From<Matrix3fArray> for Matrix3f {
  fn from(data: Matrix3fArray) -> Self {
    Matrix3f { data }
  }
}

impl From<Matrix4fArray> for Matrix4f {
  fn from(data: Matrix4fArray) -> Self {
    Matrix4f { data }
  }
}

impl Matrix4f {
  pub fn new() -> Matrix4f {
    Matrix4f::from([
      [0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0],
    ])
  }

  pub fn identity() -> Matrix4f {
    Matrix4f::from([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }
}

impl Matrix3f {
  pub fn new() -> Matrix3f {
    Matrix3f::from([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
  }
}

impl Matrix2f {
  pub fn new() -> Matrix2f {
    Matrix2f::from([[0.0, 0.0], [0.0, 0.0]])
  }
}

impl Index<usize> for Matrix2f {
  type Output = Matrix2fArrayRow;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl Index<usize> for Matrix3f {
  type Output = Matrix3fArrayRow;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl Index<usize> for Matrix4f {
  type Output = Matrix4fArrayRow;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl IndexMut<usize> for Matrix2f {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.data[index]
  }
}

impl IndexMut<usize> for Matrix3f {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.data[index]
  }
}

impl IndexMut<usize> for Matrix4f {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.data[index]
  }
}

impl FuzzyEq<Matrix2f> for Matrix2f {
  fn fuzzy_eq(&self, other: &Matrix2f) -> bool {
    self[0][0].fuzzy_eq(&other[0][0])
      && self[0][1].fuzzy_eq(&other[0][1])
      && self[1][0].fuzzy_eq(&other[1][0])
      && self[1][1].fuzzy_eq(&other[1][1])
  }
}

impl FuzzyEq<Matrix3f> for Matrix3f {
  fn fuzzy_eq(&self, other: &Matrix3f) -> bool {
    self[0][0].fuzzy_eq(&other[0][0])
      && self[0][1].fuzzy_eq(&other[0][1])
      && self[0][2].fuzzy_eq(&other[0][2])
      && self[1][0].fuzzy_eq(&other[1][0])
      && self[1][1].fuzzy_eq(&other[1][1])
      && self[1][2].fuzzy_eq(&other[1][2])
      && self[2][0].fuzzy_eq(&other[2][0])
      && self[2][1].fuzzy_eq(&other[2][1])
      && self[2][2].fuzzy_eq(&other[2][2])
  }
}

impl FuzzyEq<Matrix4f> for Matrix4f {
  fn fuzzy_eq(&self, other: &Matrix4f) -> bool {
    self[0][0].fuzzy_eq(&other[0][0])
      && self[0][1].fuzzy_eq(&other[0][1])
      && self[0][2].fuzzy_eq(&other[0][2])
      && self[0][3].fuzzy_eq(&other[0][3])
      && self[1][0].fuzzy_eq(&other[1][0])
      && self[1][1].fuzzy_eq(&other[1][1])
      && self[1][2].fuzzy_eq(&other[1][2])
      && self[1][3].fuzzy_eq(&other[1][3])
      && self[2][0].fuzzy_eq(&other[2][0])
      && self[2][1].fuzzy_eq(&other[2][1])
      && self[2][2].fuzzy_eq(&other[2][2])
      && self[2][3].fuzzy_eq(&other[2][3])
      && self[3][0].fuzzy_eq(&other[3][0])
      && self[3][1].fuzzy_eq(&other[3][1])
      && self[3][2].fuzzy_eq(&other[3][2])
      && self[3][3].fuzzy_eq(&other[3][3])
  }
}

impl Mul<Matrix4f> for Matrix4f {
  type Output = Matrix4f;

  fn mul(self, other: Matrix4f) -> Self::Output {
    let mut m = Matrix4f::new();

    for row in 0..4 {
      for column in 0..4 {
        m[row][column] = self[row][0] * other[0][column]
          + self[row][1] * other[1][column]
          + self[row][2] * other[2][column]
          + self[row][3] * other[3][column];
      }
    }
    return m;
  }
}

impl Mul<Tuple> for Matrix4f {
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

impl Matrix2f {
  pub fn determinant(&self) -> f64 {
    self[0][0] * self[1][1] - self[0][1] * self[1][0]
  }
}

impl Matrix3f {
  // @FIXME: Find a nicer way to do this.
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix2f {
    let mut m = Matrix2f::new();
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
        m[target_row][target_column] = self[source_row][source_column];

        source_column += 1;
        target_column += 1;
      }
      source_row += 1;
      source_column = 0;
      target_row += 1;
      target_column = 0;
    }

    return m;
  }

  pub fn minor(&self, row: usize, column: usize) -> f64 {
    self.submatrix(row, column).determinant()
  }

  pub fn cofactor(&self, row: usize, column: usize) -> f64 {
    let minor = self.minor(row, column);
    if (row + column) % 2 == 0 {
      // Even value
      return minor;
    } else {
      return -minor;
    }
  }

  pub fn determinant(&self) -> f64 {
    let mut determinant: f64 = 0.0;
    for column in 0..3 {
      determinant += self.cofactor(0, column) * self[0][column];
    }

    determinant
  }
}

impl Matrix4f {
  pub fn transpose(&self) -> Matrix4f {
    let mut m = Matrix4f::new();
    for row in 0..4 {
      for column in 0..4 {
        m[column][row] = self[row][column];
      }
    }
    return m;
  }

  // @FIXME: Find a nicer way to do this.
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix3f {
    let mut m = Matrix3f::new();
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
        m[target_row][target_column] = self[source_row][source_column];

        source_column += 1;
        target_column += 1;
      }
      source_row += 1;
      source_column = 0;
      target_row += 1;
      target_column = 0;
    }

    return m;
  }

  pub fn minor(&self, row: usize, column: usize) -> f64 {
    self.submatrix(row, column).determinant()
  }

  pub fn cofactor(&self, row: usize, column: usize) -> f64 {
    let minor = self.minor(row, column);
    if (row + column) % 2 == 0 {
      // Even value
      return minor;
    } else {
      return -minor;
    }
  }

  pub fn determinant(&self) -> f64 {
    let mut determinant: f64 = 0.0;
    for column in 0..4 {
      determinant += self.cofactor(0, column) * self[0][column];
    }

    determinant
  }

  pub fn is_invertible(&self) -> bool {
    self.determinant().fuzzy_ne(&0.0)
  }

  pub fn inverse(&self) -> Matrix4f {
    if !self.is_invertible() {
      panic!("Matrix is not invertible, but inverse was called!");
    }

    let mut m = Matrix4f::new();
    let determinant = self.determinant();

    for row in 0..4 {
      for column in 0..4 {
        let cofactor = self.cofactor(row, column);
        // transposed storage
        m[column][row] = cofactor / determinant;
      }
    }

    return m;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn constructing_and_inspecting_a_4x4_matrix() {
    let m = Matrix4f::from([
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
    let m = Matrix2f::from([[-3.0, 5.0], [1.0, -2.0]]);

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
  }

  #[test]
  fn a_3x3_matrix_ought_to_be_representable() {
    let m = Matrix3f::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

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
    let m1 = Matrix2f::from([[0.123456789, 1.0], [2.0, 3.0]]);
    let m2 = Matrix2f::from([[0.123456789, 1.0], [2.0, 3.0]]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_almost_identical_2x2_matrices() {
    let m1 = Matrix2f::from([[0.123456789, 1.0], [2.0, 3.0]]);
    let m2 = Matrix2f::from([[0.123456780, 1.0], [2.0, 3.0]]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_identical_3x3_matrices() {
    let m1 = Matrix3f::from([
      [0.123456789, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777777],
    ]);

    let m2 = Matrix3f::from([
      [0.123456789, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777777],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_almost_identical_3x3_matrices() {
    let m1 = Matrix3f::from([
      [0.123456789, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777777],
    ]);
    let m2 = Matrix3f::from([
      [0.123456780, 1.0, 2.0],
      [2.0, 3.0, 4.0],
      [5.0, 6.0, 7.7777777777777],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_identical_4x4_matrices() {
    let m1 = Matrix4f::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);
    let m2 = Matrix4f::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_equality_with_almost_identical_4x4_matrices() {
    let m1 = Matrix4f::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0000000000001],
    ]);
    let m2 = Matrix4f::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);

    assert_fuzzy_eq!(m1, m2);
  }

  #[test]
  fn matrix_inequality_with_non_identical_4x4_matrices() {
    let m1 = Matrix4f::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 1.0],
    ]);
    let m2 = Matrix4f::from([
      [0.123456789, 1.0, 2.0, 42.0],
      [2.0, 3.0, 4.0, -42.0],
      [5.0, 6.0, 7.7777777777777777, 23.5],
      [0.0, 0.0, 0.0, 2.0],
    ]);

    assert_fuzzy_ne!(m1, m2);
  }

  #[test]
  fn multiplying_two_4x4_matrices() {
    let m1 = Matrix4f::from([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0],
    ]);
    let m2 = Matrix4f::from([
      [-2.0, 1.0, 2.0, 3.0],
      [3.0, 2.0, 1.0, -1.0],
      [4.0, 3.0, 6.0, 5.0],
      [1.0, 2.0, 7.0, 8.0],
    ]);

    let expected_result = Matrix4f::from([
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
    let m = Matrix4f::from([
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
    let m1 = Matrix4f::from([
      [0.0, 1.0, 2.0, 4.0],
      [1.0, 2.0, 4.0, 8.0],
      [2.0, 4.0, 8.0, 16.0],
      [4.0, 8.0, 16.0, 32.0],
    ]);
    let m2 = Matrix4f::identity();

    let expected_result = m1;
    let actual_result = m1 * m2;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn transposing_a_4x4_matrix() {
    let m = Matrix4f::from([
      [0.0, 9.0, 3.0, 0.0],
      [9.0, 8.0, 0.0, 8.0],
      [1.0, 8.0, 5.0, 3.0],
      [0.0, 0.0, 5.0, 8.0],
    ]);

    let expected_result = Matrix4f::from([
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
    let m = Matrix2f::from([[1.0, 5.0], [-3.0, 2.0]]);

    let expected_result = 17.0;

    let actual_result = m.determinant();

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    let m = Matrix3f::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]]);

    let expected_result = Matrix2f::from([[-3.0, 2.0], [0.0, 6.0]]);

    let actual_result = m.submatrix(0, 2);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    let m = Matrix4f::from([
      [-6.0, 1.0, 1.0, 6.0],
      [-8.0, 5.0, 8.0, 6.0],
      [-1.0, 0.0, 8.0, 2.0],
      [-7.0, 1.0, -1.0, 1.0],
    ]);

    let expected_result = Matrix3f::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

    let actual_result = m.submatrix(2, 1);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn calculate_the_minor_of_a_3x3_matrix() {
    let m = Matrix3f::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

    let sub = m.submatrix(1, 0);
    let determinant = sub.determinant();
    let minor = m.minor(1, 0);

    assert_fuzzy_eq!(25.0, determinant);
    assert_fuzzy_eq!(25.0, minor);
  }

  #[test]
  fn calculating_a_cofactor_of_a_3x3_matrix() {
    let m = Matrix3f::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

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
    let m = Matrix3f::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

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
    let m = Matrix4f::from([
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
    let m = Matrix4f::from([
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
    let m = Matrix4f::from([
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
    let m = Matrix4f::from([
      [-5.0, 2.0, 6.0, -8.0],
      [1.0, -5.0, 1.0, 8.0],
      [7.0, 7.0, -6.0, -7.0],
      [1.0, -3.0, 7.0, 4.0],
    ]);

    let determinant = m.determinant();
    let cofactor23 = m.cofactor(2, 3);
    let cofactor32 = m.cofactor(3, 2);

    let expected_result = Matrix4f::from([
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
    let m = Matrix4f::from([
      [8.0, -5.0, 9.0, 2.0],
      [7.0, 5.0, 6.0, 1.0],
      [-6.0, 0.0, 9.0, 6.0],
      [-3.0, 0.0, -9.0, -4.0],
    ]);

    let expected_result = Matrix4f::from([
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
    let m = Matrix4f::from([
      [9.0, 3.0, 0.0, 9.0],
      [-5.0, -2.0, -6.0, -3.0],
      [-4.0, 9.0, 6.0, 4.0],
      [-7.0, 6.0, 6.0, 2.0],
    ]);

    let expected_result = Matrix4f::from([
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
    let m1 = Matrix4f::from([
      [3.0, -9.0, 7.0, 3.0],
      [3.0, -8.0, 2.0, -9.0],
      [-4.0, 4.0, 4.0, 1.0],
      [-6.0, 5.0, -1.0, 1.0],
    ]);

    let m2 = Matrix4f::from([
      [8.0, 2.0, 2.0, 2.0],
      [3.0, -1.0, 7.0, 0.0],
      [7.0, 0.0, 5.0, 4.0],
      [6.0, -2.0, 0.0, 5.0],
    ]);

    let m3 = m1 * m2;

    let actual_result = m3 * m2.inverse();

    assert_fuzzy_eq!(actual_result, m1);
  }
}
