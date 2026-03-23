use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x1: i64 = it.next().unwrap().parse().unwrap();
    let y1: i64 = it.next().unwrap().parse().unwrap();
    let r1: i64 = it.next().unwrap().parse().unwrap();

    let x2: i64 = it.next().unwrap().parse().unwrap();
    let y2: i64 = it.next().unwrap().parse().unwrap();
    let r2: i64 = it.next().unwrap().parse().unwrap();

    // Preconditions state that intermediate squared computations do not overflow.
    // Use i64 to keep integer arithmetic and avoid floating point.
    let dx = x1 - x2;
    let dy = y1 - y2;
    let d2 = dx * dx + dy * dy;

    let sum = r1 + r2;
    let sum2 = sum * sum;

    let diff = (r1 - r2).abs();
    let diff2 = diff * diff;

    let code = if d2 > sum2 {
        4 // disjoint
    } else if d2 == sum2 {
        3 // externally tangent
    } else if d2 > diff2 {
        2 // intersect at two points
    } else if d2 == diff2 {
        1 // internally tangent
    } else {
        0 // one strictly inside the other, no intersection
    };

    println!("{code}");
}
