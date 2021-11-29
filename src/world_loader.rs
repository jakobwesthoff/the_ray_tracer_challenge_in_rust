use std::collections::HashMap;
use std::error::Error;

use crate::camera::Camera;
use crate::world::World;

pub type LoaderResult = Result<(World, HashMap<String, Camera>), Box<dyn Error>>;
pub trait WorldLoader {
  fn load_world(&self, source: &str) -> LoaderResult;
}

pub mod yaml;
