use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let v: usize = it.next().unwrap().parse().unwrap();
    let e: usize = it.next().unwrap().parse().unwrap();

    // Build adjacency list and indegree counts.
    // The C code does a two-pass allocation; in Rust we can build directly.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); v];
    let mut indeg: Vec<i32> = vec![0; v];

    for _ in 0..e {
        let s: usize = it.next().unwrap().parse().unwrap();
        let t: usize = it.next().unwrap().parse().unwrap();
        adj[s].push(t);
        indeg[t] += 1;
    }

    // Kahn's algorithm using a simple queue with head index (matches C behavior).
    let mut q: Vec<usize> = Vec::with_capacity(v);
    for i in 0..v {
        if indeg[i] == 0 {
            q.push(i);
        }
    }

    let mut out = String::new();
    let mut head = 0usize;
    while head < q.len() {
        let i = q[head];
        head += 1;

        out.push_str(&format!("{i}\n"));
        for &k in &adj[i] {
            indeg[k] -= 1;
            if indeg[k] == 0 {
                q.push(k);
            }
        }
    }

    // If the graph contains cycles, behavior is undefined per spec; we do not check.
    print!("{out}");
}
