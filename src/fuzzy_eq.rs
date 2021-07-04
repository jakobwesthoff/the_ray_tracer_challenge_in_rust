const EPISILON: f64 = 0.00001;

pub trait FuzzyEq<T> {
    fn fuzzy_eq(&self, other: &T) -> bool;

    fn fuzzy_ne(&self, other: &T) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl FuzzyEq<f64> for f64 {
    fn fuzzy_eq(&self, other: &f64) -> bool {
        (*self - *other).abs() < EPISILON
    }
}

// Not really sure what I am doing here, as I don't have a great understanding of macros yet.
// Feel free to fix or enhance in the future.
// @TODO: Check if we can ensure more explicitly the two operands implement the `FuzzyEq` trait
#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val.fuzzy_ne(right_val) {
                    panic!("asserting fuzzy equality. {:?} is not fuzzy equal to {:?}", left_val, right_val);
                }
            }
        }
    });
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val.fuzzy_eq(right_val) {
                    panic!("asserting fuzzy in-equality. {:?} is fuzzy equal to {:?}", left_val, right_val);
                }
            }
        }
    });
}
