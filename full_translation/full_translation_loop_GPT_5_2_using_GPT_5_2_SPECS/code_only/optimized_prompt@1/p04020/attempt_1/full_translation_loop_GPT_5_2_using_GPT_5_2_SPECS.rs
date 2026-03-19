use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C code uses 1-based indexing and allocates A[100001].
    // We'll allocate n+2 to safely access A[i+1] when i == n-1.
    let mut a: Vec<i64> = vec![0; n + 2];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    let mut ans: i64 = 0;

    // Preserve C's integer division/modulo semantics for signed values:
    // Rust's / and % on i64 match C99+ (truncate toward zero; remainder has sign of dividend).
    for i in 1..n {
        ans += a[i] / 2;
        if a[i + 1] > 1 && (a[i] % 2 == 1) {
            a[i + 1] -= 1;
            ans += 1;
        }
    }
    ans += a[n] / 2;

    // Match C output formatting exactly.
    print!("{ans}\n");
    // stdout is flushed on program exit; explicit flush not necessary for correctness here.
}
