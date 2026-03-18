use std::io::{self, Read};

fn main() {
    // Read exactly six integers: x0 y0 r0 x1 y1 r1
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x0: i64 = it.next().unwrap().parse().unwrap();
    let y0: i64 = it.next().unwrap().parse().unwrap();
    let r0: i64 = it.next().unwrap().parse().unwrap();
    let x1: i64 = it.next().unwrap().parse().unwrap();
    let y1: i64 = it.next().unwrap().parse().unwrap();
    let r1: i64 = it.next().unwrap().parse().unwrap();

    // Use squared distances to avoid sqrt, matching the C code's semantics.
    let dx = x0 - x1;
    let dy = y0 - y1;
    let rdistance = dx * dx + dy * dy;

    let rsum = r0 + r1;
    let rplus = rsum * rsum;

    let rdiff = r0 - r1;
    let rminus = rdiff * rdiff;

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
