use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Use a large value for "infinity". We avoid overflow in Floyd-Warshall by
    // skipping additions involving INF.
    let inf: i64 = i64::MAX / 4;

    // Distance matrix initialization: dist[i][i] = 0, others = INF.
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Store edges as given; treat as undirected (graph edges between vertices).
    // If multiple edges exist between same vertices, keep the minimum weight in dist,
    // but still count each input edge separately when determining if it is on any shortest path.
    let mut edges: Vec<(usize, usize, i64)> = Vec::with_capacity(m);
    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        edges.push((a, b, w));
        if w < dist[a][b] {
            dist[a][b] = w;
            dist[b][a] = w;
        }
    }

    // Floyd-Warshall: compute all-pairs shortest paths.
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

    // Count edges that are not part of any shortest path between any pair.
    // An undirected edge (u,v,w) is on some shortest path iff there exists s such that:
    // dist[s][u] + w == dist[s][v]  OR  dist[s][v] + w == dist[s][u]
    // (i.e., the edge can be the last step of a shortest path from s to the other endpoint).
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
