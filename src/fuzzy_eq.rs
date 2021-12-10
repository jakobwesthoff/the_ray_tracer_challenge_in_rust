use std::collections::HashMap;

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

    true
  }
}

impl<T, U> FuzzyEq<HashMap<T, U>> for HashMap<T, U>
where
  T: FuzzyEq<T> + std::cmp::Eq + std::hash::Hash,
  U: FuzzyEq<U>,
  T: Clone,
  U: Clone,
{
  fn fuzzy_eq(&self, other: HashMap<T, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }

    for (key, value) in self.iter() {
      if !other.contains_key(key) {
        return false;
      }

      if value.fuzzy_ne(other.get(key).unwrap().clone()) {
        return false;
      }
    }

    true
  }
}

impl FuzzyEq<&String> for String {
  fn fuzzy_eq(&self, other: &String) -> bool {
    self.eq(other)
  }
}

impl FuzzyEq<String> for String {
  fn fuzzy_eq(&self, other: String) -> bool {
    self.eq(&other)
  }
}

impl<T> FuzzyEq<Option<T>> for Option<T>
where
  T: Clone,
  T: FuzzyEq<T>,
{
  fn fuzzy_eq(&self, other: Option<T>) -> bool {
    match (self, other) {
      (Some(ref option), Some(other)) => option.fuzzy_eq(other),
      (None, None) => true,
      _ => false,
    }
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
