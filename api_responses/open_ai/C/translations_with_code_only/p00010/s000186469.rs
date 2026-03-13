// Translated from the provided C code.
// Preserves floating-point semantics (including exact equality checks and IEEE-754 division behavior).
// Uses safe Rust only.

use std::io::{self, Read};

fn main() {
    // Read all stdin and split on whitespace, similar to successive scanf calls.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut toks = input.split_whitespace();

    // Parse the first token as an integer count.
    let count: i32 = match toks.next().and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return, // no input, exit
    };

    for _ in 0..count {
        // Read six doubles like scanf("%lf %lf %lf %lf %lf %lf", ...)
        let x1: f64 = toks.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y1: f64 = toks.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let x2: f64 = toks.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y2: f64 = toks.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let x3: f64 = toks.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y3: f64 = toks.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);

        let midx1 = (x1 + x2) / 2.0;
        let midy1 = (y1 + y2) / 2.0;
        let midx2 = (x1 + x3) / 2.0;
        let midy2 = (y1 + y3) / 2.0;

        // Note: division by zero with f64 yields +/-INF or NaN per IEEE-754, matching C double behavior.
        let slant1 = - (x2 - x1) / (y2 - y1);
        let slant2 = - (x3 - x1) / (y3 - y1);
        let slice1 = midy1 - slant1 * midx1;
        let slice2 = midy2 - slant2 * midx2;

        let mut circlex = (slice2 - slice1) / (slant1 - slant2);
        let mut circley = slant1 * circlex + slice1;

        // Preserve exact-equality checks on floats as in the C code.
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

        // Match C's printf("%0.3f %0.3f %0.3f\n", ...);
        // Using "{:.3}" prints three fractional digits, including trailing zeros.
        println!("{:.3} {:.3} {:.3}", circlex, circley, radius);
    }
}
