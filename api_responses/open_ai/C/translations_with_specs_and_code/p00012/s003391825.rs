// point_in_triangle.rs
//
// Translated from the given C program per the provided specification.
// Behavior: For each group of 8 whitespace-separated doubles read from stdin
// (x1 y1 x2 y2 x3 y3 xp yp), print "YES\n" if the point (xp,yp) lies inside
// or on the triangle formed by (x1,y1),(x2,y2),(x3,y3) per an area-based
// test (Heron's formula), otherwise print "NO\n".
//
// Notes on discrepancy:
// The original C code used `while (scanf(...) != EOF)` which is incorrect
// (scanf returns the number of items successfully read). Per the SPEC, the
// program should stop processing when a full set of eight parseable doubles
// cannot be read (EOF or non-numeric token). This Rust implementation follows
// the SPEC and stops on EOF or on the first non-numeric token, emitting one
// output line per complete set of eight parsed doubles.
//
// Implementation details:
// - Uses f64 arithmetic and f64::hypot for distances.
// - Heron's formula is applied exactly (no clamping) so NaN/Inf propagate the same
//   way as in the C program when degenerate/nonnormal inputs occur.
// - Absolute tolerance for area comparison is 1e-8.
// - No unsafe code is used.

use std::io::{self, Read};

fn heron_area(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;
    // Preserve original behavior: do not clamp the radicand. If it's negative due
    // to floating-point error, sqrt will produce NaN as in the C program.
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    // Read entire stdin to a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit normally (as per spec, program returns 0).
        return;
    }

    let mut iter = input.split_whitespace();
    const TOL: f64 = 1e-8;

    loop {
        // Attempt to read 8 tokens; if any token is missing, stop processing.
        let mut toks = [""; 8];
        for i in 0..8 {
            match iter.next() {
                Some(t) => toks[i] = t,
                None => return, // EOF or not enough tokens for another record
            }
        }

        // Parse all 8 tokens as f64. If any fails (non-numeric), stop processing.
        let parsed: Result<[f64; 8], _> = (|| {
            let mut arr = [0f64; 8];
            for i in 0..8 {
                arr[i] = toks[i].parse::<f64>()?;
            }
            Ok(arr)
        })();

        let vals = match parsed {
            Ok(v) => v,
            Err(_) => return, // Non-numeric token encountered; stop processing.
        };

        let x1 = vals[0];
        let y1 = vals[1];
        let x2 = vals[2];
        let y2 = vals[3];
        let x3 = vals[4];
        let y3 = vals[5];
        let xp = vals[6];
        let yp = vals[7];

        // Side lengths
        let a = (x1 - x2).hypot(y1 - y2);
        let b = (x2 - x3).hypot(y2 - y3);
        let c = (x1 - x3).hypot(y1 - y3);

        let ap = (x1 - xp).hypot(y1 - yp);
        let bp = (x2 - xp).hypot(y2 - yp);
        let cp = (xp - x3).hypot(yp - y3);

        // Areas via Heron's formula
        let mut s = 0.0f64;
        s += heron_area(a, ap, bp);
        s += heron_area(b, bp, cp);
        s += heron_area(c, cp, ap);

        let S = heron_area(a, b, c);

        if (s - S).abs() < TOL {
            print!("YES\n");
        } else {
            print!("NO\n");
        }
    }
}
