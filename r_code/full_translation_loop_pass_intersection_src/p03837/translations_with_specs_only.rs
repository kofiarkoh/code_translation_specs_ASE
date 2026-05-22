use std::cmp::min;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
    w: i64,
}

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut edges: Vec<Edge> = Vec::with_capacity(m);

    // Use a large "infinity" value. The spec states sums remain in range, so this is safe.
    let inf: i64 = i64::MAX / 4;

    // Distance table initialized to INF, diagonal to 0.
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Read edges; for shortest paths, keep minimum weight among parallel edges.
    for _ in 0..m {
        let a_in: usize = it.next().unwrap().parse().unwrap();
        let b_in: usize = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();

        let a = a_in - 1;
        let b = b_in - 1;

        edges.push(Edge { a, b, w });

        // Undirected graph: update both directions with minimum direct edge weight.
        dist[a][b] = min(dist[a][b], w);
        dist[b][a] = min(dist[b][a], w);
    }

    // Floyd-Warshall all-pairs shortest paths.
    for k in 0..n {
        for i in 0..n {
            let dik = dist[i][k];
            if dik == inf {
                continue;
            }
            for j in 0..n {
                let dkj = dist[k][j];
                if dkj == inf {
                    continue;
                }
                let cand = dik + dkj;
                if cand < dist[i][j] {
                    dist[i][j] = cand;
                }
            }
        }
    }

    // Count edges that are not used by any shortest path in the specified sense.
    // An edge record (a,b,w) is "used" if there exists some source s such that:
    // dist[s][a] + w == dist[s][b]  (direction as given by the input record).
    let mut unused = 0usize;
    for e in &edges {
        let mut used = false;
        for s in 0..n {
            // If dist[s][a] is INF, the sum cannot match a finite dist[s][b] under the spec's
            // assumptions; still, we avoid adding INF.
            let dsa = dist[s][e.a];
            if dsa == inf {
                continue;
            }
            if dsa + e.w == dist[s][e.b] {
                used = true;
                break;
            }
        }
        if !used {
            unused += 1;
        }
    }

    print!("{}", unused);
}
