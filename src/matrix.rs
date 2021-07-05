use crate::fuzzy_eq::*;
use std::convert::From;
use std::ops::Index;

type Matrix2fArrayRow = [f64; 2];
type Matrix2fArray = [Matrix2fArrayRow; 2];
type Matrix3fArrayRow = [f64; 3];
type Matrix3fArray = [Matrix3fArrayRow; 3];
type Matrix4fArrayRow = [f64; 4];
type Matrix4fArray = [Matrix4fArrayRow; 4];

#[derive(Debug)]
pub struct Matrix2f {
  data: Matrix2fArray,
}

#[derive(Debug)]
pub struct Matrix3f {
  data: Matrix3fArray,
}

#[derive(Debug)]
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
    let m1= Matrix3f::from([
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
}
