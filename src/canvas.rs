use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

use super::util::*;

#[derive(Debug)]
pub struct Color {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
}

impl Color {
  pub fn new(red: f64, green: f64, blue: f64) -> Self {
    Color { red, green, blue }
  }

  pub fn black() -> Self {
    Color::new(0.0, 0.0, 0.0)
  }
}

impl Add for Color {
  type Output = Color;

  fn add(self, other: Color) -> Self::Output {
    Color::new(
      self.red + other.red,
      self.green + other.green,
      self.blue + other.blue,
    )
  }
}

impl Sub for Color {
  type Output = Color;

  fn sub(self, other: Color) -> Self::Output {
    Color::new(
      self.red - other.red,
      self.green - other.green,
      self.blue - other.blue,
    )
  }
}

impl Mul<f64> for Color {
  type Output = Color;

  fn mul(self, other: f64) -> Self::Output {
    Color::new(self.red * other, self.green * other, self.blue * other)
  }
}

impl Mul<Color> for Color {
  type Output = Color;

  fn mul(self, other: Color) -> Self::Output {
    Color::new(
      self.red * other.red,
      self.green * other.green,
      self.blue * other.blue,
    )
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    f64_fuzzy_eq(self.red, other.red)
      && f64_fuzzy_eq(self.green, other.green)
      && f64_fuzzy_eq(self.blue, other.blue)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn colors_are_red_green_blue_tuples() {
    let c = Color::new(-0.5, 0.4, 1.7);

    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);
  }

  #[test]
  fn adding_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let expected_result = Color::new(1.6, 0.7, 1.0);
    let actual_result = c1 + c2;

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn substracting_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let expected_result = Color::new(0.2, 0.5, 0.5);
    let actual_result = c1 - c2;

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_a_color_by_a_scalar() {
    let c = Color::new(0.2, 0.3, 0.4);
    let multiplier = 2.0;

    let expected_result = Color::new(0.4, 0.6, 0.8);
    let actual_result = c * multiplier;

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);

    let expected_result = Color::new(0.9, 0.2, 0.04);
    let actual_result = c1 * c2;

    assert_eq!(actual_result, expected_result);
  }
}
