use std::cmp::min;
use std::io::{self, Read};

const INF: i32 = 1_000_000_000;

fn floyd_warshall(dist: &mut Vec<Vec<i32>>, n: usize) {
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
        let _m_ignored_if_n0: usize = it.next().unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let m: usize = _m_ignored_if_n0;

        let mut dist_l = vec![vec![INF; n]; n];
        let mut dist_s = vec![vec![INF; n]; n];
        for i in 0..n {
            dist_l[i][i] = 0;
            dist_s[i][i] = 0;
        }

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
            let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
            let c: i32 = it.next().unwrap().parse().unwrap();
            let t = it.next().unwrap();
            let is_l = t.as_bytes()[0] == b'L';

            if is_l {
                // Last one read overwrites earlier ones for the same category.
                dist_l[a][b] = c;
                dist_l[b][a] = c;
            } else {
                dist_s[a][b] = c;
                dist_s[b][a] = c;
            }
        }

        floyd_warshall(&mut dist_l, n);
        floyd_warshall(&mut dist_s, n);

        let r: usize = it.next().unwrap().parse().unwrap();
        let mut req: Vec<usize> = Vec::with_capacity(r);
        for _ in 0..r {
            req.push(it.next().unwrap().parse::<usize>().unwrap() - 1);
        }

        if r == 1 {
            outputs.push("0".to_string());
            continue;
        }

        let mut dp_prev = vec![INF; n];
        dp_prev[req[0]] = 0;

        for k in 0..(r - 1) {
            let a = req[k];
            let b = req[k + 1];
            let mut dp_next = vec![INF; n];

            for j in 0..n {
                let base = dp_prev[j];
                if base >= INF {
                    continue;
                }

                // Direct 'L'-only move is only considered when i == j, and updates dp_next[i].
                let direct = dist_l[a][b];
                if direct < INF {
                    let cand = base.saturating_add(direct);
                    if cand < dp_next[j] {
                        dp_next[j] = cand;
                    }
                }

                let alj = dist_l[a][j];
                if alj >= INF {
                    continue;
                }
                let base2 = base.saturating_add(alj);

                for i in 0..n {
                    let sji = dist_s[j][i];
                    if sji >= INF {
                        continue;
                    }
                    let lib = dist_l[i][b];
                    if lib >= INF {
                        continue;
                    }
                    let cand = base2.saturating_add(sji).saturating_add(lib);
                    if cand < dp_next[i] {
                        dp_next[i] = cand;
                    }
                }
            }

            dp_prev = dp_next;
        }

        let ans = dp_prev.into_iter().fold(INF, |acc, x| min(acc, x));
        outputs.push(ans.to_string());
    }

    print!("{}", outputs.join("\n"));
}
