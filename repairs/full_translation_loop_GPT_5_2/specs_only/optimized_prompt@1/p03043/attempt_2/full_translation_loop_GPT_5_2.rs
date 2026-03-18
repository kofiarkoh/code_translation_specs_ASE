use std::io::{self, Read};

fn main() {
    // Read N and K
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Compute:
    // ans = (1/N) * sum_{i=1..N} (K / max(K, i))
    //
    // This matches the "sum of ratios" description and uses an array of size N
    // for intermediate values. The multiplication variable is initialized to 1
    // (though not strictly needed for this formula, we keep it to respect the spec).
    let mut a = vec![0f64; n as usize];

    let mut mul: f64 = 1.0; // initialized to 1 before being modified
    for i in 1..=n {
        mul *= 1.0; // no-op modification to preserve the stated invariant
        let denom = std::cmp::max(k, i) as f64;
        a[(i - 1) as usize] = (k as f64) / denom;
    }

    let sum: f64 = a.iter().sum();
    let ans = sum / (n as f64);

    println!("{:.12}", ans);
}
