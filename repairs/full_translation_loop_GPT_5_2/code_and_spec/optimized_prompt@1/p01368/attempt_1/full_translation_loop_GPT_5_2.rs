use std::io::{self, Read};

const INF: i32 = 0x1010;

fn main() {
    // Read entire stdin (AOJ-style fast input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    fn read_usize(bytes: &[u8], idx: &mut usize) -> Option<usize> {
        // Skip non-digits
        while *idx < bytes.len() && (bytes[*idx] < b'0' || bytes[*idx] > b'9') {
            *idx += 1;
        }
        if *idx >= bytes.len() {
            return None;
        }
        let mut n: usize = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as usize;
            *idx += 1;
        }
        Some(n)
    }

    let mut out = String::new();

    loop {
        let n = match read_usize(bytes, &mut idx) {
            Some(v) => v,
            None => break,
        };
        if n == 0 {
            break; // matches `while (N = in())`
        }
        let m = read_usize(bytes, &mut idx).unwrap();
        let l = read_usize(bytes, &mut idx).unwrap();

        // dist matrix: N <= 102 in the original code.
        let mut dist = vec![vec![INF; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for _ in 0..m {
            let u = read_usize(bytes, &mut idx).unwrap();
            let v = read_usize(bytes, &mut idx).unwrap();
            let d = read_usize(bytes, &mut idx).unwrap() as i32;
            dist[u][v] = d;
            dist[v][u] = d;
        }

        // Floyd-Warshall with INF checks (preserves C behavior).
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

        let mut p = vec![0usize; l];
        let mut t = vec![0i32; l];
        for i in 0..l {
            p[i] = read_usize(bytes, &mut idx).unwrap();
            t[i] = read_usize(bytes, &mut idx).unwrap() as i32;
        }

        // Build bipartite graph adjacency from left tasks [0..L) to right tasks [L..2L)
        // Edge u -> (v+L) if task u can precede task v.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); l];
        for u in 0..l {
            for v in 0..l {
                if u == v {
                    continue;
                }
                let travel = dist[p[u]][p[v]];
                // In C: dist[p[u]][p[v]] <= t[v]-t[u]
                if travel <= t[v] - t[u] {
                    adj[u].push(v + l);
                }
            }
        }

        // Maximum bipartite matching via DFS augmenting paths (Kuhn algorithm),
        // mirroring the C code's `match[]` over 2L vertices.
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
                let mv = match_to[v];
                if mv < 0 || bpm(mv as usize, adj, seen, match_to) {
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
