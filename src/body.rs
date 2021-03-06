use crate::fuzzy_eq::FuzzyEq;
use crate::intersections::*;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::plane::Plane;
use crate::ray::*;
use crate::sphere::*;
use crate::tuple::*;
use crate::F;

pub trait Intersectable {
  fn material(&self) -> Material;
  fn transform(&self) -> Matrix<4>;
  fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(F, Body)>;
  fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple;

  fn intersect(&self, ray: Ray) -> Intersections {
    let object_space_ray = ray.transform(self.transform().inverse());
    let ts = self.intersect_in_object_space(object_space_ray);
    Intersections::new(
      ts.into_iter()
        .map(|(t, body)| Intersection::new(t, ray, body))
        .collect(),
    )
  }

  fn normal_at(&self, point: Tuple) -> Tuple {
    let object_space_point = self.transform().inverse() * point;

    let object_normal = self.normal_at_in_object_space(object_space_point);

    let mut world_normal = self.transform().inverse().transpose() * object_normal;
    // Hack, to ensure we have a clean vector, as due the inverse transpose the
    // w component could be affected if the transformation matrix included a
    // translation
    world_normal.w = 0.0;
    world_normal.normalize()
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Body {
  Sphere(Sphere),
  Plane(Plane),
}

impl From<Sphere> for Body {
  fn from(sphere: Sphere) -> Self {
    Body::Sphere(sphere)
  }
}

impl From<Plane> for Body {
  fn from(plane: Plane) -> Self {
    Body::Plane(plane)
  }
}

impl Intersectable for Body {
  fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(F, Body)> {
    match *self {
      Body::Sphere(ref sphere) => sphere.intersect_in_object_space(object_space_ray),
      Body::Plane(ref plane) => plane.intersect_in_object_space(object_space_ray),
    }
  }

  fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple {
    match *self {
      Body::Sphere(ref sphere) => sphere.normal_at_in_object_space(object_space_point),
      Body::Plane(ref plane) => plane.normal_at_in_object_space(object_space_point),
    }
  }

  fn material(&self) -> Material {
    match *self {
      Body::Sphere(ref sphere) => sphere.material(),
      Body::Plane(ref plane) => plane.material(),
    }
  }

  fn transform(&self) -> Matrix<4> {
    match *self {
      Body::Sphere(ref sphere) => sphere.transform(),
      Body::Plane(ref plane) => plane.transform(),
    }
  }
}

impl FuzzyEq<Body> for Body {
  fn fuzzy_eq(&self, other: Body) -> bool {
    match (*self, other) {
      (Body::Sphere(ref sphere), Body::Sphere(ref other)) => sphere.fuzzy_eq(other),
      (Body::Plane(ref plane), Body::Plane(ref other)) => plane.fuzzy_eq(other),
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn an_intersection_encapsulates_t_and_object() {
    let s = Sphere::default();

    let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));
    let i = Intersection::new(3.5, r, Body::from(s));
    assert_fuzzy_eq!(i.t, 3.5);
    assert_eq!(i.body, Body::from(s));
  }
}
