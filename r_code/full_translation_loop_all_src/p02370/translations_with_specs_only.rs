use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Adjacency list and indegree counts.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indeg: Vec<usize> = vec![0; n];

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();

        // The specification does not define vertex indexing (0-based vs 1-based).
        // Narrowest interpretation: use the integers as given as 0-based indices.
        // If input is out of range, behavior is undefined; we allow panic.
        adj[a].push(b);
        indeg[b] += 1;
    }

    // Kahn's algorithm: enqueue all vertices with indegree 0.
    let mut q = VecDeque::new();
    for v in 0..n {
        if indeg[v] == 0 {
            q.push_back(v);
        }
    }

    let mut out = String::new();
    while let Some(v) = q.pop_front() {
        out.push_str(&format!("{v}\n"));
        for &to in &adj[v] {
            // Maintain invariant: indegree accurately tracks remaining incoming edges.
            indeg[to] -= 1;
            if indeg[to] == 0 {
                q.push_back(to);
            }
        }
    }

    // If the graph contains cycles, behavior is undefined; we do not check completeness.
    print!("{out}");
}
