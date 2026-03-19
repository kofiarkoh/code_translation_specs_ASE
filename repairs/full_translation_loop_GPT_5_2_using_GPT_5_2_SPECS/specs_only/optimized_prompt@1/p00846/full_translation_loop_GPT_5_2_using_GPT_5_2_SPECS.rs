use std::io::{self, Read};

const EPS: f64 = 1e-9;

#[derive(Clone, Copy, Debug)]
struct Pt {
    x: f64,
    y: f64,
}

impl Pt {
    fn sub(self, other: Pt) -> Pt {
        Pt {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn cross(a: Pt, b: Pt) -> f64 {
    a.x * b.y - a.y * b.x
}

fn sgn(v: f64) -> i32 {
    if v > EPS {
        1
    } else if v < -EPS {
        -1
    } else {
        0
    }
}

fn on_segment(a: Pt, b: Pt, p: Pt) -> bool {
    // Collinear and within bounding box (with tolerance).
    sgn(cross(b.sub(a), p.sub(a))) == 0
        && p.x >= a.x.min(b.x) - EPS
        && p.x <= a.x.max(b.x) + EPS
        && p.y >= a.y.min(b.y) - EPS
        && p.y <= a.y.max(b.y) + EPS
}

fn segments_intersect(a: Pt, b: Pt, c: Pt, d: Pt) -> bool {
    // Standard robust-ish segment intersection with EPS-based orientation.
    let ab = b.sub(a);
    let ac = c.sub(a);
    let ad = d.sub(a);
    let cd = d.sub(c);
    let ca = a.sub(c);
    let cb = b.sub(c);

    let o1 = sgn(cross(ab, ac));
    let o2 = sgn(cross(ab, ad));
    let o3 = sgn(cross(cd, ca));
    let o4 = sgn(cross(cd, cb));

    if o1 == 0 && on_segment(a, b, c) {
        return true;
    }
    if o2 == 0 && on_segment(a, b, d) {
        return true;
    }
    if o3 == 0 && on_segment(c, d, a) {
        return true;
    }
    if o4 == 0 && on_segment(c, d, b) {
        return true;
    }

    (o1 * o2 < 0) && (o3 * o4 < 0)
}

fn point_in_convex_polygon_inclusive(poly: &[Pt], p: Pt) -> bool {
    // Inclusive point-in-convex-polygon test using consistent orientation.
    // Returns true if p is inside or on boundary.
    let n = poly.len();
    if n == 0 {
        return false;
    }
    if n == 1 {
        return (poly[0].x - p.x).abs() <= EPS && (poly[0].y - p.y).abs() <= EPS;
    }
    if n == 2 {
        return on_segment(poly[0], poly[1], p);
    }

    let mut pos = false;
    let mut neg = false;
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        let v = cross(b.sub(a), p.sub(a));
        let s = sgn(v);
        if s > 0 {
            pos = true;
        } else if s < 0 {
            neg = true;
        }
        if pos && neg {
            return false;
        }
    }
    true
}

fn polygon_intersects_square(poly: &[Pt]) -> bool {
    // Fixed closed square: (0,0), (10000,0), (10000,10000), (0,10000)
    let sq = [
        Pt { x: 0.0, y: 0.0 },
        Pt { x: 10000.0, y: 0.0 },
        Pt { x: 10000.0, y: 10000.0 },
        Pt { x: 0.0, y: 10000.0 },
    ];

    // 1) Any polygon vertex inside/on square?
    for &p in poly {
        if p.x >= -EPS && p.x <= 10000.0 + EPS && p.y >= -EPS && p.y <= 10000.0 + EPS {
            return true;
        }
    }

    // 2) Any square corner inside/on polygon?
    for &c in &sq {
        if point_in_convex_polygon_inclusive(poly, c) {
            return true;
        }
    }

    // 3) Any edge intersection (including touching)?
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        for j in 0..4 {
            let c = sq[j];
            let d = sq[(j + 1) % 4];
            if segments_intersect(a, b, c, d) {
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n <= 0 {
            break;
        }
        let n_usize = n as usize;

        let mut poly: Vec<Pt> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            let x: f64 = it.next().unwrap().parse::<i64>().unwrap() as f64;
            let y: f64 = it.next().unwrap().parse::<i64>().unwrap() as f64;
            poly.push(Pt { x, y });
        }

        let ans = if polygon_intersects_square(&poly) { '1' } else { '0' };
        out.push(ans);
        out.push('\n');
    }

    print!("{out}");
}
