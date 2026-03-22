use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let v: usize = it.next().unwrap().parse().unwrap();
    let e: usize = it.next().unwrap().parse().unwrap();

    // Build adjacency lists while preserving, for each source vertex,
    // the relative order of outgoing edges as they appear in input.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); v];
    let mut indeg: Vec<i32> = vec![0; v];

    for _ in 0..e {
        let s: usize = it.next().unwrap().parse().unwrap();
        let t: usize = it.next().unwrap().parse().unwrap();
        adj[s].push(t);
        indeg[t] += 1;
    }

    // Kahn-style topological traversal with FIFO queue.
    // Initial enqueue is in increasing vertex index order.
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 0..v {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }

    let mut out = String::new();
    while let Some(u) = q.pop_front() {
        out.push_str(&format!("{u}\n"));
        for &w in &adj[u] {
            indeg[w] -= 1;
            if indeg[w] == 0 {
                q.push_back(w);
            }
        }
    }

    print!("{out}");
}
