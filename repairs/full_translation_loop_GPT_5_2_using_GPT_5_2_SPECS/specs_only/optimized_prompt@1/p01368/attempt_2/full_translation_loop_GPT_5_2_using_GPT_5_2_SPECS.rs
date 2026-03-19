use std::collections::VecDeque;
use std::io::{self, Read};

const MAX_LOC: usize = 102;
const MAX_EVT: usize = 1001;
const INF: i64 = i64::MAX / 4;

fn hopcroft_karp(adj: &[Vec<usize>], n_left: usize, n_right: usize) -> usize {
    // Standard Hopcroft-Karp for maximum bipartite matching.
    let mut pair_u: Vec<Option<usize>> = vec![None; n_left];
    let mut pair_v: Vec<Option<usize>> = vec![None; n_right];
    let mut dist: Vec<i32> = vec![0; n_left];

    let mut matching = 0usize;

    loop {
        // BFS to build layers
        let mut q = VecDeque::new();
        for u in 0..n_left {
            if pair_u[u].is_none() {
                dist[u] = 0;
                q.push_back(u);
            } else {
                dist[u] = -1;
            }
        }

        let mut found_free_on_right = false;
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if let Some(u2) = pair_v[v] {
                    if dist[u2] == -1 {
                        dist[u2] = dist[u] + 1;
                        q.push_back(u2);
                    }
                } else {
                    found_free_on_right = true;
                }
            }
        }

        if !found_free_on_right {
            break;
        }

        fn dfs(
            u: usize,
            adj: &[Vec<usize>],
            pair_u: &mut [Option<usize>],
            pair_v: &mut [Option<usize>],
            dist: &mut [i32],
        ) -> bool {
            for &v in &adj[u] {
                if let Some(u2) = pair_v[v] {
                    if dist[u2] == dist[u] + 1 && dfs(u2, adj, pair_u, pair_v, dist) {
                        pair_u[u] = Some(v);
                        pair_v[v] = Some(u);
                        return true;
                    }
                } else {
                    pair_u[u] = Some(v);
                    pair_v[v] = Some(u);
                    return true;
                }
            }
            dist[u] = -1;
            false
        }

        for u in 0..n_left {
            if pair_u[u].is_none() && dfs(u, adj, &mut pair_u, &mut pair_v, &mut dist) {
                matching += 1;
            }
        }
    }

    matching
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let m: usize = it.next().unwrap().parse().unwrap();
        let e: usize = it.next().unwrap().parse().unwrap();

        // Preconditions guarantee bounds; we keep these constants to reflect fixed-capacity intent.
        let _ = (MAX_LOC, MAX_EVT);

        // All-pairs shortest paths initialization.
        let mut dist = vec![vec![INF; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            let w: i64 = it.next().unwrap().parse().unwrap();
            let u = a - 1;
            let v = b - 1;
            if w < dist[u][v] {
                dist[u][v] = w;
                dist[v][u] = w;
            }
        }

        let mut ev_loc: Vec<usize> = Vec::with_capacity(e);
        let mut ev_time: Vec<i64> = Vec::with_capacity(e);
        for _ in 0..e {
            let l: usize = it.next().unwrap().parse().unwrap();
            let t: i64 = it.next().unwrap().parse().unwrap();
            ev_loc.push(l - 1);
            ev_time.push(t);
        }

        // Floyd-Warshall.
        for k in 0..n {
            for i in 0..n {
                let dik = dist[i][k];
                if dik == INF {
                    continue;
                }
                for j in 0..n {
                    let dkj = dist[k][j];
                    if dkj == INF {
                        continue;
                    }
                    let nd = dik + dkj;
                    if nd < dist[i][j] {
                        dist[i][j] = nd;
                    }
                }
            }
        }

        // Build feasibility edges between events.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); e];
        for i in 0..e {
            for j in 0..e {
                if i == j {
                    continue;
                }
                let dt = ev_time[j] - ev_time[i];
                if dt < 0 {
                    continue;
                }
                let d = dist[ev_loc[i]][ev_loc[j]];
                if d != INF && d <= dt {
                    adj[i].push(j);
                }
            }
        }

        let max_links = hopcroft_karp(&adj, e, e);
        let result = e - max_links;
        out.push_str(&format!("{result}\n"));
    }

    print!("{out}");
}
