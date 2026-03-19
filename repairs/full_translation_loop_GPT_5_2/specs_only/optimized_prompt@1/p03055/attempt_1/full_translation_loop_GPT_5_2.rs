use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No input; behavior not specified, so exit quietly.
    };

    // Specification states there are 2n-2 edges.
    // We interpret this literally and read exactly that many pairs.
    let m = n.saturating_mul(2).saturating_sub(2);

    // Build adjacency list; nodes are assumed to be labeled by integers in input.
    // The specification does not define indexing; we choose the narrowest common interpretation:
    // 1-based labels in input, mapped to 0-based internally.
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let u = a - 1;
        let v = b - 1;
        if u < n && v < n {
            g[u].push(v);
            g[v].push(u);
        }
        // If out of range, behavior is unspecified; we ignore to keep code safe.
    }

    // Compute maximum depth from a traversal.
    // The specification does not define the root; choose node 0 if n>0.
    // Use BFS to compute shortest-path depths in the undirected graph.
    let mut depth: Vec<i32> = vec![-1; n];
    let mut q = VecDeque::new();
    if n > 0 {
        depth[0] = 0;
        q.push_back(0);
    }

    while let Some(u) = q.pop_front() {
        let du = depth[u];
        for &v in &g[u] {
            if depth[v] == -1 {
                depth[v] = du + 1;
                q.push_back(v);
            }
        }
    }

    let mut max_depth: i32 = 0;
    for &d in &depth {
        if d >= 0 {
            max_depth = max(max_depth, d);
        }
    }

    // Winner decision based on maximum depth parity.
    // The specification only says it depends on maximum depth; parity is the narrowest typical rule.
    let ans = if max_depth % 2 == 1 { "First" } else { "Second" };
    print!("{ans}");
}
