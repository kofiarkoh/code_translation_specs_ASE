use std::collections::VecDeque;
use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    // Input format validation: must have at least N and M
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let m: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Graph storage: adjacency list, indegree, and parent output.
    // Nodes are 1..=N; index 0 is unused.
    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut indeg: Vec<usize> = Vec::new();
    let mut parent: Vec<usize> = Vec::new();

    // Memory allocation failure handling (exit code 2).
    // Rust typically aborts on OOM; we attempt to reserve and treat failure as exit(2).
    // Note: reserve() may abort on OOM in many configurations; this is the closest safe emulation.
    if adj.try_reserve_exact(n + 1).is_err()
        || indeg.try_reserve_exact(n + 1).is_err()
        || parent.try_reserve_exact(n + 1).is_err()
    {
        process::exit(2);
    }

    adj.resize_with(n + 1, Vec::new);
    indeg.resize(n + 1, 0);
    parent.resize(n + 1, 0);

    // Read edges; if not enough integers, exit(1).
    for _ in 0..m {
        let u: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let v: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };

        // Specification assumes valid node indices and DAG; we do not add extra validation.
        // Memory allocation failure during edge addition => exit(2).
        if adj[u].try_reserve(1).is_err() {
            process::exit(2);
        }
        adj[u].push(v);
        indeg[v] = indeg[v].wrapping_add(1);
    }

    // Kahn's algorithm for topological processing.
    // Parent assignment semantics (narrowest reasonable interpretation):
    // - When a node v becomes ready (its indegree drops to 0) due to processing u,
    //   set parent[v] = u if not already set.
    // - Nodes that start with indegree 0 keep parent 0.
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }

    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            // Maintain accurate incoming edge counts.
            indeg[v] = indeg[v].wrapping_sub(1);
            if indeg[v] == 0 {
                if parent[v] == 0 {
                    parent[v] = u;
                }
                q.push_back(v);
            }
        }
    }

    // Output parents for nodes 1..=N in order.
    // Format: sequence of integers separated by spaces, trailing newline.
    let mut out = String::new();
    for i in 1..=n {
        if i > 1 {
            out.push(' ');
        }
        out.push_str(&parent[i].to_string());
    }
    out.push('\n');
    print!("{out}");
}
