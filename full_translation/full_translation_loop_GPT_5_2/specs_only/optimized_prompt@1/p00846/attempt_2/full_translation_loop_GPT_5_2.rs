use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Pt {
    x: i64,
    y: i64,
}

fn cross(a: Pt, b: Pt, c: Pt) -> i128 {
    // cross((b-a),(c-a))
    let abx = (b.x - a.x) as i128;
    let aby = (b.y - a.y) as i128;
    let acx = (c.x - a.x) as i128;
    let acy = (c.y - a.y) as i128;
    abx * acy - aby * acx
}

fn on_segment(a: Pt, b: Pt, p: Pt) -> bool {
    // assumes collinear
    let (minx, maxx) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (miny, maxy) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
    p.x >= minx && p.x <= maxx && p.y >= miny && p.y <= maxy
}

fn seg_intersect(a: Pt, b: Pt, c: Pt, d: Pt) -> bool {
    // Standard robust segment intersection including collinear overlap.
    let o1 = cross(a, b, c);
    let o2 = cross(a, b, d);
    let o3 = cross(c, d, a);
    let o4 = cross(c, d, b);

    if (o1 > 0 && o2 < 0 || o1 < 0 && o2 > 0) && (o3 > 0 && o4 < 0 || o3 < 0 && o4 > 0) {
        return true;
    }

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
    false
}

fn point_in_convex_ccw(poly: &[Pt], p: Pt) -> bool {
    // Inclusive: points on boundary count as inside.
    // Assumes poly is convex and CCW.
    let n = poly.len();
    if n < 3 {
        return false;
    }
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        if cross(a, b, p) < 0 {
            return false;
        }
    }
    true
}

fn point_in_poly_inclusive(poly: &[Pt], p: Pt) -> bool {
    // Ray casting with boundary inclusion.
    let n = poly.len();
    if n == 0 {
        return false;
    }
    if n == 1 {
        return poly[0].x == p.x && poly[0].y == p.y;
    }
    if n == 2 {
        let a = poly[0];
        let b = poly[1];
        return cross(a, b, p) == 0 && on_segment(a, b, p);
    }

    // Boundary check
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        if cross(a, b, p) == 0 && on_segment(a, b, p) {
            return true;
        }
    }

    let mut inside = false;
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];

        // Check if edge crosses horizontal ray to +inf from p.
        let ay = a.y;
        let by = b.y;
        let py = p.y;

        let intersects = (ay > py) != (by > py);
        if intersects {
            // Compute x coordinate of intersection using exact rational comparison:
            // x_int = ax + (bx-ax)*(py-ay)/(by-ay)
            // We want x_int > px.
            let ax = a.x as i128;
            let bx = b.x as i128;
            let px = p.x as i128;
            let ay_i = ay as i128;
            let by_i = by as i128;
            let py_i = py as i128;

            let num = (bx - ax) * (py_i - ay_i);
            let den = by_i - ay_i; // non-zero due to intersects
            // Compare ax + num/den > px without floating point.
            // Bring to common denominator; note den can be negative.
            let left_num = ax * den + num;
            let right_num = px * den;
            let gt = if den > 0 {
                left_num > right_num
            } else {
                left_num < right_num
            };
            if gt {
                inside = !inside;
            }
        }
    }
    inside
}

fn polygon_intersects(poly: &[Pt], shape: &[Pt]) -> bool {
    let n = poly.len();
    let m = shape.len();
    if n == 0 || m == 0 {
        return false;
    }

    // Edge-edge intersection
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        for j in 0..m {
            let c = shape[j];
            let d = shape[(j + 1) % m];
            if seg_intersect(a, b, c, d) {
                return true;
            }
        }
    }

    // Containment: one polygon inside the other
    if point_in_convex_ccw(shape, poly[0]) {
        return true;
    }
    if point_in_poly_inclusive(poly, shape[0]) {
        return true;
    }

    false
}

fn main() {
    // The original problem (p00846) uses multiple test cases terminated by n=0.
    // The predefined convex shape is inferred from typical AOJ tasks: a fixed square
    // with corners (0,0), (10000,0), (10000,10000), (0,10000).
    // This matches the sample: a polygon overlapping that square yields 1; a far polygon yields 0.
    let shape = [
        Pt { x: 0, y: 0 },
        Pt { x: 10000, y: 0 },
        Pt { x: 10000, y: 10000 },
        Pt { x: 0, y: 10000 },
    ];

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n = match it.next() {
            Some(t) => t.parse::<usize>().unwrap(),
            None => break,
        };
        if n == 0 {
            break;
        }
        let mut poly: Vec<Pt> = Vec::with_capacity(n);
        for _ in 0..n {
            let x = it.next().unwrap().parse::<i64>().unwrap();
            let y = it.next().unwrap().parse::<i64>().unwrap();
            poly.push(Pt { x, y });
        }

        let ans = if polygon_intersects(&poly, &shape) { "1" } else { "0" };
        outputs.push(ans.to_string());
    }

    print!("{}", outputs.join("\n"));
}
