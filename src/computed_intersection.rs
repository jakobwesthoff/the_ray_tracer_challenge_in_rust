use crate::intersections::Intersection;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct ComputedIntersection<'a> {
  pub intersection: &'a Intersection,
  pub point: Tuple,
  pub normalv: Tuple,
  pub eyev: Tuple,
  pub inside: bool,
}

impl<'a> ComputedIntersection<'a> {
  pub fn new(intersection: &'a Intersection, point: Tuple, normalv: Tuple, eyev: Tuple, inside: bool) -> Self {
    ComputedIntersection {
      intersection,
      point,
      normalv,
      eyev,
      inside,
    }
  }
}
