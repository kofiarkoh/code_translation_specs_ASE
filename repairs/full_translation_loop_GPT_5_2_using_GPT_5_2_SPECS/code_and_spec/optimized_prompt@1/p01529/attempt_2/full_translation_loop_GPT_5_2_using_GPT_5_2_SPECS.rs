use std::io::{self, Read};

fn main() {
    // Read all input; tokens are non-negative integers separated by non-digits.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    fn next_nonneg_i64(bytes: &[u8], idx: &mut usize) -> i64 {
        while *idx < bytes.len() && !(b'0'..=b'9').contains(&bytes[*idx]) {
            *idx += 1;
        }
        let mut n: i64 = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if (b'0'..=b'9').contains(&c) {
                n = n * 10 + (c - b'0') as i64;
                *idx += 1;
            } else {
                break;
            }
        }
        // Unlike the previous attempt, do NOT forcibly consume one delimiter here.
        // The C code does `p++` because it assumes exactly one delimiter; in Rust we
        // instead let the next call skip any amount of delimiters safely.
        n
    }

    let n = next_nonneg_i64(bytes, &mut idx) as usize;

    if n <= 1 {
        // Spec: for n = 1, output 0. (n=0 is outside preconditions; we also output 0.)
        print!("0\n");
        return;
    }

    // Prefix sums w[i] = sum(weights[0..=i])
    let mut w = vec![0i64; n];
    w[0] = next_nonneg_i64(bytes, &mut idx);
    for i in 1..n {
        w[i] = w[i - 1] + next_nonneg_i64(bytes, &mut idx);
    }

    // dp[i][j] = min cost to merge i..=j
    // k[i][j] = argmin split point (Knuth optimization)
    let inf: i64 = 0x3333_3333_3333_3333i64;
    let mut dp = vec![vec![inf; n]; n];
    let mut k = vec![vec![0usize; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // Compute by increasing interval length (len = 2..=n)
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;

            // Knuth optimization bounds:
            // opt[i][j] in [opt[i][j-1], opt[i+1][j]]
            let mut b = k[i][j - 1];
            let b_end = k[i + 1][j];

            while b <= b_end {
                let mut cost = dp[i][b] + dp[b + 1][j] + w[j];
                if i > 0 {
                    cost -= w[i - 1];
                }
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                    k[i][j] = b;
                }
                b += 1;
            }
        }
    }

    print!("{}\n", dp[0][n - 1]);
}
