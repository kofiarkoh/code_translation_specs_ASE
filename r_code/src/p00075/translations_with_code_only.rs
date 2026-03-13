// main.rs
// Translated from the provided C program.
// Behavior: read lines from stdin, each expected as "int,double,double" (commas as separators).
// For each record compute bmi = w / (t * t); if bmi >= 25 print the integer followed by a newline.
//
// Notes about translation choices:
// - The original C used scanf("%d,%lf,%lf", ... ) in a loop. That can parse records across line
//   boundaries and has subtle behavior on malformed input. This Rust translation reads stdin
//   line-by-line and expects each line to contain one record with fields separated by commas.
//   This preserves the common, intended I/O behavior while keeping the implementation safe and
//   idiomatic in Rust.
// - Fields are trimmed of surrounding whitespace before parsing. The C scanf format would skip
//   leading whitespace before numeric conversions but requires the literal commas to match;
//   trimming makes the parsing slightly more tolerant of surrounding spaces while preserving
//   correct numeric interpretation.
// - Floating-point math uses f64, matching C's double.
// - If a line does not contain exactly three comma-separated fields or parsing fails, the line is
//   ignored (no output). The original C program would have exhibited undefined or surprising
//   behavior on malformed input; this translation avoids undefined behavior while keeping the
//   observable outputs for valid inputs identical.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for maybe_line in stdin.lock().lines() {
        let line = match maybe_line {
            Ok(s) => s,
            Err(_) => break,
        };

        // Split into exactly three comma-separated fields.
        // Using splitn to allow extra commas in the third field would differ from scanf behavior;
        // here we require exactly three fields (any extra commas make the third field include them).
        let mut parts = line.splitn(3, ',');
        let a = parts.next();
        let b = parts.next();
        let c = parts.next();

        if let (Some(a), Some(b), Some(c)) = (a, b, c) {
            // Trim surrounding whitespace to allow reasonable input like "1, 80.0, 1.8"
            let a_trim = a.trim();
            let b_trim = b.trim();
            let c_trim = c.trim();

            // Parse fields. If any parse fails, skip this line.
            let parsed = match (a_trim.parse::<i32>(), b_trim.parse::<f64>(), c_trim.parse::<f64>()) {
                (Ok(num), Ok(w), Ok(t)) => Some((num, w, t)),
                _ => None,
            };

            if let Some((num, w, t)) = parsed {
                // Compute BMI as in C: double division
                let bmi = w / (t * t);
                if bmi >= 25.0 {
                    println!("{}", num);
                }
            }
        }
        // If the line didn't have three comma-separated parts, do nothing (skip).
    }
}
