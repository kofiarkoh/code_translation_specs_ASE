use std::io::{self, Read};

const INF: i32 = 0x1010; // matches C's choice; relies on problem constraints

fn main() {
    // Read entire stdin, parse as non-negative integers (like C in()).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    fn next_uint(bytes: &[u8], idx: &mut usize) -> Option<i32> {
        // Skip non-digits
        while *idx < bytes.len() && (bytes[*idx] < b'0' || bytes[*idx] > b'9') {
            *idx += 1;
        }
        if *idx >= bytes.len() {
            return None;
        }
        let mut n: i32 = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as i32;
            *idx += 1;
        }
        Some(n)
    }

    let mut out = String::new();

    loop {
        let n = match next_uint(bytes, &mut idx) {
            Some(v) => v as usize,
            None => break,
        };
        if n == 0 {
            break;
        }
        let m = next_uint(bytes, &mut idx).unwrap() as usize;
        let l = next_uint(bytes, &mut idx).unwrap() as usize;

        // All-pairs shortest paths (Floyd-Warshall), with INF for unreachable.
        let mut dist = vec![vec![INF; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for _ in 0..m {
            let u = next_uint(bytes, &mut idx).unwrap() as usize;
            let v = next_uint(bytes, &mut idx).unwrap() as usize;
            let d = next_uint(bytes, &mut idx).unwrap();
            dist[u][v] = d;
            dist[v][u] = d;
        }

        for k in 0..n {
            for u in 0..n {
                if dist[u][k] == INF {
                    continue;
                }
                for v in 0..n {
                    if dist[k][v] == INF {
                        continue;
                    }
                    let nd = dist[u][k] + dist[k][v];
                    if dist[u][v] > nd {
                        dist[u][v] = nd;
                    }
                }
            }
        }

        // Events: location p[i], time t[i]
        let mut p = vec![0usize; l];
        let mut t = vec![0i32; l];
        for i in 0..l {
            p[i] = next_uint(bytes, &mut idx).unwrap() as usize;
            t[i] = next_uint(bytes, &mut idx).unwrap();
        }

        // Build bipartite graph: left events [0..L), right events [L..2L)
        // Edge u -> (v+L) if feasible to go from u to v in time.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); l];
        for u in 0..l {
            for v in 0..l {
                if u == v {
                    continue;
                }
                let travel = dist[p[u]][p[v]];
                // Unreachable pairs have travel == INF, which will fail the <= check.
                if travel <= t[v] - t[u] {
                    adj[u].push(v + l);
                }
            }
        }

        // Maximum bipartite matching via DFS augmenting paths (Kuhn),
        // mirroring the C code's match[] over 2L nodes.
        let mut match_to: Vec<i32> = vec![-1; 2 * l];

        fn bpm(
            u: usize,
            adj: &Vec<Vec<usize>>,
            seen: &mut [bool],
            match_to: &mut [i32],
        ) -> bool {
            for &v in &adj[u] {
                if seen[v] {
                    continue;
                }
                seen[v] = true;
                let mu = match_to[v];
                if mu < 0 || bpm(mu as usize, adj, seen, match_to) {
                    match_to[u] = v as i32;
                    match_to[v] = u as i32;
                    return true;
                }
            }
            false
        }

        let mut k = l as i32;
        for u in 0..l {
            let mut seen = vec![false; 2 * l];
            if bpm(u, &adj, &mut seen, &mut match_to) {
                k -= 1;
            }
        }

        out.push_str(&format!("{k}\n"));
    }

    print!("{out}");
}
