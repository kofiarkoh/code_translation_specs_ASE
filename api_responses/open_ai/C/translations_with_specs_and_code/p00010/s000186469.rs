// circumcircle.rs
//
// Translated from the provided C program according to the specification.
// Reads an integer N then N triples of 2D points (6 doubles per case).
// For each triple computes the circumcenter and radius and prints them
// formatted with three decimal places: "<cx> <cy> <r>\n"
//
// This Rust translation preserves the original arithmetic order and
// floating-point equality checks (including exact `==` comparisons),
// so behavior for degenerate cases (collinear points, identical points,
// divisions by zero producing inf/nan) matches the original program's
// semantics as described in the spec.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and split on whitespace to emulate scanf tokenization.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, mimic the C program's behavior of doing nothing.
        return;
    }
    let mut tokens = input.split_whitespace();

    // Parse the initial integer N. If it cannot be read, do nothing (like the C code's scanf failure).
    let n: i32 = match tokens.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(val) => val,
        None => return,
    };

    // If N <= 0, process no cases (matches C for-loop behavior).
    if n <= 0 {
        return;
    }

    // For each test case, attempt to parse six f64 values. If any are missing or fail to parse,
    // stop processing further cases (this is a reasonable safe analogue to scanf partial reads).
    for _ in 0..n {
        let x1 = match tokens.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let y1 = match tokens.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let x2 = match tokens.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let y2 = match tokens.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let x3 = match tokens.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let y3 = match tokens.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };

        // Follow the same sequence of computations as the C code to preserve behavior,
        // including possible inf/nan results when dividing by zero.
        let midx1 = (x1 + x2) / 2.0;
        let midy1 = (y1 + y2) / 2.0;
        let midx2 = (x1 + x3) / 2.0;
        let midy2 = (y1 + y3) / 2.0;

        let slant1 = - (x2 - x1) / (y2 - y1); // may be inf or nan if y2 == y1
        let slant2 = - (x3 - x1) / (y3 - y1); // may be inf or nan if y3 == y1

        let slice1 = midy1 - slant1 * midx1;
        let slice2 = midy2 - slant2 * midx2;

        let mut circlex = (slice2 - slice1) / (slant1 - slant2);
        let mut circley = slant1 * circlex + slice1;

        // Replicate the exact if/else ordering from the C implementation.
        if x2 == x1 {
            circley = midy1;
            circlex = (circley - slice2) / slant2;
        } else if y2 == y1 {
            circlex = midx1;
            circley = slant2 * circlex + slice2;
        }

        if x3 == x1 {
            circley = midy2;
            circlex = (circley - slice1) / slant1;
        } else if y3 == y1 {
            circlex = midx2;
            circley = slant1 * circlex + slice1;
        }

        if x2 == x1 && y3 == y1 {
            circley = midy1;
            circlex = midx2;
        } else if x3 == x1 && y2 == y1 {
            circley = midy2;
            circlex = midx1;
        }

        let dx = circlex - x1;
        let dy = circley - y1;
        let radius = (dx * dx + dy * dy).sqrt();

        // Print with three digits after the decimal, matching C's "%0.3f %0.3f %0.3f\n".
        // Using default locale and formatting; NaN/inf will be printed according to Rust's formatting.
        println!("{:.3} {:.3} {:.3}", circlex, circley, radius);
    }
}
