use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as six signed integers (C long long semantics assumed).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t1: i64 = it.next().unwrap().parse().unwrap();
    let t2: i64 = it.next().unwrap().parse().unwrap();
    let mut a1: i64 = it.next().unwrap().parse().unwrap();
    let mut a2: i64 = it.next().unwrap().parse().unwrap();
    let mut b1: i64 = it.next().unwrap().parse().unwrap();
    let mut b2: i64 = it.next().unwrap().parse().unwrap();

    // 5.1 Total-equality check
    // Note: The spec assumes no overflow; in Rust, i64 overflow would panic in debug builds.
    // This matches the spec only under the stated assumption (no overflow occurs).
    let total_a = a1 * t1 + a2 * t2;
    let total_b = b1 * t1 + b2 * t2;

    if total_a == total_b {
        print!("infinity\n");
        return;
    }

    // 5.2 Normalization (possible swap)
    if a1 > b1 {
        std::mem::swap(&mut a1, &mut b1);
        std::mem::swap(&mut a2, &mut b2);
    }

    // 5.3 Derived quantities and integer result
    let x = (b1 - a1) * t1;
    let y = (a2 - b2) * t2;

    if x > y {
        print!("0\n");
        return;
    }

    let d = y - x;

    // Spec note: if d == 0, C behavior is undefined (division/modulo by zero).
    // In Rust, this would panic; we intentionally do not guard to mirror the spec.
    let q = x / d;
    let r = x % d;

    let ans = if r == 0 { 2 * q } else { 2 * q + 1 };
    print!("{ans}\n");
}
