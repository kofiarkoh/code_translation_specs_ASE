use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    const MAX_N: usize = 150;
    const MAX_OPT: usize = 10;
    const MAX_M: usize = 79; // skip budget up to 79, but DP uses indices up to m+1 (<=80)
    const INF: i32 = 2_100_000_000;

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Per spec, inputs fit within fixed limits; we do not add extra error handling.
        debug_assert!(n > 0 && n <= MAX_N);
        debug_assert!(m <= MAX_M);

        // Read stages/options
        let mut ic = vec![0usize; n];
        let mut a = vec![vec![0i32; MAX_OPT]; n];
        let mut b = vec![vec![0i32; MAX_OPT]; n];

        for i in 0..n {
            let cnt: usize = it.next().unwrap().parse().unwrap();
            ic[i] = cnt;
            debug_assert!(cnt >= 1 && cnt <= MAX_OPT);
            for j in 0..cnt {
                a[i][j] = it.next().unwrap().parse().unwrap();
                b[i][j] = it.next().unwrap().parse().unwrap();
            }
        }

        // DP: c[skips_used][stage][option] = min cost
        // C code uses dimensions [80][150][11] and initializes all j>=1 to INF, leaving stage 0 as 0.
        // We mirror that behavior: stage 0 costs start at 0 for all option indices (even unused).
        let mut c = vec![vec![vec![0i32; 11]; MAX_N]; 80];
        for i in 0..80 {
            for j in 1..MAX_N {
                for k in 0..11 {
                    c[i][j][k] = INF;
                }
            }
        }
        // C code also sets c[1][1][i]=0, but this is redundant given stage 0 is 0 and transitions exist.
        // We keep it to preserve semantics exactly.
        for i in 0..11 {
            c[1][1][i] = 0;
        }

        // Transitions
        for skips in 0..=m {
            for stage in 0..(n.saturating_sub(1)) {
                for k in 0..ic[stage] {
                    let cur = c[skips][stage][k];
                    // 1-step move to stage+1
                    for l in 0..ic[stage + 1] {
                        let d = (b[stage][k] + b[stage + 1][l])
                            * (a[stage][k] - a[stage + 1][l]).abs();
                        let cand = cur + d;
                        if c[skips][stage + 1][l] > cand {
                            c[skips][stage + 1][l] = cand;
                        }
                    }

                    // 2-step move to stage+2 (uses one skip), if within bounds
                    if stage + 2 == n {
                        continue;
                    }
                    for l in 0..ic[stage + 2] {
                        let d = (b[stage][k] + b[stage + 2][l])
                            * (a[stage][k] - a[stage + 2][l]).abs();
                        let cand = cur + d;
                        if c[skips + 1][stage + 2][l] > cand {
                            c[skips + 1][stage + 2][l] = cand;
                        }
                    }
                }
            }
        }

        // Result: min cost ending at stage n-1, or at stage n-2 if skips_used < m
        // Note: This matches the spec. (Also matches the C code.)
        let mut ans: i32 = 2_100_000_210; // matches C's initial min
        for skips in 0..=m {
            for j in 0..ic[n - 1] {
                let v = c[skips][n - 1][j];
                if ans > v {
                    ans = v;
                }
            }
            if skips == m {
                continue;
            }
            if n >= 2 {
                for j in 0..ic[n - 2] {
                    let v = c[skips][n - 2][j];
                    if ans > v {
                        ans = v;
                    }
                }
            }
        }

        println!("{ans}");
    }
}
