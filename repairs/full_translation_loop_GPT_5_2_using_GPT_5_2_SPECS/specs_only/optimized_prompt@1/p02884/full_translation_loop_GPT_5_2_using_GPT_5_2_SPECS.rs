use std::io::{self, Read};

fn main() {
    // Read all input; behavior is undefined on parse failure per spec.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Adjacency list; edges are intended to satisfy u < v (DAG by index).
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        g[u].push(v);
    }

    // Compute expected steps for the original graph.
    let base = expected_with_optional_removal(n, &g, None);

    // Minimum over: original, and for each node with >=2 outgoing edges,
    // one scenario removing exactly one outgoing edge chosen by internal rule.
    //
    // Internal selection rule (narrowest reasonable interpretation given spec):
    // remove the outgoing edge to the neighbor with the maximum expected value
    // under the original graph (ties broken by first occurrence in adjacency list).
    let mut ans = base;

    // Precompute base expectations to drive the removal choice.
    let base_dp = compute_dp(n, &g, None);

    for u in 1..n {
        if g[u].len() >= 2 {
            // Choose edge index to remove: argmax base_dp[v], first in list on ties.
            let mut best_idx = 0usize;
            let mut best_val = base_dp[g[u][0]];
            for (idx, &v) in g[u].iter().enumerate().skip(1) {
                let val = base_dp[v];
                if val > best_val {
                    best_val = val;
                    best_idx = idx;
                }
            }

            let cand = expected_with_optional_removal(n, &g, Some((u, best_idx)));
            if cand < ans {
                ans = cand;
            }
        }
    }

    // Output a single real number.
    // No specific formatting mandated; print with reasonable precision.
    println!("{:.10}", ans);
}

fn expected_with_optional_removal(
    n: usize,
    g: &Vec<Vec<usize>>,
    removal: Option<(usize, usize)>,
) -> f64 {
    let dp = compute_dp(n, g, removal);
    dp[1]
}

fn compute_dp(n: usize, g: &Vec<Vec<usize>>, removal: Option<(usize, usize)>) -> Vec<f64> {
    // dp[i] = expected remaining steps from node i to node n.
    // dp[n] = 0 always.
    let mut dp = vec![0.0f64; n + 1];
    dp[n] = 0.0;

    // Backward evaluation by index; edges go to higher indices.
    for i in (1..n).rev() {
        let out = &g[i];

        // Determine which outgoing edges are available in this scenario.
        // If removal targets this node, remove exactly one edge by index.
        let (rem_u, rem_idx) = removal.unwrap_or((0, usize::MAX));

        let mut sum = 0.0f64;
        let mut cnt = 0usize;

        for (idx, &v) in out.iter().enumerate() {
            if i == rem_u && idx == rem_idx {
                continue;
            }
            sum += dp[v];
            cnt += 1;
        }

        // Per spec, if cnt == 0, behavior is undefined (division by zero).
        dp[i] = 1.0 + sum / (cnt as f64);
    }

    dp
}
