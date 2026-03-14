use std::f64;
use std::io::{self, Write};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn input_point() -> Option<Point> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let parts: Vec<&str> = input.trim().split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
                return Some(Point { x, y });
            }
        }
    }
    None
}

fn len(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).hypot(a.y - b.y))
}

fn area(x: &Point, y: &Point, z: &Point) -> f64 {
    let a = len(x, y);
    let b = len(y, z);
    let c = len(z, x);
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    let mut points = Vec::new();
    while let Some(point) = input_point() {
        points.push(point);
    }

    let mut s = 0.0;
    let n = points.len();
    for i in 2..n {
        s += area(&points[0], &points[i - 1], &points[i]);
    }

    // Print the result with the same formatting as C
    println!("{:.6}", s);
}
