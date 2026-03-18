use std::io::{self, Read};

fn segment_intersection(
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    cx: i32,
    cy: i32,
    dx: i32,
    dy: i32,
) -> Option<(f64, f64)> {
    // Mirrors C function L(): computes intersection point of two line segments if they intersect.
    let acx: i64 = (cx - ax) as i64;
    let acy: i64 = (cy - ay) as i64;

    let bunbo: i64 = (bx - ax) as i64 * (dy - cy) as i64 - (by - ay) as i64 * (dx - cx) as i64;
    if bunbo == 0 {
        return None;
    }

    let mut r: i64 = (dy - cy) as i64 * acx - (dx - cx) as i64 * acy;
    let mut s: i64 = (by - ay) as i64 * acx - (bx - ax) as i64 * acy;
    let mut denom: i64 = bunbo;

    if denom < 0 {
        r = -r;
        s = -s;
        denom = -denom;
    }

    if r >= 0 && r <= denom && s >= 0 && s <= denom {
        let t = r as f64 / denom as f64;
        let x = ax as f64 + t * (bx - ax) as f64;
        let y = ay as f64 + t * (by - ay) as f64;
        Some((x, y))
    } else {
        None
    }
}

fn main() {
    // Read all input as integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut out = String::new();

    // Process in groups of 3 segments, each segment is 4 integers.
    let mut idx = 0usize;
    while idx + 12 <= nums.len() {
        let mut x1 = [0i32; 3];
        let mut y1 = [0i32; 3];
        let mut x2 = [0i32; 3];
        let mut y2 = [0i32; 3];

        for i in 0..3 {
            x1[i] = nums[idx];
            y1[i] = nums[idx + 1];
            x2[i] = nums[idx + 2];
            y2[i] = nums[idx + 3];
            idx += 4;
        }

        let p0 = segment_intersection(x1[0], y1[0], x2[0], y2[0], x1[1], y1[1], x2[1], y2[1]);
        let p1 = segment_intersection(x1[1], y1[1], x2[1], y2[1], x1[2], y1[2], x2[2], y2[2]);
        let p2 = segment_intersection(x1[2], y1[2], x2[2], y2[2], x1[0], y1[0], x2[0], y2[0]);

        if let (Some((x0, y0)), Some((x1p, y1p)), Some((x2p, y2p))) = (p0, p1, p2) {
            let x = [x0, x1p, x2p];
            let y = [y0, y1p, y2p];

            // Same area computation as C:
            // S += (X[i]-X[i+1])*(Y[i]+Y[i+1])/2; then fabs.
            let mut s_area = 0.0f64;
            for i in 0..3 {
                let j = (i + 1) % 3;
                s_area += (x[i] - x[j]) * (y[i] + y[j]) / 2.0;
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
    }

    // If fewer than 3 segments remain (not enough data), produce no output for that partial group.
    print!("{out}");
}
