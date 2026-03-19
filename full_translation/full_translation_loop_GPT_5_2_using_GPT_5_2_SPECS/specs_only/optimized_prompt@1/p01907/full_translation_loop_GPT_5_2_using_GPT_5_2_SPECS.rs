use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let base: f64 = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n + 1];
    for _ in 0..n.saturating_sub(1) {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        adj[a].push((b, w));
        adj[b].push((a, w));
    }

    // Root the tree at 1 to get depth and parent-edge weight.
    let mut visited = vec![false; n + 1];
    let mut depth: Vec<usize> = vec![0; n + 1];
    let mut parent_w: Vec<i64> = vec![0; n + 1];

    let mut q = VecDeque::new();
    visited[1] = true;
    q.push_back(1);

    while let Some(u) = q.pop_front() {
        let du = depth[u];
        for &(v, w) in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                depth[v] = du + 1;
                parent_w[v] = w;
                q.push_back(v);
            }
        }
    }

    // Scaling per depth: s(d) = base^d, with s(0)=1.
    // Compute A and B by aggregating counts and weight-sums per depth, then iterating depths
    // in increasing order with repeated multiplication (avoids pow and matches "repeated
    // multiplication" behavior).
    let mut max_d = 0usize;
    for v in 1..=n {
        max_d = max_d.max(depth[v]);
    }

    let mut cnt_at_depth: Vec<usize> = vec![0; max_d + 1];
    let mut wsum_at_depth: Vec<f64> = vec![0.0; max_d + 1];
    for v in 1..=n {
        let d = depth[v];
        cnt_at_depth[d] += 1;
        wsum_at_depth[d] += parent_w[v] as f64;
    }

    let mut a_sum: f64 = 0.0;
    let mut b_sum: f64 = 0.0;

    let mut s: f64 = 1.0; // base^0
    for d in 0..=max_d {
        b_sum += (cnt_at_depth[d] as f64) * s;
        a_sum += wsum_at_depth[d] * s;
        s *= base;
    }

    let out = a_sum * b_sum;
    println!("{:.10}", out);
}
