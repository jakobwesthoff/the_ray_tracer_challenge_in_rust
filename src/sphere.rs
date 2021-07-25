use crate::body::*;
use crate::intersections::*;
use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
  pub transform: Matrix<4>,
  pub material: Material,
}

impl Sphere {
  pub fn new(transform: Option<Matrix<4>>) -> Self {
    match transform {
      None => Sphere {
        transform: Matrix::identity(),
        material: Material::default(),
      },
      Some(transform) => Sphere {
        transform,
        material: Material::default(),
      },
    }
  }

  pub fn with_material(material: Material, transform: Option<Matrix<4>>) -> Self {
    // @TODO: Find a better way to reuse the code here, without changing all
    // current invocations
    let mut s = Self::new(transform);
    s.material = material;
    s
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

  fn normal_at(&self, point: Tuple) -> Tuple {
    let object_point = self.transform.inverse() * point;
    let object_normal = (object_point - Tuple::point(0.0, 0.0, 0.0)).normalize();
    let mut world_normal = self.transform.inverse().transpose() * object_normal;
    // Hack, to ensure we have a clean vector, as due the inverse transpose the
    // w component could be affected if the transformation matrix included a
    // translation
    world_normal.w = 0.0;
    world_normal.normalize()
  }

  fn material(&self) -> Material {
    self.material
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::canvas::Color;
  use crate::fuzzy_eq::*;
  use crate::F;
  use std::f64::consts::PI;

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

  #[test]
  fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Sphere::new(None);
    let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));

    let expected_result = Tuple::vector(1.0, 0.0, 0.0);

    assert_fuzzy_eq!(n, expected_result);
  }

  #[test]
  fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Sphere::new(None);
    let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));

    let expected_result = Tuple::vector(0.0, 1.0, 0.0);

    assert_fuzzy_eq!(n, expected_result);
  }

  #[test]
  fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let s = Sphere::new(None);
    let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));

    let expected_result = Tuple::vector(0.0, 0.0, 1.0);

    assert_fuzzy_eq!(n, expected_result);
  }

  #[test]
  fn the_normal_on_a_sphere_at_a_non_axial_point() {
    let s = Sphere::new(None);
    let sqrt3_over_3 = (3.0 as F).sqrt() / 3.0;
    let p = Tuple::point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3);
    let n = s.normal_at(p);

    let expected_result = Tuple::vector(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3);

    assert_fuzzy_eq!(n, expected_result);
  }

  #[test]
  fn computing_the_normal_on_a_translated_sphere() {
    let s = Sphere::new(Some(Matrix::translation(0.0, 1.0, 0.0)));
    let p = Tuple::point(0.0, 1.70711, -0.70711);
    let n = s.normal_at(p);

    let expected_result = Tuple::vector(0.0, 0.70711, -0.70711);

    assert_fuzzy_eq!(n, expected_result);
  }

  #[test]
  fn computing_the_normal_on_a_scaled_and_rotated_sphere() {
    let s = Sphere::new(Some(
      Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0),
    ));
    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let p = Tuple::point(0.0, sqrt2_over_2, -sqrt2_over_2);
    let n = s.normal_at(p);

    let expected_result = Tuple::vector(0.0, 0.97014, -0.24254);

    assert_fuzzy_eq!(n, expected_result);
  }

  #[test]
  fn the_normal_vector_is_always_normalized() {
    let s = Sphere::new(None);
    let sqrt3_over_3 = (3.0 as F).sqrt() / 3.0;
    let p = Tuple::point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3);
    let n = s.normal_at(p);

    assert_fuzzy_eq!(n.normalize(), n);
  }

  #[test]
  fn the_normal_vector_is_normalized_on_transformed_sphere() {
    let s = Sphere::new(Some(
      Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0),
    ));
    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let p = Tuple::point(0.0, sqrt2_over_2, -sqrt2_over_2);
    let n = s.normal_at(p);

    assert_fuzzy_eq!(n.normalize(), n);
  }

  #[test]
  fn sphere_has_default_phong_material() {
    let s = Sphere::new(None);
    let m = Material::default();

    assert_fuzzy_eq!(s.material, m);
  }

  #[test]
  fn sphere_may_be_assigned_a_material() {
    let m = Material::from(Phong::new(
      Color::new(1.0, 1.0, 0.0),
      0.05,
      0.7,
      0.95,
      400.0,
    ));
    let s = Sphere::with_material(m, None);

    assert_fuzzy_eq!(s.material, m);
  }
}
