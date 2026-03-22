use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated signed integers (C long long semantics assumed).
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
    } else {
        let d = y - x;
        // Per spec: if d == 0, division/modulo by zero is undefined in C.
        // We intentionally do not guard against it to preserve semantics.
        if x % d == 0 {
            print!("{}\n", 2 * (x / d));
        } else {
            print!("{}\n", 2 * (x / d) + 1);
        }
    }
}
