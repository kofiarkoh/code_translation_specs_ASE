// point_in_triangle.rs
//
// Translated from the provided C program according to the SPEC.
// Behavior:
//  - Read repeated test cases from stdin. Each test case is eight whitespace-separated doubles:
//      x1 y1 x2 y2 x3 y3 xp yp
//  - For each test case compute triangle area S (sides a,b,c) and s = sum of areas of
//    triangles (xp,yp) with each edge, using Heron's formula.
//  - If |s - S| < 1e-8 print "YES\n", otherwise print "NO\n".
//  - Stop silently on EOF or if a token cannot be parsed as a double (matches SPEC error handling).
//
// Implementation notes:
//  - Uses safe, idiomatic Rust (no unsafe).
//  - Uses f64::hypot, f64::sqrt, and f64::abs to mirror the C math functions.
//  - Parsing stops and program exits normally on EOF or on a non-numeric token, as specified.

use std::io::{self, Read};

fn triangle_area_by_sides(a: f64, b: f64, c: f64) -> f64 {
    // Heron's formula: sqrt(s*(s-a)*(s-b)*(s-c)), where s = (a+b+c)/2
    let s = (a + b + c) / 2.0;
    // Let floating-point behavior (including producing NaN for invalid values) be as in C.
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    // Read all input from stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently with status 0 per SPEC (no diagnostics).
        return;
    }

    let mut iter = input.split_whitespace();

    const TOL: f64 = 1e-8;

    loop {
        // Attempt to read eight f64 values for a test case.
        let mut vals = [0f64; 8];
        let mut ok = true;
        for i in 0..8 {
            match iter.next() {
                Some(tok) => {
                    // If parse fails (non-numeric token), treat like EOF and exit normally.
                    match tok.parse::<f64>() {
                        Ok(v) => vals[i] = v,
                        Err(_) => { ok = false; break; }
                    }
                }
                None => { ok = false; break; } // EOF reached
            }
        }
        if !ok {
            break;
        }

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

        // Distances from test point to triangle vertices
        let ap = (x1 - xp).hypot(y1 - yp);
        let bp = (x2 - xp).hypot(y2 - yp);
        // Note: hypot is symmetric; preserve original order from C for clarity.
        let cp = (xp - x3).hypot(yp - y3);

        // Areas by Heron's formula
        let mut s = triangle_area_by_sides(a, ap, bp);
        s += triangle_area_by_sides(b, bp, cp);
        s += triangle_area_by_sides(c, cp, ap);
        let S = triangle_area_by_sides(a, b, c);

        if (s - S).abs() < TOL {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
