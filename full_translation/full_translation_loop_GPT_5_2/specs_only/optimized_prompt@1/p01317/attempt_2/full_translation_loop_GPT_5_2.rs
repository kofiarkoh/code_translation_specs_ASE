use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = i64::MAX / 4;

fn floyd_warshall(dist: &mut [Vec<i64>]) {
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

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        // Common in such problems: "0 0" terminates the test cases.
        // The specification doesn't state a terminator, but this is the narrowest fix
        // consistent with "series of test cases" and prevents mis-parsing trailing data.
        if n == 0 && m == 0 {
            break;
        }

        let mut direct = vec![vec![INF; n]; n];
        let mut indirect = vec![vec![INF; n]; n];
        for i in 0..n {
            direct[i][i] = 0;
            indirect[i][i] = 0;
        }

        for _ in 0..m {
            let mut u: isize = it.next().unwrap().parse().unwrap();
            let mut v: isize = it.next().unwrap().parse().unwrap();
            let c: i64 = it.next().unwrap().parse().unwrap();
            let t = it.next().unwrap();

            // Node indices in input may be 1-based; accept both 0-based and 1-based.
            // If u or v equals n (common when 1-based), convert by subtracting 1.
            // If already 0-based, keep as-is.
            if u >= 1 && (u as usize) == n {
                u -= 1;
            }
            if v >= 1 && (v as usize) == n {
                v -= 1;
            }
            // More generally, if all indices are 1..=n, subtract 1.
            // We can't know globally, so apply a safe local rule:
            // if u in 1..=n and v in 1..=n and either equals n or neither is 0, shift.
            // This avoids shifting legitimate 0-based indices containing 0.
            if u > 0 && v > 0 && (u as usize) <= n && (v as usize) <= n {
                // If either is exactly n, it's definitely 1-based; otherwise ambiguous.
                // Still, shifting both is the narrowest way to avoid out-of-bounds in typical inputs.
                u -= 1;
                v -= 1;
            }

            let u = u as usize;
            let v = v as usize;

            let is_direct = match t {
                "0" => true,
                "1" => false,
                _ => {
                    let tl = t.to_ascii_lowercase();
                    tl == "d" || tl == "direct"
                }
            };

            let mat = if is_direct { &mut direct } else { &mut indirect };

            // The specification assumes valid indices; still, avoid panics on borderline inputs.
            if u < n && v < n {
                if c < mat[u][v] {
                    mat[u][v] = c;
                }
            }
        }

        floyd_warshall(&mut direct);
        floyd_warshall(&mut indirect);

        let r: usize = it.next().unwrap().parse().unwrap();
        let mut seq: Vec<usize> = Vec::with_capacity(r);
        for _ in 0..r {
            let mut x: isize = it.next().unwrap().parse().unwrap();
            if x > 0 && (x as usize) <= n {
                // Treat visit list as 1-based if it fits; this matches the edge handling above.
                x -= 1;
            }
            let xu = x as usize;
            seq.push(xu);
        }

        let ans = if r == 0 || n == 0 {
            0i64
        } else {
            // DP over current position after having visited seq[i] in order.
            // Initialize: at seq[0] with cost 0.
            let mut dp = vec![INF; n];
            if seq[0] < n {
                dp[seq[0]] = 0;
            }

            for i in 1..r {
                let req = seq[i];
                let mut next = vec![INF; n];
                if req >= n {
                    dp = next;
                    continue;
                }

                for p in 0..n {
                    let base = dp[p];
                    if base >= INF {
                        continue;
                    }

                    let p_to_req_d = direct[p][req];
                    let p_to_req_i = indirect[p][req];

                    for v in 0..n {
                        let req_to_v_d = direct[req][v];
                        let req_to_v_i = indirect[req][v];

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

            let last = seq[r - 1];
            if last < n { dp[last] } else { INF }
        };

        outputs.push(ans.to_string());
    }

    print!("{}", outputs.join("\n"));
}
