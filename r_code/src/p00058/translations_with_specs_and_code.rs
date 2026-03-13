// Translated from the provided C program following the SPEC.
// Note: The SPEC documents that the original C program did not validate
// that scanf read all eight values (which could lead to undefined behavior).
// This Rust translation follows the SPEC's *intended* safe behavior: it
// processes input as successive groups of exactly eight doubles and ignores
// any trailing incomplete group. This differs from the original C in that
// we do not attempt to reproduce the C program's undefined/unspecified behavior.

use std::io::{self, Read};

fn main() {
    // Read entire stdin as text
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (matching C's behavior of no special error reporting).
        return;
    }

    // Parse whitespace-separated tokens into f64. If a token fails to parse,
    // stop parsing further tokens (SPEC assumes tokens are valid doubles).
    let mut nums: Vec<f64> = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<f64>() {
            Ok(v) => nums.push(v),
            Err(_) => break,
        }
    }

    // Process complete groups of 8 doubles only.
    for chunk in nums.chunks(8) {
        if chunk.len() < 8 {
            break;
        }
        // Mapping: a[0]=x1, a[1]=y1, a[2]=x2, a[3]=y2, a[4]=x3, a[5]=y3, a[6]=x4, a[7]=y4
        let x1 = chunk[0];
        let y1 = chunk[1];
        let x2 = chunk[2];
        let y2 = chunk[3];
        let x3 = chunk[4];
        let y3 = chunk[5];
        let x4 = chunk[6];
        let y4 = chunk[7];

        let dot = (x1 - x2) * (x3 - x4) + (y1 - y2) * (y3 - y4);

        if dot == 0.0 {
            // Exact equality per SPEC
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
