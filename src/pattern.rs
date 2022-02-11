use crate::body::{Body, Intersectable};
use crate::canvas::Color;
use crate::fuzzy_eq::FuzzyEq;
use crate::tuple::Tuple;

pub trait Stencil {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color;

  fn color_at(&self, position: Tuple, body: &Body) -> Color {
    // Transform into object space
    let object_position = body.transform().inverse() * position;

    // FIXME: Have pattern support its own transform and really
    //        transform into pattern space
    self.color_at_in_pattern_space(object_position)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pattern {
  Striped(Striped),
}

impl FuzzyEq<Pattern> for Pattern {
  fn fuzzy_eq(&self, other: Pattern) -> bool {
    match (self, other) {
      (Pattern::Striped(ref striped), Pattern::Striped(other)) => striped.fuzzy_eq(other),
    }
  }
}

impl Stencil for Pattern {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    match *self {
      Pattern::Striped(ref striped) => striped.color_at_in_pattern_space(position),
    }
  }
}

impl From<Striped> for Pattern {
  fn from(striped: Striped) -> Self {
    Pattern::Striped(striped)
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Striped {
  color_a: Color,
  color_b: Color,
}

impl Default for Striped {
  fn default() -> Self {
    Self {
      color_a: Color::black(),
      color_b: Color::white(),
    }
  }
}

impl Striped {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }
}

impl FuzzyEq<Striped> for Striped {
  fn fuzzy_eq(&self, other: Striped) -> bool {
    self.color_a.fuzzy_eq(other.color_b) && self.color_b.fuzzy_eq(other.color_b)
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
}
