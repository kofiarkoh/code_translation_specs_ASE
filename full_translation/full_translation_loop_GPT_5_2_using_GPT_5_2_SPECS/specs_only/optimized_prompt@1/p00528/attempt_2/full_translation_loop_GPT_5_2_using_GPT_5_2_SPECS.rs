use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let w: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let h: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut raw: Vec<Point> = Vec::with_capacity(n + 2);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        // Input coordinates are assumed within bounds by the specification.
        // Interpret them as 1-based grid coordinates (common in such problems),
        // converting to 0-based internal coordinates.
        raw.push(Point { x: x - 1, y: y - 1 });
    }

    let start = Point { x: 0, y: 0 };
    let goal = Point { x: w - 1, y: h - 1 };

    // Deduplicate points; start/goal are implicitly added if missing.
    let mut map: HashMap<(i64, i64), usize> = HashMap::new();
    let mut pts: Vec<Point> = Vec::with_capacity(raw.len() + 2);

    let mut push_uniq = |p: Point, map: &mut HashMap<(i64, i64), usize>, pts: &mut Vec<Point>| {
        let key = (p.x, p.y);
        if !map.contains_key(&key) {
            let idx = pts.len();
            pts.push(p);
            map.insert(key, idx);
        }
    };

    for p in raw {
        push_uniq(p, &mut map, &mut pts);
    }
    push_uniq(start, &mut map, &mut pts);
    push_uniq(goal, &mut map, &mut pts);

    let s = map[&(start.x, start.y)];
    let t = map[&(goal.x, goal.y)];
    let m = pts.len();

    // Induced connectivity rule (narrowest interpretation consistent with the spec wording):
    // - Travel is only possible along connections induced by special points.
    // - "Manhattan differences along shared coordinates" => edges only between points that share
    //   the same x or the same y (axis-aligned), with cost = |delta| along that axis.
    // - "fixed internal switching cost at each special point" is not specified numerically; the
    //   only well-defined choice without inventing behavior is 0, so no extra per-node cost.
    //
    // To keep degree bounded (as hinted by fixed per-node adjacency capacity), connect each point
    // only to its immediate neighbors in sorted order within each x-group and y-group.
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); m];

    let mut add_edge = |a: usize, b: usize, cost: i64, adj: &mut Vec<Vec<(usize, i64)>>| {
        if a == b {
            return;
        }
        let c = if cost < 0 { 0 } else { cost };
        adj[a].push((b, c));
        adj[b].push((a, c));
    };

    // Group by x: sort by (x, y), connect consecutive with same x.
    let mut idxs: Vec<usize> = (0..m).collect();
    idxs.sort_by_key(|&i| (pts[i].x, pts[i].y));
    for win in idxs.windows(2) {
        let a = win[0];
        let b = win[1];
        if pts[a].x == pts[b].x {
            let cost = (pts[a].y - pts[b].y).abs();
            add_edge(a, b, cost, &mut adj);
        }
    }

    // Group by y: sort by (y, x), connect consecutive with same y.
    idxs.sort_by_key(|&i| (pts[i].y, pts[i].x));
    for win in idxs.windows(2) {
        let a = win[0];
        let b = win[1];
        if pts[a].y == pts[b].y {
            let cost = (pts[a].x - pts[b].x).abs();
            add_edge(a, b, cost, &mut adj);
        }
    }

    // Dijkstra.
    let inf: i64 = i64::MAX / 4;
    let mut dist = vec![inf; m];
    dist[s] = 0;
    let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), s));

    while let Some((Reverse(d), u)) = pq.pop() {
        if d != dist[u] {
            continue;
        }
        if u == t {
            break;
        }
        for &(v, wcost) in &adj[u] {
            let nd = d.saturating_add(wcost);
            if nd < dist[v] {
                dist[v] = nd;
                pq.push((Reverse(nd), v));
            }
        }
    }

    if dist[t] >= inf {
        println!("-1");
    } else {
        println!("{}", dist[t]);
    }
}
