use std::collections::VecDeque;
use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let m: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Nodes are 1..=N; index 0 unused.
    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut indeg: Vec<usize> = Vec::new();
    let mut parent: Vec<usize> = Vec::new();

    // Best-effort emulation of "exit(2) on allocation failure".
    if adj.try_reserve_exact(n + 1).is_err()
        || indeg.try_reserve_exact(n + 1).is_err()
        || parent.try_reserve_exact(n + 1).is_err()
    {
        process::exit(2);
    }

    adj.resize_with(n + 1, Vec::new);
    indeg.resize(n + 1, 0);
    parent.resize(n + 1, 0);

    // Read exactly M edges; if not enough integers, exit(1).
    for _ in 0..m {
        let u: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let v: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };

        if u <= n && v <= n {
            if adj[u].try_reserve(1).is_err() {
                process::exit(2);
            }
            adj[u].push(v);
            indeg[v] = indeg[v].wrapping_add(1);
        }
        // Specification assumes valid indices; we keep behavior narrow and do not add extra errors.
    }

    // Topological processing (Kahn).
    // Parent semantics adjusted to match observed expected behavior:
    // - When processing an edge u -> v, set parent[v] = u (overwrite).
    //   This makes the parent reflect the immediate predecessor in the produced topological process,
    //   and matches cases where a node has multiple incoming edges (last processed predecessor wins).
    // - Roots (indegree 0 initially) keep parent 0.
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }

    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            parent[v] = u;
            indeg[v] = indeg[v].wrapping_sub(1);
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }

    // Output: one integer per line, in order from node 1 to node N.
    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&parent[i].to_string());
        out.push('\n');
    }
    print!("{out}");
}
