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

        // Typical multi-testcase terminator.
        if n == 0 && m == 0 {
            break;
        }

        let mut ldist = vec![vec![INF; n]; n];
        let mut sdist = vec![vec![INF; n]; n];
        for i in 0..n {
            ldist[i][i] = 0;
            sdist[i][i] = 0;
        }

        for _ in 0..m {
            // Input in samples is 1-based.
            let u_in: usize = it.next().unwrap().parse().unwrap();
            let v_in: usize = it.next().unwrap().parse().unwrap();
            let c: i64 = it.next().unwrap().parse().unwrap();
            let t = it.next().unwrap();

            let u = u_in - 1;
            let v = v_in - 1;

            // Edge type in samples: 'L' and 'S'. Treat them as the two matrices.
            // The specification only says "direct or indirect"; we map:
            // - 'L' => ldist
            // - 'S' => sdist
            // Any other token: default to ldist (narrowest; avoids inventing extra types).
            let is_l = match t {
                "L" | "l" => true,
                "S" | "s" => false,
                _ => {
                    let tl = t.to_ascii_lowercase();
                    tl == "l" || tl == "direct"
                }
            };

            let mat = if is_l { &mut ldist } else { &mut sdist };

            // The sample behavior matches undirected edges (roads/lines), so update both directions.
            if c < mat[u][v] {
                mat[u][v] = c;
            }
            if c < mat[v][u] {
                mat[v][u] = c;
            }
        }

        floyd_warshall(&mut ldist);
        floyd_warshall(&mut sdist);

        let r: usize = it.next().unwrap().parse().unwrap();
        let mut seq: Vec<usize> = Vec::with_capacity(r);
        for _ in 0..r {
            let x_in: usize = it.next().unwrap().parse().unwrap();
            seq.push(x_in - 1);
        }

        // DP with two "modes": last edge type used to arrive at current required node.
        // dpL[i] = min cost to have visited up to i and be at seq[i], last leg used L
        // dpS[i] = same, last leg used S
        //
        // Transition between required nodes uses the corresponding all-pairs distance matrix.
        // This matches the idea of two path types and yields the sample outputs.
        let ans = if r <= 1 {
            0i64
        } else {
            let mut dp_l = 0i64;
            let mut dp_s = 0i64;

            for i in 1..r {
                let a = seq[i - 1];
                let b = seq[i];

                let ll = ldist[a][b];
                let ss = sdist[a][b];

                let next_l = if ll >= INF {
                    INF
                } else {
                    min(dp_l, dp_s).saturating_add(ll)
                };
                let next_s = if ss >= INF {
                    INF
                } else {
                    min(dp_l, dp_s).saturating_add(ss)
                };

                dp_l = next_l;
                dp_s = next_s;
            }

            min(dp_l, dp_s)
        };

        outputs.push(ans.to_string());
    }

    print!("{}", outputs.join("\n"));
}
