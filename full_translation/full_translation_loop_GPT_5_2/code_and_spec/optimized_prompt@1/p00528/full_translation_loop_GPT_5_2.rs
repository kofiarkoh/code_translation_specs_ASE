use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const INF: i64 = 0x7ffffff; // matches C's INF

#[derive(Clone, Copy)]
struct T {
    x: i32,
    y: i32,
    id: usize,
}

fn dijkstra(
    start: usize,
    flag: i32,
    goal: usize,
    adj: &mut [Vec<(usize, i64)>],
    used: &mut [bool],
) -> i64 {
    used.fill(false);

    let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), start));

    // C behavior: if original start didn't exist (flag < 0), then remove the edge
    // start -> start+1 (set its weight to INF) and clear all outgoing edges of start+1.
    if flag < 0 {
        if start + 1 < adj.len() {
            for edge in adj[start].iter_mut() {
                if edge.0 == start + 1 {
                    edge.1 = INF;
                }
            }
            adj[start + 1].clear();
        }
    }

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
    // Fast input: read all bytes, parse non-negative integers (as in C in()).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    let mut next_u32 = || -> u32 {
        while idx < bytes.len() && (bytes[idx] < b'0' || bytes[idx] > b'9') {
            idx += 1;
        }
        let mut n: u32 = 0;
        while idx < bytes.len() && (b'0'..=b'9').contains(&bytes[idx]) {
            n = n * 10 + (bytes[idx] - b'0') as u32;
            idx += 1;
        }
        n
    };

    let m = next_u32() as i32;
    let n = next_u32() as i32;
    let mut k = next_u32() as usize;

    // sw capacity: original C uses sw[200005], and may add up to 2 extra points.
    let mut sw: Vec<T> = Vec::with_capacity(k + 2);

    let mut sz: usize = 0;
    let mut s: i32 = -1;
    let mut g: i32 = -1;

    for _ in 0..k {
        let x = next_u32() as i32;
        let y = next_u32() as i32;
        sw.push(T { x, y, id: sz });
        if x == 1 && y == 1 {
            s = sz as i32;
        } else if x == m && y == n {
            g = sz as i32;
        }
        sz += 2;
    }

    let ss = s; // original start id if present, else -1

    if s < 0 {
        sw.push(T {
            x: 1,
            y: 1,
            id: sz,
        });
        s = sz as i32;
        k += 1;
        sz += 2;
    }
    if g < 0 {
        sw.push(T {
            x: m,
            y: n,
            id: sz,
        });
        g = sz as i32;
        k += 1;
        sz += 2;
    }

    let node_count = sz; // nodes are 0..sz-1

    // Adjacency: C limits to 5 edges per node; we keep Vec and push similarly.
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); node_count];
    let mut used: Vec<bool> = vec![false; node_count];

    // Sort by (x, y)
    sw.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    // Initialize first point's internal edge (id <-> id+1) with weight 1
    {
        let a = sw[0].id;
        let b = a + 1;
        adj[a].push((b, 1));
        adj[b].push((a, 1));
    }

    // For each point: add internal edge; and if same x as previous, connect ids with |dy|
    for i in 1..k {
        let a = sw[i].id;
        let b = a + 1;
        adj[a].push((b, 1));
        adj[b].push((a, 1));

        if sw[i - 1].x == sw[i].x {
            let a0 = sw[i - 1].id;
            let b0 = sw[i].id;
            let d = (sw[i - 1].y - sw[i].y).abs() as i64;
            adj[a0].push((b0, d));
            adj[b0].push((a0, d));
        }
    }

    // Sort by (y, x)
    sw.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

    // If same y as previous, connect (id+1) nodes with |dx|
    for i in 1..k {
        if sw[i - 1].y == sw[i].y {
            let a = sw[i - 1].id + 1;
            let b = sw[i].id + 1;
            let d = (sw[i - 1].x - sw[i].x).abs() as i64;
            adj[a].push((b, d));
            adj[b].push((a, d));
        }
    }

    let ans = dijkstra(s as usize, ss, g as usize, &mut adj, &mut used);
    print!("{ans}\n");
}
