use crate::body::*;
use crate::computed_intersection::ComputedIntersection;
use crate::ray::Ray;
use crate::F;
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
    let normalv = self.body.normal_at(position);
    let eyev = -self.ray.direction;

    ComputedIntersection::new(self, position, normalv, eyev)
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
  use crate::sphere::Sphere;
  use crate::tuple::Tuple;
  use crate::fuzzy_eq::*;

  #[test]
  fn the_hit_when_all_intersections_have_positive_t() {
    let s = Sphere::new(None);

    let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

    let i1 = Intersection::new(1.0, r, Body::from(s));
    let i2 = Intersection::new(2.0, r, Body::from(s));

    let xs = Intersections::new(vec![i2, i1]);

    assert_eq!(xs.hit(), Some(&i1));
  }

  #[test]
  fn the_hit_when_some_intersections_have_negative_t() {
    let s = Sphere::new(None);

    let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

    let i1 = Intersection::new(-1.0, r, Body::from(s));
    let i2 = Intersection::new(1.0, r, Body::from(s));

    let xs = Intersections::new(vec![i2, i1]);

    assert_eq!(xs.hit(), Some(&i2));
  }

  #[test]
  fn the_hit_when_all_intersections_have_negative_t() {
    let s = Sphere::new(None);

    let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));

    let i1 = Intersection::new(-2.0, r, Body::from(s));
    let i2 = Intersection::new(-1.0, r, Body::from(s));

    let xs = Intersections::new(vec![i2, i1]);

    assert_eq!(xs.hit(), None);
  }

  #[test]
  fn precomputing_the_state_of_an_intersection() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let body = Body::from(Sphere::new(None));
    let i = Intersection::new(4.0, r, body);
    let c = i.get_computed();

    assert_eq!(c.intersection, &i);
    assert_fuzzy_eq!(c.point, Tuple::point(0.0, 0.0, -1.0));
    assert_fuzzy_eq!(c.eyev, Tuple::vector(0.0, 0.0, -1.0));
    assert_fuzzy_eq!(c.normalv, Tuple::vector(0.0, 0.0, -1.0));
  }
}
