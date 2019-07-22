extern crate shapes;

use shapes::shape::Shape;
use shapes::circle::Circle;

use std::fmt;

fn main() {
    let s = Circle::new();
    let s2 = Circle::with_radius(2 as f64);

    println!("{}", s);
    println!("{}", s2);
}
