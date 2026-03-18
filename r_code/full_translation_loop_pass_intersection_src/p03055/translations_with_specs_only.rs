use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Read all remaining pairs as edges (spec edge count is inconsistent with tree inputs).
    let mut raw_edges: Vec<(usize, usize)> = Vec::new();
    loop {
        let a = match it.next() {
            Some(v) => v.parse::<usize>().unwrap(),
            None => break,
        };
        let b = match it.next() {
            Some(v) => v.parse::<usize>().unwrap(),
            None => break,
        };
        raw_edges.push((a, b));
    }

    // Decide indexing: if any endpoint is 0, treat as 0-based; otherwise 1-based.
    let zero_based = raw_edges.iter().any(|&(a, b)| a == 0 || b == 0);

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b) in raw_edges {
        let (u, v) = if zero_based {
            (a, b)
        } else {
            // 1-based to 0-based; if malformed (0), skip safely.
            if a == 0 || b == 0 {
                continue;
            }
            (a - 1, b - 1)
        };
        if u < n && v < n {
            g[u].push(v);
            g[v].push(u);
        }
    }

    // Compute tree diameter length in edges via two BFS passes.
    // This matches the sample: diameter length 3 => odd => "First".
    fn bfs_farthest(g: &[Vec<usize>], start: usize) -> (usize, i32, Vec<i32>) {
        let n = g.len();
        let mut dist = vec![-1i32; n];
        let mut q = VecDeque::new();
        dist[start] = 0;
        q.push_back(start);

        while let Some(u) = q.pop_front() {
            let du = dist[u];
            for &v in &g[u] {
                if dist[v] == -1 {
                    dist[v] = du + 1;
                    q.push_back(v);
                }
            }
        }

        let mut far = start;
        let mut best = dist[start];
        for (i, &d) in dist.iter().enumerate() {
            if d > best {
                best = d;
                far = i;
            }
        }
        (far, best, dist)
    }

    let diameter_len: i32 = if n == 0 {
        0
    } else {
        let (a, _, _) = bfs_farthest(&g, 0);
        let (_b, d, _) = bfs_farthest(&g, a);
        d
    };

    let ans = if (diameter_len & 1) == 1 { "First" } else { "Second" };
    print!("{ans}");
}
