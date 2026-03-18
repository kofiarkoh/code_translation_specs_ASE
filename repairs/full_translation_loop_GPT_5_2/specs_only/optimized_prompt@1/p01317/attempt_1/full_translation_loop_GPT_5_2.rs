use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = i64::MAX / 4;

fn floyd_warshall(dist: &mut Vec<Vec<i64>>) {
    let n = dist.len();
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
                let cand = dik.saturating_add(dkj);
                if cand < dist[i][j] {
                    dist[i][j] = cand;
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The specification says "a series of test cases" but does not define a terminator.
    // Narrowest reasonable interpretation: consume until EOF, each test case begins with (n, m).
    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        // Two distance matrices: direct and indirect.
        // Initialize with INF, diagonal 0.
        let mut direct = vec![vec![INF; n]; n];
        let mut indirect = vec![vec![INF; n]; n];
        for i in 0..n {
            direct[i][i] = 0;
            indirect[i][i] = 0;
        }

        // Each edge: u v cost type (direct or indirect).
        // The specification does not define encoding of "type"; accept common encodings:
        // - integer 0/1
        // - strings like "D"/"I", "direct"/"indirect" (case-insensitive)
        for _ in 0..m {
            let u: usize = it.next().unwrap().parse().unwrap();
            let v: usize = it.next().unwrap().parse().unwrap();
            let c: i64 = it.next().unwrap().parse().unwrap();
            let t = it.next().unwrap();

            let is_direct = match t {
                "0" => true,
                "1" => false,
                _ => {
                    let tl = t.to_ascii_lowercase();
                    tl == "d" || tl == "direct"
                }
            };

            let mat = if is_direct { &mut direct } else { &mut indirect };
            // The specification does not state directed vs undirected; choose directed as narrowest.
            if c < mat[u][v] {
                mat[u][v] = c;
            }
        }

        // All-pairs shortest paths for each matrix.
        floyd_warshall(&mut direct);
        floyd_warshall(&mut indirect);

        // Visit sequence.
        let r: usize = it.next().unwrap().parse().unwrap();
        let mut seq: Vec<usize> = Vec::with_capacity(r);
        for _ in 0..r {
            seq.push(it.next().unwrap().parse().unwrap());
        }

        // Dynamic programming:
        // dp[i][v] = min cost after processing i-th visit (0-based), being at node v.
        // Transition: from previous position p to v using either direct or indirect shortest path.
        // Additionally, we must "visit" seq[i] in order; narrowest interpretation:
        // after step i, you must be at seq[i]. Thus dp collapses to a scalar.
        //
        // However, the specification mentions "DP table maintains the minimum costs to reach each node
        // in the sequence of visits" and complexity O(R * N^2), implying dp over nodes.
        // Narrowest consistent interpretation: at each stage i, you may be at any node v, but you must
        // have visited seq[i] (i.e., the i-th required node) at that stage. Model this as:
        // dp_next[v] = min over p: dp[p] + min( direct[p][seq[i]] + direct[seq[i]][v],
        //                                       indirect[p][seq[i]] + indirect[seq[i]][v],
        //                                       direct[p][seq[i]] + indirect[seq[i]][v],
        //                                       indirect[p][seq[i]] + direct[seq[i]][v] )
        // i.e., you can traverse to the required node and then continue to v, using either matrix
        // for each leg. This preserves the existence of two path types without inventing extra rules.
        //
        // Initial state: before visiting anything, you are at seq[0] with cost 0 (narrowest).
        // If r == 0, cost is 0.
        let ans = if r == 0 {
            0i64
        } else {
            let mut dp = vec![INF; n];
            dp[seq[0]] = 0;

            for i in 1..r {
                let req = seq[i];
                let mut next = vec![INF; n];

                for p in 0..n {
                    let base = dp[p];
                    if base >= INF {
                        continue;
                    }

                    // Cost to reach required node from p using either matrix.
                    let p_to_req_d = direct[p][req];
                    let p_to_req_i = indirect[p][req];

                    for v in 0..n {
                        let req_to_v_d = direct[req][v];
                        let req_to_v_i = indirect[req][v];

                        // Combine legs; skip INF legs.
                        let mut best_leg = INF;

                        if p_to_req_d < INF && req_to_v_d < INF {
                            best_leg = min(best_leg, p_to_req_d.saturating_add(req_to_v_d));
                        }
                        if p_to_req_i < INF && req_to_v_i < INF {
                            best_leg = min(best_leg, p_to_req_i.saturating_add(req_to_v_i));
                        }
                        if p_to_req_d < INF && req_to_v_i < INF {
                            best_leg = min(best_leg, p_to_req_d.saturating_add(req_to_v_i));
                        }
                        if p_to_req_i < INF && req_to_v_d < INF {
                            best_leg = min(best_leg, p_to_req_i.saturating_add(req_to_v_d));
                        }

                        if best_leg < INF {
                            let cand = base.saturating_add(best_leg);
                            if cand < next[v] {
                                next[v] = cand;
                            }
                        }
                    }
                }

                dp = next;
            }

            // After visiting all required nodes, narrowest: you end at the last required node.
            // But dp is over all v; choose dp[last] as the traversal cost to end at last.
            // If no path exists, this will be INF as required by the specification.
            dp[seq[r - 1]]
        };

        outputs.push(ans.to_string());
    }

    print!("{}", outputs.join("\n"));
}
