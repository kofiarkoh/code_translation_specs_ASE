use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

const EPS: f64 = 1e-9;

// Fixed internal limits are mentioned but not specified; we do not enforce hard caps here.
// The specification allows undefined behavior if limits are exceeded.

#[derive(Clone, Copy, Debug)]
struct PtI {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
struct PtF {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug)]
struct Seg {
    a: PtI,
    b: PtI,
    line_id: usize,
}

fn to_f(p: PtI) -> PtF {
    PtF {
        x: p.x as f64,
        y: p.y as f64,
    }
}

fn cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}

fn sub(a: PtF, b: PtF) -> PtF {
    PtF {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

fn dot(a: PtF, b: PtF) -> f64 {
    a.x * b.x + a.y * b.y
}

fn on_segment(p: PtF, a: PtF, b: PtF) -> bool {
    // Collinearity is assumed checked by caller if needed; we just do bounding-box with EPS.
    let minx = a.x.min(b.x) - EPS;
    let maxx = a.x.max(b.x) + EPS;
    let miny = a.y.min(b.y) - EPS;
    let maxy = a.y.max(b.y) + EPS;
    p.x >= minx && p.x <= maxx && p.y >= miny && p.y <= maxy
}

fn seg_intersection_unique(s1: &Seg, s2: &Seg) -> Option<PtF> {
    // Returns a unique intersection point if segments intersect at a single point.
    // Parallel/collinear overlaps do not yield a unique point and return None,
    // per specification (their effect only via shared endpoints or other detected points).
    let p = to_f(s1.a);
    let r = sub(to_f(s1.b), to_f(s1.a));
    let q = to_f(s2.a);
    let s = sub(to_f(s2.b), to_f(s2.a));

    let rxs = cross(r.x, r.y, s.x, s.y);
    let q_p = sub(q, p);
    let qpxr = cross(q_p.x, q_p.y, r.x, r.y);

    if rxs.abs() <= EPS {
        // Parallel (including collinear): no unique intersection point added.
        return None;
    }

    let t = cross(q_p.x, q_p.y, s.x, s.y) / rxs;
    let u = qpxr / rxs;

    if t >= -EPS && t <= 1.0 + EPS && u >= -EPS && u <= 1.0 + EPS {
        Some(PtF {
            x: p.x + t * r.x,
            y: p.y + t * r.y,
        })
    } else {
        None
    }
}

fn quantize_point(p: PtF) -> (i64, i64) {
    // The original behavior is not fully inferable; we emulate a tolerance-based point identity
    // by snapping to an EPS grid. This preserves the "floating-point comparisons with tolerance"
    // constraint and avoids inventing exact rational arithmetic.
    let qx = (p.x / EPS).round() as i64;
    let qy = (p.y / EPS).round() as i64;
    (qx, qy)
}

fn greedy_coloring(adj: &Vec<Vec<bool>>, order: &[usize]) -> usize {
    let n = adj.len();
    let mut color = vec![0usize; n]; // 0 = uncolored, colors are 1..k
    let mut used = vec![false; n + 1];

    let mut maxc = 0usize;
    for &v in order {
        used.fill(false);
        for u in 0..n {
            if adj[v][u] && color[u] != 0 {
                used[color[u]] = true;
            }
        }
        let mut c = 1usize;
        while c <= n && used[c] {
            c += 1;
        }
        if c > n {
            // Should not happen; but keep semantics narrow: assign n.
            c = n;
        }
        color[v] = c;
        maxc = maxc.max(c);
    }
    maxc
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n_lines: usize = n_opt.unwrap().parse().unwrap();
        if n_lines == 0 {
            break;
        }

        let mut segments: Vec<Seg> = Vec::new();

        for line_id in 0..n_lines {
            let m: usize = it.next().unwrap().parse().unwrap();
            if m == 0 {
                continue;
            }
            let mut pts: Vec<PtI> = Vec::with_capacity(m);
            for _ in 0..m {
                let x: i64 = it.next().unwrap().parse().unwrap();
                let y: i64 = it.next().unwrap().parse().unwrap();
                pts.push(PtI { x, y });
            }
            if pts.len() >= 2 {
                for i in 0..(pts.len() - 1) {
                    segments.push(Seg {
                        a: pts[i],
                        b: pts[i + 1],
                        line_id,
                    });
                }
            }
        }

        // Build conflict relation between line identifiers.
        // Invariant: symmetric and includes self-conflict.
        let mut adj = vec![vec![false; n_lines]; n_lines];
        for i in 0..n_lines {
            adj[i][i] = true;
        }

        // Map from quantized point to set of line_ids present at that point.
        let mut point_lines: BTreeMap<(i64, i64), BTreeSet<usize>> = BTreeMap::new();

        // Add all endpoints as points.
        for s in &segments {
            let a = quantize_point(to_f(s.a));
            let b = quantize_point(to_f(s.b));
            point_lines.entry(a).or_default().insert(s.line_id);
            point_lines.entry(b).or_default().insert(s.line_id);
        }

        // Add unique intersection points between non-parallel segment pairs.
        for i in 0..segments.len() {
            for j in (i + 1)..segments.len() {
                let si = &segments[i];
                let sj = &segments[j];
                if let Some(p) = seg_intersection_unique(si, sj) {
                    // Ensure the computed point lies on both segments (with EPS), guarding numeric drift.
                    let pf = p;
                    let a1 = to_f(si.a);
                    let b1 = to_f(si.b);
                    let a2 = to_f(sj.a);
                    let b2 = to_f(sj.b);
                    if on_segment(pf, a1, b1) && on_segment(pf, a2, b2) {
                        let key = quantize_point(pf);
                        let entry = point_lines.entry(key).or_default();
                        entry.insert(si.line_id);
                        entry.insert(sj.line_id);
                    }
                }
            }
        }

        // For each point, all lines present there conflict pairwise.
        for (_pt, lines) in point_lines.iter() {
            let ids: Vec<usize> = lines.iter().copied().collect();
            for a in 0..ids.len() {
                for b in a..ids.len() {
                    let u = ids[a];
                    let v = ids[b];
                    adj[u][v] = true;
                    adj[v][u] = true;
                }
            }
        }

        // Two runs of the same greedy coloring strategy with two different vertex orders.
        // The specification mentions a "special-case adjustment for one specific line-count value"
        // but does not state which value or what adjustment; we cannot invent it.
        // Therefore we implement the "smaller of two runs" without any special-case.
        let mut order1: Vec<usize> = (0..n_lines).collect();
        let mut order2: Vec<usize> = (0..n_lines).collect();

        // Order1: descending degree, tie by id ascending.
        order1.sort_by(|&a, &b| {
            let da = adj[a].iter().filter(|&&x| x).count();
            let db = adj[b].iter().filter(|&&x| x).count();
            match db.cmp(&da) {
                Ordering::Equal => a.cmp(&b),
                other => other,
            }
        });

        // Order2: ascending degree, tie by id ascending.
        order2.sort_by(|&a, &b| {
            let da = adj[a].iter().filter(|&&x| x).count();
            let db = adj[b].iter().filter(|&&x| x).count();
            match da.cmp(&db) {
                Ordering::Equal => a.cmp(&b),
                other => other,
            }
        });

        let c1 = greedy_coloring(&adj, &order1);
        let c2 = greedy_coloring(&adj, &order2);
        let ans = c1.min(c2);

        outputs.push(format!("{ans}"));
    }

    print!("{}", outputs.join("\n"));
}
