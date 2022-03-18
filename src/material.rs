use crate::body::Body;
use crate::canvas::Color;
use crate::fuzzy_eq::*;
use crate::light::PointLight;
use crate::pattern::{Pattern, Stencil};
use crate::tuple::Tuple;
use crate::F;

pub trait Illuminated {
  fn lighting(
    &self,
    body: &Body,
    light: PointLight,
    position: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    in_shadow: bool,
  ) -> Color;
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
  fn lighting(
    &self,
    body: &Body,
    light: PointLight,
    position: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    in_shadow: bool,
  ) -> Color {
    match *self {
      Material::Phong(ref m) => m.lighting(body, light, position, eyev, normalv, in_shadow),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Phong {
  pub color: Color,
  pub pattern: Option<Pattern>,
  pub ambient: F,
  pub diffuse: F,
  pub specular: F,
  pub shininess: F,
}

impl Default for Phong {
  fn default() -> Self {
    Phong {
      color: Color::new(1.0, 1.0, 1.0),
      pattern: None,
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
      reflective: 0.0,
    }
  }
}

impl Phong {
  pub fn with_color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }

  pub fn with_ambient(mut self, ambient: F) -> Self {
    self.ambient = ambient;
    self
  }

  pub fn with_diffuse(mut self, diffuse: F) -> Self {
    self.diffuse = diffuse;
    self
  }

  pub fn with_specular(mut self, specular: F) -> Self {
    self.specular = specular;
    self
  }

  pub fn with_shininess(mut self, shininess: F) -> Self {
    self.shininess = shininess;
    self
  }

  pub fn with_pattern(mut self, pattern: Pattern) -> Self {
    self.pattern = Some(pattern);
    self
  }
}

impl FuzzyEq<Phong> for Phong {
  fn fuzzy_eq(&self, other: Phong) -> bool {
    self.color.fuzzy_eq(other.color)
      && self.ambient.fuzzy_eq(other.ambient)
      && self.diffuse.fuzzy_eq(other.diffuse)
      && self.specular.fuzzy_eq(other.specular)
      && self.shininess.fuzzy_eq(other.shininess)
      && self.pattern.fuzzy_eq(other.pattern)
  }
}

impl Illuminated for Phong {
  fn lighting(
    &self,
    body: &Body,
    light: PointLight,
    position: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    in_shadow: bool,
  ) -> Color {
    let ambient_light: Color;
    let diffuse_light: Color;
    let specular_light: Color;

    let mut color = self.color;
    if let Some(pattern) = self.pattern {
      color = pattern.color_at(position, body);
    }

    let effective_color = color * light.intensity;
    let lightv = (light.position - position).normalize();

    ambient_light = effective_color * self.ambient;

    if in_shadow {
      return ambient_light;
    }

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
  use crate::sphere::Sphere;

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
  fn phong_material_can_be_constructed_with_builder() {
    let color = Color::new(1.0, 1.0, 0.0);
    let ambient = 0.05;
    let diffuse = 0.7;
    let specular = 0.95;
    let shininess = 400.0;

    let m = Phong::default()
      .with_color(color)
      .with_ambient(ambient)
      .with_diffuse(diffuse)
      .with_specular(specular)
      .with_shininess(shininess);

    assert_fuzzy_eq!(m.color, color);
    assert_fuzzy_eq!(m.ambient, ambient);
    assert_fuzzy_eq!(m.diffuse, diffuse);
    assert_fuzzy_eq!(m.specular, specular);
    assert_fuzzy_eq!(m.shininess, shininess);
  }

  #[test]
  fn lighting_with_the_eye_between_the_light_and_the_surface() {
    let m = Phong::default();
    let body = Body::from(Sphere::default());
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

    let expected_result = Color::new(1.9, 1.9, 1.9);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_by_45_degrees() {
    let m = Phong::default();
    let body = Body::from(Sphere::default());
    let position = Tuple::point(0.0, 0.0, 0.0);

    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let eyev = Tuple::vector(0.0, sqrt2_over_2, -sqrt2_over_2);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

    let expected_result = Color::new(1.0, 1.0, 1.0);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_opposite_surface_light_offset_by_45_degrees() {
    let m = Phong::default();
    let body = Body::from(Sphere::default());
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

    let expected_result = Color::new(0.7364, 0.7364, 0.7364);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_in_path_of_the_reflection_vector() {
    let m = Phong::default();
    let body = Body::from(Sphere::default());
    let position = Tuple::point(0.0, 0.0, 0.0);

    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let eyev = Tuple::vector(0.0, -sqrt2_over_2, -sqrt2_over_2);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

    let expected_result = Color::new(1.6364, 1.6364, 1.6364);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_light_behind_the_surface() {
    let m = Phong::default();
    let body = Body::from(Sphere::default());
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

    let expected_result = Color::new(0.1, 0.1, 0.1);

    assert_fuzzy_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_surface_in_shadow() {
    let m = Phong::default();
    let body = Body::from(Sphere::default());
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(&body, light, position, eyev, normalv, true);

    let expected_result = Color::new(0.1, 0.1, 0.1);

    assert_fuzzy_eq!(actual_result, expected_result);
  }
}
