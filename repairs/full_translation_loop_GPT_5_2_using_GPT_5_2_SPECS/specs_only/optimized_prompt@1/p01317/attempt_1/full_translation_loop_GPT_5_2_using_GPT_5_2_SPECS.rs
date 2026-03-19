use std::io::{self, Read};

const MAX_V: usize = 205;
const MAX_R: usize = 1005;
// Sentinel "effectively infinite" cost. Must be large but safe for i32 additions per spec.
const INF: i32 = 1_000_000_000;

fn main() {
    // Read entire stdin as whitespace-separated tokens.
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
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        // Spec says behavior not inferable beyond fixed capacities; we still allocate dynamically
        // but keep logic consistent with intended bounds.
        let mut d0 = vec![vec![INF; n]; n];
        let mut d1 = vec![vec![INF; n]; n];
        for i in 0..n {
            d0[i][i] = 0;
            d1[i][i] = 0;
        }

        for _ in 0..m {
            let u_in: usize = it.next().unwrap().parse().unwrap();
            let v_in: usize = it.next().unwrap().parse().unwrap();
            let w: i32 = it.next().unwrap().parse().unwrap();
            let cat: i32 = it.next().unwrap().parse().unwrap();

            // Input is 1-based per spec.
            let u = u_in - 1;
            let v = v_in - 1;

            // Undirected graph.
            if cat == 0 {
                if w < d0[u][v] {
                    d0[u][v] = w;
                    d0[v][u] = w;
                }
            } else {
                if w < d1[u][v] {
                    d1[u][v] = w;
                    d1[v][u] = w;
                }
            }
        }

        // Floyd-Warshall per category: O(n^3).
        floyd_warshall(&mut d0);
        floyd_warshall(&mut d1);

        let r: usize = it.next().unwrap().parse().unwrap();
        let mut req = vec![0usize; r];
        for i in 0..r {
            let v_in: usize = it.next().unwrap().parse().unwrap();
            req[i] = v_in - 1;
        }

        // DP over stages: dp[j] = min cost after satisfying up to current required vertex,
        // while being in "state vertex" j (the "next-state vertex" b from the last composite step,
        // or the required vertex itself after a direct step).
        let mut dp = vec![INF; n];
        dp[req[0]] = 0;

        for k in 0..(r.saturating_sub(1)) {
            let cur_req = req[k];
            let next_req = req[k + 1];

            let mut ndp = vec![INF; n];

            // For each current state s with finite cost, transition to next stage.
            for s in 0..n {
                let base = dp[s];
                if base >= INF {
                    continue;
                }

                // Option 1: direct first-category cost between required vertices.
                // This ignores current state s; per spec, the step cost is between required vertices.
                // However, the invariant says we maintain a state; the narrowest consistent
                // interpretation is that the state is the current required vertex at each stage
                // unless a composite step sets it. Therefore, we only allow transitions from
                // states that can "realize" being at cur_req with the accumulated cost.
                //
                // To preserve the described state-machine semantics, we must incorporate the
                // cost to get from current state s to cur_req using first-category edges
                // before paying the step cost. This is implied by the composite definition
                // (first-category from current required vertex), and by the state invariant.
                //
                // Narrowest consistent behavior: at stage k, being in state s means we are
                // physically at s after having satisfied req[k]. To start the next step, we
                // must move (using first-category) from s to cur_req at no extra "rule" cost?
                // The spec does not define such a move. Therefore, we interpret state as the
                // "next-state vertex" b that is already aligned for the next step via
                // first-category to the next required vertex, and the current required vertex
                // itself for direct steps. Under this interpretation, the step cost depends
                // on s as the "current required vertex" proxy only when s == cur_req.
                //
                // Thus, only allow transitions when s == cur_req.
                if s == cur_req {
                    // Direct: end state becomes next required vertex.
                    let c = d0[cur_req][next_req];
                    if c < INF {
                        let val = sat_add(base, c);
                        if val < ndp[next_req] {
                            ndp[next_req] = val;
                        }
                    }

                    // Composite: choose transfer vertex a and next-state vertex b.
                    // Cost = d0[cur_req][a] + d1[a][b] + d0[b][next_req], end state = b.
                    for a in 0..n {
                        let c1 = d0[cur_req][a];
                        if c1 >= INF {
                            continue;
                        }
                        for b in 0..n {
                            let c2 = d1[a][b];
                            if c2 >= INF {
                                continue;
                            }
                            let c3 = d0[b][next_req];
                            if c3 >= INF {
                                continue;
                            }
                            let val = sat_add(base, sat_add(c1, sat_add(c2, c3)));
                            if val < ndp[b] {
                                ndp[b] = val;
                            }
                        }
                    }
                }
            }

            dp = ndp;
        }

        // After satisfying all required vertices, the minimal cost is the minimum over states.
        let mut ans = INF;
        for &v in &dp {
            if v < ans {
                ans = v;
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}

fn floyd_warshall(d: &mut [Vec<i32>]) {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            let dik = d[i][k];
            if dik >= INF {
                continue;
            }
            for j in 0..n {
                let dkj = d[k][j];
                if dkj >= INF {
                    continue;
                }
                let cand = sat_add(dik, dkj);
                if cand < d[i][j] {
                    d[i][j] = cand;
                }
            }
        }
    }
}

fn sat_add(a: i32, b: i32) -> i32 {
    // Spec guarantees sums fit in i32 for valid inputs; this is a defensive clamp to INF
    // to preserve "effectively infinite" propagation on out-of-spec cases.
    match a.checked_add(b) {
        Some(x) => x.min(INF),
        None => INF,
    }
}
