// triangle_point_test.rs
//
// Translates the intended behavior described in the SPEC into Rust.
// Note: The original C code's gaiseki implementation does not compute the
// oriented cross product as described in the SPEC. The SPEC requires
// oriented 2D cross-products for edge (A->B) vs (A->P):
//    cross = (Bx - Ax)*(Py - Ay) - (By - Ay)*(Px - Ax)
// The C code used a different formula. Per instruction, the SPEC is treated
// as authoritative; this implementation follows the SPEC.
//
// Input format (per SPEC): repeated records until EOF, each record is
// eight double-precision numbers in this order: x0,y0,x1,y1,x2,y2,x3,y3
// using literal commas between numbers (format analogous to "%lf,%lf,...").
// The parser implemented here matches that format closely:
//  - Leading whitespace before each number is allowed (as %lf does).
//  - A literal comma must appear between the first seven numbers and the next conversion.
//  - Whitespace between a number and the comma (i.e., trailing whitespace before comma)
//    will cause the record to be rejected (matching scanf behavior where a literal comma
//    must match the next input character).
//
// For each fully and successfully parsed record, prints exactly one line:
// "YES\n" if all three oriented cross-products (for edges (x0,y0)->(x1,y1),
// (x1,y1)->(x2,y2), (x2,y2)->(x0,y0) with point (x3,y3)) are >= 0;
// otherwise prints "NO\n".
//
// If input is malformed or a full 8-number record cannot be parsed, parsing stops
// (no partial/undefined behavior is emulated; only fully parsed records are processed).

use std::io::{self, Read};

fn oriented_cross(ax: f64, ay: f64, bx: f64, by: f64, px: f64, py: f64) -> f64 {
    // cross of (B - A) and (P - A)
    (bx - ax) * (py - ay) - (by - ay) * (px - ax)
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit silently (no records processed).
        return;
    }

    let s = input.as_str();
    let bytes = s.as_bytes();
    let len = s.len();
    let mut pos = 0usize;

    loop {
        // Parse 8 numbers with literal commas between the first 7 numbers.
        let mut vals = [0f64; 8];
        let mut ok = true;

        for i in 0..8 {
            // Skip leading whitespace (scanf %lf skips leading whitespace)
            while pos < len && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            if pos >= len {
                ok = false;
                break;
            }

            if i < 7 {
                // Find the next comma which separates this number and the next.
                let rem = &s[pos..];
                match rem.find(',') {
                    Some(rel_idx) => {
                        let comma_idx = pos + rel_idx;
                        let token = &s[pos..comma_idx];
                        // Allow leading whitespace in token (scanf does), but do NOT trim trailing whitespace,
                        // because whitespace before the comma would make scanf fail to match the literal comma.
                        let token_trim_start = token.trim_start_matches(|c: char| c.is_whitespace());
                        if token_trim_start.is_empty() {
                            ok = false;
                            break;
                        }
                        match token_trim_start.parse::<f64>() {
                            Ok(v) => {
                                vals[i] = v;
                                // Advance pos to after the comma.
                                pos = comma_idx + 1;
                            }
                            Err(_) => {
                                // Malformed number -> stop parsing further (treat as EOF/malformed).
                                ok = false;
                                break;
                            }
                        }
                    }
                    None => {
                        // No comma found -> cannot parse full record.
                        ok = false;
                        break;
                    }
                }
            } else {
                // Last number: parse longest prefix (after trimming leading whitespace)
                let token_rest = &s[pos..];
                let trimmed = token_rest.trim_start_matches(|c: char| c.is_whitespace());
                // leading whitespace count to advance pos appropriately later
                let leading_ws = token_rest.len() - trimmed.len();
                if trimmed.is_empty() {
                    ok = false;
                    break;
                }

                // Find the longest prefix of `trimmed` that parses as f64.
                // We try increasing lengths and remember the last successful parse.
                let mut last_ok_end: Option<usize> = None;
                let mut last_ok_val: f64 = 0.0;
                // To be pragmatic and avoid extremely expensive repeated parsing on very long inputs,
                // we cap the search to trimmed.len(), which is safe; typical numeric tokens are short.
                for end in 1..=trimmed.len() {
                    let candidate = &trimmed[..end];
                    // Attempt to parse the candidate as f64; candidate must be exactly the numeric token.
                    if let Ok(v) = candidate.parse::<f64>() {
                        last_ok_end = Some(end);
                        last_ok_val = v;
                    }
                }

                match last_ok_end {
                    Some(end) => {
                        vals[i] = last_ok_val;
                        // Advance pos by leading_ws + end
                        pos = pos + leading_ws + end;
                    }
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
        }

        if !ok {
            break;
        }

        // We parsed a full record of 8 doubles: compute the three cross-products as per SPEC.
        let x0 = vals[0];
        let y0 = vals[1];
        let x1 = vals[2];
        let y1 = vals[3];
        let x2 = vals[4];
        let y2 = vals[5];
        let x3 = vals[6];
        let y3 = vals[7];

        let c1 = oriented_cross(x0, y0, x1, y1, x3, y3);
        let c2 = oriented_cross(x1, y1, x2, y2, x3, y3);
        let c3 = oriented_cross(x2, y2, x0, y0, x3, y3);

        if c1 >= 0.0 && c2 >= 0.0 && c3 >= 0.0 {
            // Exactly "YES\n"
            println!("YES");
        } else {
            // Exactly "NO\n"
            println!("NO");
        }

        // Continue parsing next record until EOF or malformed input encountered.
    }
}
