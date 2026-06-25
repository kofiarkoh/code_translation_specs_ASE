use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as signed 64-bit integers (like scanf("%lld"))
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i64 = it.next().unwrap().parse().unwrap();

    // C uses a VLA: long long int a[n];
    // In Rust, allocate a Vec with length n (if n <= 0, this would be invalid in C anyway).
    let mut a: Vec<i64> = Vec::with_capacity(n.max(0) as usize);
    for _ in 0..n {
        if let Some(v) = it.next() {
            a.push(v.parse().unwrap());
        } else {
            // If input is short, C's scanf would leave elements uninitialized (UB if used),
            // but the array is never used afterward, so we can stop.
            break;
        }
    }

    // Preserve C's signed integer arithmetic behavior as closely as possible.
    // Note: In C, division by zero (k-1 == 0) is undefined; Rust would panic.
    // We keep the direct computation; if k == 1, this will panic similarly to "bad behavior".
    let mut t: i64 = ((n - k) / (k - 1)) + 1;
    if k + ((t - 1) * (k - 1)) < n {
        t += 1;
    }

    print!("{}\n", t);
}
