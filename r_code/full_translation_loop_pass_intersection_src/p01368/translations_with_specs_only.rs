use std::cmp::min;
use std::collections::VecDeque;
use std::io::{self, Read};

const INF: i64 = i64::MAX / 4;

fn hopcroft_karp(adj: &[Vec<usize>], n_left: usize, n_right: usize) -> usize {
    // Standard Hopcroft–Karp for maximum bipartite matching.
    // Left side: 0..n_left, Right side: 0..n_right.
    let mut pair_u: Vec<Option<usize>> = vec![None; n_left];
    let mut pair_v: Vec<Option<usize>> = vec![None; n_right];
    let mut dist: Vec<i32> = vec![0; n_left];

    fn bfs(
        adj: &[Vec<usize>],
        pair_u: &[Option<usize>],
        pair_v: &[Option<usize>],
        dist: &mut [i32],
    ) -> bool {
        let mut q = VecDeque::new();
        for u in 0..pair_u.len() {
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
        found_free_on_right
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

    let mut matching = 0usize;
    while bfs(adj, &pair_u, &pair_v, &mut dist) {
        for u in 0..n_left {
            if pair_u[u].is_none() && dfs(u, adj, &mut pair_u, &mut pair_v, &mut dist) {
                matching += 1;
            }
        }
    }
    matching
}

fn main() {
    // Input format is not further specified; we interpret it as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

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
        let k: usize = it.next().unwrap().parse().unwrap();

        // Fixed supported limit is stated, but behavior is unspecified if exceeded; we just allocate.
        let mut dist = vec![vec![INF; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            let w: i64 = it.next().unwrap().parse().unwrap();
            // Node indices are assumed valid by precondition; interpret as 0-based.
            // If input is 1-based, behavior is unspecified by the spec; we do not guess.
            dist[a][b] = min(dist[a][b], w);
            dist[b][a] = min(dist[b][a], w);
        }

        // Floyd–Warshall all-pairs shortest paths.
        for mid in 0..n {
            for i in 0..n {
                let dik = dist[i][mid];
                if dik >= INF {
                    continue;
                }
                for j in 0..n {
                    let dkj = dist[mid][j];
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

        let mut task_node: Vec<usize> = Vec::with_capacity(k);
        let mut task_time: Vec<i64> = Vec::with_capacity(k);
        for _ in 0..k {
            let v: usize = it.next().unwrap().parse().unwrap();
            let t: i64 = it.next().unwrap().parse().unwrap();
            task_node.push(v);
            task_time.push(t);
        }

        // Build feasibility edges between distinct tasks:
        // i -> j if shortest_path(node[i], node[j]) <= time[j] - time[i].
        // Unreachable pairs are infeasible.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); k];
        for i in 0..k {
            for j in 0..k {
                if i == j {
                    continue;
                }
                let dt = task_time[j] - task_time[i];
                if dt < 0 {
                    continue;
                }
                let d = dist[task_node[i]][task_node[j]];
                if d < INF && d <= dt {
                    adj[i].push(j);
                }
            }
        }

        // Minimum path cover in a directed graph (tasks as vertices) under this feasibility relation
        // is |V| - maximum_matching in the standard bipartite construction.
        let max_matching = hopcroft_karp(&adj, k, k);
        let min_chains = k - max_matching;

        outputs.push(min_chains.to_string());
    }

    print!("{}", outputs.join("\n"));
}
