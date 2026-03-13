// translated_spec.rs
//
// Translation of the given Specification into Rust.
// - Reads a sequence of test cases from stdin.
// - For each test case: reads n circles (x, y, r), then m queries (tx, ty, sx, sy).
// - For each query prints "Safe" if any circle "hits" the segment, otherwise "Danger".
// - Follows the specification semantics closely, including degenerate-segment handling
//   and endpoint-inclusion rules.
//
// Notes on numerical behavior:
// - Uses f64 for geometric computations. A tiny epsilon is used for inclusive checks
//   to be robust against floating-point rounding while preserving the spec's inclusive
//   boundary behavior (points at distance exactly r count as "inside" for endpoint checks).
// - Degenerate segments (both endpoints equal) are treated as not hit (always "Danger").
//
// The program stops processing when it cannot read the next expected integer for `n`
// or when a non-positive `n` is encountered (matching the scanf-based behavior described).

use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

fn main() {
    // Read all stdin and parse whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut iter = input.split_whitespace();

    // Helper to get next i64; returns None if no more tokens or parse fails.
    let mut next_i64 = || -> Option<i64> {
        iter.next().and_then(|s| s.parse::<i64>().ok())
    };

    const EPS: f64 = 1e-12;

    // Outer loop: while we can read n and n > 0
    loop {
        let n = match next_i64() {
            Some(v) => v,
            None => break, // EOF or parse failure: stop processing further test cases
        };
        if n <= 0 {
            break;
        }

        // Read n circles
        let mut circles: Vec<Circle> = Vec::with_capacity(n as usize);
        let mut fail = false;
        for _ in 0..n {
            let x = match next_i64() { Some(v) => v, None => { fail = true; break; } };
            let y = match next_i64() { Some(v) => v, None => { fail = true; break; } };
            let r = match next_i64() { Some(v) => v, None => { fail = true; break; } };
            circles.push(Circle { x, y, r });
        }
        if fail {
            break;
        }

        // Read m
        let m = match next_i64() {
            Some(v) => v,
            None => break,
        };

        // For each query, perform hit checks and print result
        for _ in 0..m {
            let tx = match next_i64() { Some(v) => v, None => { fail = true; break; } };
            let ty = match next_i64() { Some(v) => v, None => { fail = true; break; } };
            let sx = match next_i64() { Some(v) => v, None => { fail = true; break; } };
            let sy = match next_i64() { Some(v) => v, None => { fail = true; break; } };

            if fail {
                break;
            }

            let mut is_safe = false;

            // Degenerate segment: treated as not hit per specification.
            if tx == sx && ty == sy {
                // is_safe remains false -> "Danger"
            } else {
                for &c in &circles {
                    if circle_hits_segment(c, tx, ty, sx, sy, EPS) {
                        is_safe = true;
                        break;
                    }
                }
            }

            if is_safe {
                println!("Safe");
            } else {
                println!("Danger");
            }
        }

        if fail {
            break;
        }
    }
}

// Returns true if the circle (cx,cy,r) "hits" the segment from (tx,ty) to (sx,sy)
// according to the Specification:
// - Exactly one endpoint at distance <= r and the other outside -> hit.
// - Or the segment intersects the circle (one or two intersection points within segment extents inclusive) -> hit.
// - Degenerate segment (endpoints equal) is treated as not hit (caller handles this).
fn circle_hits_segment(c: Circle, tx: i64, ty: i64, sx: i64, sy: i64, eps: f64) -> bool {
    let cx = c.x as f64;
    let cy = c.y as f64;
    let r = c.r as f64;

    let tx_f = tx as f64;
    let ty_f = ty as f64;
    let sx_f = sx as f64;
    let sy_f = sy as f64;

    // squared distances from endpoints to center
    let dx1 = tx_f - cx;
    let dy1 = ty_f - cy;
    let d1_sq = dx1 * dx1 + dy1 * dy1;

    let dx2 = sx_f - cx;
    let dy2 = sy_f - cy;
    let d2_sq = dx2 * dx2 + dy2 * dy2;

    let r_sq = r * r;

    // If both endpoints are inside or on the circle, it's NOT counted as a hit.
    if d1_sq <= r_sq + eps && d2_sq <= r_sq + eps {
        return false;
    }

    let d1_inside = d1_sq <= r_sq + eps;
    let d2_inside = d2_sq <= r_sq + eps;

    // Exactly one endpoint inside/on and the other outside -> hit.
    if d1_inside ^ d2_inside {
        return true;
    }

    // At this point both endpoints are outside (or numeric borderline considered outside).
    // Check whether the segment intersects the circle within segment extents [0,1].
    let vx = sx_f - tx_f;
    let vy = sy_f - ty_f;

    let a = vx * vx + vy * vy;
    // If a == 0 the segment is degenerate; caller normally handles eq endpoints, but treat as not hit here.
    if a <= eps {
        return false;
    }

    // Solve quadratic for u in parametric p(u) = t + u*(s-t), 0 <= u <= 1
    let ox = tx_f - cx;
    let oy = ty_f - cy;
    let b = 2.0 * (ox * vx + oy * vy);
    let c_term = ox * ox + oy * oy - r_sq;

    let disc = b * b - 4.0 * a * c_term;

    if disc < -eps {
        // No real roots: no intersection
        return false;
    }

    // Compute roots (handle small negative disc due to numeric error)
    let disc_nonneg = if disc < 0.0 { 0.0 } else { disc };
    let sqrt_disc = disc_nonneg.sqrt();

    let u1 = (-b - sqrt_disc) / (2.0 * a);
    let u2 = (-b + sqrt_disc) / (2.0 * a);

    // If either intersection parameter lies within [0,1] (inclusive, with eps), it's a hit.
    if (u1 >= -eps && u1 <= 1.0 + eps) || (u2 >= -eps && u2 <= 1.0 + eps) {
        return true;
    }

    false
}
