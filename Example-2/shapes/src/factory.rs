use std::collections::HashMap;
use std::fmt;
use std::error::Error;

use crate::shape::Shape;
use crate::square::Square;
use crate::circle::Circle;
use crate::triangle::Triangle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;


pub struct Factory {
    known_shapes: HashMap<&'static str, Box<Shape>>,
}

impl Factory {
    pub fn new() -> Self {
        let mut factory = Factory{ known_shapes: HashMap::new() };

        factory.known_shapes.insert("Triangle", Box::new(Triangle::new()));
        factory.known_shapes.insert("Right Triangle", Box::new(RightTriangle::new()));
        factory.known_shapes.insert("Equilateral Triangle", Box::new(EquilateralTriangle::new()));
        factory.known_shapes.insert("Square", Box::new(Square::new()));
        factory.known_shapes.insert("Circle", Box::new(Circle::new()));

        factory
    }

    /// Create a Shape
    ///
    /// :param: name the shape to be created
    ///
    /// :return: A shape with the specified name
    ///   or null if no matching shape is found
    ///
    /// Rust Refactoring required....
    /// <https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928#30353928>
    // pub fn create(&self, name: &str) -> Option<Box<Shape>>{
        // match self.known_shapes.get(name) {
            // Some(&shape_box) => Some(Box::new((*shape_box).clone())),
            // None => None
        // }
    // }

    pub fn is_known(&self, name: &str) -> bool {
        // """
        // Determine whether a given shape is known

        // :param: name the shape for which to query
        // """

        self.known_shapes.contains_key(name)
    }

    pub fn number_known(&self) -> usize {
        self.known_shapes.len()
    }
}

impl fmt::Display for Factory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, _) in &self.known_shapes {
            writeln!(f, "  {}", name)?;
        }

        Ok(())
    }
}

