use std::io::{self, Read};

const MOD: i64 = 998_244_353;

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n_i64: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // unspecified; choose to do nothing if no input
    };

    // Specification: if N is negative, behavior is not defined.
    // We choose the narrowest reasonable behavior: treat as empty to avoid panics/alloc issues.
    if n_i64 < 0 {
        println!("1");
        return;
    }

    let n: usize = n_i64 as usize;

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        // No explicit error handling per spec; unwrap matches "not defined" on invalid input.
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    a.sort();

    if n == 0 {
        println!("1");
        return;
    }

    // Spec assumes nonnegative inputs for defined gcd behavior.
    // We implement gcd on u64; for negative values, casting changes semantics (unspecified by spec).
    let mut acc: i64 = 1;
    for (i, &val) in a.iter().enumerate() {
        let g = gcd_u64(val as u64, i as u64) as i64;
        acc = (acc * (g % MOD)) % MOD;
    }

    println!("{acc}");
}
