pub fn f64_fuzzy_eq(left: f64, right: f64) -> bool {
    let epsilon = 0.00001;
    (left - right).abs() < epsilon
}
