use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code reads: x, k, d (despite variables declared as x,d,k)
    let x_in: i64 = it.next().unwrap().parse().unwrap();
    let mut k: i64 = it.next().unwrap().parse().unwrap();
    let d: i64 = it.next().unwrap().parse().unwrap();

    // Spec: divisor must be non-zero; if zero, behavior undefined (we do not guard).
    let x = x_in.abs();
    let mut now = x;

    // Note: C uses integer division x/d. This assumes d > 0 for intended behavior.
    // If d < 0, C's truncating division and the algorithm's meaning become odd;
    // we preserve the same arithmetic structure.
    if x / d <= k {
        let q = x / d;
        now -= d * q;
        k -= q;
        if k % 2 != 0 {
            now -= d;
        }
    } else {
        now -= d * k;
    }

    print!("{}", now.abs());
}
