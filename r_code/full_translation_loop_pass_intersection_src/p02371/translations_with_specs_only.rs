use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

fn bfs_farthest(start: usize, adj: &Vec<Vec<(usize, i32)>>) -> (usize, i32) {
    let n = adj.len();
    let mut dist: Vec<i32> = vec![-1; n];
    let mut q = VecDeque::new();

    dist[start] = 0;
    q.push_back(start);

    while let Some(u) = q.pop_front() {
        let du = dist[u];
        for &(v, w) in &adj[u] {
            if dist[v] == -1 {
                // Under the tree + non-negative weights preconditions, this is the unique path.
                // The spec states diameter fits in i32; we keep i32 arithmetic.
                dist[v] = du + w;
                q.push_back(v);
            }
        }
    }

    let mut best_v = start;
    let mut best_d = 0i32;
    for (i, &d) in dist.iter().enumerate() {
        if d > best_d {
            best_d = d;
            best_v = i;
        }
    }
    (best_v, best_d)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // unspecified behavior; choose to do nothing if no input
    };

    let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];

    // Reads exactly (n - 1) triples: u v w
    for _ in 0..n.saturating_sub(1) {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        let w: i32 = it.next().unwrap().parse().unwrap();
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    // Tree diameter via two traversals:
    // 1) from an arbitrary node (0) find farthest A
    // 2) from A find farthest distance = diameter length
    let start = if n == 0 { 0 } else { 0 };
    let (a, _) = if n == 0 {
        (0usize, 0i32)
    } else {
        bfs_farthest(start, &adj)
    };
    let (_, diameter) = if n == 0 {
        (0usize, 0i32)
    } else {
        bfs_farthest(a, &adj)
    };

    // Output a single integer and nothing else.
    // For n==0 (not a valid tree per spec), we print 0 as a narrow, deterministic choice.
    let out = if n == 0 { 0 } else { max(0, diameter) };
    print!("{}", out);
}
