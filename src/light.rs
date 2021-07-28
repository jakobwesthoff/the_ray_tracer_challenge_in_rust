use crate::canvas::Color;
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
  pub position: Tuple,
  pub intensity: Color,
}

impl PointLight {
  pub fn new(position: Tuple, intensity: Color) -> Self {
    PointLight {
      position,
      intensity,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fuzzy_eq::*;

  #[test]
  fn a_point_light_has_a_position_and_intensity() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = Tuple::point(0.0, 0.0, 0.0);
    let l = PointLight::new(position, intensity);

    assert_fuzzy_eq!(l.position, position);
    assert_fuzzy_eq!(l.intensity, intensity);
  }
}
