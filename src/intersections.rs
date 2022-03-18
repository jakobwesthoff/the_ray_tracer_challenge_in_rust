use crate::computed_intersection::ComputedIntersection;
use crate::ray::Ray;
use crate::F;
use crate::{body::*, EPSILON};
use core::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
  pub t: F,
  pub ray: Ray,
  pub body: Body,
}

impl Intersection {
  pub fn new(t: F, ray: Ray, body: Body) -> Intersection {
    Intersection { t, ray, body }
  }

  pub fn get_computed(&self) -> ComputedIntersection {
    let position = self.ray.position(self.t);
    let mut normalv = self.body.normal_at(position);
    let eyev = -self.ray.direction;
    let inside = normalv.dot(eyev) < 0.0;

    if inside {
      normalv = -normalv;
    }

    let over_point = position + normalv * EPSILON;

    let reflectv = self.ray.direction.reflect(normalv);

    ComputedIntersection::new(self, position, over_point, normalv, eyev, reflectv, inside)
  }
}

pub struct Intersections {
  data: Vec<Intersection>,
}

impl Intersections {
  pub fn new(mut intersections: Vec<Intersection>) -> Self {
    intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    Intersections {
      data: intersections,
    }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn hit(&self) -> Option<&Intersection> {
    for intersection in self.data.iter() {
      if intersection.t > 0.0 {
        return Some(intersection);
      }
    }

    None
  }
}

impl From<Vec<Intersection>> for Intersections {
  fn from(v: Vec<Intersection>) -> Self {
    Self::new(v)
  }
}

impl Index<usize> for Intersections {
  type Output = Intersection;
  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl IntoIterator for Intersections {
  type Item = Intersection;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_iter()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fuzzy_eq::*;
  use crate::material::Material;
  use crate::matrix::Matrix;
  use crate::plane::Plane;
  use crate::sphere::Sphere;
  use crate::tuple::Tuple;

  #[test]
  fn the_hit_when_all_intersections_have_positive_t() {
    let s = Sphere::default();

    let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

    let i1 = Intersection::new(1.0, r, Body::from(s));
    let i2 = Intersection::new(2.0, r, Body::from(s));

    let xs = Intersections::new(vec![i2, i1]);

    assert_eq!(xs.hit(), Some(&i1));
  }

  #[test]
  fn the_hit_when_some_intersections_have_negative_t() {
    let s = Sphere::default();

    let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

    let i1 = Intersection::new(-1.0, r, Body::from(s));
    let i2 = Intersection::new(1.0, r, Body::from(s));

    let xs = Intersections::new(vec![i2, i1]);

    assert_eq!(xs.hit(), Some(&i2));
  }

  #[test]
  fn the_hit_when_all_intersections_have_negative_t() {
    let s = Sphere::default();

    let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

    let i1 = Intersection::new(-2.0, r, Body::from(s));
    let i2 = Intersection::new(-1.0, r, Body::from(s));

    let xs = Intersections::new(vec![i2, i1]);

    assert_eq!(xs.hit(), None);
  }

  #[test]
  fn precomputing_the_state_of_an_intersection() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let body = Body::from(Sphere::default());
    let i = Intersection::new(4.0, r, body);
    let c = i.get_computed();

    assert_eq!(c.intersection, &i);
    assert_fuzzy_eq!(c.point, Tuple::point(0.0, 0.0, -1.0));
    assert_fuzzy_eq!(c.eyev, Tuple::vector(0.0, 0.0, -1.0));
    assert_fuzzy_eq!(c.normalv, Tuple::vector(0.0, 0.0, -1.0));
  }

  #[test]
  fn precomputing_reflection_vector() {
    let body = Body::from(Plane::default());
    let r = Ray::new(
      Tuple::point(0.0, 1.0, -1.0),
      Tuple::vector(0.0, -(2.0 as F).sqrt() / 2.0, (2.0 as F).sqrt() / 2.0),
    );
    let intersection = Intersection::new((2.0 as F).sqrt(), r, body);

    let computations = intersection.get_computed();

    assert_fuzzy_eq!(
      computations.reflectv,
      Tuple::vector(0.0, (2.0 as F).sqrt() / 2.0, (2.0 as F).sqrt() / 2.0)
    );
  }

  #[test]
  fn the_hit_when_an_intersection_occurs_on_the_outside() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let body = Body::from(Sphere::default());
    let i = Intersection::new(4.0, r, body);
    let c = i.get_computed();

    assert_eq!(c.inside, false);
  }

  #[test]
  fn the_hit_when_an_intersection_occurs_on_the_inside() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let body = Body::from(Sphere::default());
    let i = Intersection::new(1.0, r, body);
    let c = i.get_computed();

    assert_eq!(c.inside, true);
    assert_eq!(c.normalv, Tuple::vector(0.0, 0.0, -1.0));
  }

  #[test]
  fn the_hit_should_offset_the_point() {
    let material = Material::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s1 = Sphere::new(material, Matrix::translation(0.0, 0.0, 1.0));
    let i = Intersection::new(5.0, r, s1.into());
    let c = i.get_computed();
    assert!(c.over_point.z < -EPSILON / 2.0);
    assert!(c.point.z > c.over_point.z);
  }
}
