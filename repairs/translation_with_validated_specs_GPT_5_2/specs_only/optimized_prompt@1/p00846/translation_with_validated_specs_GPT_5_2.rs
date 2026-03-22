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

// Signed area *2 of triangle (a,b,p): positive if p is to the left of directed edge a->b.
fn side(a: Pt, b: Pt, p: Pt) -> f64 {
    cross(b.sub(a), p.sub(a))
}

// Intersect segment p->q with the infinite line a->b, assuming they are not parallel in the
// crossing cases per the specification precondition.
fn intersect_segment_with_line(p: Pt, q: Pt, a: Pt, b: Pt) -> Pt {
    let r = q.sub(p);
    let s = b.sub(a);
    let denom = cross(r, s);
    // If denom is 0, lines are parallel; spec precondition rules out needing this intersection
    // in ambiguous cases. Still, avoid NaN by returning p (narrowest behavior).
    if denom.abs() < EPS {
        return p;
    }
    // Solve p + t*r intersects line through a in direction s:
    // cross((a - p), s) / cross(r, s)
    let t = cross(a.sub(p), s) / denom;
    Pt {
        x: p.x + t * r.x,
        y: p.y + t * r.y,
    }
}

// Clip convex/possibly-nonconvex polygon `poly` by half-plane "left of directed edge a->b".
// Uses tolerance: points with side >= -EPS are kept.
fn clip_by_halfplane(poly: &[Pt], a: Pt, b: Pt) -> Vec<Pt> {
    if poly.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<Pt> = Vec::new();
    let n = poly.len();
    for i in 0..n {
        let cur = poly[i];
        let prev = poly[(i + n - 1) % n];
        let cur_in = side(a, b, cur) >= -EPS;
        let prev_in = side(a, b, prev) >= -EPS;

        if cur_in {
            if !prev_in {
                // entering: add intersection then current
                let ip = intersect_segment_with_line(prev, cur, a, b);
                out.push(ip);
            }
            out.push(cur);
        } else if prev_in {
            // leaving: add intersection only
            let ip = intersect_segment_with_line(prev, cur, a, b);
            out.push(ip);
        }
    }

    // Remove near-duplicate consecutive points to keep polygon well-formed.
    // This is a minimal robustness step consistent with tolerance-based behavior.
    let mut cleaned: Vec<Pt> = Vec::new();
    for p in out {
        if cleaned.is_empty() {
            cleaned.push(p);
        } else {
            let last = *cleaned.last().unwrap();
            if (p.x - last.x).abs() > 1e-12 || (p.y - last.y).abs() > 1e-12 {
                cleaned.push(p);
            }
        }
    }
    if cleaned.len() >= 2 {
        let first = cleaned[0];
        let last = *cleaned.last().unwrap();
        if (first.x - last.x).abs() <= 1e-12 && (first.y - last.y).abs() <= 1e-12 {
            cleaned.pop();
        }
    }
    cleaned
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let Some(tok) = it.next() else { break };
        let m: i64 = tok.parse().unwrap();
        if m <= 0 {
            break;
        }
        let m_usize = m as usize;

        let mut verts: Vec<Pt> = Vec::with_capacity(m_usize);
        for _ in 0..m_usize {
            let x: i64 = it.next().unwrap().parse().unwrap();
            let y: i64 = it.next().unwrap().parse().unwrap();
            verts.push(Pt {
                x: x as f64,
                y: y as f64,
            });
        }

        // Start with fixed square [0,10000] x [0,10000], as a convex polygon in CCW order.
        let mut region = vec![
            Pt { x: 0.0, y: 0.0 },
            Pt { x: 10000.0, y: 0.0 },
            Pt { x: 10000.0, y: 10000.0 },
            Pt { x: 0.0, y: 10000.0 },
        ];

        // Apply half-plane cuts for each directed edge of the input polygon.
        for i in 0..m_usize {
            if region.is_empty() {
                break;
            }
            let a = verts[i];
            let b = verts[(i + 1) % m_usize];
            region = clip_by_halfplane(&region, a, b);
        }

        outputs.push(if region.is_empty() { "0" } else { "1" }.to_string());
    }

    print!("{}", outputs.join("\n"));
}
