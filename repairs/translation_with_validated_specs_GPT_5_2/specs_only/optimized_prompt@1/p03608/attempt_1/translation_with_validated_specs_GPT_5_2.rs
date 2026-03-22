use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = 1_000_000_000_000i64; // large sentinel; exact value not specified

fn main() {
    // Read all input as whitespace-separated integers.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();
    let r: usize = it.next().unwrap().parse().unwrap();

    let mut req: Vec<usize> = Vec::with_capacity(r);
    for _ in 0..r {
        let v: usize = it.next().unwrap().parse().unwrap();
        // Inputs are intended 1..N; no validation per spec.
        req.push(v - 1);
    }

    // All-pairs distance table initialized to INF, diagonal 0.
    let mut dist = vec![vec![INF; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Read edges; keep only smallest weight among multiple edges.
    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let c: i64 = it.next().unwrap().parse().unwrap();
        let u = a - 1;
        let v = b - 1;
        if c < dist[u][v] {
            dist[u][v] = c;
            dist[v][u] = c;
        }
    }

    // Floyd-Warshall all-pairs shortest paths.
    // Use saturating-like behavior to avoid panics; spec says overflow is undefined,
    // but Rust must remain defined. We keep INF large and guard additions.
    for k in 0..n {
        for i in 0..n {
            let dik = dist[i][k];
            if dik >= INF {
                continue;
            }
            for j in 0..n {
                let dkj = dist[k][j];
                if dkj >= INF {
                    continue;
                }
                let cand = dik + dkj;
                if cand < dist[i][j] {
                    dist[i][j] = cand;
                }
            }
        }
    }

    // Build distance matrix among required vertices.
    let mut rd = vec![vec![INF; r]; r];
    for i in 0..r {
        for j in 0..r {
            rd[i][j] = dist[req[i]][req[j]];
        }
    }

    // DFS over permutations using shared mutable state, mirroring the intended structure.
    fn dfs(
        r: usize,
        rd: &Vec<Vec<i64>>,
        used: &mut Vec<bool>,
        last: Option<usize>,
        depth: usize,
        acc: i64,
        best: &mut i64,
    ) {
        if depth == r {
            if acc < *best {
                *best = acc;
            }
            return;
        }
        for i in 0..r {
            if used[i] {
                continue;
            }
            used[i] = true;
            let next_acc = match last {
                None => acc,
                Some(p) => acc + rd[p][i],
            };
            if next_acc < *best {
                dfs(r, rd, used, Some(i), depth + 1, next_acc, best);
            } else {
                // Even if pruning is not mentioned, it does not change observable output.
                // It only reduces work.
            }
            used[i] = false;
        }
    }

    let mut used = vec![false; r];
    let mut best = INF;
    dfs(r, &rd, &mut used, None, 0, 0, &mut best);

    println!("{}", best);
}
