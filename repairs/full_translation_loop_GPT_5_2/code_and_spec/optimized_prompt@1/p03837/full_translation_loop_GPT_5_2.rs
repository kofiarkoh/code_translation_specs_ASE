use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<usize> = Vec::with_capacity(m);
    let mut b: Vec<usize> = Vec::with_capacity(m);
    let mut c: Vec<i64> = Vec::with_capacity(m);

    for _ in 0..m {
        // C code reads 1-based vertices then decrements.
        let ai: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let bi: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let ci: i64 = it.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    // Distance matrix initialization (Floyd-Warshall).
    // Use a large value similar to C's 1_000_000_000.
    let inf: i64 = 1_000_000_000;
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Undirected edges; keep the minimum weight among parallel edges.
    for i in 0..m {
        let ai = a[i];
        let bi = b[i];
        let ci = c[i];
        if ci < dist[ai][bi] {
            dist[ai][bi] = ci;
        }
        if ci < dist[bi][ai] {
            dist[bi][ai] = ci;
        }
    }

    // Floyd-Warshall: O(N^3)
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let cand = dist[i][k] + dist[k][j];
                if cand < dist[i][j] {
                    dist[i][j] = cand;
                }
            }
        }
    }

    // Count edges not used in any shortest path between any pair.
    // Matches the C logic: an edge is considered "used" if there exists some j
    // such that dist[j][a] + c == dist[j][b] (directional check only, as in C).
    let mut ans: i64 = m as i64;
    for i in 0..m {
        let mut f = true;
        for j in 0..n {
            if dist[j][a[i]] + c[i] == dist[j][b[i]] {
                f = false;
                break;
            }
        }
        if !f {
            ans -= 1;
        }
    }

    print!("{ans}\n");
}
