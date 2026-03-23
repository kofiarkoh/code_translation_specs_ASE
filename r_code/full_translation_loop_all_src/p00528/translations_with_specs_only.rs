use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Cell {
    x: i64,
    y: i64,
    idx: usize, // index in S (0..slen)
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    // Multiset S: keep duplicates as distinct entries.
    let mut cells: Vec<Cell> = Vec::with_capacity(k + 2);
    for idx in 0..k {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        cells.push(Cell { x, y, idx });
    }

    // Add (1,1) if not already present (by coordinate existence, not by multiplicity).
    let has_start = cells.iter().any(|c| c.x == 1 && c.y == 1);
    if !has_start {
        let idx = cells.len();
        cells.push(Cell { x: 1, y: 1, idx });
    }

    // Add (m,n) if not already present.
    let has_goal = cells.iter().any(|c| c.x == m && c.y == n);
    if !has_goal {
        let idx = cells.len();
        cells.push(Cell { x: m, y: n, idx });
    }

    // Reassign idx to be 0..slen-1 in current order (stable, duplicates preserved).
    for (i, c) in cells.iter_mut().enumerate() {
        c.idx = i;
    }
    let slen = cells.len();
    let node_count = slen * 2;

    // Find start and goal indices by first occurrence (narrowest reasonable choice).
    let start_cell_idx = cells
        .iter()
        .position(|c| c.x == 1 && c.y == 1)
        .unwrap();
    let goal_cell_idx = cells
        .iter()
        .position(|c| c.x == m && c.y == n)
        .unwrap();

    let start_node = start_cell_idx * 2; // (1,1)^(0)
    let goal0 = goal_cell_idx * 2;
    let goal1 = goal_cell_idx * 2 + 1;

    // Adjacency list: undirected weighted graph.
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); node_count];

    // 5.1 Intra-cell edges cost 1 between p^(0) and p^(1).
    for i in 0..slen {
        let a = i * 2;
        let b = i * 2 + 1;
        adj[a].push((b, 1));
        adj[b].push((a, 1));
    }

    // 5.2 Same-row connections between ^(0) nodes only, consecutive in (x,y) sort.
    {
        let mut v = cells.clone();
        v.sort_by(|a, b| {
            let cx = a.x.cmp(&b.x);
            if cx != std::cmp::Ordering::Equal {
                cx
            } else {
                a.y.cmp(&b.y)
            }
        });

        for w in v.windows(2) {
            let p = w[0];
            let q = w[1];
            if p.x == q.x {
                let cost = (p.y - q.y).abs();
                let u = p.idx * 2; // ^(0)
                let v = q.idx * 2; // ^(0)
                adj[u].push((v, cost));
                adj[v].push((u, cost));
            }
        }
    }

    // 5.3 Same-column connections between ^(1) nodes only, consecutive in (y,x) sort.
    {
        let mut v = cells.clone();
        v.sort_by(|a, b| {
            let cy = a.y.cmp(&b.y);
            if cy != std::cmp::Ordering::Equal {
                cy
            } else {
                a.x.cmp(&b.x)
            }
        });

        for w in v.windows(2) {
            let p = w[0];
            let q = w[1];
            if p.y == q.y {
                let cost = (p.x - q.x).abs();
                let u = p.idx * 2 + 1; // ^(1)
                let v = q.idx * 2 + 1; // ^(1)
                adj[u].push((v, cost));
                adj[v].push((u, cost));
            }
        }
    }

    // Dijkstra from start_node; early exit when goal0 or goal1 is popped.
    let inf: i64 = i64::MAX / 4;
    let mut dist = vec![inf; node_count];
    dist[start_node] = 0;

    let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), start_node));

    let mut answer: Option<i64> = None;

    while let Some((Reverse(d), u)) = pq.pop() {
        if d != dist[u] {
            continue;
        }
        if u == goal0 || u == goal1 {
            answer = Some(d);
            break;
        }
        for &(v, w) in &adj[u] {
            // Edge weights are non-negative per spec; use i64 arithmetic.
            let nd = d.saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                pq.push((Reverse(nd), v));
            }
        }
    }

    if let Some(ans) = answer {
        print!("{}", ans);
    } else {
        print!("-1");
    }
}
