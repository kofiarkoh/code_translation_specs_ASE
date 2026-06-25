use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Read};

const EPS: f64 = 1e-7;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn from_i64(x: i64, y: i64) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    a: Point,
    b: Point,
    route: usize,
}

fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPS
}

fn point_eq(a: Point, b: Point) -> bool {
    feq(a.x, b.x) && feq(a.y, b.y)
}

fn cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}

fn seg_intersection_point(s1: &Segment, s2: &Segment) -> Option<Point> {
    // Computes a single intersection point for non-parallel segments.
    // Parallel/collinear cases are treated as having no computable intersection point here,
    // beyond shared endpoints which are handled separately by adding endpoints to the point set.
    let p = s1.a;
    let r = Point {
        x: s1.b.x - s1.a.x,
        y: s1.b.y - s1.a.y,
    };
    let q = s2.a;
    let s = Point {
        x: s2.b.x - s2.a.x,
        y: s2.b.y - s2.a.y,
    };

    let rxs = cross(r.x, r.y, s.x, s.y);
    if rxs.abs() <= EPS {
        return None;
    }

    let qmp = Point {
        x: q.x - p.x,
        y: q.y - p.y,
    };
    let t = cross(qmp.x, qmp.y, s.x, s.y) / rxs;
    let u = cross(qmp.x, qmp.y, r.x, r.y) / rxs;

    if t < -EPS || t > 1.0 + EPS || u < -EPS || u > 1.0 + EPS {
        return None;
    }

    Some(Point {
        x: p.x + t * r.x,
        y: p.y + t * r.y,
    })
}

fn quantize_point(p: Point) -> (i64, i64) {
    // Fixed tolerance equality: points within EPS are treated as identical.
    // We emulate this by snapping to an EPS-sized grid.
    let qx = (p.x / EPS).round() as i64;
    let qy = (p.y / EPS).round() as i64;
    (qx, qy)
}

fn greedy_welsh_powell_variant(adj: &Vec<Vec<bool>>, order: &[usize]) -> usize {
    // Heuristic:
    // - Process vertices in the given order.
    // - For each uncolored vertex, start a new color and then scan forward in the same order,
    //   coloring any still-uncolored vertex that does not conflict with any vertex already
    //   assigned this color.
    // - A route is considered conflicting with itself for internal bookkeeping.
    let n = adj.len();
    let mut color = vec![0usize; n];
    let mut used_colors = 0usize;

    for &v in order {
        if color[v] != 0 {
            continue;
        }
        used_colors += 1;
        color[v] = used_colors;

        for &u in order {
            if color[u] != 0 {
                continue;
            }
            let mut ok = true;
            for i in 0..n {
                if color[i] == used_colors && adj[u][i] {
                    ok = false;
                    break;
                }
            }
            if ok {
                color[u] = used_colors;
            }
        }
    }

    used_colors
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
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut segments: Vec<Segment> = Vec::new();

        for route in 0..n {
            let m: usize = it.next().unwrap().parse().unwrap();
            if m == 0 {
                continue;
            }
            let mut pts: Vec<Point> = Vec::with_capacity(m);
            for _ in 0..m {
                let x: i64 = it.next().unwrap().parse().unwrap();
                let y: i64 = it.next().unwrap().parse().unwrap();
                pts.push(Point::from_i64(x, y));
            }
            for i in 0..(m - 1) {
                segments.push(Segment {
                    a: pts[i],
                    b: pts[i + 1],
                    route,
                });
            }
        }

        // Build set of all endpoints and all computable pairwise segment intersection points.
        // For each such point, record which routes pass through it (via endpoints or intersections).
        let mut point_routes: HashMap<(i64, i64), Vec<usize>> = HashMap::new();

        // Endpoints
        for s in &segments {
            for &p in &[s.a, s.b] {
                let key = quantize_point(p);
                let entry = point_routes.entry(key).or_insert_with(Vec::new);
                entry.push(s.route);
            }
        }

        // Pairwise intersections
        for i in 0..segments.len() {
            for j in (i + 1)..segments.len() {
                let si = &segments[i];
                let sj = &segments[j];
                if let Some(p) = seg_intersection_point(si, sj) {
                    let key = quantize_point(p);
                    let entry = point_routes.entry(key).or_insert_with(Vec::new);
                    entry.push(si.route);
                    entry.push(sj.route);
                }
            }
        }

        // Construct undirected conflict graph among routes based on co-occurrence at any point.
        let mut adj = vec![vec![false; n]; n];
        for i in 0..n {
            adj[i][i] = true; // self-conflict invariant for heuristic bookkeeping
        }

        for (_k, routes) in point_routes.iter() {
            // Deduplicate routes at this point (tolerance already applied by quantization).
            let mut uniq: Vec<usize> = routes.clone();
            uniq.sort_unstable();
            uniq.dedup();

            for a in 0..uniq.len() {
                for b in a..uniq.len() {
                    let ra = uniq[a];
                    let rb = uniq[b];
                    adj[ra][rb] = true;
                    adj[rb][ra] = true;
                }
            }
        }

        // Determine processing order: Welsh–Powell typically uses descending degree.
        // The spec only states "Welsh–Powell variant" and "two traversal orders (forward, reverse)".
        // Narrow interpretation: sort by non-increasing degree, then run heuristic in that order
        // and in the reverse of that order, taking the minimum color count.
        let mut verts: Vec<usize> = (0..n).collect();
        let degrees: Vec<usize> = (0..n)
            .map(|i| adj[i].iter().filter(|&&b| b).count())
            .collect();

        verts.sort_by(|&a, &b| {
            let da = degrees[a];
            let db = degrees[b];
            match db.cmp(&da) {
                Ordering::Equal => a.cmp(&b),
                other => other,
            }
        });

        let mut order_fwd = verts.clone();
        let mut order_rev = verts.clone();
        order_rev.reverse();

        let c1 = greedy_welsh_powell_variant(&adj, &order_fwd);
        let c2 = greedy_welsh_powell_variant(&adj, &order_rev);
        let ans = c1.min(c2);

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
