use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse integers separated by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Specification: reads three integer values; third is unused.
    // If fewer than three are present, the behavior is not specified; we choose to treat missing
    // values as 0 to keep the program total and compilable without inventing extra I/O.
    let a: i128 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i128 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let _c: i128 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // Compute half the product, rounded to the nearest whole number.
    // We interpret "rounded to the nearest whole number" as rounding half away from zero:
    //   round((a*b)/2) with ties at .5 going to the integer with larger magnitude.
    // This preserves the stated invariant while being well-defined for negative products too.
    let p = a.saturating_mul(b);

    let result: i128 = if p >= 0 {
        (p + 1) / 2
    } else {
        (p - 1) / 2
    };

    print!("{}", result);
}
