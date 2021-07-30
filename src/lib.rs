use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
mod fuzzy_eq;

type F = f64;

pub mod body;
pub mod canvas;
pub mod computed_intersection;
pub mod intersections;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod tuple;
pub mod world;

use web_sys::CanvasRenderingContext2d;

use crate::body::Intersectable;

use crate::canvas::Color;
use crate::light::PointLight;
use crate::material::{Illuminated, Material, Phong};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

// #[wasm_bindgen]
// extern "C" {
//   // Use `js_namespace` here to bind `console.log(..)` instead of just
//   // `log(..)`
//   #[wasm_bindgen(js_namespace = console)]
//   fn log(s: &str);
// }

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.
// macro_rules! console_log {
//     // Note that this is using the `log` function imported above during
//     // `bare_bones`
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
pub struct World {
  ray_origin: Tuple,
  wall_position_z: F,
  wall_size: F,

  canvas_pixel_world_size: F,

  sphere: Sphere,

  light: PointLight,
}

#[wasm_bindgen]
impl World {
  pub fn new(canvas_size: usize) -> Self {
    let material = Material::from(Phong::with_color(Color::new(0.1, 0.75, 1.0)));
    let wall_size = 10.0;

    World {
      ray_origin: Tuple::point(0.0, 0.0, -5.0),
      wall_position_z: 11.0,
      wall_size,
      canvas_pixel_world_size: wall_size / canvas_size as f64,
      sphere: Sphere::with_material(material, None),

      light: PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0)),
    }
  }

  pub fn render(&self, context: &CanvasRenderingContext2d, x: f64, y: f64) {
    let half = self.wall_size / 2.0;
    let world_x = -half + x * self.canvas_pixel_world_size;
    let world_y = half - y * self.canvas_pixel_world_size;

    let wall_point = Tuple::point(world_x, world_y, self.wall_position_z);

    let ray = Ray::new(self.ray_origin, (wall_point - self.ray_origin).normalize());

    let xs = self.sphere.intersect(ray);

    let hit = xs.hit();

    if let Some(hit) = hit {
      let computed = hit.get_computed();
      let color =
        hit
          .body
          .material()
          .lighting(self.light, computed.point, computed.eyev, computed.normalv);

      let rgba = format!(
        "rgba({},{},{}, 1.0)",
        (color.red * 255.0).round(),
        (color.green * 255.0).round(),
        (color.blue * 255.0).round(),
      );

      context.set_fill_style(&rgba.into());
      context.fill_rect(x, y, 1.0, 1.0);
    } else {
      let rgba = "rgba(0, 0, 0, 1.0)";

      context.set_fill_style(&rgba.into());
      context.fill_rect(x, y, 1.0, 1.0);
    }
  }
}
