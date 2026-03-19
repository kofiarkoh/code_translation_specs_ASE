use std::io::{self, Read};

fn main() {
    // Read exactly six integers: x1 y1 r1 x2 y2 r2
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x1: i128 = it.next().unwrap().parse().unwrap();
    let y1: i128 = it.next().unwrap().parse().unwrap();
    let r1: i128 = it.next().unwrap().parse().unwrap();
    let x2: i128 = it.next().unwrap().parse().unwrap();
    let y2: i128 = it.next().unwrap().parse().unwrap();
    let r2: i128 = it.next().unwrap().parse().unwrap();

    // The specification does not define the exact mapping of {0,1,2,3,4} to circle relations.
    // Use the narrowest reasonable interpretation: classify by the number of intersection points:
    // 0 -> no intersection
    // 1 -> tangent (one intersection point)
    // 2 -> two intersection points
    // 3 -> coincident circles (infinitely many intersection points)
    // 4 -> invalid/unused (not produced)
    //
    // This is a common 5-outcome categorization; since only 4 are needed for geometry,
    // the 5th value remains unused to avoid inventing extra behavior.

    let dx = x1 - x2;
    let dy = y1 - y2;
    let d2 = dx * dx + dy * dy;

    let sum = r1 + r2;
    let diff = (r1 - r2).abs();

    let sum2 = sum * sum;
    let diff2 = diff * diff;

    let out = if dx == 0 && dy == 0 && r1 == r2 {
        3
    } else if d2 > sum2 || d2 < diff2 {
        0
    } else if d2 == sum2 || d2 == diff2 {
        1
    } else {
        2
    };

    print!("{out}");
}
