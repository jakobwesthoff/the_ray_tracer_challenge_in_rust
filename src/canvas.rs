use num_traits::Float;
use num_traits::Num;
use std::ops::{Add, Mul, Sub};
use std::vec::Vec;

use super::fuzzy_eq::*;

#[derive(Debug, Clone, Copy)]
pub struct Color<T = f64>
where
  T: Float,
{
  pub red: T,
  pub green: T,
  pub blue: T,
}

impl<T: Float> Color<T> {
  pub fn new(red: T, green: T, blue: T) -> Self {
    Color { red, green, blue }
  }

  pub fn black() -> Self {
    Color::new(T::zero(), T::zero(), T::zero())
  }

  pub fn clamp(&self, lower_bound: T, upper_bound: T) -> Color<T> {
    Color::new(
      self.red.min(upper_bound).max(lower_bound),
      self.green.min(upper_bound).max(lower_bound),
      self.blue.min(upper_bound).max(lower_bound),
    )
  }
}

impl<T: Float> Add for Color<T> {
  type Output = Color<T>;

  fn add(self, other: Color<T>) -> Self::Output {
    Color::new(
      self.red + other.red,
      self.green + other.green,
      self.blue + other.blue,
    )
  }
}

impl<T: Float> Sub for Color<T> {
  type Output = Color<T>;

  fn sub(self, other: Color<T>) -> Self::Output {
    Color::new(
      self.red - other.red,
      self.green - other.green,
      self.blue - other.blue,
    )
  }
}

impl<T, U> Mul<U> for Color<T>
where
  T: Float,
  T: From<U>,
  U: Num,
{
  type Output = Color<T>;

  fn mul(self, other: U) -> Self::Output {
    let multiplicator: T = other.into();
    Color::new(
      self.red * multiplicator,
      self.green * multiplicator,
      self.blue * multiplicator,
    )
  }
}

impl<T> Mul<Color<T>> for Color<T>
where
  T: Float,
{
  type Output = Color<T>;

  fn mul(self, other: Color<T>) -> Self::Output {
    Color::new(
      self.red * other.red.into(),
      self.green * other.green.into(),
      self.blue * other.blue.into(),
    )
  }
}

impl<T> FuzzyEq<Color<T>> for Color<T>
where
  T: Float,
  T: FuzzyEq<T>,
{
  fn fuzzy_eq(&self, other: &Self) -> bool {
    self.red.fuzzy_eq(&other.red)
      && self.green.fuzzy_eq(&other.green)
      && self.blue.fuzzy_eq(&other.blue)
  }
}

pub struct Canvas<T = f64>
where
  T: Float,
{
  pub width: usize,
  pub height: usize,

  pixels: Vec<Color<T>>,
}

impl<T: Float> Canvas<T> {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      pixels: vec![Color::black(); width * height],
    }
  }

  pub fn pixel_at(&self, x: usize, y: usize) -> Color<T> {
    self.pixels[self.get_pixel_index(x, y)]
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, color: Color<T>) {
    let index = self.get_pixel_index(x, y);
    self.pixels[index] = color;
  }

  fn get_pixel_index(&self, x: usize, y: usize) -> usize {
    y * self.width + x
  }

  fn create_ppm_header(&self) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend(String::from("P3\n").into_bytes());
    header.extend(format!("{} {}\n", self.width, self.height).into_bytes());
    header.extend(format!("{}\n", 255).into_bytes());

    return header;
  }

  fn create_ppm_pixel_data(&self) -> Vec<u8> {
    let mut pixel_strings: Vec<String> = Vec::new();
    for pixel in self.pixels.iter() {
      let clamped_color = pixel.clamp(T::zero(), T::one());
      let r: u8 = (clamped_color.red * T::from(255.0).unwrap())
        .round()
        .to_u8()
        .unwrap();
      let g: u8 = (clamped_color.green * T::from(255.0).unwrap())
        .round()
        .to_u8()
        .unwrap();
      let b: u8 = (clamped_color.blue * T::from(255.0).unwrap())
        .round()
        .to_u8()
        .unwrap();

      pixel_strings.push(format!("{}", r));
      pixel_strings.push(format!("{}", g));
      pixel_strings.push(format!("{}", b));
    }

    let mut pixel_data: Vec<u8> = Vec::new();

    let mut column_count: usize = 0;
    let mut last_image_row: usize = 0;

    for (i, pixel_string) in pixel_strings.iter().enumerate() {
      // Line break for each row
      let current_image_row = i / (self.width * 3);
      if current_image_row != last_image_row {
        last_image_row = current_image_row;
        pixel_data.extend(String::from("\n").into_bytes());
        column_count = 0;
      }

      let mut needed_space: usize = 0;

      if column_count != 0 {
        needed_space += 1; // space
      }
      needed_space += pixel_string.len();

      // Do not exceed 70 characters per line
      if column_count + needed_space > 70 {
        pixel_data.extend(String::from("\n").into_bytes());
        column_count = 0;
      }

      if column_count != 0 {
        pixel_data.extend(String::from(" ").into_bytes());
        column_count += 1;
      }

      pixel_data.extend(pixel_string.clone().into_bytes());
      column_count += pixel_string.len();
    }

    // Insert newline at the end of data
    pixel_data.extend(String::from("\n").into_bytes());

    return pixel_data;
  }
}

pub trait ToPPM {
  fn to_ppm(&self) -> Vec<u8>;
}

impl<T> ToPPM for Canvas<T>
where
  T: Float,
{
  fn to_ppm(&self) -> Vec<u8> {
    let header = self.create_ppm_header();
    let pixel_data = self.create_ppm_pixel_data();

    let mut ppm = Vec::new();
    ppm.extend(header);
    ppm.extend(pixel_data);

    return ppm;
  }
}

#[cfg(test)]
mod tests {
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
