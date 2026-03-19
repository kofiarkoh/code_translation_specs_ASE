use std::io::{self, Read};

fn segment_intersection(
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    cx: i64,
    cy: i64,
    dx: i64,
    dy: i64,
) -> Option<(f64, f64)> {
    // Mirrors the C logic:
    // bunbo = (bx-ax)*(dy-cy) - (by-ay)*(dx-cx)
    // r = (dy-cy)*(cx-ax) - (dx-cx)*(cy-ay)
    // s = (by-ay)*(cx-ax) - (bx-ax)*(cy-ay)
    // If bunbo < 0, negate r,s,bunbo; then require 0<=r,s<=bunbo.
    let acx = cx - ax;
    let acy = cy - ay;

    let abx = bx - ax;
    let aby = by - ay;
    let cdx = dx - cx;
    let cdy = dy - cy;

    let mut bunbo = abx * cdy - aby * cdx;
    if bunbo == 0 {
        // Parallel or collinear treated as no valid intersection.
        return None;
    }

    let mut r = cdy * acx - cdx * acy;
    let mut s = aby * acx - abx * acy;

    if bunbo < 0 {
        bunbo = -bunbo;
        r = -r;
        s = -s;
    }

    if r >= 0 && r <= bunbo && s >= 0 && s <= bunbo {
        let t = (r as f64) / (bunbo as f64);
        let x = (ax as f64) + t * (abx as f64);
        let y = (ay as f64) + t * (aby as f64);
        Some((x, y))
    } else {
        None
    }
}

fn main() {
    // Read all input; stop on incomplete final batch (spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut x1 = [0i64; 3];
    let mut y1 = [0i64; 3];
    let mut x2 = [0i64; 3];
    let mut y2 = [0i64; 3];

    let mut i = 0usize;
    let mut out = String::new();

    loop {
        let a = it.next();
        if a.is_none() {
            break;
        }
        let b = it.next();
        let c = it.next();
        let d = it.next();
        if b.is_none() || c.is_none() || d.is_none() {
            // Incomplete quadruple: ignore trailing incomplete batch.
            break;
        }

        let ax: i64 = a.unwrap().parse().unwrap();
        let ay: i64 = b.unwrap().parse().unwrap();
        let bx: i64 = c.unwrap().parse().unwrap();
        let by: i64 = d.unwrap().parse().unwrap();

        x1[i] = ax;
        y1[i] = ay;
        x2[i] = bx;
        y2[i] = by;

        i += 1;
        if i == 3 {
            let p0 = segment_intersection(x1[0], y1[0], x2[0], y2[0], x1[1], y1[1], x2[1], y2[1]);
            let p1 = segment_intersection(x1[1], y1[1], x2[1], y2[1], x1[2], y1[2], x2[2], y2[2]);
            let p2 = segment_intersection(x1[2], y1[2], x2[2], y2[2], x1[0], y1[0], x2[0], y2[0]);

            if let (Some((x0, y0)), Some((x1p, y1p)), Some((x2p, y2p))) = (p0, p1, p2) {
                let xs = [x0, x1p, x2p];
                let ys = [y0, y1p, y2p];

                // Same area computation as C:
                // S += (X[i]-X[i+1])*(Y[i]+Y[i+1])/2; then fabs.
                let mut s_area = 0.0f64;
                for j in 0..3 {
                    let k = (j + 1) % 3;
                    s_area += (xs[j] - xs[k]) * (ys[j] + ys[k]) / 2.0;
                }
                let s_area = s_area.abs();

                if s_area < 1e-6 {
                    out.push_str("kyo\n");
                } else if s_area < 100000.0 - 1e-6 {
                    out.push_str("syo-kichi\n");
                } else if s_area < 1000000.0 - 1e-6 {
                    out.push_str("kichi\n");
                } else if s_area < 1900000.0 - 1e-6 {
                    out.push_str("chu-kichi\n");
                } else {
                    out.push_str("dai-kichi\n");
                }
            } else {
                out.push_str("kyo\n");
            }

            i = 0;
        }
    }

    print!("{out}");
}
