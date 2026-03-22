use std::io::{self, Read};

fn main() {
    // Read all input (unspecified behavior on malformed input per spec).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut x = [0i64; 2];
    let mut y = [0i64; 2];
    let mut r = [0i64; 2];

    for i in 0..2 {
        x[i] = it.next().unwrap().parse().unwrap();
        y[i] = it.next().unwrap().parse().unwrap();
        r[i] = it.next().unwrap().parse().unwrap();
    }

    // Use squared comparisons only (no floating point), matching the C logic/spec invariant.
    let dx = x[0] - x[1];
    let dy = y[0] - y[1];
    let rdistance = dx * dx + dy * dy;

    let rp = r[0] + r[1];
    let rplus = rp * rp;

    let rm = r[0] - r[1];
    let rminus = rm * rm;

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
