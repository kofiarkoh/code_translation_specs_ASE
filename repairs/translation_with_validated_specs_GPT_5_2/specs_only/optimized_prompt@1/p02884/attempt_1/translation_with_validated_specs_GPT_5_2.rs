use std::cmp::Ordering;
use std::io::{self, Read};

fn compute_expected(n: usize, adj: &Vec<Vec<usize>>, removed: Option<(usize, usize)>) -> Vec<f64> {
    let mut e = vec![0.0f64; n + 1];
    e[n] = 0.0;

    // Recurrence: E[u] = 1 + average(E[v] for v in outgoing(u)), with optional removal of one edge.
    // We compute in decreasing node index order, matching the implied "well-defined" precondition
    // (outgoing edges to >= itself ensure no forward dependency on smaller indices is required).
    for u in (1..n).rev() {
        let mut sum = 0.0f64;
        let mut cnt = 0usize;

        for &v in &adj[u] {
            if let Some((ru, rv)) = removed {
                if ru == u && rv == v {
                    continue;
                }
            }
            sum += e[v];
            cnt += 1;
        }

        // Per spec, if cnt == 0 behavior is undefined; we do not guard.
        e[u] = 1.0 + sum / (cnt as f64);
    }

    e
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Fixed-size internal storage is implied by the spec, but no explicit bound is given.
    // We allocate based on input sizes.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        adj[u].push(v);
    }

    // Compute expected values for the original graph.
    let e0 = compute_expected(n, &adj, None);
    let mut best = e0[1];

    // Candidate set C: for each u with outdegree > 1, choose the outgoing edge (u,v)
    // whose destination has the largest expected-step value under the original graph.
    // If multiple tie, choose the first encountered (narrowest reasonable interpretation).
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for u in 1..n {
        if adj[u].len() > 1 {
            let mut best_v = adj[u][0];
            let mut best_ev = e0[best_v];
            for &v in &adj[u][1..] {
                let ev = e0[v];
                match ev.partial_cmp(&best_ev).unwrap_or(Ordering::Equal) {
                    Ordering::Greater => {
                        best_ev = ev;
                        best_v = v;
                    }
                    _ => {}
                }
            }
            candidates.push((u, best_v));
        }
    }

    // Evaluate each graph with one candidate edge removed and take the minimum E(1).
    for (u, v) in candidates {
        let e = compute_expected(n, &adj, Some((u, v)));
        if e[1] < best {
            best = e[1];
        }
    }

    // Fixed decimal precision; the spec does not state how many digits.
    // Use 10 digits after the decimal as a conventional fixed precision.
    println!("{:.10}", best);
}
