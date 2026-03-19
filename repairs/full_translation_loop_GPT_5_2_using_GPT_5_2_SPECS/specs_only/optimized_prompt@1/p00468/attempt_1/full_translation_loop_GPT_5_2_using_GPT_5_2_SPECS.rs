use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Fixed-capacity adjacency matrix for vertices 0..=500.
        // This mirrors the spec's implied 501x501 representation and its corner cases:
        // if endpoints are outside 0..=500, behavior is not guaranteed by the spec;
        // we keep indexing guarded to preserve memory safety in Rust.
        let mut adj = vec![vec![false; 501]; 501];

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            if a <= 500 && b <= 500 {
                adj[a][b] = true;
                adj[b][a] = true;
            }
        }

        // Mark vertices within distance <= 2 from vertex 1, excluding vertex 1.
        let mut marked = vec![false; 501];

        // Direct neighbors of 1.
        for v in 2..=n.min(500) {
            if adj[1][v] {
                marked[v] = true;
            }
        }

        // Neighbors of neighbors: for each u adjacent to 1, mark all v adjacent to u.
        for u in 2..=n.min(500) {
            if adj[1][u] {
                for v in 2..=n.min(500) {
                    if adj[u][v] {
                        marked[v] = true;
                    }
                }
            }
        }

        // Ensure vertex 1 is never counted (even if self-loops exist).
        marked[1] = false;

        let mut count = 0usize;
        for v in 2..=n.min(500) {
            if marked[v] {
                count += 1;
            }
        }

        println!("{count}");
    }
}
