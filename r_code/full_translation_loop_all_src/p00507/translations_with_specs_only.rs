use std::io::{self, Read};

fn num_digits(mut x: u64) -> u32 {
    // Decimal digit count for non-negative integers; 0 has 1 digit.
    if x == 0 {
        return 1;
    }
    let mut d = 0u32;
    while x > 0 {
        d += 1;
        x /= 10;
    }
    d
}

fn pow10_u64(exp: u32) -> u64 {
    // Preconditions guarantee no overflow for concatenation results.
    let mut p = 1u64;
    for _ in 0..exp {
        p *= 10;
    }
    p
}

fn main() {
    // Read all input; unspecified behavior for invalid input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<u64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: u64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Precondition: n >= 3 and at least 3 ordered distinct pairs exist.
    // Track three smallest concatenation values seen so far (with duplicates).
    let mut best1: u64 = u64::MAX;
    let mut best2: u64 = u64::MAX;
    let mut best3: u64 = u64::MAX;

    // Quadratic enumeration of all ordered pairs (p, q), p != q.
    for i in 0..n {
        let di = num_digits(a[i]);
        let mul = pow10_u64(di);
        for j in 0..n {
            if i == j {
                continue;
            }
            // Concatenate a[j] followed by a[i] as decimal digits:
            // value = a[j] * 10^{digits(a[i])} + a[i]
            let v = a[j] * mul + a[i];

            // Insert into (best1, best2, best3) maintaining non-decreasing order.
            if v <= best1 {
                best3 = best2;
                best2 = best1;
                best1 = v;
            } else if v <= best2 {
                best3 = best2;
                best2 = v;
            } else if v <= best3 {
                best3 = v;
            }
        }
    }

    // Output the third smallest (3rd order statistic), counting duplicates.
    println!("{}", best3);
}
