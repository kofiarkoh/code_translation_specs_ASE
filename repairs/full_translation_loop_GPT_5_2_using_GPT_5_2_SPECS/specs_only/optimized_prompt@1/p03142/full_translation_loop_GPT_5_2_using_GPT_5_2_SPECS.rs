use std::collections::VecDeque;
use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    // If N and M cannot be read, terminate with nonzero exit status.
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let m: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Exactly (N-1+M) edges follow.
    let ecount = (n - 1).saturating_add(m);

    // Graph storage: adjacency list for outgoing edges.
    // Incoming-edge counts are maintained and only decrease.
    let mut out: Vec<Vec<usize>> = vec![Vec::new(); n + 1]; // 1-based indexing
    let mut indeg: Vec<usize> = vec![0; n + 1];

    for _ in 0..ecount {
        // If any required edge pair cannot be read, terminate with nonzero exit status.
        let a: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let b: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };

        // Preconditions say labels are within 1..N; behavior undefined otherwise.
        out[a].push(b);
        indeg[b] = indeg[b].wrapping_add(1);
    }

    // Recorded predecessor that first caused a vertex to become newly zero-incoming.
    // Once nonzero, it is not changed thereafter.
    let mut pred: Vec<usize> = vec![0; n + 1];

    // Initialize queue with all vertices that start with zero incoming-edge count.
    let mut q = VecDeque::new();
    for v in 1..=n {
        if indeg[v] == 0 {
            q.push_back(v);
        }
    }

    // Process vertices only when their current incoming-edge count is zero.
    while let Some(u) = q.pop_front() {
        // Conceptually remove outgoing edges of u, decreasing indegree of destinations.
        // Each directed edge contributes at most once (we traverse each adjacency once).
        for &v in &out[u] {
            // Incoming-edge counts never increase; they should be positive here if well-formed.
            // Use wrapping_sub to emulate low-level behavior if the input violates invariants.
            let before = indeg[v];
            indeg[v] = indeg[v].wrapping_sub(1);

            // Record predecessor only on a transition from positive to zero.
            // Vertices that start at zero do not get pred set unless they had been positive
            // immediately before becoming zero (which cannot happen if they started at zero).
            if before > 0 && indeg[v] == 0 {
                if pred[v] == 0 {
                    pred[v] = u;
                }
                q.push_back(v);
            }
        }
    }

    // Output N lines: pred[i] (or 0 if never recorded).
    let mut out_s = String::new();
    for i in 1..=n {
        out_s.push_str(&format!("{}\n", pred[i]));
    }
    print!("{out_s}");
}
