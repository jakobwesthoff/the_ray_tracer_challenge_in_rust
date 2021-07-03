use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};
use std::vec::Vec;

use super::util::*;

#[derive(Debug, Clone, Copy)]
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

pub struct Canvas {
  pub width: usize,
  pub height: usize,

  pixels: Vec<Color>,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      pixels: vec![Color::black(); width * height],
    }
  }

  pub fn pixel_at(&self, x: usize, y: usize) -> Color {
    self.pixels[self.get_pixel_index(x, y)]
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
    let index = self.get_pixel_index(x, y);
    self.pixels[index] = color;
  }

  fn get_pixel_index(&self, x: usize, y: usize) -> usize {
    y * self.width + x
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

  #[test]
  fn creating_a_canvas() {
    let c = Canvas::new(10, 20);

    assert_eq!(10, c.width);
    assert_eq!(20, c.height);

    for x in 0..c.width - 1 {
      for y in 0..c.height - 1 {
        assert_eq!(c.pixel_at(x, y), Color::black())
      }
    }
  }

  #[test]
  fn writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);

    let red = Color::new(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red);

    let expected_result = Color::new(1.0, 0.0, 0.0);

    assert_eq!(expected_result, c.pixel_at(2, 3));
  }
}
