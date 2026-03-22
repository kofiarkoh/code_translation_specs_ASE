use std::io::{self, Read};

fn l(
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    cx: i32,
    cy: i32,
    dx: i32,
    dy: i32,
) -> Option<(f64, f64)> {
    // Emulate C's 32-bit signed int arithmetic (including wraparound on overflow)
    // as closely as possible using wrapping operations.
    let acx = cx.wrapping_sub(ax);
    let acy = cy.wrapping_sub(ay);

    let bunbo = (bx.wrapping_sub(ax))
        .wrapping_mul(dy.wrapping_sub(cy))
        .wrapping_sub((by.wrapping_sub(ay)).wrapping_mul(dx.wrapping_sub(cx)));

    if bunbo != 0 {
        let mut r = (dy.wrapping_sub(cy))
            .wrapping_mul(acx)
            .wrapping_sub((dx.wrapping_sub(cx)).wrapping_mul(acy));
        let mut s = (by.wrapping_sub(ay))
            .wrapping_mul(acx)
            .wrapping_sub((bx.wrapping_sub(ax)).wrapping_mul(acy));
        let mut bunbo_pos = bunbo;

        if bunbo_pos < 0 {
            r = r.wrapping_neg();
            s = s.wrapping_neg();
            bunbo_pos = bunbo_pos.wrapping_neg();
        }

        if r >= 0 && r <= bunbo_pos && s >= 0 && s <= bunbo_pos {
            let t = (r as f64) / (bunbo_pos as f64);
            let x = (ax as f64) + t * ((bx.wrapping_sub(ax)) as f64);
            let y = (ay as f64) + t * ((by.wrapping_sub(ay)) as f64);
            return Some((x, y));
        }
    }
    None
}

fn main() {
    // Read all input; C code uses scanf in a loop.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut x1 = [0i32; 3];
    let mut y1 = [0i32; 3];
    let mut x2 = [0i32; 3];
    let mut y2 = [0i32; 3];

    let mut i: usize = 0;

    loop {
        let a = match it.next() {
            Some(v) => v,
            None => break,
        };
        let b = match it.next() {
            Some(v) => v,
            None => break,
        };
        let c = match it.next() {
            Some(v) => v,
            None => break,
        };
        let d = match it.next() {
            Some(v) => v,
            None => break,
        };

        // If parsing fails, C's scanf would stop; we stop as well.
        let ax: i32 = match a.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let ay: i32 = match b.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let bx: i32 = match c.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let by: i32 = match d.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        x1[i] = ax;
        y1[i] = ay;
        x2[i] = bx;
        y2[i] = by;

        i += 1;
        if i == 3 {
            let mut x = [0f64; 3];
            let mut y = [0f64; 3];

            let ok0 = l(x1[0], y1[0], x2[0], y2[0], x1[1], y1[1], x2[1], y2[1]);
            let ok1 = l(x1[1], y1[1], x2[1], y2[1], x1[2], y1[2], x2[2], y2[2]);
            let ok2 = l(x1[2], y1[2], x2[2], y2[2], x1[0], y1[0], x2[0], y2[0]);

            if let (Some((x0, y0)), Some((x1p, y1p)), Some((x2p, y2p))) = (ok0, ok1, ok2) {
                x[0] = x0;
                y[0] = y0;
                x[1] = x1p;
                y[1] = y1p;
                x[2] = x2p;
                y[2] = y2p;

                let mut s_area = 0f64;
                for j in 0..3 {
                    let k = (j + 1) % 3;
                    s_area += (x[j] - x[k]) * (y[j] + y[k]) / 2.0;
                }
                s_area = s_area.abs();

                if s_area < 1e-6 {
                    println!("kyo");
                } else if s_area < 100000.0 - 1e-6 {
                    println!("syo-kichi");
                } else if s_area < 1000000.0 - 1e-6 {
                    println!("kichi");
                } else if s_area < 1900000.0 - 1e-6 {
                    println!("chu-kichi");
                } else {
                    println!("dai-kichi");
                }
            } else {
                println!("kyo");
            }

            i = 0;
        }
    }
}
