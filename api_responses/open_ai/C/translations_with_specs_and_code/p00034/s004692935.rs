// translate_c_to_rust.rs
//
// Translated from the provided C program according to the SPEC.
// Behavior:
//  - Reads repeated records from stdin. Each record is 10 integers (buckets) followed by v1 and v2.
//  - For each full record (12 integers) prints a single integer and newline:
//      the smallest k in 1..10 such that x <= running_sum(k), where
//      x = (v1 as f64) / ((v1+v2) as f64) * S, S = sum of the 10 buckets.
//    If no such k exists, prints 0.
//
// Notes about parsing and undefined input:
//  - The original C used scanf with commas (e.g. "%d,") as separators. The SPEC states
//    commas are expected but whitespace handling is implementation-dependent.
//  - This Rust version accepts integers separated by any non-sign/digit characters
//    (commas, whitespace, etc.). It requires a full 12 integers for a record. If fewer
//    than 12 integers remain, processing stops (safe behavior). The SPEC allowed
//    undefined behavior when v1/v2 are missing; here we choose to stop instead of
//    attempting to reproduce undefined behavior.
//  - Floating-point behavior for v1+v2 == 0 follows IEEE semantics (producing NaN or +/-inf),
//    and comparisons with NaN yield false, resulting in output 0 (matching the likely C outcome).
//
// Implementation choices:
//  - Uses safe, idiomatic Rust only (no unsafe).
//  - Parses stdin entirely into a string and extracts integer tokens.
//  - Uses i64 for running sums when converting to f64 to avoid accidental overflow
//    differences; individual values are parsed into i32 to match the SPEC's "C int" assumption.
//
// Strict I/O: prints exactly one integer per processed record followed by '\n'.

use std::io::{self, Read};

fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (no output). This keeps behavior simple.
        return;
    }

    // Split into tokens that look like integers.
    // We split on any character that is not a digit or sign, then try to parse tokens as i32.
    let mut nums: Vec<i32> = Vec::new();
    for token in input
        .split(|c: char| !(c.is_ascii_digit() || c == '+' || c == '-'))
        .filter(|t| !t.is_empty())
    {
        // Try to parse as i32; ignore tokens that fail to parse.
        if let Ok(n) = token.parse::<i32>() {
            nums.push(n);
        } else {
            // If a token does not parse as i32, skip it.
            // The SPEC allows undefined/malformed input; skipping is a safe choice.
        }
    }

    let mut idx = 0usize;
    let mut out = String::new();

    // Process records while we have at least 12 integers remaining.
    while idx + 12 <= nums.len() {
        let r_slice = &nums[idx..idx + 10];
        let v1 = nums[idx + 10];
        let v2 = nums[idx + 11];
        idx += 12;

        // Compute S as sum of r[0..9]. Use i64 for accumulation then cast to f64.
        let s_i64: i64 = r_slice.iter().map(|&x| x as i64).sum();

        // Compute x = (double)v1 / (double)(v1+v2) * (double)S
        let denom = (v1 as i64) + (v2 as i64);
        let x = (v1 as f64) / (denom as f64) * (s_i64 as f64);

        // Scan buckets accumulating running sum and find first index where x <= running_sum
        let mut running: i64 = 0;
        let mut num = 0;
        for (i, &val) in r_slice.iter().enumerate() {
            running += val as i64;
            if x <= (running as f64) {
                num = i as i32 + 1;
                break;
            }
        }

        out.push_str(&format!("{}\n", num));
    }

    // Write all outputs to stdout.
    // (This preserves the one-integer-per-line output formatting.)
    print!("{}", out);
}
