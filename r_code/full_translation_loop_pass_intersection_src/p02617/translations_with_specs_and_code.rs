use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();

    // Baseline:
    // Spec: B = sum_{k=1..N} k(k+1)/2
    // C code computes the same value via a reversed-index equivalent.
    let mut ans: i64 = 0;
    for k in 1..=n {
        ans += k * (k + 1) / 2;
    }

    // Subtract reductions for N-1 pairs:
    // R += min(u,w) * (N - max(u,w) + 1)
    for _ in 1..=n.saturating_sub(1) {
        let u: i64 = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        let (x, y) = if u <= w { (u, w) } else { (w, u) };
        ans -= x * (n - y + 1);
    }

    // Exact output formatting: integer + newline.
    print!("{ans}\n");
}
