use crate::EPSILON;
use crate::body::{Body, Intersectable};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plane {
  material: Material,
  transform: Matrix<4>
}

impl Default for Plane {
  fn default() -> Self {
    Self {
      transform: Matrix::identity(),
      material: Default::default(),
    }
  }
}

impl Plane {
  pub fn new(material: Material, transform: Matrix<4>) -> Self {
    Self {
      material,
      transform,
    }
  }

  pub fn with_material(mut self, material: Material) -> Self {
    self.material = material;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }
}

impl Intersectable for Plane {
    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn intersect_in_object_space(&self, object_space_ray: crate::ray::Ray) -> Vec<(crate::F, crate::body::Body)> {
        if object_space_ray.direction.y.abs() <=EPSILON {
          return vec![];
        }

        let t = -object_space_ray.origin.y / object_space_ray.direction.y;
        vec![(t, Body::from(*self))]
    }

    fn normal_at_in_object_space(&self, _object_space_point: crate::tuple::Tuple) -> crate::tuple::Tuple {
      Tuple::vector(0.0, 1.0, 0.0)
    }
}



#[cfg(test)]
mod tests {
  use super::*;
  use crate::fuzzy_eq::*;
use crate::ray::Ray;

  #[test]
  fn normal_of_a_plane_is_constant_everywhere() {
    let p = Plane::default();
    let n1 = p.normal_at_in_object_space(Tuple::point(0.0, 0.0, 0.0));
    let n2 = p.normal_at_in_object_space(Tuple::point(10.0, 0.0, -10.0));
    let n3 = p.normal_at_in_object_space(Tuple::point(-5.0, 0.0, 150.0));

    assert_fuzzy_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
    assert_fuzzy_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
    assert_fuzzy_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
  }

  #[test]
  fn intersect_with_a_ray_parallel_to_the_plane() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,10.0, 0.0), Tuple::vector(0.0,0.0,1.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 0);
  }

  #[test]
  fn intersect_with_a_coplanar_ray() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,0.0, 0.0), Tuple::vector(0.0,0.0,1.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 0);
  }

  #[test]
  fn intersect_from_above() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,1.0, 0.0), Tuple::vector(0.0,-1.0,0.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 1);
    assert_fuzzy_eq!(ts[0].0, 1.0); // t
    assert_eq!(ts[0].1, Body::from(p)) // body
  }

  #[test]
  fn intersect_from_below() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,-1.0, 0.0), Tuple::vector(0.0,1.0,0.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 1);
    assert_fuzzy_eq!(ts[0].0, 1.0); // t
    assert_eq!(ts[0].1, Body::from(p)) // body
  }
}
