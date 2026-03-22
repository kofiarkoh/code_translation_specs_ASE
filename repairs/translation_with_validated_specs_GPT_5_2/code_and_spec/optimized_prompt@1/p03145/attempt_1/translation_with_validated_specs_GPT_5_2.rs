use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse three integers; behavior is unspecified if parsing fails or fewer than 3 exist.
    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let _c: i64 = it.next().unwrap().parse().unwrap(); // ignored per spec

    // Compute round((a*b)/2). Use f64 to match C's double rounding with "%.0lf".
    // Preconditions state a*b is representable in integer arithmetic domain (no overflow).
    let ans = 0.5f64 * ((a * b) as f64);

    // Print with no fractional part, no trailing newline (matches C printf("%.0lf", ...)).
    print!("{:.0}", ans);
}
