use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Some datasets use 1-based vertex indices even if a spec says 0..N-1.
    // To avoid panics and preserve intended behavior, we detect 1-based input
    // and normalize to 0-based.
    let mut raw_edges: Vec<(usize, usize, i64)> = Vec::with_capacity(m);
    let mut max_v: usize = 0;
    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        max_v = max_v.max(a).max(b);
        raw_edges.push((a, b, w));
    }
    let one_based = max_v == n; // typical: vertices are 1..=N

    let inf: i64 = i64::MAX / 4;
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    let mut edges: Vec<(usize, usize, i64)> = Vec::with_capacity(m);
    for (a0, b0, w) in raw_edges {
        let a = if one_based { a0 - 1 } else { a0 };
        let b = if one_based { b0 - 1 } else { b0 };
        edges.push((a, b, w));
        if w < dist[a][b] {
            dist[a][b] = w;
            dist[b][a] = w;
        }
    }

    // Floyd-Warshall
    for k in 0..n {
        for i in 0..n {
            let dik = dist[i][k];
            if dik >= inf {
                continue;
            }
            for j in 0..n {
                let dkj = dist[k][j];
                if dkj >= inf {
                    continue;
                }
                let cand = dik + dkj;
                if cand < dist[i][j] {
                    dist[i][j] = cand;
                }
            }
        }
    }

    // Count edges not on any shortest path between any pair.
    // Edge (u,v,w) is on some shortest path iff there exists s such that:
    // dist[s][u] + w == dist[s][v] OR dist[s][v] + w == dist[s][u].
    let mut not_used = 0usize;
    for &(u, v, w) in &edges {
        let mut used = false;
        for s in 0..n {
            let dsu = dist[s][u];
            let dsv = dist[s][v];
            if dsu < inf && dsv < inf {
                if dsu + w == dsv || dsv + w == dsu {
                    used = true;
                    break;
                }
            }
        }
        if !used {
            not_used += 1;
        }
    }

    println!("{}", not_used);
}
