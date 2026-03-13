// Translated from the provided C program according to the specification.
// Preserves semantics: for each query segment, prints "Safe" if any circle
// "hits" the segment (exactly one endpoint inside/on circle, or the segment
// intersects the circle within its segment extents), otherwise "Danger".
use std::io::{self, Read};

const EPS: f64 = 1e-7;

#[derive(Copy, Clone)]
struct Wall {
    x: i64,
    y: i64,
    r: i64,
}

#[derive(Copy, Clone)]
struct Query {
    tx: i64,
    ty: i64,
    sx: i64,
    sy: i64,
}

fn hitcheck_circle(w: Wall, q: Query) -> bool {
    // Use wider integer types to reduce overflow risk in intermediate integer arithmetic.
    // Follow the logic of the C version closely.
    let mut in_count = 0;

    // Bounding box sieve (integer)
    let mut minx = q.tx;
    let mut maxx = q.sx;
    if minx > maxx {
        std::mem::swap(&mut minx, &mut maxx);
    }
    let mut miny = q.ty;
    let mut maxy = q.sy;
    if miny > maxy {
        std::mem::swap(&mut miny, &mut maxy);
    }

    // If circle's bounding box doesn't intersect segment's bounding box, no hit.
    if w.x + w.r < minx || maxx < w.x - w.r {
        return false;
    }
    if w.y + w.r < miny || maxy < w.y - w.r {
        return false;
    }

    // Check endpoints: distance squared <= r*r counts as "inside/on"
    let dx1 = q.tx - w.x;
    let dy1 = q.ty - w.y;
    let dx2 = q.sx - w.x;
    let dy2 = q.sy - w.y;
    let rr = (w.r as i128) * (w.r as i128);
    if (dx1 as i128) * (dx1 as i128) + (dy1 as i128) * (dy1 as i128) <= rr {
        in_count += 1;
    }
    if (dx2 as i128) * (dx2 as i128) + (dy2 as i128) * (dy2 as i128) <= rr {
        in_count += 1;
    }
    if in_count == 1 {
        return true;
    }
    if in_count == 2 {
        return false;
    }

    // Line equation coefficients for the line through the segment:
    // a*x + b*y + c = 0 (as in C code)
    let a = q.ty - q.sy; // i64
    let b = q.sx - q.tx;
    // c = (q.tx-q.sx)*q.ty - (q.ty-q.sy)*q.tx; (matches C)
    let c = (q.tx - q.sx) * q.ty - (q.ty - q.sy) * q.tx;

    // Check if distance from circle center to infinite line <= r using integer arithmetic:
    // (a*wx + b*wy + c)^2 <= r^2 * (a^2 + b^2)
    let lhs = {
        let val = (a as i128) * (w.x as i128) + (b as i128) * (w.y as i128) + (c as i128);
        val * val
    };
    let rhs = {
        let coeff = (a as i128) * (a as i128) + (b as i128) * (b as i128);
        rr * coeff
    };
    if lhs <= rhs {
        // Now check whether the intersection points (if any) lie within the segment extents.
        // Convert to the algebra used in C: change sign of c to have a*x + b*y = c
        let c_neg = -(c as i128) as f64;
        let a_f = a as f64;
        let b_f = b as f64;
        let wx = w.x as f64;
        let wy = w.y as f64;
        let r_f = w.r as f64;

        let minx_f = minx as f64;
        let maxx_f = maxx as f64;
        let miny_f = miny as f64;
        let maxy_f = maxy as f64;

        if b == 0 {
            // Vertical line case in C branch (b==0)
            // C computes yy = r*r - ((c/a - w.x)^2) where c here is c_neg (c after negation), note c/a is double division.
            // Guard division by zero: a == 0 would mean degenerate segment (filtered by caller), but check anyway.
            if a == 0 {
                return false;
            }
            let ca_over_a = c_neg / a_f; // c/a as f64
            let diff = ca_over_a - wx;
            let yy = r_f * r_f - diff * diff;
            // If yy < 0 due to tiny negative rounding, clamp small negative to 0
            let yy = if yy < 0.0 && yy > -1e-12 { 0.0 } else { yy };
            if yy < 0.0 {
                return false;
            }
            let y1 = wy + yy.sqrt();
            let y2 = wy - yy.sqrt();
            // Check whether either y intersects the segment's y-range (with EPS tolerance), using same inequality style as C
            if (miny_f < y1 + EPS && y1 < maxy_f + EPS) || (miny_f < y2 + EPS && y2 < maxy_f + EPS) {
                return true;
            }
        } else {
            // General case (b != 0)
            let cb = c_neg / b_f; // c/b
            let ab = a_f / b_f;   // a/b
            // Solve quadratic for x: x2k * x^2 + xk * x + tk = 0 (as in C)
            let x2k = 1.0 + ab * ab;
            let xk = -2.0 * wx + 2.0 * (wy - cb) * ab;
            let tk = wx * wx + (wy - cb) * (wy - cb) - r_f * r_f;
            let disc = xk * xk - 4.0 * x2k * tk;
            // Clamp tiny negative discriminants due to floating point error to zero
            let disc = if disc < 0.0 && disc > -1e-12 { 0.0 } else { disc };
            if disc < 0.0 {
                return false;
            }
            let sqrt_disc = disc.sqrt();
            let denom = 2.0 * x2k;
            // Avoid division by zero (x2k should be > 0 since 1+ab^2 >= 1)
            if denom == 0.0 {
                return false;
            }
            let x1 = (-xk + sqrt_disc) / denom;
            let x2 = (-xk - sqrt_disc) / denom;
            // Check whether either intersection x lies within segment x-range (with EPS tolerance), using same inequality style as C
            if (minx_f < x1 + EPS && x1 < maxx_f + EPS) || (minx_f < x2 + EPS && x2 < maxx_f + EPS) {
                return true;
            }
        }
    }

    false
}

fn main() {
    // Read all stdin, split by whitespace to integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut toks = input.split_whitespace();
    loop {
        // Read n
        let n = match toks.next() {
            Some(s) => match s.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        if n <= 0 {
            break;
        }
        // Read n walls
        let mut walls: Vec<Wall> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let x = match toks.next() {
                Some(s) => match s.parse::<i64>() { Ok(v) => v, Err(_) => return },
                None => return,
            };
            let y = match toks.next() {
                Some(s) => match s.parse::<i64>() { Ok(v) => v, Err(_) => return },
                None => return,
            };
            let r = match toks.next() {
                Some(s) => match s.parse::<i64>() { Ok(v) => v, Err(_) => return },
                None => return,
            };
            walls.push(Wall { x, y, r });
        }
        // Read m
        let m = match toks.next() {
            Some(s) => match s.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        for _ in 0..m {
            let tx = match toks.next() {
                Some(s) => match s.parse::<i64>() { Ok(v) => v, Err(_) => return },
                None => return,
            };
            let ty = match toks.next() {
                Some(s) => match s.parse::<i64>() { Ok(v) => v, Err(_) => return },
                None => return,
            };
            let sx = match toks.next() {
                Some(s) => match s.parse::<i64>() { Ok(v) => v, Err(_) => return },
                None => return,
            };
            let sy = match toks.next() {
                Some(s) => match s.parse::<i64>() { Ok(v) => v, Err(_) => return },
                None => return,
            };
            let q = Query { tx, ty, sx, sy };
            let mut hidden = false;
            // Degenerate segment: treated as not hit ("Danger")
            if q.tx != q.sx || q.ty != q.sy {
                for &w in &walls {
                    if hitcheck_circle(w, q) {
                        hidden = true;
                        break;
                    }
                }
            }
            if hidden {
                println!("Safe");
            } else {
                println!("Danger");
            }
        }
    }
}
