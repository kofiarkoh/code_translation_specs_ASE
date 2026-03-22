use std::io::{self, Read};

fn main() {
    // No explicit error handling per specification; malformed/insufficient input => undefined.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Fixed-size storage constraint is a precondition; we do not enforce it.
    // Only x influences the result; y is read and ignored.
    let mut diffs: Vec<i64> = Vec::new();
    diffs.reserve(m);

    for _ in 0..m {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let _y: i64 = it.next().unwrap().parse().unwrap();

        if x < n {
            // Preconditions guarantee (n - x) and sums fit in the integer type.
            diffs.push(n - x);
        }
    }

    let q = diffs.len();
    if q <= 1 {
        println!("0");
        return;
    }

    // Quadratic-time ordering operation as implied by the specification.
    // Sort ascending to take the smallest (q - 1) values.
    for i in 0..q {
        for j in (i + 1)..q {
            if diffs[j] < diffs[i] {
                diffs.swap(i, j);
            }
        }
    }

    let mut sum: i64 = 0;
    for i in 0..(q - 1) {
        sum += diffs[i];
    }

    println!("{}", sum);
}
