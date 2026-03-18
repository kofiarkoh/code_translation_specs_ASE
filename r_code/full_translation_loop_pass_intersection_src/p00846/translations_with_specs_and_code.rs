use std::f64;

const EPS: f64 = 1e-8;

#[derive(Clone, Copy, Debug, Default)]
struct PP {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug, Default)]
struct Line {
    s: PP,
    e: PP,
}

fn dcmp(x: f64) -> i32 {
    if x.abs() < EPS {
        0
    } else if x < 0.0 {
        -1
    } else {
        1
    }
}

fn cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}

fn line_intersection(a: &Line, vs: &PP, ve: &PP) -> PP {
    // Intersection of segment (a.s -> a.e) with the infinite line through (vs -> ve),
    // matching the C formula exactly.
    let a1 = cross(ve.x - vs.x, ve.y - vs.y, a.s.x - vs.x, a.s.y - vs.y);
    let a2 = cross(ve.x - vs.x, ve.y - vs.y, a.e.x - vs.x, a.e.y - vs.y);
    PP {
        x: (a.s.x * a2 - a.e.x * a1) / (a2 - a1),
        y: (a.s.y * a2 - a.e.y * a1) / (a2 - a1),
    }
}

/// Clips polygon `p` (with `p[0..=n]` where `p[n] == p[0]`) by the half-plane to the
/// left of directed line `u` (i.e., cross(u.e-u.s, point-u.s) >= 0).
/// Writes result into `po`, also closed (last equals first), and returns new vertex count.
fn convex_cut(u: &Line, n: usize, p: &[PP], po: &mut Vec<PP>) -> usize {
    po.clear();
    if n == 0 {
        return 0;
    }

    for i in 0..n {
        let d1 = dcmp(cross(
            u.e.x - u.s.x,
            u.e.y - u.s.y,
            p[i].x - u.s.x,
            p[i].y - u.s.y,
        ));
        let d2 = dcmp(cross(
            u.e.x - u.s.x,
            u.e.y - u.s.y,
            p[i + 1].x - u.s.x,
            p[i + 1].y - u.s.y,
        ));

        if d1 >= 0 {
            po.push(p[i]);
        }
        if d1 * d2 < 0 {
            let ip = line_intersection(u, &p[i], &p[i + 1]);
            po.push(ip);
        }
    }

    if po.is_empty() {
        return 0;
    }
    // Close polygon
    let first = po[0];
    po.push(first);
    po.len() - 1
}

fn main() {
    // SPEC does not mention multiple test cases, but the C code processes until n<=0.
    // We preserve the C I/O behavior as the source-of-truth for termination.
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Predefined convex shape: axis-aligned square [0,10000]x[0,10000], closed.
    let a: [PP; 5] = [
        PP { x: 0.0, y: 0.0 },
        PP { x: 10000.0, y: 0.0 },
        PP { x: 10000.0, y: 10000.0 },
        PP { x: 0.0, y: 10000.0 },
        PP { x: 0.0, y: 0.0 },
    ];

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n <= 0 {
            break;
        }
        let n = n as usize;

        // Read polygon vertices, store as f64, and close it.
        let mut p: Vec<PP> = Vec::with_capacity(n + 1);
        for _ in 0..n {
            let x: i32 = it.next().unwrap().parse().unwrap();
            let y: i32 = it.next().unwrap().parse().unwrap();
            p.push(PP {
                x: x as f64,
                y: y as f64,
            });
        }
        p.push(p[0]);

        // Start with the predefined convex polygon, then clip by each edge of input polygon.
        let mut pol0: Vec<PP> = a.to_vec(); // already closed
        let mut pol1: Vec<PP> = Vec::with_capacity(128);

        let mut k: usize = 4; // number of vertices excluding the closing point
        let mut k1 = 0usize;
        let mut k2 = 1usize;

        for i in 0..n {
            let u = Line { s: p[i], e: p[i + 1] };
            if k1 == 0 {
                k = convex_cut(&u, k, &pol0, &mut pol1);
            } else {
                k = convex_cut(&u, k, &pol1, &mut pol0);
            }
            if k == 0 {
                break;
            }
            // toggle buffers
            k1 = k2;
            k2 = 1 - k2;
        }

        if k > 0 {
            out.push_str("1\n");
        } else {
            out.push_str("0\n");
        }
    }

    print!("{out}");
}

// Needed for read_to_string
use std::io::Read;
