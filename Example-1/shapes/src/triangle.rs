use crate::shape::Shape;

use std::fmt;

/// Define a General Triangle with 3 sides.
#[derive(Clone)]
pub struct Triangle {
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle { side_a: 1.0, side_b: 1.0, side_c: 1.0 }
    }

    pub fn with_sides(a: f64, b: f64, c:f64) -> Self {
        Triangle { side_a: a, side_b: b, side_c: c }
    }
}


impl Shape for Triangle {
    fn name(&self) -> &'static str {
        "Triangle"
    }

    /// Compute perimeter by adding 3 sides together.
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }

    /// Compute the area using Heron's Formula. Use
    ///
    /// $s = \frac{1}{2}Perimeter$
    /// and
    /// $Area = \sqrt{ s(s-a)(s-b)(s-c) }$
    fn area(&self) -> f64 {

        let s = self.perimeter() / 2.0;

        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    use std::ptr;
    use hamcrest2::prelude::*;

    #[test]
    fn test_default_constructor() {
        let generic = Triangle::new();

        assert_that!(generic.name(), equal_to("Triangle"));
        assert_that!(generic.side_a, close_to(1.0, 0.01));
        assert_that!(generic.side_b, close_to(1.0, 0.01));
        assert_that!(generic.side_c, close_to(1.0, 0.01));
    }

    #[test]
    fn test_with_sides() {
        let generic = Triangle::with_sides(3.0, 4.0, 5.0);

        assert_that!(generic.name(), equal_to("Triangle"));
        assert_that!(generic.side_a, close_to(3.0, 0.01));
        assert_that!(generic.side_b, close_to(4.0, 0.01));
        assert_that!(generic.side_c, close_to(5.0, 0.01));
    }


}
