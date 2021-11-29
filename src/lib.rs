#[macro_use]
mod fuzzy_eq;

type F = f64;
pub const EPSILON: f64 = 0.00001;

pub mod animator;
pub mod body;
pub mod camera;
pub mod canvas;
pub mod computed_intersection;
pub mod intersections;
pub mod light;
pub mod material;
pub mod matrix;
pub mod plane;
pub mod ray;
pub mod sphere;
pub mod tuple;
pub mod world;
pub mod world_loader;
