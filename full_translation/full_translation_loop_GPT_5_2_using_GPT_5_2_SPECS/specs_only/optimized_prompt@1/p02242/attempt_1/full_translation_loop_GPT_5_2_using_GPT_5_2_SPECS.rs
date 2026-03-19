use std::io::{self, Read};

fn main() {
    // Input is "unsigned decimal integers separated by non-digit characters".
    // We implement a simple scanner that extracts maximal digit runs as u32.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    fn next_u32(bytes: &[u8], idx: &mut usize) -> u32 {
        while *idx < bytes.len() && !bytes[*idx].is_ascii_digit() {
            *idx += 1;
        }
        let mut val: u32 = 0;
        while *idx < bytes.len() && bytes[*idx].is_ascii_digit() {
            val = val
                .wrapping_mul(10)
                .wrapping_add((bytes[*idx] - b'0') as u32);
            *idx += 1;
        }
        val
    }

    let n_u32 = next_u32(bytes, &mut idx);
    let n = n_u32 as usize;

    // Fixed maximum of 100 outgoing edges stored per vertex.
    // We store adjacency as Vec<Vec<(v, w)>>; preconditions guarantee k <= 100.
    let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];

    for _ in 0..n {
        let u = next_u32(bytes, &mut idx) as usize;
        let k = next_u32(bytes, &mut idx) as usize;
        let mut edges: Vec<(usize, i32)> = Vec::with_capacity(k);
        for _ in 0..k {
            let v = next_u32(bytes, &mut idx) as usize;
            let w = next_u32(bytes, &mut idx) as i32; // non-negative per preconditions
            edges.push((v, w));
        }
        // Each vertex’s adjacency description is provided at most once (precondition).
        adj[u] = edges;
    }

    // Dijkstra with O(n^2 + m) selection by scanning all vertices.
    // Costs are either -1 (unreached) or non-negative i32.
    let mut dist: Vec<i32> = vec![-1; n];
    let mut visited: Vec<bool> = vec![false; n];

    if n > 0 {
        dist[0] = 0;
    }

    for _ in 0..n {
        // Select unvisited vertex with smallest non-negative dist.
        let mut min_v: Option<usize> = None;
        let mut min_d: i32 = 0;

        for v in 0..n {
            if visited[v] {
                continue;
            }
            let d = dist[v];
            if d < 0 {
                continue;
            }
            match min_v {
                None => {
                    min_v = Some(v);
                    min_d = d;
                }
                Some(_) => {
                    if d < min_d {
                        min_v = Some(v);
                        min_d = d;
                    }
                }
            }
        }

        let u = match min_v {
            None => break, // no more reachable vertices
            Some(u) => u,
        };

        visited[u] = true;

        // Relax outgoing edges.
        // Preconditions guarantee no overflow for any reachable shortest-path cost.
        let du = dist[u];
        for &(v, w) in &adj[u] {
            if visited[v] {
                continue;
            }
            let cand = du + w;
            if dist[v] < 0 || cand < dist[v] {
                dist[v] = cand;
            }
        }
    }

    let mut out = String::new();
    for i in 0..n {
        out.push_str(&format!("{} {}\n", i, dist[i]));
    }
    print!("{}", out);
}
