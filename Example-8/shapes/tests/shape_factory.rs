#[cfg(test)]
#[macro_use]
extern crate hamcrest2;
extern crate shapes;

use hamcrest2::prelude::*;

use shapes::factory;
use shapes::factory::Factory;
use shapes::factory::KnownShape;
use shapes::square::Square;
use shapes::circle::Circle;
use shapes::triangle::Triangle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;

use std::io::BufReader;
use stringreader::StringReader;

#[test]
fn test_is_known() {
    let factory = Factory::new();

    assert!(factory.is_known("Circle"));
    assert!(factory.is_known("Square"));
    assert!(factory.is_known("Triangle"));
    assert!(factory.is_known("Right Triangle"));
    assert!(factory.is_known("Equilateral Triangle"));
}

#[test]
fn test_number_known() {
    let factory = Factory::new();

    assert_that!(factory.number_known(), is(equal_to(5)));
}

#[test]
fn test_str() {
    let factory = Factory::new();
    let f_str = factory.to_string();

    assert!(f_str.contains("  Circle"));
    assert!(f_str.contains("  Square"));
    assert!(f_str.contains("  Triangle"));
    assert!(f_str.contains("  Right Triangle"));
    assert!(f_str.contains("  Equilateral Triangle"));
}

#[test]
fn test_create() {
    // I need to write this test...
}

#[test]
fn test_create_with() {
    let factory = Factory::new();

    let a_shape = factory.create_with("Triangle", &[3.0, 4.0, 5.0]).unwrap();
    let ref_shape = Triangle::with_sides(3.0, 4.0, 5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = factory.create_with("Right Triangle", &[3.0, 4.0]).unwrap();
    let ref_shape = RightTriangle::with_base_height(3.0, 4.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = factory.create_with("Equilateral Triangle", &[5.0]).unwrap();
    let ref_shape = EquilateralTriangle::with_side(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = factory.create_with("Circle", &[5.0]).unwrap();
    let ref_shape = Circle::with_radius(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = factory.create_with("Square", &[5.0]).unwrap();
    let ref_shape = Square::with_side(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

#[test]
fn test_read_shapes() {

    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle
        Right Triangle
        Equilateral Triangle
        Square
        Circle
        1337 Haxor"#;

    let str_reader = StringReader::new(raw_str);
    let str_reader = BufReader::new(str_reader);

    let factory = Factory::new();

    let some_shapes = factory::read_shapes(str_reader, factory);

    assert_that!(some_shapes.len(), is(equal_to(5)));

    assert!(some_shapes[0].to_string().contains("Triangle"));
    assert!(some_shapes[1].to_string().contains("Right Triangle"));
    assert!(some_shapes[2].to_string().contains("Equilateral Triangle"));
    assert!(some_shapes[3].to_string().contains("Square"));
    assert!(some_shapes[4].to_string().contains("Circle"));
}

#[test]
fn test_read_shapes_with() {
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle; 1 2 3
        Right Triangle; 3 4
        Equilateral Triangle; 5
        Square; 5
        Circle; 5
        1337 Haxor; invalid input"#;

    let str_reader = StringReader::new(raw_str);
    let str_reader = BufReader::new(str_reader);

    let factory = Factory::new();

    let some_shapes = factory::read_shapes_with(str_reader, factory);
    println!("{:?}", some_shapes);
    assert_that!(some_shapes.len(), is(equal_to(5)));

    assert!(some_shapes[0].to_string().contains("Triangle"));
    assert!(some_shapes[1].to_string().contains("Right Triangle"));
    assert!(some_shapes[2].to_string().contains("Equilateral Triangle"));
    assert!(some_shapes[3].to_string().contains("Square"));
    assert!(some_shapes[4].to_string().contains("Circle"));
}

#[test]
fn test_known_shapes_display() {
    // @TODO write tests for the KnownShape Display trait implementation
    // assert!(false); // Fail

    let tri = shapes::triangle::Triangle::new();
    let a_shape = KnownShape::Triangle(shapes::triangle::Triangle::new());
    assert_that!(tri.to_string(), equal_to(a_shape.to_string()));

    let tri = shapes::right_triangle::RightTriangle::new();
    let a_shape = KnownShape::RightTriangle(RightTriangle::new());
    assert_that!(tri.to_string(), equal_to(a_shape.to_string()));

    let tri = shapes::equilateral_triangle::EquilateralTriangle::new();
    let a_shape = KnownShape::EquilateralTriangle(EquilateralTriangle::new());
    assert_that!(tri.to_string(), equal_to(a_shape.to_string()));

    let square = shapes::square::Square::new();
    let a_shape = KnownShape::Square(Square::new());
    assert_that!(square.to_string(), equal_to(a_shape.to_string()));

    let circle = shapes::circle::Circle::new();
    let a_shape = KnownShape::Circle(Circle::new());
    assert_that!(circle.to_string(), equal_to(a_shape.to_string()));
}
