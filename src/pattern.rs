use crate::body::{Body, Intersectable};
use crate::canvas::Color;
use crate::fuzzy_eq::FuzzyEq;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub trait Stencil {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color;
  fn transform(&self) -> Matrix<4>;

  fn color_at(&self, position: Tuple, body: &Body) -> Color {
    // Transform into object space
    let object_position = body.transform().inverse() * position;

    // Transform into pattern space
    let pattern_position = self.transform().inverse() * object_position;

    self.color_at_in_pattern_space(pattern_position)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pattern {
  Striped(Striped),
  Gradient(Gradient),
  Ring(Ring),
  CheckerBoard(CheckerBoard),
}

impl FuzzyEq<Pattern> for Pattern {
  fn fuzzy_eq(&self, other: Pattern) -> bool {
    match (self, other) {
      (Pattern::Striped(ref striped), Pattern::Striped(other)) => striped.fuzzy_eq(other),
      (Pattern::Gradient(ref gradient), Pattern::Gradient(other)) => gradient.fuzzy_eq(other),
      (Pattern::Ring(ref ring), Pattern::Ring(other)) => ring.fuzzy_eq(other),
      (Pattern::CheckerBoard(ref checkerboard), Pattern::CheckerBoard(other)) => {
        checkerboard.fuzzy_eq(other)
      }
      _ => false,
    }
  }
}

impl Stencil for Pattern {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    match *self {
      Pattern::Striped(ref striped) => striped.color_at_in_pattern_space(position),
      Pattern::Gradient(ref gradient) => gradient.color_at_in_pattern_space(position),
      Pattern::Ring(ref ring) => ring.color_at_in_pattern_space(position),
      Pattern::CheckerBoard(ref checkerboard) => checkerboard.color_at_in_pattern_space(position),
    }
  }

  fn transform(&self) -> Matrix<4> {
    match *self {
      Pattern::Striped(ref striped) => striped.transform(),
      Pattern::Gradient(ref gradient) => gradient.transform(),
      Pattern::Ring(ref ring) => ring.transform(),
      Pattern::CheckerBoard(ref checkerboard) => checkerboard.transform(),
    }
  }
}

impl From<Striped> for Pattern {
  fn from(striped: Striped) -> Self {
    Pattern::Striped(striped)
  }
}

impl From<Gradient> for Pattern {
  fn from(gradient: Gradient) -> Self {
    Pattern::Gradient(gradient)
  }
}

impl From<Ring> for Pattern {
  fn from(ring: Ring) -> Self {
    Pattern::Ring(ring)
  }
}

impl From<CheckerBoard> for Pattern {
  fn from(checkerboard: CheckerBoard) -> Self {
    Pattern::CheckerBoard(checkerboard)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Striped {
  color_a: Color,
  color_b: Color,
  transform: Matrix<4>,
}

impl Default for Striped {
  fn default() -> Self {
    Self {
      color_a: Color::black(),
      color_b: Color::white(),
      transform: Matrix::identity(),
    }
  }
}

impl Striped {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }
}

impl FuzzyEq<Striped> for Striped {
  fn fuzzy_eq(&self, other: Striped) -> bool {
    self.color_a.fuzzy_eq(other.color_a)
      && self.color_b.fuzzy_eq(other.color_b)
      && self.transform.fuzzy_eq(other.transform)
  }
}

impl Stencil for Striped {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    let x = position.x;
    if x.floor() as isize % 2 == 0 {
      self.color_a
    } else {
      self.color_b
    }
  }

  fn transform(&self) -> Matrix<4> {
    self.transform
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Gradient {
  color_a: Color,
  color_b: Color,
  transform: Matrix<4>,
}

impl Default for Gradient {
  fn default() -> Self {
    Self {
      color_a: Color::red(),
      color_b: Color::green(),
      transform: Default::default(),
    }
  }
}

impl Gradient {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }
}

impl FuzzyEq<Gradient> for Gradient {
  fn fuzzy_eq(&self, other: Gradient) -> bool {
    self.color_a.fuzzy_eq(other.color_a)
      && self.color_b.fuzzy_eq(other.color_b)
      && self.transform.fuzzy_eq(other.transform)
  }
}

impl Stencil for Gradient {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    let x = position.x;
    let fraction_of_x = x - x.floor();
    let distance_of_colors = self.color_b - self.color_a;

    self.color_a + distance_of_colors * fraction_of_x
  }

  fn transform(&self) -> Matrix<4> {
    self.transform
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ring {
  color_a: Color,
  color_b: Color,
  transform: Matrix<4>,
}

impl Default for Ring {
  fn default() -> Self {
    Self {
      color_a: Color::yellow(),
      color_b: Color::blue(),
      transform: Default::default(),
    }
  }
}

impl Ring {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }
}

impl FuzzyEq<Ring> for Ring {
  fn fuzzy_eq(&self, other: Ring) -> bool {
    self.color_a.fuzzy_eq(other.color_a)
      && self.color_b.fuzzy_eq(other.color_b)
      && self.transform.fuzzy_eq(other.transform)
  }
}

impl Stencil for Ring {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    let x = position.x;
    let y = position.y;

    let distance_from_center = (x.powf(2.0) + y.powf(2.0)).sqrt();

    if distance_from_center.floor() as i64 % 2 == 0 {
      self.color_a
    } else {
      self.color_b
    }
  }

  fn transform(&self) -> Matrix<4> {
    self.transform
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CheckerBoard {
  color_a: Color,
  color_b: Color,
  third_dimension: bool,
  transform: Matrix<4>,
}

impl Default for CheckerBoard {
  fn default() -> Self {
    Self {
      color_a: Color::black(),
      color_b: Color::white(),
      third_dimension: true,
      transform: Default::default(),
    }
  }
}

impl CheckerBoard {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }

  pub fn with_third_dimension(mut self, third_dimension: bool) -> Self {
    self.third_dimension = third_dimension;
    self
  }
}

impl FuzzyEq<CheckerBoard> for CheckerBoard {
  fn fuzzy_eq(&self, other: CheckerBoard) -> bool {
    self.color_a.fuzzy_eq(other.color_a)
      && self.color_b.fuzzy_eq(other.color_b)
      && self.transform.fuzzy_eq(other.transform)
      && self.third_dimension == other.third_dimension
  }
}

impl Stencil for CheckerBoard {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    let x = position.x;
    let y = position.y;
    let z = position.z;

    if (self.third_dimension && ((x.floor() + y.floor() + z.floor()) as i64 % 2) == 0)
      || (!self.third_dimension && ((x.floor() + y.floor()) as i64 % 2) == 0)
    {
      self.color_a
    } else {
      self.color_b
    }
  }

  fn transform(&self) -> Matrix<4> {
    self.transform
  }
}

#[cfg(test)]
mod tests {
  use crate::sphere::Sphere;

  use super::*;

  #[test]
  fn a_stripe_pattern_is_constant_in_y() {
    let pattern = Striped::default();
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 1.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 2.0, 0.0), &body)
    );
  }

  #[test]
  fn a_stripe_pattern_is_constant_in_z() {
    let pattern = Striped::default();
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 1.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 2.0), &body)
    );
  }

  #[test]
  fn a_stripe_pattern_alternates_in_x() {
    let pattern = Striped::default();
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.9, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(1.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.1, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(-0.1, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(-1.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(-1.1, 0.0, 0.0), &body)
    );
  }

  #[test]
  fn striped_pattern_adheres_to_object_transform() {
    let transform = Matrix::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::from(Striped::default().with_colors(Color::black(), Color::white()));
    let body = Body::from(Sphere::default().with_transform(transform));

    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(1.5, 0.0, 0.0), &body)
    );
  }

  #[test]
  fn striped_pattern_adheres_to_pattern_transform() {
    let transform = Matrix::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::from(
      Striped::default()
        .with_colors(Color::black(), Color::white())
        .with_transform(transform),
    );
    let body = Body::from(Sphere::default());

    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(1.5, 0.0, 0.0), &body)
    );
  }

  #[test]
  fn striped_pattern_adheres_to_object_and_pattern_transform() {
    let transform = Matrix::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::from(
      Striped::default()
        .with_colors(Color::black(), Color::white())
        .with_transform(transform),
    );
    let body = Body::from(Sphere::default().with_transform(transform));

    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(3.5, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(4.0, 0.0, 0.0), &body)
    );
  }
}
