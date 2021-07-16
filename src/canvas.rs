pub mod to_png;
pub mod to_ppm;
pub mod to_rgba32;

use crate::F;
use std::ops::{Add, Mul, Sub};
use std::vec::Vec;

use super::fuzzy_eq::*;

pub trait Sized {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
  pub red: F,
  pub green: F,
  pub blue: F,
}

impl Color {
  pub fn new(red: F, green: F, blue: F) -> Self {
    Color { red, green, blue }
  }

  pub fn black() -> Self {
    Color::new(0.0, 0.0, 0.0)
  }

  pub fn clamp(&self, lower_bound: F, upper_bound: F) -> Color {
    Color::new(
      self.red.min(upper_bound).max(lower_bound),
      self.green.min(upper_bound).max(lower_bound),
      self.blue.min(upper_bound).max(lower_bound),
    )
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

impl Mul<F> for Color {
  type Output = Color;

  fn mul(self, other: F) -> Self::Output {
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

impl FuzzyEq<Color> for Color {
  fn fuzzy_eq(&self, other: Self) -> bool {
    self.red.fuzzy_eq(other.red)
      && self.green.fuzzy_eq(other.green)
      && self.blue.fuzzy_eq(other.blue)
  }
}

pub struct Canvas {
  pub width: usize,
  pub height: usize,

  pixels: Vec<Color>,
}

impl Sized for Canvas {
  fn width(&self) -> usize {
    self.width
  }
  fn height(&self) -> usize {
    self.height
  }
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
  use super::to_ppm::ToPPM;
  use super::*;

  #[test]
  fn colors_are_red_green_blue_tuples() {
    let c = Color::new(-0.5, 0.4, 1.7);

    assert_fuzzy_eq!(c.red, -0.5);
    assert_fuzzy_eq!(c.green, 0.4);
    assert_fuzzy_eq!(c.blue, 1.7);
  }

  #[test]
  fn adding_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let expected_result = Color::new(1.6, 0.7, 1.0);
    let actual_result = c1 + c2;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn substracting_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let expected_result = Color::new(0.2, 0.5, 0.5);
    let actual_result = c1 - c2;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_a_color_by_a_scalar() {
    let c = Color::new(0.2, 0.3, 0.4);
    let multiplier = 2.0;

    let expected_result = Color::new(0.4, 0.6, 0.8);
    let actual_result = c * multiplier;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn multiplying_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);

    let expected_result = Color::new(0.9, 0.2, 0.04);
    let actual_result = c1 * c2;

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn clamping_colors() {
    let c = Color::new(2.3, -6.7, 0.8);

    let expected_result = Color::new(1.0, 0.0, 0.8);
    let actual_result = c.clamp(0.0, 1.0);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn creating_a_canvas() {
    let c: Canvas = Canvas::new(10, 20);

    assert_eq!(10, c.width);
    assert_eq!(20, c.height);

    for x in 0..c.width - 1 {
      for y in 0..c.height - 1 {
        assert_fuzzy_eq!(c.pixel_at(x, y), Color::black())
      }
    }
  }

  #[test]
  fn writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);

    let red = Color::new(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red);

    let expected_result = Color::new(1.0, 0.0, 0.0);

    assert_fuzzy_eq!(expected_result, c.pixel_at(2, 3));
  }

  #[test]
  fn constructing_the_ppm_header() {
    let c: Canvas = Canvas::new(5, 3);
    let ppm_image = c.to_ppm();
    let actual_result = &ppm_image[..11];
    /*
     * Header consisting of:
     * Magic Bytes: P3
     * Width and Height: 5 3
     * Maximum Color Value (0-255): 255
     */
    let expected_result = String::from("P3\n5 3\n255\n").into_bytes();

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn constructing_the_ppm_pixel_data() {
    let mut canvas = Canvas::new(5, 3);
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);

    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);

    let actual_result = canvas.to_ppm();
    let header = String::from("P3\n5 3\n255\n").into_bytes();
    let pixel_data = String::from(
      "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n",
    ).into_bytes();
    let mut expected_result: Vec<u8> = Vec::new();
    expected_result.extend(header);
    expected_result.extend(pixel_data);

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn splitting_long_lines_ppm_files() {
    let mut canvas = Canvas::new(10, 2);
    let color = Color::new(1.0, 0.8, 0.6);

    for x in 0..10 {
      for y in 0..2 {
        canvas.write_pixel(x, y, color);
      }
    }

    let actual_result = canvas.to_ppm();
    let header = String::from("P3\n10 2\n255\n").into_bytes();
    let pixel_data = String::from(
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n",
    ).into_bytes();
    let mut expected_result: Vec<u8> = Vec::new();
    expected_result.extend(header);
    expected_result.extend(pixel_data);

    assert_eq!(actual_result, expected_result);
  }
}
