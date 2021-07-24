use crate::body::*;
use crate::intersections::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
  pub transform: Matrix<4>,
}

impl Sphere {
  pub fn new(transform: Option<Matrix<4>>) -> Self {
    match transform {
      None => Sphere {
        transform: Matrix::identity(),
      },
      Some(transform) => Sphere { transform },
    }
  }
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: Ray) -> Intersections {
    let object_space_ray = ray.transform(self.transform.inverse());

    let sphere_to_ray = object_space_ray.origin - Tuple::point(0.0, 0.0, 0.0);
    let a = object_space_ray.direction.dot(object_space_ray.direction);
    let b = 2.0 * object_space_ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
    let descriminant = b.powi(2) - 4.0 * a * c;

    if descriminant < 0.0 {
      Intersections::new(vec![])
    } else {
      let t1 = (-b - descriminant.sqrt()) / (2.0 * a);
      let t2 = (-b + descriminant.sqrt()) / (2.0 * a);
      Intersections::new(vec![
        Intersection::new(t1, Body::from(*self)),
        Intersection::new(t2, Body::from(*self)),
      ])
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fuzzy_eq::*;

  #[test]
  fn a_ray_intersects_a_sphere_at_two_points() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new(None);

    let xs = s.intersect(r);

    assert_eq!(2, xs.len());
    assert_fuzzy_eq!(4.0, xs[0].t);
    assert_fuzzy_eq!(6.0, xs[1].t);
  }

  #[test]
  fn a_ray_intersects_a_sphere_at_a_tangent() {
    let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new(None);

    let xs = s.intersect(r);

    assert_eq!(2, xs.len());
    assert_fuzzy_eq!(5.0, xs[0].t);
    assert_fuzzy_eq!(5.0, xs[1].t);
  }

  #[test]
  fn a_ray_misses_a_sphere() {
    let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new(None);

    let xs = s.intersect(r);

    assert_eq!(0, xs.len());
  }

  #[test]
  fn a_ray_originates_inside_a_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new(None);

    let xs = s.intersect(r);

    assert_eq!(2, xs.len());
    assert_fuzzy_eq!(-1.0, xs[0].t);
    assert_fuzzy_eq!(1.0, xs[1].t);
  }

  #[test]
  fn a_sphere_is_behind_a_ray() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new(None);

    let xs = s.intersect(r);

    assert_eq!(2, xs.len());
    assert_fuzzy_eq!(-6.0, xs[0].t);
    assert_fuzzy_eq!(-4.0, xs[1].t);
  }

  #[test]
  fn a_spheres_default_transform() {
    let s = Sphere::new(None);
    assert_fuzzy_eq!(s.transform, Matrix::identity());
  }

  #[test]
  fn changing_a_spheres_transform() {
    let mut s = Sphere::new(None);
    let m = Matrix::translation(2.0, 3.0, 4.0);
    s.transform = m;

    assert_fuzzy_eq!(s.transform, m);
  }

  #[test]
  fn intersecting_a_scaled_sphere_with_a_ray() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new(Some(Matrix::scaling(2.0, 2.0, 2.0)));

    let xs = s.intersect(r);

    assert_eq!(2, xs.len());
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
  }

  #[test]
  fn intersecting_a_translated_sphere_with_a_ray() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new(Some(Matrix::translation(5.0, 0.0, 0.0)));

    let xs = s.intersect(r);

    assert_eq!(0, xs.len());
  }
}
