use num_traits::Float;
use std::ops;

use crate::fuzzy_eq::*;

#[derive(Debug, Copy, Clone)]
pub struct Tuple<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

/**
 * Tuple type related functions
 */
impl<T> Tuple<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            w: T::one(),
        }
    }

    pub fn vector(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            w: T::zero(),
        }
    }

    pub fn is_point(&self) -> bool {
        self.w == T::one()
    }

    pub fn is_vector(&self) -> bool {
        self.w == T::zero()
    }
}

impl<T> FuzzyEq<Tuple<T>> for Tuple<T>
where
    T: Float,
    T: FuzzyEq<T>,
{
    fn fuzzy_eq(&self, other: &Self) -> bool {
        return self.x.fuzzy_eq(&other.x)
            && self.y.fuzzy_eq(&other.y)
            && self.z.fuzzy_eq(&other.z)
            && self.w.fuzzy_eq(&other.w);
    }
}

impl<T> ops::Add<Self> for Tuple<T>
where
    T: Float,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl<T> ops::Sub<Self> for Tuple<T>
where
    T: Float,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl<T> ops::Neg for Tuple<T>
where
    T: Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T> ops::Mul<T> for Tuple<T>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl<T> ops::Div<T> for Tuple<T>
where
    T: Float,
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
        )
    }
}

/**
* Tuple math operations
*/
impl<T> Tuple<T>
where
    T: Float,
{
    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: &Tuple<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple<T>) -> Tuple<T> {
        if !self.is_vector() || !other.is_vector() {
            panic!("Cross product can only be calculated for two vectors.");
        }

        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_does_fill_properties() {
        let point = Tuple::point(4.3, -4.2, 3.1);

        assert_fuzzy_eq!(point.x, 4.3);
        assert_fuzzy_eq!(point.y, -4.2);
        assert_fuzzy_eq!(point.z, 3.1);
    }

    #[test]
    fn point_has_w_value_of_1() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert_fuzzy_eq!(point.w, 1.0);
    }

    #[test]
    fn point_says_it_is_a_point() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert!(point.is_point());
    }

    #[test]
    fn vector_new_does_fill_properties() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);

        assert_fuzzy_eq!(vector.x, 4.3);
        assert_fuzzy_eq!(vector.y, -4.2);
        assert_fuzzy_eq!(vector.z, 3.1);
    }

    #[test]
    fn vector_has_w_value_of_0() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);

        assert_fuzzy_eq!(vector.w, 0.0);
    }

    #[test]
    fn point_says_it_is_a_vector() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);
        assert!(vector.is_vector());
    }

    #[test]
    fn tuples_are_added_up() {
        let tuple_one = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple_two = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let expected_tuple = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_fuzzy_eq!(tuple_one + tuple_two, expected_tuple);
    }

    #[test]
    fn substraction_of_points_result_in_vector() {
        let point_one = Tuple::point(3.0, 2.0, 1.0);
        let point_two = Tuple::point(5.0, 6.0, 7.0);

        let expected_result = Tuple::vector(-2.0, -4.0, -6.0);
        let actual_result = point_one - point_two;

        assert!(actual_result.is_vector());
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn substracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        let expected_result = Tuple::point(-2.0, -4.0, -6.0);

        let actual_result = p - v;

        assert!(actual_result.is_point());
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn substracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        let expected_result = Tuple::vector(-2.0, -4.0, -6.0);
        let actual_result = v1 - v2;

        assert!(actual_result.is_vector());
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn substracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        let expected_result = Tuple::vector(-5.0, -6.0, -7.0);
        let actual_result = zero - v2;

        assert!(actual_result.is_vector());
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected_result = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        let actual_result = -a;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let multiplier: f64 = 3.5;

        let expected_result = Tuple::new(3.5, -7.0, 10.5, -14.0);
        let actual_result = a * multiplier;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let multiplier: f64 = 0.5;

        let expected_result = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual_result = a * multiplier;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let devisor: f64 = 2.0;

        let expected_result = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual_result = a / devisor;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_the_magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);

        let expected_result = 1.0;
        let actual_result = v.magnitude();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);

        let expected_result = 1.0;
        let actual_result = v.magnitude();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);

        let expected_result = 1.0;
        let actual_result = v.magnitude();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_the_magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let expected_result = (14.0 as f64).sqrt();
        let actual_result = v.magnitude();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn compute_the_magnitude_of_negative_vector_1_2_3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);

        let expected_result = (14.0 as f64).sqrt();
        let actual_result = v.magnitude();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn normalizing_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);

        let expected_result = Tuple::vector(1.0, 0.0, 0.0);
        let actual_result = v.normalize();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        // real values -> 1.0/sqrt(14), 2.0/sqrt(14), 3.0/sqrt(14)
        let expected_result = Tuple::vector(0.26726, 0.53452, 0.80178);
        let actual_result = v.normalize();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn magnitude_of_a_normalized_vector_is_1() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let expected_result = 1.0;
        let actual_result = v.normalize().magnitude();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn the_dot_product_of_two_vector() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let expected_result = 20.0;
        let actual_result = a.dot(&b);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn the_cross_product_of_two_vectors_1() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let expected_result = Tuple::vector(-1.0, 2.0, -1.0);
        let actual_result = a.cross(&b);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn the_cross_product_of_two_vectors_2() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let expected_result = Tuple::vector(1.0, -2.0, 1.0);
        let actual_result = b.cross(&a);

        assert_fuzzy_eq!(actual_result, expected_result);
    }
}
