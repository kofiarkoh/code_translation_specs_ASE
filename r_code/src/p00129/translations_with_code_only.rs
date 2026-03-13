// translation.rs
//
// Translated from the provided C code. This Rust version preserves the original
// C semantics as closely as possible:
// - Integers are treated as 32-bit signed (i32).
// - All integer arithmetic that could overflow in C is performed with wrapping
//   semantics (two's-complement wrap) to emulate typical C implementation behavior.
// - Floating-point computations use f64 and mirror the casts/ordering in the C code.
//
// Note: We avoid unsafe; wrapping arithmetic is done via the safe wrapping_* APIs.

use std::io::{self, Read};

const EPS: f64 = 1e-7;

#[derive(Clone, Copy)]
struct Wall {
    x: i32,
    y: i32,
    r: i32,
}

#[derive(Clone, Copy)]
struct Query {
    tx: i32,
    ty: i32,
    sx: i32,
    sy: i32,
}

fn hitcheck_circle(w: Wall, q: Query) -> bool {
    // Emulate C 'int' wrapping behavior using i32::wrapping_*
    // Sieve: compute min/max with plain comparisons (i32)
    let mut minx = q.tx;
    let mut maxx = q.sx;
    if minx > maxx {
        let temp = minx;
        minx = maxx;
        maxx = temp;
    }
    let mut miny = q.ty;
    let mut maxy = q.sy;
    if miny > maxy {
        let temp = miny;
        miny = maxy;
        maxy = temp;
    }

    // if(w.x+w.r<minx || maxx<w.x-w.r)return 0;
    let wx_pr = w.x.wrapping_add(w.r);
    let wx_mr = w.x.wrapping_sub(w.r);
    if wx_pr < minx || maxx < wx_mr {
        return false;
    }
    // if(w.y+w.r<miny || maxy<w.y-w.r)return 0;
    let wy_pr = w.y.wrapping_add(w.r);
    let wy_mr = w.y.wrapping_sub(w.r);
    if wy_pr < miny || maxy < wy_mr {
        return false;
    }

    // check if dist<=r for endpoints
    let mut in_count = 0;
    let dx1 = q.tx.wrapping_sub(w.x);
    let dy1 = q.ty.wrapping_sub(w.y);
    let dist1 = dx1.wrapping_mul(dx1).wrapping_add(dy1.wrapping_mul(dy1));
    let rr = w.r.wrapping_mul(w.r);
    if dist1 <= rr {
        in_count += 1;
    }
    let dx2 = q.sx.wrapping_sub(w.x);
    let dy2 = q.sy.wrapping_sub(w.y);
    let dist2 = dx2.wrapping_mul(dx2).wrapping_add(dy2.wrapping_mul(dy2));
    if dist2 <= rr {
        in_count += 1;
    }
    if in_count == 1 {
        return true;
    }
    if in_count == 2 {
        return false;
    }

    // compute line parameters (integers with wrapping)
    let a = q.ty.wrapping_sub(q.sy);
    let b = q.sx.wrapping_sub(q.tx);
    let mut c = q.tx.wrapping_sub(q.sx).wrapping_mul(q.ty)
        .wrapping_sub(q.ty.wrapping_sub(q.sy).wrapping_mul(q.tx));
    // integer inequality: (a*w.x+b*w.y+c)^2 <= w.r*w.r*(a*a+b*b)
    let left = a.wrapping_mul(w.x)
        .wrapping_add(b.wrapping_mul(w.y))
        .wrapping_add(c);
    let left_sq = left.wrapping_mul(left);
    let right = w.r.wrapping_mul(w.r)
        .wrapping_mul(a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b)));
    if left_sq <= right {
        // proceed to check actual intersection using doubles
        c = c.wrapping_neg(); // convert to a*x+b*y=c (with wrapping neg)
        if b == 0 {
            // vertical line case: b==0 => a != 0 (since segment not degenerate)
            // compute yy = w.r*w.r - ((double)c/a - w.x)^2
            let rr_d = (w.r.wrapping_mul(w.r)) as f64;
            let ca = (c as f64) / (a as f64);
            let diff = ca - (w.x as f64);
            let yy = rr_d - diff * diff;
            let y1 = yy.sqrt(); // may be NaN if yy < 0 -> mirrors C behavior
            let y1 = y1 + (w.y as f64);
            let y2 = -yy.sqrt() + (w.y as f64);
            let miny_d = miny as f64;
            let maxy_d = maxy as f64;
            return (miny_d < y1 + EPS && y1 < maxy_d + EPS)
                || (miny_d < y2 + EPS && y2 < maxy_d + EPS);
        } else {
            // general case b != 0
            let cb = (c as f64) / (b as f64);
            let ab = (a as f64) / (b as f64);
            let x2k = 1.0 + ab * ab;
            let xk = -2.0 * (w.x as f64) + 2.0 * ((w.y as f64) - cb) * ab;
            let tk = (w.x.wrapping_mul(w.x) as f64)
                + ((w.y as f64) - cb) * ((w.y as f64) - cb)
                - (w.r.wrapping_mul(w.r) as f64);
            let disc = xk * xk - 4.0 * x2k * tk;
            let sqrt_disc = disc.sqrt(); // NaN if disc < 0, matching C
            let x1 = (-xk + sqrt_disc) / (2.0 * x2k);
            let x2 = (-xk - sqrt_disc) / (2.0 * x2k);
            let minx_d = minx as f64;
            let maxx_d = maxx as f64;
            return (minx_d < x1 + EPS && x1 < maxx_d + EPS)
                || (minx_d < x2 + EPS && x2 < maxx_d + EPS);
        }
    }

    false
}

fn main() {
    // Read all stdin, split whitespace, parse as i32 tokens
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read stdin");
    let toks: Vec<&str> = s.split_whitespace().collect();
    let mut idx = 0usize;
    let mut out_lines: Vec<String> = Vec::new();

    while idx < toks.len() {
        let n: i32 = match toks[idx].parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        idx += 1;
        if n <= 0 {
            break;
        }
        // read n walls
        let mut walls: Vec<Wall> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            if idx + 2 >= toks.len() {
                // insufficient tokens
                break;
            }
            let x: i32 = toks[idx].parse().unwrap_or(0);
            let y: i32 = toks[idx + 1].parse().unwrap_or(0);
            let r: i32 = toks[idx + 2].parse().unwrap_or(0);
            idx += 3;
            walls.push(Wall { x, y, r });
        }
        if idx >= toks.len() {
            break;
        }
        let m: i32 = toks[idx].parse().unwrap_or(0);
        idx += 1;
        for _ in 0..m {
            if idx + 3 >= toks.len() {
                break;
            }
            let q = Query {
                tx: toks[idx].parse().unwrap_or(0),
                ty: toks[idx + 1].parse().unwrap_or(0),
                sx: toks[idx + 2].parse().unwrap_or(0),
                sy: toks[idx + 3].parse().unwrap_or(0),
            };
            idx += 4;
            let mut hidden = false;
            if q.tx != q.sx || q.ty != q.sy {
                for &w in &walls {
                    if hitcheck_circle(w, q) {
                        hidden = true;
                        break;
                    }
                }
            }
            if hidden {
                out_lines.push(String::from("Safe"));
            } else {
                out_lines.push(String::from("Danger"));
            }
        }
    }

    // Print outputs preserving newline formatting
    for line in out_lines {
        println!("{}", line);
    }
}
