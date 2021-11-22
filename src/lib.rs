#[macro_use]
mod fuzzy_eq;

type F = f64;
pub const EPSILON: f64 = 0.00001;

pub mod canvas;
pub mod matrix;
pub mod tuple;
pub mod ray;
pub mod body;
pub mod intersections;
pub mod sphere;
pub mod plane;
pub mod light;
pub mod material;
pub mod world;
pub mod computed_intersection;
pub mod camera;
pub mod animator;
