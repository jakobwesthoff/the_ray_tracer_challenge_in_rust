use crate::canvas::Color;
use crate::F;

pub trait Illuminated {}

pub enum Material {
  Phong(Phong),
}

impl Illuminated for Material {}

pub struct Phong {
  pub color: Color,
  pub ambient: F,
  pub diffuse: F,
  pub specular: F,
  pub shininess: F,
}

impl Default for Phong {
  fn default() -> Self {
    Phong {
      color: Color::new(1.0, 1.0, 1.0),
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
    }
  }
}

impl Phong {
  pub fn new(color: Color, ambient: F, diffuse: F, specular: F, shininess: F) -> Self {
    Phong {
      color,
      ambient,
      diffuse,
      specular,
      shininess,
    }
  }
}

impl Illuminated for Phong {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fuzzy_eq::*;

  #[test]
  fn default_phong_material() {
    let m = Phong::default();

    assert_fuzzy_eq!(m.color, Color::new(1.0, 1.0, 1.0));
    assert_fuzzy_eq!(m.ambient, 0.1);
    assert_fuzzy_eq!(m.diffuse, 0.9);
    assert_fuzzy_eq!(m.specular, 0.9);
    assert_fuzzy_eq!(m.shininess, 200.0);
  }

  #[test]
  fn phong_material_can_be_constructed_with_properties() {
    let color = Color::new(1.0, 1.0, 0.0);
    let ambient = 0.05;
    let diffuse = 0.7;
    let specular = 0.95;
    let shininess = 400.0;

    let m = Phong::new(color, ambient, diffuse, specular, shininess);

    assert_fuzzy_eq!(m.color, color);
    assert_fuzzy_eq!(m.ambient, ambient);
    assert_fuzzy_eq!(m.diffuse, diffuse);
    assert_fuzzy_eq!(m.specular, specular);
    assert_fuzzy_eq!(m.shininess, shininess);
  }
}
