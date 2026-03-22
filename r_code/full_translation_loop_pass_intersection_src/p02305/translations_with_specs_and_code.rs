use std::io::{self, Read};

fn main() {
    // Read all input, parse as integers (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x1: i64 = it.next().unwrap().parse().unwrap();
    let y1: i64 = it.next().unwrap().parse().unwrap();
    let r1: i64 = it.next().unwrap().parse().unwrap();
    let x2: i64 = it.next().unwrap().parse().unwrap();
    let y2: i64 = it.next().unwrap().parse().unwrap();
    let r2: i64 = it.next().unwrap().parse().unwrap();

    // Use squared comparisons to avoid sqrt, matching the C logic/spec.
    // Spec assumes no overflow; we use i64 to reduce risk but do not add checks.
    let dx = x1 - x2;
    let dy = y1 - y2;
    let rdistance = dx * dx + dy * dy;

    let sum = r1 + r2;
    let diff = r1 - r2;
    let rplus = sum * sum;
    let rminus = diff * diff;

    let ans = if rdistance > rplus {
        4
    } else if rdistance == rplus {
        3
    } else if rminus < rdistance && rdistance < rplus {
        2
    } else if rdistance == rminus {
        1
    } else {
        // rdistance < rminus
        0
    };

    print!("{ans}\n");
}
