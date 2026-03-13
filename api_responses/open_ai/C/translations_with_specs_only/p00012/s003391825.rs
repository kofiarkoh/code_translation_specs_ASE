// point_in_triangle.rs
//
// Translates the provided specification into Rust.
// Reads records of 8 whitespace-separated doubles (x1 y1 x2 y2 x3 y3 xp yp) from stdin.
// For each full record successfully parsed, prints "YES" if the query point lies inside
// or on the boundary of the triangle (using an area-based test with absolute tolerance 1e-8),
// otherwise prints "NO".
//
// Parsing behavior: if any token fails to parse as a f64 (or EOF is encountered before
// eight tokens are read), the program stops processing further records and exits normally
// (status code 0). This mirrors the scanf-based semantics described in the specification.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split on whitespace tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, behave like normal termination (no further output).
        return;
    }

    // Token iterator
    let mut tokens = input.split_whitespace();

    const EPS: f64 = 1e-8;

    loop {
        // Try to collect exactly 8 tokens for one record.
        let mut vals = [0f64; 8];
        let mut ok = true;
        for i in 0..8 {
            match tokens.next() {
                Some(tok) => {
                    // If parse fails, stop processing further records (like scanf failure).
                    match tok.parse::<f64>() {
                        Ok(v) => vals[i] = v,
                        Err(_) => {
                            ok = false;
                            break;
                        }
                    }
                }
                None => {
                    // Not enough tokens left -> end of input
                    ok = false;
                    break;
                }
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

        // Area via absolute value of the 2D cross-product (shoelace) divided by 2.
        // This avoids sqrt and matches the area-based criterion (sum of subtriangle areas).
        let tri_area = ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs()) * 0.5;

        // Sub-triangle areas: replace each vertex in turn with the query point.
        let a1 = ((xp * (y2 - y3) + x2 * (y3 - yp) + x3 * (yp - y2)).abs()) * 0.5;
        let a2 = ((x1 * (yp - y3) + xp * (y3 - y1) + x3 * (y1 - yp)).abs()) * 0.5;
        let a3 = ((x1 * (y2 - yp) + x2 * (yp - y1) + xp * (y1 - y2)).abs()) * 0.5;

        let sum_sub = a1 + a2 + a3;

        if (sum_sub - tri_area).abs() <= EPS {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
