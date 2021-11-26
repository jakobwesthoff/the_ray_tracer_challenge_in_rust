use crate::EPSILON;

pub trait FuzzyEq<T: Clone> {
  fn fuzzy_eq(&self, other: T) -> bool;

  fn fuzzy_ne(&self, other: T) -> bool {
    !self.fuzzy_eq(other)
  }
}

impl FuzzyEq<f64> for f64 {
  fn fuzzy_eq(&self, other: f64) -> bool {
    (*self - other).abs() < EPSILON
  }
}

impl<T> FuzzyEq<Vec<T>> for Vec<T>
where
  T: FuzzyEq<T>,
  T: Clone,
{
  fn fuzzy_eq(&self, other: Vec<T>) -> bool {
    if self.len() != other.len() {
      return false;
    }

    for (index, item) in self.iter().enumerate() {
      if item.fuzzy_ne(other[index].clone()) {
        return false;
      }
    }

    return true;
  }
}

// Not really sure what I am doing here, as I don't have a great understanding of macros yet.
// Feel free to fix or enhance in the future.
// @TODO: Check if we can ensure more explicitly the two operands implement the `FuzzyEq` trait
#[macro_export]
macro_rules! assert_fuzzy_eq {
  ($left:expr, $right:expr $(,)?) => {{
    match (&$left, $right) {
      (left_val, right_val) => {
        if left_val.fuzzy_ne(right_val.clone()) {
          panic!(
            "asserting fuzzy equality. {:?} is not fuzzy equal to {:?}",
            left_val, right_val
          );
        }
      }
    }
  }};
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
  ($left:expr, $right:expr $(,)?) => {{
    match (&$left, $right) {
      (left_val, right_val) => {
        if left_val.fuzzy_eq(right_val) {
          panic!(
            "asserting fuzzy in-equality. {:?} is fuzzy equal to {:?}",
            left_val, right_val
          );
        }
      }
    }
  }};
}
