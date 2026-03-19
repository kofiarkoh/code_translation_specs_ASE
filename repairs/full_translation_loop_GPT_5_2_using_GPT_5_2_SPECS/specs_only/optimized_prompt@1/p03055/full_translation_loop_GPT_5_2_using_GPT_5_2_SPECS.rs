use std::cmp::Reverse;
use std::collections::VecDeque;
use std::io::{self, Read};

fn bfs_farthest(start: usize, adj: &[Vec<usize>]) -> (usize, i32) {
    let n = adj.len();
    // depth == 0 means unvisited; visited vertices get positive depths (1-based).
    let mut depth = vec![0i32; n];
    let mut q = VecDeque::new();

    depth[start] = 1;
    q.push_back(start);

    let mut far_v = start;
    let mut far_d = 1i32;

    while let Some(v) = q.pop_front() {
        let dv = depth[v];
        if dv > far_d {
            far_d = dv;
            far_v = v;
        }
        for &to in &adj[v] {
            if depth[to] == 0 {
                depth[to] = dv + 1;
                q.push_back(to);
            }
        }
    }

    (far_v, far_d)
}

fn main() {
    // No explicit error handling per specification; malformed/insufficient input => unspecified.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed internal capacity is implied by the spec (<= 200000); we do not enforce it.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..(n.saturating_sub(1)) {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        // Input is 1-based; convert to 0-based. Preconditions guarantee validity.
        let u = a - 1;
        let v = b - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    // The spec mentions "sorting-like processing of edges"; sorting adjacency lists matches that
    // flavor without changing BFS semantics.
    for nbrs in &mut adj {
        nbrs.sort_unstable_by_key(|&x| Reverse(x));
        nbrs.sort_unstable();
    }

    // First BFS from vertex 0 to find a farthest vertex.
    let (s, _) = bfs_farthest(0, &adj);
    // Second BFS from that farthest vertex; maximum depth equals diameter length in vertices.
    let (_, d) = bfs_farthest(s, &adj);

    if d % 3 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
