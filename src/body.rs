use crate::material::Material;
use crate::tuple::*;
use crate::intersections::*;
use crate::ray::*;
use crate::sphere::*;

pub trait Intersectable {
  fn intersect(&self, ray: Ray) -> Intersections;
  fn normal_at(&self, point: Tuple) -> Tuple;
  fn material(&self) -> Material;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Body {
  Sphere(Sphere),
}

impl From<Sphere> for Body {
  fn from(sphere: Sphere) -> Self {
    Body::Sphere(sphere)
  }
}

impl Intersectable for Body {
  fn intersect(&self, ray: Ray) -> Intersections {
    match *self {
      Body::Sphere(ref sphere) => sphere.intersect(ray),
    }
  }

  fn normal_at(&self, point: Tuple) -> Tuple {
    match *self {
      Body::Sphere(ref sphere) => sphere.normal_at(point),
    }
  }

  fn material(&self) -> Material {
    match *self {
      Body::Sphere(ref sphere) => sphere.material(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fuzzy_eq::*;

  #[test]
  fn an_intersection_encapsulates_t_and_object() {
    let s = Sphere::new(None);

    let i = Intersection::new(3.5, Body::from(s));
    assert_fuzzy_eq!(i.t, 3.5);
    assert_eq!(i.body, Body::from(s));
  }
}
