use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code loops: while (scanf("%d%d", &n, &m), n) { ... }
    // i.e., stop when n == 0 (even if more input exists).
    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break, // malformed/truncated input: spec allows undefined; we just stop.
        };

        if n == 0 {
            break;
        }

        let mut c: Vec<i32> = Vec::with_capacity(m);
        for _ in 0..m {
            let v: i32 = it.next().unwrap().parse().unwrap();
            c.push(v);
        }

        // dp rows are only needed for i and i+1; C uses a huge static dp[20001][256]
        // initialized to -1. We preserve semantics with rolling arrays.
        let mut prev = vec![-1i32; 256];
        let mut curr = vec![-1i32; 256];
        prev[128] = 0;

        for _ in 0..n {
            let x: i32 = it.next().unwrap().parse().unwrap();

            // reset curr to -1 (equivalent to memset for next row)
            curr.fill(-1);

            for j in 0..256 {
                let pj = prev[j];
                if pj >= 0 {
                    let j_i32 = j as i32;
                    for &ck in &c {
                        let mut t = j_i32 + ck;
                        if t < 0 {
                            t = 0;
                        } else if t > 255 {
                            t = 255;
                        }
                        let diff = x - t;
                        let y = pj + diff * diff;
                        let ti = t as usize;
                        if curr[ti] < 0 {
                            curr[ti] = y;
                        } else if y < curr[ti] {
                            curr[ti] = y;
                        }
                    }
                }
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        // If no valid transformation exists, C prints 1<<30 (1073741824).
        // Note: This matches the C behavior even though the spec says "large constant".
        let mut res: i32 = 1 << 30;
        for &v in &prev {
            if v >= 0 && v < res {
                res = v;
            }
        }

        println!("{res}");
    }
}
