use std::io::{self, Read};

const INF: i32 = 1_000_000_000;

fn main() {
    // Robust tokenization: accept any non-digit separators; allow leading '-' just in case.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let bytes = s.as_bytes();
    let mut idx = 0usize;

    let mut out = String::new();

    loop {
        let n_opt = next_i32(bytes, &mut idx);
        if n_opt.is_none() {
            break;
        }
        let n = n_opt.unwrap() as usize;
        let m = match next_i32(bytes, &mut idx) {
            Some(x) => x as usize,
            None => break,
        };

        if n == 0 {
            break;
        }

        let mut d0 = vec![vec![INF; n]; n];
        let mut d1 = vec![vec![INF; n]; n];
        for i in 0..n {
            d0[i][i] = 0;
            d1[i][i] = 0;
        }

        for _ in 0..m {
            let u_in = next_i32(bytes, &mut idx).unwrap() as usize;
            let v_in = next_i32(bytes, &mut idx).unwrap() as usize;
            let w = next_i32(bytes, &mut idx).unwrap();
            let cat = next_i32(bytes, &mut idx).unwrap();

            // Input is 1-based.
            let u = u_in - 1;
            let v = v_in - 1;

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

        floyd_warshall(&mut d0);
        floyd_warshall(&mut d1);

        let r = next_i32(bytes, &mut idx).unwrap() as usize;
        let mut req = vec![0usize; r];
        for i in 0..r {
            req[i] = (next_i32(bytes, &mut idx).unwrap() as usize) - 1;
        }

        // DP semantics per spec:
        // At stage k (having satisfied req[k]), we may be in any vertex state s.
        // Transition to stage k+1:
        //   - direct: cost d0[req[k]][req[k+1]], next state = req[k+1]
        //   - composite: choose a,b: d0[req[k]][a] + d1[a][b] + d0[b][req[k+1]], next state = b
        // Note: step cost depends only on required vertices (and chosen a,b), not on prior state.
        // Therefore dp over states collapses to a scalar; but we keep state DP to match spec.
        let mut dp = vec![INF; n];
        dp[req[0]] = 0;

        for k in 0..r.saturating_sub(1) {
            let cur = req[k];
            let nxt = req[k + 1];

            let mut best_prev = INF;
            for &v in &dp {
                if v < best_prev {
                    best_prev = v;
                }
            }

            let mut ndp = vec![INF; n];

            // Direct option: end at nxt.
            let direct = d0[cur][nxt];
            if direct < INF {
                ndp[nxt] = sat_add(best_prev, direct);
            }

            // Composite option: end at b.
            for a in 0..n {
                let c1 = d0[cur][a];
                if c1 >= INF {
                    continue;
                }
                for b in 0..n {
                    let c2 = d1[a][b];
                    if c2 >= INF {
                        continue;
                    }
                    let c3 = d0[b][nxt];
                    if c3 >= INF {
                        continue;
                    }
                    let step = sat_add(c1, sat_add(c2, c3));
                    let val = sat_add(best_prev, step);
                    if val < ndp[b] {
                        ndp[b] = val;
                    }
                }
            }

            dp = ndp;
        }

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
    // Spec guarantees sums fit in i32 for valid inputs; clamp to INF defensively.
    match a.checked_add(b) {
        Some(x) => x.min(INF),
        None => INF,
    }
}

fn next_i32(bytes: &[u8], idx: &mut usize) -> Option<i32> {
    let n = bytes.len();
    while *idx < n {
        let c = bytes[*idx];
        if (c as char).is_ascii_digit() || c == b'-' {
            break;
        }
        *idx += 1;
    }
    if *idx >= n {
        return None;
    }

    let mut sign = 1i32;
    if bytes[*idx] == b'-' {
        sign = -1;
        *idx += 1;
    }

    let mut val: i32 = 0;
    let mut any = false;
    while *idx < n {
        let c = bytes[*idx];
        if !(c as char).is_ascii_digit() {
            break;
        }
        any = true;
        val = val.saturating_mul(10).saturating_add((c - b'0') as i32);
        *idx += 1;
    }
    if !any {
        // Skip this char and try again; keeps behavior defined on odd separators like "-\n".
        return next_i32(bytes, idx);
    }
    Some(val.saturating_mul(sign))
}
