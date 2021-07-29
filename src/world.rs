use crate::body::{Body, Intersectable};
use crate::canvas::Color;
use crate::intersections::Intersections;
use crate::light::PointLight;
use crate::material::Illuminated;
use crate::ray::Ray;
pub struct World {
  pub bodies: Vec<Body>,
  pub lights: Vec<PointLight>,
}

impl World {
  pub fn new(bodies: Vec<Body>, lights: Vec<PointLight>) -> Self {
    World { bodies, lights }
  }

  pub fn intersect(&self, ray: Ray) -> Intersections {
    let xs = self
      .bodies
      .iter()
      .flat_map(|body| body.intersect(ray))
      .collect();
    Intersections::new(xs)
  }

  pub fn color_at(&self, ray: Ray) -> Color {
    let xs = self.intersect(ray);
    let hit = xs.hit();
    if let Some(hit) = hit {
      let c = hit.get_computed();
      let material = hit.body.material();
      // @TODO: Implement proper lighting using multiple light sources
      material.lighting(self.lights[0], c.point, c.eyev, c.normalv)
    } else {
      Color::black()
    }
  }
}

impl Default for World {
  fn default() -> Self {
    World {
      bodies: vec![],
      lights: vec![],
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::canvas::Color;
  use crate::fuzzy_eq::*;
  use crate::material::{Material, Phong};
  use crate::matrix::Matrix;
  use crate::sphere::Sphere;
  use crate::tuple::Tuple;

  use super::*;

  fn create_default_world() -> World {
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let material = Phong {
      color: Color::new(0.8, 1.0, 0.6),
      diffuse: 0.7,
      specular: 0.2,
      ..Phong::default()
    };
    let s1 = Body::from(Sphere::with_material(Material::from(material), None));
    let s2 = Body::from(Sphere::new(Some(Matrix::scaling(0.5, 0.5, 0.5))));

    World::new(vec![s1, s2], vec![light])
  }

  #[test]
  fn the_default_world() {
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let material = Phong {
      color: Color::new(0.8, 1.0, 0.6),
      diffuse: 0.7,
      specular: 0.2,
      ..Phong::default()
    };
    let s1 = Body::from(Sphere::with_material(Material::from(material), None));
    let s2 = Body::from(Sphere::new(Some(Matrix::scaling(0.5, 0.5, 0.5))));

    let world = create_default_world();

    assert_eq!(2, world.bodies.len());
    assert_eq!(1, world.lights.len());

    assert!(world.bodies.contains(&s1));
    assert!(world.bodies.contains(&s2));

    assert!(world.lights.contains(&light));
  }

  #[test]
  fn intersect_a_world_with_a_ray() {
    let w = create_default_world();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = w.intersect(r);

    assert_eq!(4, xs.len());
    assert_fuzzy_eq!(4.0, xs[0].t);
    assert_fuzzy_eq!(4.5, xs[1].t);
    assert_fuzzy_eq!(5.5, xs[2].t);
    assert_fuzzy_eq!(6.0, xs[3].t);
  }

  #[test]
  fn the_color_when_a_ray_misses() {
    let w = create_default_world();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
    let c = w.color_at(r);

    assert_fuzzy_eq!(c, Color::black());
  }

  #[test]
  fn the_color_when_a_ray_hits() {
    let w = create_default_world();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let c = w.color_at(r);

    assert_fuzzy_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
  }
}
