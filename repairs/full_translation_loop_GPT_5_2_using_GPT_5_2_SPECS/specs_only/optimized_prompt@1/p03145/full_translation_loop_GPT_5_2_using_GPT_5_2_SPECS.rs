use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Specification precondition: at least three integers are present.
    // No explicit error handling is required; unwrap() matches "not defined" on invalid/missing input.
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let _c: i64 = it.next().unwrap().parse().unwrap(); // ignored for all observable behavior

    // Compute (a * b) / 2 using floating-point, then round to nearest integer for display.
    // Preconditions guarantee no integer overflow in the product and that the floating result is representable.
    let area = ((a * b) as f64) / 2.0;
    let out = area.round() as i64;

    // No guaranteed trailing newline.
    print!("{}", out);
}
