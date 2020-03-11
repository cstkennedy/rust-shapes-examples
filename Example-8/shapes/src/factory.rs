use std::collections::HashSet;
use std::fmt;
use std::io::BufRead;
use std::convert::From;

use itertools::Itertools;

use crate::shape::Shape;
use crate::square::Square;
use crate::circle::Circle;
use crate::triangle::Triangle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;

#[derive(Debug)]
pub enum KnownShape {
    Triangle(Triangle),
    RightTriangle(RightTriangle),
    EquilateralTriangle(EquilateralTriangle),
    Square(Square),
    Circle(Circle),
}

impl fmt::Display for KnownShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            KnownShape::Triangle(s) => write!(f, "{}", s),
            KnownShape::RightTriangle(s) => write!(f, "{}", s),
            KnownShape::EquilateralTriangle(s) => write!(f, "{}", s),
            KnownShape::Square(s) => write!(f, "{}", s),
            KnownShape::Circle(s) => write!(f, "{}", s),
        }
    }
}

impl Shape for KnownShape {
    fn name(&self) -> &'static str {
        match &*self {
            KnownShape::Triangle(s) => s.name(),
            KnownShape::RightTriangle(s) => s.name(),
            KnownShape::EquilateralTriangle(s) => s.name(),
            KnownShape::Square(s) => s.name(),
            KnownShape::Circle(s) => s.name(),
        }
    }

    fn area(&self) -> f64 {
        match &*self {
            KnownShape::Triangle(s) => s.area(),
            KnownShape::RightTriangle(s) => s.area(),
            KnownShape::EquilateralTriangle(s) => s.area(),
            KnownShape::Square(s) => s.area(),
            KnownShape::Circle(s) => s.area(),
        }
    }

    fn perimeter(&self) -> f64 {
        match &*self {
            KnownShape::Triangle(s) => s.perimeter(),
            KnownShape::RightTriangle(s) => s.perimeter(),
            KnownShape::EquilateralTriangle(s) => s.perimeter(),
            KnownShape::Square(s) => s.perimeter(),
            KnownShape::Circle(s) => s.perimeter(),
        }
    }
}

lazy_static! {
    static ref KNOWN_SHAPES: HashSet<&'static str> = {
        let mut known_shapes = HashSet::new();

        known_shapes.insert("Triangle");
        known_shapes.insert("Right Triangle");
        known_shapes.insert("Equilateral Triangle");
        known_shapes.insert("Square");
        known_shapes.insert("Circle");

        known_shapes
    };

    static ref NUMBER_KNOWN: usize = KNOWN_SHAPES.len();
}

/// Create a Shape
///
/// # Arguments
///
///   * `name` shape to be created
///
pub fn create(name: &str) -> Option<KnownShape> {
    match name  {
        "Triangle" => Some(Triangle::new().into()),
        "Right Triangle" => Some(RightTriangle::new().into()),
        "Equilateral Triangle" => Some(EquilateralTriangle::new().into()),
        "Square" => Some(Square::new().into()),
        "Circle" => Some(Circle::new().into()),
        _ =>  None
    }
}

/// Create a Shape with specified dimensions.
///
/// # Arguments
///
///   * `name` shape to be created
///   * `dims` input dimensions
///
pub fn create_with(name: &str, dims: &[f64]) -> Option<KnownShape> {
    match name  {
        "Triangle" => {
            Some(Triangle::from(dims).into())
        },
        "Right Triangle" => {
            Some(RightTriangle::from(dims).into())
        },
        "Equilateral Triangle" => {
            Some(EquilateralTriangle::from(dims).into())
        },
        "Square" => {
            Some(Square::from(dims).into())
        },
        "Circle" => {
            Some(Circle::from(dims).into())
        },
        _ =>  None
    }
}

/// Determine whether a given shape is known
///
/// # Arguments
///
///  * `name` the shape for which to query
///
pub fn is_known(name: &str) -> bool {
    KNOWN_SHAPES.contains(name)
}

pub fn number_known() -> usize {
    *NUMBER_KNOWN
}

/// List the known shapes, one per line
///
pub fn list_known() -> String {
    KNOWN_SHAPES.iter()
        .sorted()
        .map(|s| format!("  {}\n", s))
        .collect()
}

/// Create shapes based on names from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes<B: BufRead>(ins: B)-> Vec<KnownShape> {
    let mut shapes: Vec<KnownShape> = Vec::new();

    for line in ins.lines() {
        let n = line.unwrap();
        let n = n.trim();
        if let Some(s) = create(n) {
            shapes.push(s)
        }
    }

    shapes
}

/// Create shapes based on names *and dimension data* from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes_with<B>(ins: B)-> Vec<KnownShape>
    where B: BufRead  {

    let mut shapes: Vec<KnownShape> = Vec::new();

    for line in ins.lines() {
        let raw_line = line.unwrap();

        if raw_line.is_empty() {
            continue;
        }

        let split_line: Vec<String> = raw_line.trim().split(';')
            .map(|s| s.to_string())
            .collect();

        // There is no line data (i.e., no ';')
        if split_line.len() != 2 {
            continue;
        }

        let n = &split_line[0];
        let dims_str = &split_line[1];

        // Mistake -> s.len() > 0 != s.is_empty() -> I forgot the leading '!'
        let dims: Vec<f64> = dims_str.split(' ')
            .filter(|s| !s.is_empty())
            .map(|dim| {
                match dim.trim().parse() {
                    Ok(d) => d,
                    Err(_e) => 0.0,
                }
            }).collect();

        if let Some(s) = create_with(&n, &dims) {
            shapes.push(s);
        }
    }

    shapes
}

impl From<Triangle> for KnownShape {
    fn from(item: Triangle) -> Self {
        KnownShape::Triangle(item)
    }
}

impl From<RightTriangle> for KnownShape {
    fn from(item: RightTriangle) -> Self {
        KnownShape::RightTriangle(item)
    }
}

impl From<EquilateralTriangle> for KnownShape {
    fn from(item: EquilateralTriangle) -> Self {
        KnownShape::EquilateralTriangle(item)
    }
}

impl From<Square> for KnownShape {
    fn from(item: Square) -> Self {
        KnownShape::Square(item)
    }
}

impl From<Circle> for KnownShape {
    fn from(item: Circle) -> Self {
        KnownShape::Circle(item)
    }
}

impl From<&[f64]> for Triangle {
    fn from(dims: &[f64]) -> Self {
        Triangle::with_sides(dims[0], dims[1], dims[2])
    }
}

impl From<&[f64]> for RightTriangle  {
    fn from(dims: &[f64]) -> Self {
        RightTriangle::with_base_height(dims[0], dims[1])
    }
}

impl From<&[f64]> for EquilateralTriangle {
    fn from(dims: &[f64]) -> Self {
        EquilateralTriangle::with_side(dims[0])
    }
}

impl From<&[f64]> for Square {
    fn from(dims: &[f64]) -> Self {
        Square::with_side(dims[0])
    }
}

impl From<&[f64]> for Circle {
    fn from(dims: &[f64]) -> Self {
        Circle::with_radius(dims[0])
    }
}

