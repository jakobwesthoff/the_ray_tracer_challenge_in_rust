use std::collections::HashMap;
use std::f64::consts::PI;

use super::{LoaderResult, WorldLoader};
use anyhow::*;
use itertools::Itertools;
use yaml_rust::{yaml, YamlLoader};

use crate::body::Body;
use crate::camera::Camera;
use crate::canvas::Color;
use crate::light::PointLight;
use crate::material::{Material, Phong};
use crate::matrix::Matrix;
use crate::pattern::{CheckerBoard, Gradient, Pattern, Ring, Striped};
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;
use crate::F;

#[derive(Clone)]
enum Segment {
  Key(String),
  Index(usize),
}

#[derive(Clone, Default)]
struct Path(Vec<Segment>);

impl Path {
  pub fn push(&mut self, segment: Segment) {
    self.0.push(segment);
  }

  pub fn pop(&mut self) {
    self.0.pop();
  }
}

impl ToString for Path {
  fn to_string(&self) -> String {
    self
      .0
      .iter()
      .map(|segment| match segment {
        Segment::Key(key) => format!(".{}", key),
        Segment::Index(index) => format!("[{}]", index),
      })
      .join("")
  }
}

macro_rules! key {
  ($yaml:expr) => {
    &yaml::Yaml::String($yaml.into())
  };
}

type ParserResult<T = ()> = anyhow::Result<T>;

#[derive(Default)]
pub struct YamlParser<'a> {
  data: &'a str,
  path: Path,
  lights: Vec<PointLight>,
  bodies: Vec<Body>,
  cameras: HashMap<String, Camera>,
}
impl<'a> YamlParser<'a> {
  pub fn new(data: &'a str) -> Self {
    Self {
      data,
      path: Path::default(),
      lights: Vec::new(),
      bodies: Vec::new(),
      cameras: HashMap::new(),
    }
  }

  #[inline(always)]
  fn get_value_from_hash<'b>(
    &self,
    hash: &'b yaml::Hash,
    key: impl AsRef<str>,
  ) -> ParserResult<&'b yaml::Yaml> {
    let yaml_key = yaml::Yaml::String(key.as_ref().into());
    if !hash.contains_key(&yaml_key) {
      Err(anyhow!(
        "Tried to get value with key '{}' from hash at {}: Key not found.",
        key.as_ref(),
        self.path.to_string()
      ))
    } else {
      Ok(&hash[&yaml_key])
    }
  }

  #[inline(always)]
  #[allow(clippy::ptr_arg)]
  fn get_index_from_array<'b>(
    &self,
    array: &'b yaml::Array,
    index: usize,
  ) -> ParserResult<&'b yaml::Yaml> {
    if index > array.len() {
      Err(anyhow!(
        "Tried to get value with index {} from array at {}: Index not found (Array length = {}).",
        index,
        self.path.to_string(),
        array.len()
      ))
    } else {
      Ok(&array[index])
    }
  }

  #[inline(always)]
  fn value_to_string<'b>(&self, yaml: &'b yaml::Yaml) -> ParserResult<&'b impl AsRef<str>> {
    match yaml {
      yaml::Yaml::String(content) => Ok(content),
      _ => Err(anyhow!(
        "Expected string value at {}, but found {:?}",
        self.path.to_string(),
        yaml
      )),
    }
  }

  #[inline(always)]
  fn hash_value_to_string<'b>(
    &mut self,
    hash: &'b yaml::Hash,
    key: impl AsRef<str>,
  ) -> ParserResult<&'b impl AsRef<str>> {
    self.path.push(Segment::Key(key.as_ref().into()));
    let value = self.get_value_from_hash(hash, key)?;
    let result = self.value_to_string(value);
    self.path.pop();
    result
  }

  #[inline(always)]
  fn value_to_int(&self, yaml: &yaml::Yaml) -> ParserResult<i64> {
    match yaml {
      yaml::Yaml::Integer(content) => Ok(*content),
      _ => Err(anyhow!(
        "Expected integer value at {}, but found {:?}",
        self.path.to_string(),
        yaml
      )),
    }
  }

  #[inline(always)]
  fn hash_value_to_int(&mut self, hash: &yaml::Hash, key: impl AsRef<str>) -> ParserResult<i64> {
    self.path.push(Segment::Key(key.as_ref().into()));
    let value = self.get_value_from_hash(hash, key)?;
    let result = self.value_to_int(value);
    self.path.pop();
    result
  }

  #[inline(always)]
  fn value_to_float(&self, yaml: &yaml::Yaml) -> ParserResult<F> {
    match yaml {
      yaml::Yaml::Integer(content) => Ok(*content as f64),
      yaml::Yaml::Real(_) => match yaml.as_f64() {
        Some(content) => Ok(content),
        _ => Err(anyhow!(
          "Expected float value at {}, but found {:?}",
          self.path.to_string(),
          yaml
        )),
      },
      _ => Err(anyhow!(
        "Expected float value at {}, but found {:?}",
        self.path.to_string(),
        yaml
      )),
    }
  }

  #[inline(always)]
  fn hash_value_to_float(&mut self, hash: &yaml::Hash, key: impl AsRef<str>) -> ParserResult<f64> {
    self.path.push(Segment::Key(key.as_ref().into()));
    let value = self.get_value_from_hash(hash, key)?;
    let result = self.value_to_float(value);
    self.path.pop();
    result
  }

  #[inline(always)]
  fn value_to_array<'b>(&self, yaml: &'b yaml::Yaml) -> ParserResult<&'b yaml::Array> {
    match yaml {
      yaml::Yaml::Array(ref content) => Ok(content),
      _ => Err(anyhow!(
        "Expected array at {}, but found {:?}",
        self.path.to_string(),
        yaml
      )),
    }
  }

  #[inline(always)]
  fn value_to_hash<'b>(&self, yaml: &'b yaml::Yaml) -> ParserResult<&'b yaml::Hash> {
    match yaml {
      yaml::Yaml::Hash(ref content) => Ok(content),
      _ => Err(anyhow!(
        "Expected hash at {}, but found {:?}",
        self.path.to_string(),
        yaml
      )),
    }
  }

  #[inline(always)]
  fn value_to_bool(&self, yaml: &yaml::Yaml) -> ParserResult<bool> {
    match yaml {
      yaml::Yaml::Boolean(content) => Ok(*content),
      _ => Err(anyhow!(
        "Expected boolean value at {}, but found {:?}",
        self.path.to_string(),
        yaml
      )),
    }
  }

  #[inline(always)]
  fn hash_value_to_bool(&mut self, hash: &yaml::Hash, key: impl AsRef<str>) -> ParserResult<bool> {
    self.path.push(Segment::Key(key.as_ref().into()));
    let value = self.get_value_from_hash(hash, key)?;
    let result = self.value_to_bool(value);
    self.path.pop();
    result
  }

  pub fn parse_yaml(&mut self) -> LoaderResult {
    let yaml = YamlLoader::load_from_str(self.data)?;
    self.visit_documents(&yaml)
  }

  fn visit_documents(
    &mut self,
    documents_array: &[yaml_rust::Yaml],
  ) -> ParserResult<(World, HashMap<String, Camera>)> {
    self.path.push(Segment::Key("document".into()));
    for (index, document) in documents_array.iter().enumerate() {
      self.path.push(Segment::Index(index));
      self.visit_document(document)?;
      self.path.pop();
    }
    self.path.pop();

    let cameras_clone = self.cameras.clone();
    let bodies_clone = self.bodies.clone();
    let lights_clone = self.lights.clone();
    Ok((World::new(bodies_clone, lights_clone), cameras_clone))
  }

  fn visit_document(&mut self, document: &yaml_rust::Yaml) -> ParserResult {
    self.path.push(Segment::Key("item".into()));
    let document_array = self.value_to_array(document)?;
    for (index, item) in document_array.iter().enumerate() {
      self.path.push(Segment::Index(index));
      self.visit_item(item)?;
      self.path.pop();
    }
    self.path.pop();
    Ok(())
  }

  fn visit_item(&mut self, item: &yaml::Yaml) -> ParserResult {
    let item_hash = self.value_to_hash(item)?;
    if item_hash.contains_key(key!("light")) {
      let light_value = self.get_value_from_hash(item_hash, "light")?;
      self.path.push(Segment::Key("light".into()));
      let light = self.visit_light(light_value)?;
      self.path.pop();

      self.lights.push(light);
    } else if item_hash.contains_key(key!("body")) {
      let body_value = self.get_value_from_hash(item_hash, "body")?;
      self.path.push(Segment::Key("body".into()));
      let body = self.visit_body(body_value)?;
      self.path.pop();
      self.bodies.push(body);
    } else if item_hash.contains_key(key!("camera")) {
      let camera_value = self.get_value_from_hash(item_hash, "camera")?;
      self.path.push(Segment::Key("camera".into()));
      let (name, camera) = self.visit_camera(camera_value)?;
      self.path.pop();
      self.cameras.insert(name, camera);
    } else {
      return Err(anyhow!(format!(
        "Unknown item type found at {}",
        self.path.to_string()
      )));
    }
    Ok(())
  }

  fn visit_light(&mut self, light: &yaml::Yaml) -> ParserResult<PointLight> {
    let light_hash = self.value_to_hash(light)?;
    let light_type = self.hash_value_to_string(light_hash, "type")?;

    if light_type.as_ref() == "point_light" {
      let light_at_value = self.get_value_from_hash(light_hash, "at")?;
      self.path.push(Segment::Key("at".into()));
      let light_at = self.visit_point(light_at_value)?;
      self.path.pop();

      let light_intensity_value = self.get_value_from_hash(light_hash, "intensity")?;
      self.path.push(Segment::Key("intensity".into()));
      let light_intensity = self.visit_color(light_intensity_value)?;
      self.path.pop();

      Ok(PointLight::new(light_at, light_intensity))
    } else {
      Err(anyhow!(
        "Unknown light type '{}' found at {}",
        light_type.as_ref(),
        self.path.to_string()
      ))
    }
  }

  fn visit_point(&mut self, point: &yaml::Yaml) -> ParserResult<Tuple> {
    let point_array = self.value_to_array(point)?;
    let x_value = self.get_index_from_array(point_array, 0)?;
    self.path.push(Segment::Index(0));
    let x = self.value_to_float(x_value)?;
    self.path.pop();
    let y_value = self.get_index_from_array(point_array, 1)?;
    self.path.push(Segment::Index(1));
    let y = self.value_to_float(y_value)?;
    self.path.pop();
    let z_value = self.get_index_from_array(point_array, 2)?;
    self.path.push(Segment::Index(2));
    let z = self.value_to_float(z_value)?;
    self.path.pop();
    Ok(Tuple::point(x, y, z))
  }

  fn visit_vector(&mut self, vector: &yaml::Yaml) -> ParserResult<Tuple> {
    let vector_array = self.value_to_array(vector)?;
    let x_value = self.get_index_from_array(vector_array, 0)?;
    self.path.push(Segment::Index(0));
    let x = self.value_to_float(x_value)?;
    self.path.pop();
    let y_value = self.get_index_from_array(vector_array, 1)?;
    self.path.push(Segment::Index(1));
    let y = self.value_to_float(y_value)?;
    self.path.pop();
    let z_value = self.get_index_from_array(vector_array, 2)?;
    self.path.push(Segment::Index(2));
    let z = self.value_to_float(z_value)?;
    self.path.pop();
    Ok(Tuple::vector(x, y, z))
  }

  fn visit_color(&mut self, color: &yaml::Yaml) -> ParserResult<Color> {
    let color_array = self.value_to_array(color)?;
    let r_value = self.get_index_from_array(color_array, 0)?;
    self.path.push(Segment::Index(0));
    let r = self.value_to_float(r_value)?;
    self.path.pop();
    let g_value = self.get_index_from_array(color_array, 1)?;
    self.path.push(Segment::Index(1));
    let g = self.value_to_float(g_value)?;
    self.path.pop();
    let b_value = self.get_index_from_array(color_array, 2)?;
    self.path.push(Segment::Index(2));
    let b = self.value_to_float(b_value)?;
    self.path.pop();
    Ok(Color::new(r, g, b))
  }

  fn visit_pattern(&mut self, pattern: &yaml::Yaml) -> ParserResult<Pattern> {
    let pattern_hash = self.value_to_hash(pattern)?;
    let pattern_type = self.hash_value_to_string(pattern_hash, "type")?;

    return match pattern_type.as_ref() {
      "striped" => self.visit_striped_pattern(pattern_hash),
      "gradient" => self.visit_gradient_pattern(pattern_hash),
      "ring" => self.visit_ring_pattern(pattern_hash),
      "checkerboard" => self.visit_checkerboard_pattern(pattern_hash),
      _ => Err(anyhow!(
        "Unknown Pattern type '{}' found at {}",
        pattern_type.as_ref(),
        self.path.to_string()
      )),
    };
  }

  fn visit_striped_pattern(&mut self, pattern_hash: &yaml::Hash) -> ParserResult<Pattern> {
    let color_a_value = self.get_value_from_hash(pattern_hash, "colorA")?;
    self.path.push(Segment::Key("colorA".into()));
    let color_a = self.visit_color(color_a_value)?;
    self.path.pop();
    let color_b_value = self.get_value_from_hash(pattern_hash, "colorB")?;
    self.path.push(Segment::Key("colorB".into()));
    let color_b = self.visit_color(color_b_value)?;
    self.path.pop();

    let mut transform = Matrix::identity();
    if pattern_hash.contains_key(key!("transforms")) {
      let transforms_value = self.get_value_from_hash(pattern_hash, "transforms")?;
      self.path.push(Segment::Key("transform".into()));
      transform = self.visit_transforms(transforms_value)?;
      self.path.pop();
    }

    Ok(Pattern::from(
      Striped::default()
        .with_colors(color_a, color_b)
        .with_transform(transform),
    ))
  }

  fn visit_gradient_pattern(&mut self, pattern_hash: &yaml::Hash) -> ParserResult<Pattern> {
    let color_a_value = self.get_value_from_hash(pattern_hash, "colorA")?;
    self.path.push(Segment::Key("colorA".into()));
    let color_a = self.visit_color(color_a_value)?;
    self.path.pop();
    let color_b_value = self.get_value_from_hash(pattern_hash, "colorB")?;
    self.path.push(Segment::Key("colorB".into()));
    let color_b = self.visit_color(color_b_value)?;
    self.path.pop();

    let mut transform = Matrix::identity();
    if pattern_hash.contains_key(key!("transforms")) {
      let transforms_value = self.get_value_from_hash(pattern_hash, "transforms")?;
      self.path.push(Segment::Key("transform".into()));
      transform = self.visit_transforms(transforms_value)?;
      self.path.pop();
    }

    Ok(Pattern::from(
      Gradient::default()
        .with_colors(color_a, color_b)
        .with_transform(transform),
    ))
  }

  fn visit_ring_pattern(&mut self, pattern_hash: &yaml::Hash) -> ParserResult<Pattern> {
    let color_a_value = self.get_value_from_hash(pattern_hash, "colorA")?;
    self.path.push(Segment::Key("colorA".into()));
    let color_a = self.visit_color(color_a_value)?;
    self.path.pop();
    let color_b_value = self.get_value_from_hash(pattern_hash, "colorB")?;
    self.path.push(Segment::Key("colorB".into()));
    let color_b = self.visit_color(color_b_value)?;
    self.path.pop();

    let mut transform = Matrix::identity();
    if pattern_hash.contains_key(key!("transforms")) {
      let transforms_value = self.get_value_from_hash(pattern_hash, "transforms")?;
      self.path.push(Segment::Key("transform".into()));
      transform = self.visit_transforms(transforms_value)?;
      self.path.pop();
    }

    Ok(Pattern::from(
      Ring::default()
        .with_colors(color_a, color_b)
        .with_transform(transform),
    ))
  }

  fn visit_checkerboard_pattern(&mut self, pattern_hash: &yaml::Hash) -> ParserResult<Pattern> {
    let color_a_value = self.get_value_from_hash(pattern_hash, "colorA")?;
    self.path.push(Segment::Key("colorA".into()));
    let color_a = self.visit_color(color_a_value)?;
    self.path.pop();
    let color_b_value = self.get_value_from_hash(pattern_hash, "colorB")?;
    self.path.push(Segment::Key("colorB".into()));
    let color_b = self.visit_color(color_b_value)?;
    self.path.pop();

    let third_dimension;
    if pattern_hash.contains_key(key!("3d")) {
      third_dimension = self.hash_value_to_bool(pattern_hash, "3d")?;
    } else {
      third_dimension = true;
    }

    let mut transform = Matrix::identity();
    if pattern_hash.contains_key(key!("transforms")) {
      let transforms_value = self.get_value_from_hash(pattern_hash, "transforms")?;
      self.path.push(Segment::Key("transform".into()));
      transform = self.visit_transforms(transforms_value)?;
      self.path.pop();
    }

    Ok(Pattern::from(
      CheckerBoard::default()
        .with_colors(color_a, color_b)
        .with_transform(transform)
        .with_third_dimension(third_dimension),
    ))
  }

  fn visit_body(&mut self, body: &yaml::Yaml) -> ParserResult<Body> {
    let mut material = Material::default();
    let mut transform = Matrix::identity();

    let body_hash = self.value_to_hash(body)?;
    let body_type = self.hash_value_to_string(body_hash, "type")?;

    if body_hash.contains_key(key!("material")) {
      let material_value = self.get_value_from_hash(body_hash, "material")?;
      self.path.push(Segment::Key("material".into()));
      material = self.visit_material(material_value)?;
      self.path.pop();
    }

    if body_hash.contains_key(key!("transforms")) {
      let transforms_value = self.get_value_from_hash(body_hash, "transforms")?;
      self.path.push(Segment::Key("transform".into()));
      transform = self.visit_transforms(transforms_value)?;
      self.path.pop();
    }

    match body_type.as_ref() {
      "sphere" => Ok(Body::from(Sphere::new(material, transform))),
      "plane" => Ok(Body::from(Plane::new(material, transform))),
      _ => Err(anyhow!(
        "Unknown body type '{}' found at {}",
        body_type.as_ref(),
        self.path.to_string()
      )),
    }
  }

  fn visit_material(&mut self, material: &yaml::Yaml) -> ParserResult<Material> {
    let material_hash = self.value_to_hash(material)?;
    let material_type = self.hash_value_to_string(material_hash, "type")?;

    if material_type.as_ref() == "phong" {
      let mut phong_material = Phong::default();

      if material_hash.contains_key(key!("color")) {
        let color_value = self.get_value_from_hash(material_hash, "color")?;
        self.path.push(Segment::Key("color".into()));
        let material_color = self.visit_color(color_value)?;
        self.path.pop();
        phong_material = phong_material.with_color(material_color);
      }
      if material_hash.contains_key(key!("pattern")) {
        let pattern_value = self.get_value_from_hash(material_hash, "pattern")?;
        self.path.push(Segment::Key("pattern".into()));
        let pattern = self.visit_pattern(pattern_value)?;
        self.path.pop();
        phong_material = phong_material.with_pattern(pattern);
      }
      if material_hash.contains_key(key!("diffuse")) {
        let material_diffuse = self.hash_value_to_float(material_hash, "diffuse")?;
        phong_material = phong_material.with_diffuse(material_diffuse);
      }
      if material_hash.contains_key(key!("ambient")) {
        let material_ambient = self.hash_value_to_float(material_hash, "ambient")?;
        phong_material = phong_material.with_ambient(material_ambient);
      }
      if material_hash.contains_key(key!("specular")) {
        let material_specular = self.hash_value_to_float(material_hash, "specular")?;
        phong_material = phong_material.with_specular(material_specular);
      }
      if material_hash.contains_key(key!("shininess")) {
        let material_shininess = self.hash_value_to_float(material_hash, "shininess")?;
        phong_material = phong_material.with_shininess(material_shininess);
      }

      Ok(Material::from(phong_material))
    } else {
      Err(anyhow!(
        "Unknown material type '{}' found at {}",
        material_type.as_ref(),
        self.path.to_string()
      ))
    }
  }

  fn visit_transforms(&mut self, transforms: &yaml::Yaml) -> ParserResult<Matrix<4>> {
    let transforms_array = self.value_to_array(transforms)?;
    let mut combined_transform = Matrix::identity();
    for (index, transform) in transforms_array.iter().enumerate().rev() {
      self.path.push(Segment::Index(index));
      let next_transform = self.visit_transform(transform)?;
      combined_transform = combined_transform * next_transform;
      self.path.pop();
    }

    Ok(combined_transform)
  }

  fn visit_transform(&mut self, transform: &yaml::Yaml) -> ParserResult<Matrix<4>> {
    let transform_hash = self.value_to_hash(transform)?;
    let transform_type = self.hash_value_to_string(transform_hash, "type")?;

    if transform_type.as_ref() == "translate" {
      let to_value = self.get_value_from_hash(transform_hash, "to")?;
      self.path.push(Segment::Key("to".into()));
      let v = self.visit_vector(to_value)?;
      self.path.pop();
      Ok(Matrix::translation(v.x, v.y, v.z))
    } else if transform_type.as_ref() == "scale" {
      let to_value = self.get_value_from_hash(transform_hash, "to")?;
      self.path.push(Segment::Key("to".into()));
      let v = self.visit_vector(to_value)?;
      self.path.pop();
      Ok(Matrix::scaling(v.x, v.y, v.z))
    } else if transform_type.as_ref() == "rotate_x" {
      let radians = self.visit_radians_or_degrees(transform_hash)?;
      Ok(Matrix::rotation_x(radians))
    } else if transform_type.as_ref() == "rotate_y" {
      let radians = self.visit_radians_or_degrees(transform_hash)?;
      Ok(Matrix::rotation_y(radians))
    } else if transform_type.as_ref() == "rotate_z" {
      let radians = self.visit_radians_or_degrees(transform_hash)?;
      Ok(Matrix::rotation_z(radians))
    } else {
      Err(anyhow!(
        "Unknown transform type '{}' found at {}",
        transform_type.as_ref(),
        self.path.to_string()
      ))
    }
  }

  fn visit_radians_or_degrees(&mut self, transform_hash: &yaml::Hash) -> ParserResult<f64> {
    if transform_hash.contains_key(key!("radians")) {
      self.hash_value_to_float(transform_hash, "radians")
    } else if transform_hash.contains_key(key!("degrees")) {
      let degrees = self.hash_value_to_float(transform_hash, "degrees")?;
      Ok((degrees / 180.0) * PI)
    } else {
      Err(anyhow!(
        "Expected either 'degrees' or 'radians' key, but found nothing at {}",
        self.path.to_string()
      ))
    }
  }

  fn visit_camera(&mut self, camera: &yaml::Yaml) -> ParserResult<(String, Camera)> {
    let camera_hash = self.value_to_hash(camera)?;
    let camera_name = self.hash_value_to_string(camera_hash, "name")?;
    let width = self.hash_value_to_int(camera_hash, "width")?;
    let height = self.hash_value_to_int(camera_hash, "height")?;
    let fov = self.hash_value_to_float(camera_hash, "field_of_view")?;
    let to_value = self.get_value_from_hash(camera_hash, "to")?;
    self.path.push(Segment::Key("to".into()));
    let to = self.visit_point(to_value)?;
    self.path.pop();
    let from_value = self.get_value_from_hash(camera_hash, "from")?;
    self.path.push(Segment::Key("from".into()));
    let from = self.visit_point(from_value)?;
    self.path.pop();
    let up_value = self.get_value_from_hash(camera_hash, "up")?;
    self.path.push(Segment::Key("up".into()));
    let up = self.visit_vector(up_value)?;
    self.path.pop();

    let camera = Camera::new(width.abs() as usize, height.abs() as usize, fov)
      .look_at_from_position(from, to, up);
    Ok((camera_name.as_ref().into(), camera))
  }
}

#[derive(Default)]
pub struct Loader {}
impl WorldLoader for Loader {
  fn load_world<T: AsRef<str>>(&self, source: T) -> LoaderResult {
    let mut parser = YamlParser::new(source.as_ref());
    parser.parse_yaml()
  }
}

#[cfg(test)]
mod tests {
  use crate::body::Body;
  use crate::body::Intersectable;
  use crate::camera::Camera;
  use crate::canvas::Color;
  use crate::light::PointLight;
  use crate::material::Material;
  use crate::material::Phong;
  use crate::matrix::Matrix;
  use crate::sphere::Sphere;
  use crate::tuple::Tuple;

  use super::*;
  use crate::fuzzy_eq::*;

  #[test]
  fn load_simple_yaml_world() {
    let source = r##"
---
- camera:
    name: output1
    width: 800
    height: 600
    field_of_view: 0.785 # 45degrees
    from: [1, 2, 3.4]
    to: [5.6, 7, 8]
    up: [9.10, 11, -1.2]

- light:
    type: point_light
    at: [1.1, 2.2, 3.3]
    intensity: [0.4, 0.5, 0.6]

- body:
    type: sphere
    material:
      type: phong
      color: [1,1,1]
      diffuse: 0.7
      ambient: 0.1
      specular: 0.0
      shininess: 200
    transforms:
      - type: translate
        to: [1, 2, 3]
      - type: rotate_x
        radians: 3.14
"##;

    let expected_world = World::new(
      vec![Body::from(Sphere::new(
        Material::from(Phong::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.7, 0.0, 200.0)),
        Matrix::rotation_x(3.14) * Matrix::translation(1.0, 2.0, 3.0),
      ))],
      vec![PointLight::new(
        Tuple::point(1.1, 2.2, 3.3),
        Color::new(0.4, 0.5, 0.6),
      )],
    );

    let mut expected_cameras = HashMap::new();
    expected_cameras.insert(
      String::from("output1"),
      Camera::new(800, 600, 0.785).look_at_from_position(
        Tuple::point(1.0, 2.0, 3.4),
        Tuple::point(5.6, 7.0, 8.0),
        Tuple::vector(9.10, 11.0, -1.2),
      ),
    );

    let yaml_loader = Loader::default();

    let (loaded_world, loaded_cameras) = yaml_loader.load_world(source).unwrap();
    assert_fuzzy_eq!(loaded_world, expected_world);
    assert_fuzzy_eq!(loaded_cameras, expected_cameras);
  }

  #[test]
  fn load_multiple_cameras() {
    let source = r##"
---
- camera:
    name: output1
    width: 800
    height: 600
    field_of_view: 0.785 # 45degrees
    from: [1, 2, 3.4]
    to: [5.6, 7, 8]
    up: [9.10, 11, -1.2]

- camera:
    name: output2
    width: 1920
    height: 1080
    field_of_view: 1.047 # PI/3
    from: [1, 0.5, -5]
    to: [0, 1, 0]
    up: [0, 1, 0]

- light:
    type: point_light
    at: [1.1, 2.2, 3.3]
    intensity: [0.4, 0.5, 0.6]

- body:
    type: sphere
    material:
      type: phong
      color: [1,1,1]
      diffuse: 0.7
      ambient: 0.1
      specular: 0.0
      shininess: 200
    transforms:
      - type: translate
        to: [1, 2, 3]
      - type: rotate_x
        radians: 3.14
"##;

    let expected_world = World::new(
      vec![Body::from(Sphere::new(
        Material::from(Phong::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.7, 0.0, 200.0)),
        Matrix::rotation_x(3.14) * Matrix::translation(1.0, 2.0, 3.0),
      ))],
      vec![PointLight::new(
        Tuple::point(1.1, 2.2, 3.3),
        Color::new(0.4, 0.5, 0.6),
      )],
    );

    let mut expected_cameras = HashMap::new();
    expected_cameras.insert(
      String::from("output1"),
      Camera::new(800, 600, 0.785).look_at_from_position(
        Tuple::point(1.0, 2.0, 3.4),
        Tuple::point(5.6, 7.0, 8.0),
        Tuple::vector(9.10, 11.0, -1.2),
      ),
    );
    expected_cameras.insert(
      String::from("output2"),
      Camera::new(1920, 1080, 1.047).look_at_from_position(
        Tuple::point(1.0, 0.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
      ),
    );

    let yaml_loader = Loader::default();

    let (loaded_world, loaded_cameras) = yaml_loader.load_world(source).unwrap();
    assert_fuzzy_eq!(loaded_world, expected_world);
    assert_fuzzy_eq!(loaded_cameras, expected_cameras);
  }

  #[test]
  fn complex_scene_multiple_cameras_multiple_bodies() {
    let source = r##"
---
- light:
    type: point_light
    at: [-10, 10, -10]
    intensity: [1, 1, 1]

# Floor
- body:
    type: plane
    material:
      type: phong
      color: [0.5, 0.45, 0.45]
      specular: 0.0

# Left Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0.635, 0, 1]
    transforms:
      - type: scale
        to: [0.33, 0.33, 0.33]
      - type: translate
        to: [-1.5, 0.33, -0.75]

# Middle Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [1, 0, 0.635]
      diffuse: 0.9
      specular: 1.8
    transforms:
      - type: translate
        to: [-0.5, 1.0, 0.5]

# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, 0.635, 1]
    transforms:
      - type: scale
        to: [0.5, 0.5, 0.5]
      - type: translate
        to: [1.5, 0.5, -0.5]

# Camera
- camera:
    name: main_camera
    width: 3840
    height: 2160
    field_of_view: 1.047 # PI/3
    from: [0, 1.5, -5]
    to: [0, 1, 0]
    up: [0, 1, 0]

# Camera further moved down
- camera:
    name: second_camera
    width: 1920
    height: 1080
    field_of_view: 1.047 # PI/3
    from: [1, 0.5, -5]
    to: [0, 1, 0]
    up: [0, 1, 0]

"##;

    let expected_world = World::new(
      vec![
        // # Floor
        // - body:
        //     type: plane
        //     material:
        //       type: phong
        //       color: [0.5, 0.45, 0.45]
        //       specular: 0.0
        Body::from(
          Plane::default().with_material(Material::from(
            Phong::default()
              .with_color(Color::new(0.5, 0.45, 0.45))
              .with_specular(0.0),
          )),
        ),
        // # Left Sphere
        // - body:
        //     type: sphere
        //     material:
        //       type: phong
        //       color: [0.635, 0, 1]
        //     transforms:
        //       - type: scale
        //         to: [0.33, 0.33, 0.33]
        //       - type: translate
        //         to: [-1.5, 0.33, -0.75]
        Body::from(
          Sphere::default()
            .with_material(Material::from(
              Phong::default().with_color(Color::new(0.635, 0.0, 1.0)),
            ))
            .with_transform(
              Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33),
            ),
        ),
        // # Middle Sphere
        // - body:
        //     type: sphere
        //     material:
        //       type: phong
        //       color: [1, 0, 0.635]
        //       diffuse: 0.9
        //       specular: 1.8
        //     transforms:
        //       - type: translate
        //         to: [-0.5, 1.0, 0.5]
        Body::from(
          Sphere::default()
            .with_material(Material::from(
              Phong::default()
                .with_color(Color::new(1.0, 0.0, 0.635))
                .with_diffuse(0.9)
                .with_specular(1.8),
            ))
            .with_transform(Matrix::translation(-0.5, 1.0, 0.5)),
        ),
        // # Right Sphere
        // - body:
        //     type: sphere
        //     material:
        //       type: phong
        //       color: [0, 0.635, 1]
        //     transforms:
        //       - type: scale
        //         to: [0.5, 0.5, 0.5]
        //       - type: translate
        //         to: [1.5, 0.5, -0.5]
        Body::from(
          Sphere::default()
            .with_material(Material::from(
              Phong::default().with_color(Color::new(0.0, 0.635, 1.0)),
            ))
            .with_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5)),
        ),
      ],
      vec![
        // - light:
        //     type: point_light
        //     at: [-10, 10, -10]
        //     intensity: [1, 1, 1]
        PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)),
      ],
    );

    let mut expected_cameras = HashMap::new();
    // # Camera
    // - camera:
    //     name: main_camera
    //     width: 3840
    //     height: 2160
    //     field_of_view: 1.047 # PI/3
    //     from: [0, 1.5, -5]
    //     to: [0, 1, 0]
    //     up: [0, 1, 0]
    expected_cameras.insert(
      String::from("main_camera"),
      Camera::new(3840, 2160, 1.047).look_at_from_position(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
      ),
    );

    // # Camera further moved down
    // - camera:
    //     name: second_camera
    //     width: 1920
    //     height: 1080
    //     field_of_view: 1.047 # PI/3
    //     from: [1, 0.5, -5]
    //     to: [0, 1, 0]
    //     up: [0, 1, 0]
    expected_cameras.insert(
      String::from("second_camera"),
      Camera::new(1920, 1080, 1.047).look_at_from_position(
        Tuple::point(1.0, 0.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
      ),
    );

    let yaml_loader = Loader::default();

    let (loaded_world, loaded_cameras) = yaml_loader.load_world(source).unwrap();
    assert_fuzzy_eq!(loaded_world, expected_world);
    assert_fuzzy_eq!(loaded_cameras, expected_cameras);
  }

  #[test]
  fn unknown_base_item() {
    let source = r##"
---
- something_unknown:
    great: isnt it?
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!("Unknown item type found at .document[0].item[0]");
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn unknown_light_type() {
    let source = r##"
---
- light:
    type: bright_but_unknown_light
    at: [-10, 10, -10]
    intensity: [1, 1, 1]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected =
      anyhow!("Unknown light type 'bright_but_unknown_light' found at .document[0].item[0].light");
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn missing_light_at_property() {
    let source = r##"
---
- light:
    type: point_light
    intensity: [1, 1, 1]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Tried to get value with key 'at' from hash at .document[0].item[0].light: Key not found."
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn light_type_is_not_a_string() {
    let source = r##"
---
- light:
    type: 423.0
    at: [-10, 10, -10]
    intensity: [1, 1, 1]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Expected string value at .document[0].item[0].light.type, but found Real(\"423.0\")"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn light_at_is_not_point_array() {
    let source = r##"
---
- light:
    type: point_light
    at: [-10, some_strange_string, -10]
    intensity: [1, 1, 1]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!("Expected float value at .document[0].item[0].light.at[1], but found String(\"some_strange_string\")");
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn light_intensity_is_not_color_array() {
    let source = r##"
---
- light:
    type: point_light
    at: [-10, 10, -10]
    intensity: [1, 1, blub]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Expected float value at .document[0].item[0].light.intensity[2], but found String(\"blub\")"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn unknown_body_type() {
    let source = r##"
---
# Right Sphere
- body:
    type: unknown_something
    material:
      type: phong
      color: [0, 0.635, 1]
    transforms:
      - type: scale
        to: [0.5, 0.5, 0.5]
      - type: translate
        to: [1.5, 0.5, -0.5]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected =
      anyhow!("Unknown body type 'unknown_something' found at .document[0].item[0].body");
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn unknown_material_type() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: hyper_realistic_everything
      color: [0, 0.635, 1]
    transforms:
      - type: scale
        to: [0.5, 0.5, 0.5]
      - type: translate
        to: [1.5, 0.5, -0.5]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Unknown material type 'hyper_realistic_everything' found at .document[0].item[0].body.material"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn faulty_material_color() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, sparkly_red, 1]
    transforms:
      - type: scale
        to: [0.5, 0.5, 0.5]
      - type: translate
        to: [1.5, 0.5, -0.5]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Expected float value at .document[0].item[0].body.material.color[1], but found String(\"sparkly_red\")"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn faulty_material_shininess() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, 0.5, 1]
      shininess: really_shiny
    transforms:
      - type: scale
        to: [0.5, 0.5, 0.5]
      - type: translate
        to: [1.5, 0.5, -0.5]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Expected float value at .document[0].item[0].body.material.shininess, but found String(\"really_shiny\")"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn unknown_transform_type() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, 0.5, 1]
    transforms:
      - type: woop_di_duuu
        to: [0.5, 0.5, 0.5]
      - type: translate
        to: [1.5, 0.5, -0.5]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Unknown transform type 'woop_di_duuu' found at .document[0].item[0].body.transform[0]"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn wrong_to_in_scale_transform() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, 0.5, 1]
    transforms:
      - type: scale
        to: foobar
      - type: translate
        to: [1.5, 0.5, -0.5]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Expected array at .document[0].item[0].body.transform[0].to, but found String(\"foobar\")"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn wrong_to_in_translate_transform() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, 0.5, 1]
    transforms:
      - type: scale
        to: [1.5, 0.5, -0.5]
      - type: translate
        to: [1.5, 0.5, nope]
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Expected float value at .document[0].item[0].body.transform[1].to[2], but found String(\"nope\")"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn wrong_radians_in_rotate_y_transform() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, 0.5, 1]
    transforms:
      - type: scale
        to: [1.5, 0.5, -0.5]
      - type: translate
        to: [1.5, 0.5, 12]
      - type: rotate_y
        radians: 90deg
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(result.is_err());
    let actual = result.unwrap_err();
    let expected = anyhow!(
      "Expected float value at .document[0].item[0].body.transform[2].radians, but found String(\"90deg\")"
    );
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn specify_body_rotation_transformation_in_degrees() {
    let source = r##"
---
# Right Sphere
- body:
    type: sphere
    material:
      type: phong
      color: [0, 0.5, 1]
    transforms:
      - type: rotate_x
        degrees: 180
      - type: rotate_y
        degrees: 90
      - type: rotate_z
        degrees: 423
"##;

    let yaml_loader = Loader::default();
    let result = yaml_loader.load_world(source);
    assert!(!result.is_err());
    let (world, _camera_hash) = result.unwrap();

    assert_eq!(1, world.bodies.len());
    let body = world.bodies[0];

    let expected_transform = Matrix::rotation_z(423.0 / 180.0 * PI)
      * Matrix::rotation_y(90.0 / 180.0 * PI)
      * Matrix::rotation_x(PI);

    assert_eq!(expected_transform, body.transform());
  }

  #[test]
  fn striped_pattern_in_body_is_parsed() {
    let source = r##"
---
- body:
    type: sphere
    material:
      type: phong
      pattern:
        type: striped
        colorA: [0,0,0]
        colorB: [1,1,1]
        transforms:
          - type: scale
            to: [.2,.2,.2]
          - type: rotate_z
            degrees: 45
"##;

    let color_a = Color::new(0.0, 0.0, 0.0);
    let color_b = Color::new(1.0, 1.0, 1.0);
    let pattern_transform =
      Matrix::rotation_z((45.0 / 180.0) * PI) * Matrix::scaling(0.2, 0.2, 0.2);
    let pattern = Pattern::from(
      Striped::default()
        .with_colors(color_a, color_b)
        .with_transform(pattern_transform),
    );

    let material = Material::from(Phong::default().with_pattern(pattern));
    let body_transform = Matrix::identity();
    let sphere = Sphere::default()
      .with_material(material)
      .with_transform(body_transform);
    let body = Body::from(sphere);

    let yaml_loader = Loader::default();

    let (loaded_world, _) = yaml_loader.load_world(source).unwrap();
    assert_eq!(1, loaded_world.bodies.len());
    assert_fuzzy_eq!(body, loaded_world.bodies[0]);
  }

  #[test]
  fn colorful_striped_pattern_in_body_is_parsed() {
    let source = r##"
---
- body:
    type: sphere
    material:
      type: phong
      pattern:
        type: striped
        colorA: [0.1,0.2,0.3]
        colorB: [0.4,0.5,0.6]
        transforms:
          - type: scale
            to: [.2,.2,.2]
          - type: rotate_z
            degrees: 90
"##;

    let color_a = Color::new(0.1, 0.2, 0.3);
    let color_b = Color::new(0.4, 0.5, 0.6);
    let pattern_transform =
      Matrix::rotation_z((90.0 / 180.0) * PI) * Matrix::scaling(0.2, 0.2, 0.2);
    let pattern = Pattern::from(
      Striped::default()
        .with_colors(color_a, color_b)
        .with_transform(pattern_transform),
    );

    let material = Material::from(Phong::default().with_pattern(pattern));
    let body_transform = Matrix::identity();
    let sphere = Sphere::default()
      .with_material(material)
      .with_transform(body_transform);
    let body = Body::from(sphere);

    let yaml_loader = Loader::default();

    let (loaded_world, _) = yaml_loader.load_world(source).unwrap();
    assert_eq!(1, loaded_world.bodies.len());
    assert_fuzzy_eq!(body, loaded_world.bodies[0]);
  }
}
