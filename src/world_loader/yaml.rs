use std::collections::HashMap;
use std::error::Error;

use super::{LoaderResult, WorldLoader};
use itertools::Itertools;
use yaml_rust::{yaml, ScanError, YamlLoader};

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

#[derive(Clone)]
enum Segment {
  Key(String),
  Index(usize),
}

#[derive(Clone)]
struct Path(Vec<Segment>);

impl Path {
  pub fn push(&mut self, segment: Segment) {
    self.0.push(segment);
  }

  pub fn pop(&mut self) {
    self.0.pop();
  }

  fn new() -> Path {
    Path(Vec::new())
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

pub struct ParseError {
  message: String,
  path: Path,
}

impl ParseError {
  fn new(path: Path, message: String) -> ParseError {
    ParseError { message, path }
  }
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "YAML ParseError at {}: {}",
      self.path.to_string(),
      self.message
    )
  }
}

impl std::fmt::Debug for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ParseError")
      .field("message", &self.message)
      .field("path", &self.path.to_string())
      .finish()
  }
}

impl Error for ParseError {
  fn description(&self) -> &str {
    self.message.as_str()
  }
}
macro_rules! key {
  ($yaml:expr) => {
    &yaml::Yaml::String($yaml.into())
  };
}

macro_rules! safe_key {
  ($state:expr, $yaml:expr, $key:expr) => {{
    if !$yaml.contains_key(key!(String::from($key))) {
      Err(Box::new(ParseError::new(
        $state.path.clone(),
        format!("Expected to find key {}, but didn't", $key),
      )))
    } else {
      Ok(($state, &$yaml[key!(String::from($key))]))
    }
  }};
}

macro_rules! safe_index {
  ($state:expr, $yaml:expr, $index:expr) => {{
    if $yaml.len() > $index {
      Ok(($state, &$yaml[$index]))
    } else {
      Err(Box::new(ParseError::new(
        $state.path.clone(),
        format!(
          "Expected to find index {}, but the array only has {} elements",
          $index,
          $yaml.len()
        ),
      )))
    }
  }};
}

macro_rules! to_string {
  ($state:expr, $yaml:expr) => {
    match $yaml {
      yaml::Yaml::String(content) => Ok(($state, content)),
      _ => Err(Box::new(ParseError::new(
        $state.path.clone(),
        format!("Expected string, got {:?}", $yaml),
      ))),
    }
  };

  ($state:expr, $yaml:expr, $key:expr) => {{
    let (state, value) = safe_key!($state, $yaml, $key)?;
    to_string!(state, value)
  }};
}

macro_rules! to_int {
  ($state:expr, $yaml:expr) => {
    match $yaml {
      yaml::Yaml::Integer(content) => Ok(($state, content)),
      _ => Err(Box::new(ParseError::new(
        $state.path.clone(),
        format!("Expected integer, got {:?}", $yaml),
      ))),
    }
  };

  ($state:expr, $yaml:expr, $key:expr) => {{
    let (state, value) = safe_key!($state, $yaml, $key)?;
    to_int!(state, value)
  }};
}

macro_rules! to_float {
  ($state:expr, $yaml:expr) => {
    match $yaml {
      yaml::Yaml::Integer(ref content) => Ok(($state, *content as f64)),
      yaml::Yaml::Real(_) => match $yaml.as_f64() {
        Some(content) => Ok(($state, content)),
        _ => Err(Box::new(ParseError::new(
          $state.path.clone(),
          format!("Expected float, got {:?}", $yaml),
        ))),
      },
      _ => Err(Box::new(ParseError::new(
        $state.path.clone(),
        format!("Expected float, got {:?}", $yaml),
      ))),
    }
  };

  ($state:expr, $yaml:expr, $key:expr) => {{
    let (state, value) = safe_key!($state, $yaml, $key)?;
    to_float!(state, value)
  }};
}

macro_rules! to_array {
  ($state:expr, $yaml:expr) => {
    match $yaml {
      yaml::Yaml::Array(content) => Ok(($state, content)),
      _ => Err(Box::new(ParseError::new(
        $state.path.clone(),
        format!("Expected array, got {:?}", $yaml),
      ))),
    }
  };

  ($path:expr, $yaml:expr, $key:expr) => {
    to_array!($path, safe_key!($path, $yaml, $key)?)
  };
}

macro_rules! to_hash {
  ($state:expr, $yaml:expr) => {
    match $yaml {
      yaml::Yaml::Hash(content) => Ok(($state, content)),
      _ => Err(Box::new(ParseError::new(
        $state.path.clone(),
        format!("Expected hash, got {:?}", $yaml),
      ))),
    }
  };

  ($path:expr, $yaml:expr, $key:expr) => {
    to_hash!($path, safe_key!($path, $yaml, $key)?)
  };
}

macro_rules! with_state {
  ($state:ident, $segment:expr, $op:expr) => {{
    $state.path.push($segment);
    let (mut state, return_value) = $op?;
    state.path.pop();
    (state, return_value)
  }};
}
struct ParserState {
  path: Path,
  lights: Vec<PointLight>,
  bodies: Vec<Body>,
  cameras: HashMap<String, Camera>,
}

impl ParserState {
  fn new() -> Self {
    Self {
      path: Path::new(),
      lights: Vec::new(),
      bodies: Vec::new(),
      cameras: HashMap::new(),
    }
  }
}

type ParserResult<T = ()> = Result<(ParserState, T), Box<dyn Error>>;

#[derive(Default)]
pub struct Yaml {}
impl Yaml {
  pub fn new() -> Self {
    Self {}
  }

  fn parse_yaml(&self, source: &str) -> Result<Vec<yaml_rust::Yaml>, ScanError> {
    YamlLoader::load_from_str(source)
  }

  fn visit_documents(
    &self,
    mut state: ParserState,
    documents_array: &[yaml_rust::Yaml],
  ) -> ParserResult<(World, HashMap<String, Camera>)> {
    state.path.push(Segment::Key("".into()));
    for (index, document) in documents_array.iter().enumerate() {
      let result = with_state!(
        state,
        Segment::Index(index),
        self.visit_document(state, document)
      );
      state = result.0;
    }
    state.path.pop();

    let cameras_clone = state.cameras.clone();
    let bodies_clone = state.bodies.clone();
    let lights_clone = state.lights.clone();
    Ok((
      state,
      (World::new(bodies_clone, lights_clone), cameras_clone),
    ))
  }

  fn visit_document(&self, mut state: ParserState, document: &yaml_rust::Yaml) -> ParserResult {
    state.path.push(Segment::Key("root".into()));
    let (mut state, document_array) = to_array!(state, document)?;
    for (index, item) in document_array.iter().enumerate() {
      let result = with_state!(state, Segment::Index(index), self.visit_item(state, item));
      state = result.0;
    }
    state.path.pop();
    Ok((state, ()))
  }

  fn visit_item(&self, state: ParserState, item: &yaml::Yaml) -> ParserResult {
    let (mut state, item_hash) = to_hash!(state, item)?;
    if item_hash.contains_key(key!("light")) {
      let (mut new_state, light_value) = safe_key!(state, item_hash, "light")?;
      let (mut new_state, light) = with_state!(
        new_state,
        Segment::Key("light".into()),
        self.visit_light(new_state, light_value)
      );
      new_state.lights.push(light);
      state = new_state;
    } else if item_hash.contains_key(key!("body")) {
      let (mut new_state, body_value) = safe_key!(state, item_hash, "body")?;
      let (mut new_state, body) = with_state!(
        new_state,
        Segment::Key("body".into()),
        self.visit_body(new_state, body_value)
      );
      new_state.bodies.push(body);
      state = new_state;
    } else if item_hash.contains_key(key!("camera")) {
      let (mut new_state, camera_value) = safe_key!(state, item_hash, "camera")?;
      let (mut new_state, (name, camera)) = with_state!(
        new_state,
        Segment::Key("camera".into()),
        self.visit_camera(new_state, camera_value)
      );
      new_state.cameras.insert(name, camera);
      state = new_state;
    }
    Ok((state, ()))
  }

  fn visit_light(&self, state: ParserState, light: &yaml::Yaml) -> ParserResult<PointLight> {
    let (mut state, light_hash) = to_hash!(state, light)?;
    let (state, light_type) = with_state!(
      state,
      Segment::Key("type".into()),
      to_string!(state, light_hash, "type")
    );

    if light_type.eq("point_light") {
      let (mut state, light_at_value) = safe_key!(state, light_hash, "at")?;
      let (state, light_at) = with_state!(
        state,
        Segment::Key("at".into()),
        self.visit_point(state, light_at_value)
      );
      let (mut state, light_intensity_value) = safe_key!(state, light_hash, "intensity")?;
      let (state, light_intensity) = with_state!(
        state,
        Segment::Key("intensity".into()),
        self.visit_color(state, light_intensity_value)
      );
      Ok((state, PointLight::new(light_at, light_intensity)))
    } else {
      Err(Box::new(ParseError::new(
        state.path,
        format!("Found light, with unknown light type {}.", light_type),
      )))
    }
  }

  fn visit_point(&self, state: ParserState, point: &yaml::Yaml) -> ParserResult<Tuple> {
    let (state, point_array) = to_array!(state, point)?;
    let (mut state, x_value) = safe_index!(state, point_array, 0)?;
    let (state, x) = with_state!(state, Segment::Index(0), to_float!(state, x_value));
    let (mut state, y_value) = safe_index!(state, point_array, 1)?;
    let (state, y) = with_state!(state, Segment::Index(1), to_float!(state, y_value));
    let (mut state, z_value) = safe_index!(state, point_array, 2)?;
    let (state, z) = with_state!(state, Segment::Index(2), to_float!(state, z_value));
    Ok((state, Tuple::point(x, y, z)))
  }

  fn visit_vector(&self, state: ParserState, vector: &yaml::Yaml) -> ParserResult<Tuple> {
    let (state, vector_array) = to_array!(state, vector)?;
    let (mut state, x_value) = safe_index!(state, vector_array, 0)?;
    let (state, x) = with_state!(state, Segment::Index(0), to_float!(state, x_value));
    let (mut state, y_value) = safe_index!(state, vector_array, 1)?;
    let (state, y) = with_state!(state, Segment::Index(1), to_float!(state, y_value));
    let (mut state, z_value) = safe_index!(state, vector_array, 2)?;
    let (state, z) = with_state!(state, Segment::Index(2), to_float!(state, z_value));
    Ok((state, Tuple::vector(x, y, z)))
  }

  fn visit_color(&self, state: ParserState, color: &yaml::Yaml) -> ParserResult<Color> {
    let (state, color_array) = to_array!(state, color)?;
    let (mut state, r_value) = safe_index!(state, color_array, 0)?;
    let (state, r) = with_state!(state, Segment::Index(0), to_float!(state, r_value));
    let (mut state, g_value) = safe_index!(state, color_array, 1)?;
    let (state, g) = with_state!(state, Segment::Index(1), to_float!(state, g_value));
    let (mut state, b_value) = safe_index!(state, color_array, 2)?;
    let (state, b) = with_state!(state, Segment::Index(2), to_float!(state, b_value));
    Ok((state, Color::new(r, g, b)))
  }

  fn visit_body(&self, state: ParserState, body: &yaml::Yaml) -> ParserResult<Body> {
    let mut material = Material::default();
    let mut transform = Matrix::identity();

    let (mut state, body_hash) = to_hash!(state, body)?;

    let (mut state, body_type) = with_state!(
      state,
      Segment::Key("type".into()),
      to_string!(state, body_hash, "type")
    );

    if body_hash.contains_key(key!("material")) {
      let (new_state, material_value) = safe_key!(state, body_hash, "material")?;
      let material_result = self.visit_material(new_state, material_value)?;
      state = material_result.0;
      material = material_result.1;
    }

    if body_hash.contains_key(key!("transforms")) {
      let (new_state, transforms_value) = safe_key!(state, body_hash, "transforms")?;
      let transforms_result = self.visit_transforms(new_state, transforms_value)?;
      state = transforms_result.0;
      transform = transforms_result.1;
    }

    match body_type.as_str() {
      "sphere" => Ok((state, Body::from(Sphere::new(material, transform)))),
      "plane" => Ok((state, Body::from(Plane::new(material, transform)))),
      _ => Err(Box::new(ParseError::new(
        state.path,
        format!("Unknown body type {} found.", body_type),
      ))),
    }
  }

  fn visit_material(&self, state: ParserState, material: &yaml::Yaml) -> ParserResult<Material> {
    let (mut state, material_hash) = to_hash!(state, material)?;
    let (state, material_type) = with_state!(
      state,
      Segment::Key("type".into()),
      to_string!(state, material_hash, "type")
    );

    if material_type.eq("phong") {
      let (mut state, material_color) = safe_key!(state, material_hash, "color")?;
      let (state, material_color) = with_state!(
        state,
        Segment::Key("color".into()),
        self.visit_color(state, material_color)
      );
      let (mut state, material_diffuse) = safe_key!(state, material_hash, "diffuse")?;
      let (state, material_diffuse) = with_state!(
        state,
        Segment::Key("diffuse".into()),
        to_float!(state, material_diffuse)
      );
      let (mut state, material_ambient) = safe_key!(state, material_hash, "ambient")?;
      let (state, material_ambient) = with_state!(
        state,
        Segment::Key("ambient".into()),
        to_float!(state, material_ambient)
      );
      let (mut state, material_specular) = safe_key!(state, material_hash, "specular")?;
      let (state, material_specular) = with_state!(
        state,
        Segment::Key("specular".into()),
        to_float!(state, material_specular)
      );
      let (mut state, material_shininess) = safe_key!(state, material_hash, "shininess")?;
      let (state, material_shininess) = with_state!(
        state,
        Segment::Key("shininess".into()),
        to_float!(state, material_shininess)
      );
      Ok((
        state,
        Material::from(Phong::new(
          material_color,
          material_ambient,
          material_diffuse,
          material_specular,
          material_shininess,
        )),
      ))
    } else {
      Err(Box::new(ParseError::new(
        state.path,
        format!(
          "Found material, with unknown material type {}.",
          material_type
        ),
      )))
    }
  }

  fn visit_transforms(
    &self,
    state: ParserState,
    transforms: &yaml::Yaml,
  ) -> ParserResult<Matrix<4>> {
    let (mut state, transforms_array) = to_array!(state, transforms)?;
    let mut combined_transform = Matrix::identity();
    for (index, transform) in transforms_array.iter().enumerate().rev() {
      state.path.push(Segment::Index(index));
      let (new_state, next_transform) = self.visit_transform(state, transform)?;
      combined_transform = combined_transform * next_transform;
      state = new_state;
      state.path.pop();
    }

    Ok((state, combined_transform))
  }

  fn visit_transform(&self, state: ParserState, transform: &yaml::Yaml) -> ParserResult<Matrix<4>> {
    let (mut state, transform_hash) = to_hash!(state, transform)?;
    let (state, transform_type) = with_state!(
      state,
      Segment::Key("type".into()),
      to_string!(state, transform_hash, "type")
    );

    if transform_type.eq("translate") {
      let (new_state, to) = safe_key!(state, transform_hash, "to")?;
      let (new_state, v) = self.visit_vector(new_state, to)?;
      Ok((new_state, Matrix::translation(v.x, v.y, v.z)))
    } else if transform_type.eq("scale") {
      let (new_state, to) = safe_key!(state, transform_hash, "to")?;
      let (new_state, v) = self.visit_vector(new_state, to)?;
      Ok((new_state, Matrix::scaling(v.x, v.y, v.z)))
    } else if transform_type.eq("rotate_x") {
      let (new_state, radians) = to_float!(state, transform_hash, "radians")?;
      Ok((new_state, Matrix::rotation_x(radians)))
    } else if transform_type.eq("rotate_y") {
      let (new_state, radians) = to_float!(state, transform_hash, "radians")?;
      Ok((new_state, Matrix::rotation_y(radians)))
    } else if transform_type.eq("rotate_z") {
      let (new_state, radians) = to_float!(state, transform_hash, "radians")?;
      Ok((new_state, Matrix::rotation_z(radians)))
    } else {
      Err(Box::new(ParseError::new(
        state.path,
        format!(
          "Found transform, with unknown transform type {}.",
          transform_type
        ),
      )))
    }
  }

  fn visit_camera(
    &self,
    state: ParserState,
    camera: &yaml::Yaml,
  ) -> ParserResult<(String, Camera)> {
    let (mut state, camera_hash) = to_hash!(state, camera)?;
    let (mut state, camera_name) = with_state!(
      state,
      Segment::Key("name".into()),
      to_string!(state, camera_hash, "name")
    );
    let (mut state, width) = with_state!(
      state,
      Segment::Key("width".into()),
      to_int!(state, camera_hash, "width")
    );
    let (mut state, height) = with_state!(
      state,
      Segment::Key("height".into()),
      to_int!(state, camera_hash, "height")
    );
    let (state, fov) = with_state!(
      state,
      Segment::Key("fov".into()),
      to_float!(state, camera_hash, "field_of_view")
    );
    let (mut state, to_value) = safe_key!(state, camera_hash, "to")?;
    let (state, to) = with_state!(
      state,
      Segment::Key("to".into()),
      self.visit_point(state, to_value)
    );
    let (mut state, from_value) = safe_key!(state, camera_hash, "from")?;
    let (state, from) = with_state!(
      state,
      Segment::Key("from".into()),
      self.visit_point(state, from_value)
    );
    let (mut state, up_value) = safe_key!(state, camera_hash, "up")?;
    let (state, up) = with_state!(
      state,
      Segment::Key("up".into()),
      self.visit_vector(state, up_value)
    );

    let camera = Camera::new(width.abs() as usize, height.abs() as usize, fov)
      .look_at_from_position(from, to, up);
    Ok((state, (camera_name.into(), camera)))
  }
}

impl WorldLoader for Yaml {
  fn load_world(&self, source: &str) -> LoaderResult {
    let documents = self.parse_yaml(source)?;
    let state = ParserState::new();
    let (_, result) = self.visit_documents(state, &documents)?;
    Ok(result)
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

    let yaml_loader = Yaml::new();

    let (loaded_world, loaded_cameras) = yaml_loader.load_world(source).unwrap();
    assert_fuzzy_eq!(loaded_world, expected_world);
    assert_fuzzy_eq!(loaded_cameras, expected_cameras);
  }
}
