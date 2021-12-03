use std::collections::HashMap;

use crate::camera::Camera;
use crate::world::World;

pub type LoaderResult = anyhow::Result<(World, HashMap<String, Camera>)>;
pub trait WorldLoader {
  fn load_world<T: AsRef<str>>(&self, source: T) -> LoaderResult;
}

pub mod yaml;
