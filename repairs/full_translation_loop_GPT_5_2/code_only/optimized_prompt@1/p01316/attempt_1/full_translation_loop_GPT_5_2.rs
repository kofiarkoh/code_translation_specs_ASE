use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // dp[20001][256] in C, initialized to -1 each test case.
    // Use i32 to match C int behavior on typical platforms; values stay well within i32.
    let mut dp = vec![[ -1i32; 256 ]; 20001];
    let mut c = [0i32; 16];

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

        for i in 0..m {
            c[i] = it.next().unwrap().parse().unwrap();
        }

        // memset(dp, -1, sizeof(dp));
        for row in dp.iter_mut() {
            row.fill(-1);
        }

        dp[0][128] = 0;

        for i in 0..n {
            let x: i32 = it.next().unwrap().parse().unwrap();
            let prev = dp[i]; // copy of the row (256 ints)
            let curr = &mut dp[i + 1];
            // In C, dp was memset to -1, so curr starts at -1 for each i+1 row.
            // We keep that invariant because dp was fully reset before the test case.

            for j in 0..256 {
                let pj = prev[j];
                if pj >= 0 {
                    for k in 0..m {
                        let mut t = j as i32 + c[k];
                        if t < 0 {
                            t = 0;
                        } else if t > 255 {
                            t = 255;
                        }
                        let t_usize = t as usize;

                        // f(x, t) = (x - t)^2
                        let d = x - t;
                        let y = pj + d * d;

                        let curv = curr[t_usize];
                        curr[t_usize] = if curv < 0 { y } else { curv.min(y) };
                    }
                }
            }
        }

        let curr = &dp[n];
        let mut res: i32 = 1 << 30;
        for i in 0..256 {
            let v = curr[i];
            if v >= 0 {
                res = res.min(v);
            }
        }

        // Exact output formatting: one integer and newline.
        println!("{res}");
    }
}
