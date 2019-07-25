extern crate shapes;

use shapes::shape::Shape;
use shapes::circle::Circle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;
use shapes::square::Square;
use shapes::factory::Factory;

use std::fmt;
use std::vec::Vec;

const PROGRAM_HEADING: [&'static str; 2] = ["Objects & Traits: 2-D Shapes",
                                            "Thomas J. Kennedy"];

fn main() {
    // Print Program Heading
    println!("{}", "-".repeat(80));

    for &line in PROGRAM_HEADING.iter() {
        println!("{:^80}", line);
    }

    println!("{}", "-".repeat(80));

    // What happens when the number of shapes is non-trivial?
    //
    // Suppose we were to expand our Shape hierarchy to include the following
    // shapes:
    //   - Isosceles Triangle
    //   - Circle
    //   - Ellipse
    //   - Rectangle
    //   - Square
    //   - Rhombus
    //   - Parallelogram
    //   - Kite
    //   - Generalized Polygon
    //
    // How would we manage the addition of new Shapes?
    //
    // A common approach is to make use of the Factory Model.  This Model exists
    // in a number of languages--e.g.:
    //   - C++
    //   - Java
    //   - Python
    //   - Rust
    //   - PHP
    //   - C#
    //
    // A class that contains static members is created.  As new classes are
    // created, the Factory Class is updated.
    //
    // In this example, our factory class is called ShapeFactory The
    // ShapeFactory could be designed as a singleton class.  Our ShapeFactory is
    // simply a tracker--i.e., records are static and will be updated manually
    // at compile time.

    let shape_factory = Factory::new();

    // Examine the ShapeFactory
    println!("{}", "*".repeat(38));
    println!("{:^38}", "Available Shapes");
    println!("{}", "*".repeat(38));

    // List the available shapes
    print!("{}", shape_factory);
    println!("{}", "-".repeat(38));
    print!("{:>2} shapes available.", shape_factory.number_known());
    println!();

    // Create 5 "Random" Shapes
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    // shapes.push(shape_factory.create("Triangle"))?;
    // shapes = [ShapeFactory.create("Triangle"),
              // ShapeFactory.create("Right Triangle"),
              // ShapeFactory.create("Equilateral Triangle"),
              // ShapeFactory.create("Square"),
              // ShapeFactory.create("Circle")];
    //           ShapeFactory.create("1337 Haxor")];

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Shapes That Exist");
    println!("{}", "*".repeat(38));
    //print!("{:<24}: {:>4}", "Original Size", size);
    //print!("{:<24}: {:>4}", "Invalid Shapes", (size - len(shapes))))
    //print!("{:<24}: {:>4}", "New Size", len(shapes)))
    println!();

    // Print all the shapes
    println!("{}", "*".repeat(38));
    println!("{:^38}", "Display All Shapes");
    println!("{}", "*".repeat(38));

    // for s in shapes.iter() {
        // print!("{}", s);
    // }
}
