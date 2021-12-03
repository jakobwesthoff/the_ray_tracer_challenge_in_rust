use std::collections::HashMap;

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

macro_rules! with_path {
  ($state:ident, $segment:expr, $op:expr) => {{
    $state.path.push($segment);
    let return_value = $op?;
    $state.path.pop();
    return_value
  }};
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
  fn get_value_from_hash(
    &self,
    hash: &'a yaml::Hash,
    key: impl AsRef<str>,
  ) -> ParserResult<&'a yaml::Yaml> {
    let yaml_key = yaml::Yaml::String(key.as_ref().into());
    if !hash.contains_key(&yaml_key) {
      Err(anyhow!(
        "Tried to get value with key {} from hash at {}: Key not found.",
        key.as_ref(),
        self.path.to_string()
      ))
    } else {
      Ok(&hash[&yaml_key])
    }
  }

  #[inline(always)]
  #[allow(clippy::ptr_arg)]
  fn get_index_from_array(
    &self,
    array: &'a yaml::Array,
    index: usize,
  ) -> ParserResult<&'a yaml::Yaml> {
    if index > array.len() {
      Err(anyhow!(
        "Tried to get value with index {} from hash at {}: Index not found (Array length = {}).",
        index,
        self.path.to_string(),
        array.len()
      ))
    } else {
      Ok(&array[index])
    }
  }

  #[inline(always)]
  fn value_to_string(&self, yaml: &'a yaml::Yaml) -> ParserResult<&'a impl AsRef<str>> {
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
  fn hash_value_to_string(
    &self,
    hash: &'a yaml::Hash,
    key: impl AsRef<str>,
  ) -> ParserResult<&'a impl AsRef<str>> {
    let value = self.get_value_from_hash(hash, key)?;
    self.value_to_string(value)
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
  fn hash_value_to_int(&self, hash: &yaml::Hash, key: impl AsRef<str>) -> ParserResult<i64> {
    let value = self.get_value_from_hash(hash, key)?;
    self.value_to_int(value)
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
  fn hash_value_to_float(&self, hash: &yaml::Hash, key: impl AsRef<str>) -> ParserResult<f64> {
    let value = self.get_value_from_hash(hash, key)?;
    self.value_to_float(value)
  }

  #[inline(always)]
  fn value_to_array(&self, yaml: &'a yaml::Yaml) -> ParserResult<&'a yaml::Array> {
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
  fn value_to_hash(&self, yaml: &'a yaml::Yaml) -> ParserResult<&'a yaml::Hash> {
    match yaml {
      yaml::Yaml::Hash(ref content) => Ok(content),
      _ => Err(anyhow!(
        "Expected hash at {}, but found {:?}",
        self.path.to_string(),
        yaml
      )),
    }
  }

  pub fn parse_yaml(&mut self) -> LoaderResult {
    let yaml = YamlLoader::load_from_str(self.data)?;
    self.visit_documents(&yaml)
  }

  fn visit_documents(
    &mut self,
    documents_array: &[yaml_rust::Yaml],
  ) -> ParserResult<(World, HashMap<String, Camera>)> {
    self.path.push(Segment::Key("".into()));
    for (index, document) in documents_array.iter().enumerate() {
      with_path!(self, Segment::Index(index), self.visit_document(document));
    }
    self.path.pop();

    let cameras_clone = self.cameras.clone();
    let bodies_clone = self.bodies.clone();
    let lights_clone = self.lights.clone();
    Ok((World::new(bodies_clone, lights_clone), cameras_clone))
  }

  fn visit_document(&mut self, document: &yaml_rust::Yaml) -> ParserResult {
    self.path.push(Segment::Key("root".into()));
    let document_array = self.value_to_array(document)?;
    for (index, item) in document_array.iter().enumerate() {
      with_path!(self, Segment::Index(index), self.visit_item(item));
    }
    self.path.pop();
    Ok(())
  }

  fn visit_item(&mut self, item: &yaml::Yaml) -> ParserResult {
    let item_hash = self.value_to_hash(item)?;
    if item_hash.contains_key(key!("light")) {
      let light_value = self.get_value_from_hash(item_hash, "light")?;
      let light = with_path!(
        self,
        Segment::Key("light".into()),
        self.visit_light(light_value)
      );
      self.lights.push(light);
    } else if item_hash.contains_key(key!("body")) {
      let body_value = self.get_value_from_hash(item_hash, "body")?;
      let body = with_path!(
        self,
        Segment::Key("body".into()),
        self.visit_body(body_value)
      );
      self.bodies.push(body);
    } else if item_hash.contains_key(key!("camera")) {
      let camera_value = self.get_value_from_hash(item_hash, "camera")?;
      let (name, camera) = with_path!(
        self,
        Segment::Key("camera".into()),
        self.visit_camera(camera_value)
      );
      self.cameras.insert(name, camera);
    }
    Ok(())
  }

  fn visit_light(&mut self, light: &yaml::Yaml) -> ParserResult<PointLight> {
    let light_hash = self.value_to_hash(light)?;
    let light_type = with_path!(
      self,
      Segment::Key("type".into()),
      self.hash_value_to_string(light_hash, "type")
    );

    if light_type.as_ref() == "point_light" {
      let light_at_value = self.get_value_from_hash(light_hash, "at")?;
      let light_at = with_path!(
        self,
        Segment::Key("at".into()),
        self.visit_point(light_at_value)
      );
      let light_intensity_value = self.get_value_from_hash(light_hash, "intensity")?;
      let light_intensity = with_path!(
        self,
        Segment::Key("intensity".into()),
        self.visit_color(light_intensity_value)
      );
      Ok(PointLight::new(light_at, light_intensity))
    } else {
      Err(anyhow!(
        "Unknown light type {} found at {}.",
        light_type.as_ref(),
        self.path.to_string()
      ))
    }
  }

  fn visit_point(&mut self, point: &yaml::Yaml) -> ParserResult<Tuple> {
    let point_array = self.value_to_array(point)?;
    let x_value = self.get_index_from_array(point_array, 0)?;
    let x = with_path!(self, Segment::Index(0), self.value_to_float(x_value));
    let y_value = self.get_index_from_array(point_array, 1)?;
    let y = with_path!(self, Segment::Index(1), self.value_to_float(y_value));
    let z_value = self.get_index_from_array(point_array, 2)?;
    let z = with_path!(self, Segment::Index(2), self.value_to_float(z_value));
    Ok(Tuple::point(x, y, z))
  }

  fn visit_vector(&mut self, vector: &yaml::Yaml) -> ParserResult<Tuple> {
    let vector_array = self.value_to_array(vector)?;
    let x_value = self.get_index_from_array(vector_array, 0)?;
    let x = with_path!(self, Segment::Index(0), self.value_to_float(x_value));
    let y_value = self.get_index_from_array(vector_array, 1)?;
    let y = with_path!(self, Segment::Index(1), self.value_to_float(y_value));
    let z_value = self.get_index_from_array(vector_array, 2)?;
    let z = with_path!(self, Segment::Index(2), self.value_to_float(z_value));
    Ok(Tuple::vector(x, y, z))
  }

  fn visit_color(&mut self, color: &yaml::Yaml) -> ParserResult<Color> {
    let color_array = self.value_to_array(color)?;
    let r_value = self.get_index_from_array(color_array, 0)?;
    let r = with_path!(self, Segment::Index(0), self.value_to_float(r_value));
    let g_value = self.get_index_from_array(color_array, 1)?;
    let g = with_path!(self, Segment::Index(1), self.value_to_float(g_value));
    let b_value = self.get_index_from_array(color_array, 2)?;
    let b = with_path!(self, Segment::Index(2), self.value_to_float(b_value));
    Ok(Color::new(r, g, b))
  }

  fn visit_body(&mut self, body: &yaml::Yaml) -> ParserResult<Body> {
    let mut material = Material::default();
    let mut transform = Matrix::identity();

    let body_hash = self.value_to_hash(body)?;

    let body_type = with_path!(
      self,
      Segment::Key("type".into()),
      self.hash_value_to_string(body_hash, "type")
    );

    if body_hash.contains_key(key!("material")) {
      let material_value = self.get_value_from_hash(body_hash, "material")?;
      material = self.visit_material(material_value)?;
    }

    if body_hash.contains_key(key!("transforms")) {
      let transforms_value = self.get_value_from_hash(body_hash, "transforms")?;
      transform = self.visit_transforms(transforms_value)?;
    }

    match body_type.as_ref() {
      "sphere" => Ok(Body::from(Sphere::new(material, transform))),
      "plane" => Ok(Body::from(Plane::new(material, transform))),
      _ => Err(anyhow!(
        "Unknown body type {} found at {}.",
        body_type.as_ref(),
        self.path.to_string()
      )),
    }
  }

  fn visit_material(&mut self, material: &yaml::Yaml) -> ParserResult<Material> {
    let material_hash = self.value_to_hash(material)?;
    let material_type = with_path!(
      self,
      Segment::Key("type".into()),
      self.hash_value_to_string(material_hash, "type")
    );

    if material_type.as_ref() == "phong" {
      let mut phong_material = Phong::default();

      if material_hash.contains_key(key!("color")) {
        let material_color = with_path!(
          self,
          Segment::Key("color".into()),
          self.visit_color(self.get_value_from_hash(material_hash, "color")?)
        );
        phong_material = phong_material.with_color(material_color);
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
        "Unknown material type {} found at {}.",
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
      let v = self.visit_vector(self.get_value_from_hash(transform_hash, "to")?)?;
      Ok(Matrix::translation(v.x, v.y, v.z))
    } else if transform_type.as_ref() == "scale" {
      let v = self.visit_vector(self.get_value_from_hash(transform_hash, "to")?)?;
      Ok(Matrix::scaling(v.x, v.y, v.z))
    } else if transform_type.as_ref() == "rotate_x" {
      let radians = self.hash_value_to_float(transform_hash, "radians")?;
      Ok(Matrix::rotation_x(radians))
    } else if transform_type.as_ref() == "rotate_y" {
      let radians = self.hash_value_to_float(transform_hash, "radians")?;
      Ok(Matrix::rotation_y(radians))
    } else if transform_type.as_ref() == "rotate_z" {
      let radians = self.hash_value_to_float(transform_hash, "radians")?;
      Ok(Matrix::rotation_z(radians))
    } else {
      Err(anyhow!(
        "Unknown transform type {} found at {}.",
        transform_type.as_ref(),
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
    let to = with_path!(self, Segment::Key("to".into()), self.visit_point(to_value));
    let from_value = self.get_value_from_hash(camera_hash, "from")?;
    let from = with_path!(
      self,
      Segment::Key("from".into()),
      self.visit_point(from_value)
    );
    let up_value = self.get_value_from_hash(camera_hash, "up")?;
    let up = with_path!(self, Segment::Key("up".into()), self.visit_vector(up_value));

    let camera = Camera::new(width.abs() as usize, height.abs() as usize, fov)
      .look_at_from_position(from, to, up);
    Ok((camera_name.as_ref().into(), camera))
  }
}

#[derive(Default)]
pub struct Yaml {}
impl WorldLoader for Yaml {
  fn load_world<T: AsRef<str>>(&self, source: T) -> LoaderResult {
    let mut parser = YamlParser::new(source.as_ref());
    parser.parse_yaml()
  }
}

#[cfg(test)]
mod tests {
  use crate::body::Body;
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

    let yaml_loader = Yaml::default();

    let (loaded_world, loaded_cameras) = yaml_loader.load_world(source).unwrap();
    assert_fuzzy_eq!(loaded_world, expected_world);
    assert_fuzzy_eq!(loaded_cameras, expected_cameras);
  }
}
