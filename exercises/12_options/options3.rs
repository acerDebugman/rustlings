use std::ops::Deref;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Deref for Point {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point.as_deref() {
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
