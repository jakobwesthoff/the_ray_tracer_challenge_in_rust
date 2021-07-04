pub trait FuzzyEq<T> {
    fn fuzzy_eq(&self, other: &T) -> bool;

    fn fuzzy_ne(&self, other: &T) -> bool {
        !self.fuzzy_eq(other)
    }
}

pub fn f64_fuzzy_eq(left: f64, right: f64) -> bool {
    let epsilon = 0.00001;
    (left - right).abs() < epsilon
}

// Not really sure what I am doing here, as I don't have a great understanding of macros yet.
// Feel free to fix or enhance in the future.
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
