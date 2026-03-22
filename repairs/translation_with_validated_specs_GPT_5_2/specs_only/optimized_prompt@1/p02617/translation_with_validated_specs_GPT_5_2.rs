use std::io::{self, Read};

fn main() {
    // Read entire stdin; malformed/insufficient input => unspecified per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();

    // Baseline:
    // B = sum_{k=1..N} k(k+1)/2
    // Compute in i64; overflow behavior is unspecified by spec, so we do not guard.
    let mut ans: i64 = 0;
    for k in 1..=n {
        ans += k * (k + 1) / 2;
    }

    // Process exactly N-1 pairs, subtracting x * (N - y + 1) where x=min(a,b), y=max(a,b).
    for _ in 0..(n - 1) {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();
        let x = a.min(b);
        let y = a.max(b);
        ans -= x * (n - y + 1);
    }

    println!("{ans}");
}
