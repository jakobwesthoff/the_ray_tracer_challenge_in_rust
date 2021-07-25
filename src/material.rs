use crate::canvas::Color;
use crate::fuzzy_eq::*;
use crate::light::PointLight;
use crate::tuple::Tuple;
use crate::F;

pub trait Illuminated {
  fn lighting(&self, light: PointLight, position: Tuple, eyev: Tuple, normalv: Tuple) -> Color;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Material {
  Phong(Phong),
}

impl From<Phong> for Material {
  fn from(phong: Phong) -> Self {
    Material::Phong(phong)
  }
}

impl Default for Material {
  fn default() -> Self {
    Material::from(Phong::default())
  }
}

impl FuzzyEq<Material> for Material {
  fn fuzzy_eq(&self, other: Material) -> bool {
    match (self, other) {
      (Material::Phong(ref m), Material::Phong(other)) => m.fuzzy_eq(other),
      // Add default case (different types) to return false, once more than one
      // Material exists
      // _ => false,
    }
  }
}

impl Illuminated for Material {
  fn lighting(&self, light: PointLight, position: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
    match *self {
      Material::Phong(ref m) => m.lighting(light, position, eyev, normalv),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl FuzzyEq<Phong> for Phong {
  fn fuzzy_eq(&self, other: Phong) -> bool {
    self.color.fuzzy_eq(other.color)
      && self.ambient.fuzzy_eq(other.ambient)
      && self.diffuse.fuzzy_eq(other.diffuse)
      && self.specular.fuzzy_eq(other.specular)
      && self.shininess.fuzzy_eq(other.shininess)
  }
}

impl Illuminated for Phong {
  fn lighting(&self, light: PointLight, position: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
    let ambient_light: Color;
    let diffuse_light: Color;
    let specular_light: Color;

    let effective_color = self.color * light.intensity;
    
    let lightv = (light.position - position).normalize();

    ambient_light = effective_color * self.ambient;

    let light_dot_normal = lightv.dot(normalv);
    
    if light_dot_normal < 0.0 {
      // Light is on the other side of the surface
      diffuse_light = Color::black();
      specular_light = Color::black();
    } else {
      // Light is on the side the surface is pointing to.
      diffuse_light = effective_color * self.diffuse * light_dot_normal;

      let reflectv = -lightv.reflect(normalv);
      let reflect_dot_eye = reflectv.dot(eyev);

      if reflect_dot_eye <= 0.0 {
        specular_light = Color::black();
      } else {
        let factor = reflect_dot_eye.powf(self.shininess);
        specular_light = light.intensity * self.specular * factor;
      }
    }

    ambient_light + diffuse_light + specular_light
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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

  #[test]
  fn lighting_with_the_eye_between_the_light_and_the_surface() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(1.9, 1.9, 1.9);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_by_45_degrees() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let eyev = Tuple::vector(0.0, sqrt2_over_2, -sqrt2_over_2);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(1.0, 1.0, 1.0);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_opposite_surface_light_offset_by_45_degrees() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(0.7364, 0.7364, 0.7364);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_in_path_of_the_reflection_vector() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let eyev = Tuple::vector(0.0, -sqrt2_over_2, -sqrt2_over_2);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(1.6364, 1.6364, 1.6364);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_light_behind_the_surface() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(0.1, 0.1, 0.1);

    assert_fuzzy_eq!(actual_result, expected_result);
  }
}
