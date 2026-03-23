use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Preconditions say all required inputs are provided; if not, behavior is undefined.
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();
    let d: i32 = it.next().unwrap().parse().unwrap();

    // Closed intervals [a, b] and [c, d], with a <= b and c <= d per preconditions.
    // Intersection is [max(a,c), min(b,d)] if overlapping.
    // Output is R - L (not inclusive count), and 0 if no overlap.
    let l = a.max(c);
    let r = b.min(d);

    let overlap = if r <= l { 0 } else { r - l };

    println!("{}", overlap);
}
