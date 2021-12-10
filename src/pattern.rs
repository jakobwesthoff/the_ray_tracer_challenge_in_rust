use crate::canvas::Color;
use crate::fuzzy_eq::FuzzyEq;
use crate::tuple::Tuple;

pub trait Stencil {
  fn color_at(&self, position: Tuple) -> Color;
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
  fn color_at(&self, position: Tuple) -> Color {
    match *self {
      Pattern::Striped(ref striped) => striped.color_at(position),
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
  fn color_at(&self, position: Tuple) -> Color {
    let x = position.x;
    if x.floor() as isize % 2 == 0 {
      self.color_a
    } else {
      self.color_b
    }
  }
}
