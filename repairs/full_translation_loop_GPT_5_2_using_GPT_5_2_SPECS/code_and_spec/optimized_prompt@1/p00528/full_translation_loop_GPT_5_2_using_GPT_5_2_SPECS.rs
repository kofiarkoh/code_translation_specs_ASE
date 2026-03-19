use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const INF: i64 = 0x7ffffff; // matches C's INF

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    id: usize, // base id; paired node is id+1
}

fn dijkstra(
    start: usize,
    flag: i32,
    goal: usize,
    adj: &mut Vec<Vec<(usize, i64)>>,
    used: &mut Vec<bool>,
) -> i64 {
    // Reset used array (C relies on zero-initialized global; we must clear per run)
    used.fill(false);

    // C has a special-case "flag < 0" branch that is never taken from main
    // (flag is ss, and ss is either -1 or a valid id; but if ss == -1 then start is not -1,
    // so start+1 edge removal would be nonsensical). We preserve the behavior:
    // only apply if flag < 0.
    if flag < 0 {
        // Find edge start -> start+1 and set its weight to INF; also clear adjacency of start+1.
        let target = start + 1;
        for e in adj[start].iter_mut() {
            if e.0 == target {
                e.1 = INF;
            }
        }
        adj[target].clear();
    }

    let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), start));

    while let Some((Reverse(d), s)) = pq.pop() {
        if s == goal || s == goal + 1 {
            return d;
        }
        if used[s] {
            continue;
        }
        used[s] = true;

        for &(e, w) in &adj[s] {
            if !used[e] {
                pq.push((Reverse(d + w), e));
            }
        }
    }
    -1
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut k: usize = it.next().unwrap().parse().unwrap();

    // Collect points; each point corresponds to two graph nodes: id and id+1.
    let mut sw: Vec<Point> = Vec::with_capacity(k + 2);
    let mut sz: usize = 0;
    let mut s: i32 = -1;
    let mut g: i32 = -1;

    for _ in 0..k {
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        sw.push(Point { x, y, id: sz });
        if x == 1 && y == 1 {
            s = sz as i32;
        } else if x == m && y == n {
            g = sz as i32;
        }
        sz += 2;
    }

    let ss = s; // original s before implicit insertion (as in C)

    if s < 0 {
        sw.push(Point { x: 1, y: 1, id: sz });
        s = sz as i32;
        k += 1;
        sz += 2;
    }
    if g < 0 {
        sw.push(Point { x: m, y: n, id: sz });
        g = sz as i32;
        k += 1;
        sz += 2;
    }

    // Build adjacency list for 2*k nodes (ids are even-ish but not required).
    let node_count = sz;
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); node_count];

    // Sort by x then y, add internal edges and x-shared edges between base nodes.
    sw.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    // For each point: connect id <-> id+1 with cost 1.
    // Additionally, for consecutive points with same x: connect base ids with cost |dy|.
    for i in 0..k {
        let a = sw[i].id;
        let b = a + 1;
        adj[a].push((b, 1));
        adj[b].push((a, 1));

        if i > 0 && sw[i - 1].x == sw[i].x {
            let a0 = sw[i - 1].id;
            let b0 = sw[i].id;
            let d = (sw[i - 1].y - sw[i].y).abs() as i64;
            adj[a0].push((b0, d));
            adj[b0].push((a0, d));
        }
    }

    // Sort by y then x, add y-shared edges between the +1 nodes.
    sw.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    for i in 1..k {
        if sw[i - 1].y == sw[i].y {
            let a = sw[i - 1].id + 1;
            let b = sw[i].id + 1;
            let d = (sw[i - 1].x - sw[i].x).abs() as i64;
            adj[a].push((b, d));
            adj[b].push((a, d));
        }
    }

    let start = s as usize;
    let goal = g as usize;
    let mut used = vec![false; node_count];

    let ans = dijkstra(start, ss, goal, &mut adj, &mut used);
    print!("{ans}\n");
}
