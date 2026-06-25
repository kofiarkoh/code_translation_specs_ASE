use std::io::{self, Read};

fn main() {
    // Read all input (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap(); // Spec assumes n >= 1.

    // Transform: b[i] = x[i] - (i+1)
    let mut b: Vec<i64> = Vec::with_capacity(n);
    for i in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        b.push(x - (i as i64 + 1));
    }

    // Minimum sum of absolute deviations is achieved at any median.
    b.sort_unstable();
    let median = b[n / 2];

    let mut ans: i64 = 0;
    for &v in &b {
        ans += (v - median).abs();
    }

    print!("{ans}\n");

    // Note: The provided C code uses a more complex incremental method and also
    // contains an out-of-bounds access risk (a[i+1] when i == n-1). The spec
    // defines the intended behavior as the median-based minimum, which is what
    // is implemented here.
}
