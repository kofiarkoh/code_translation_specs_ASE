use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, parse two integers like scanf("%d %d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse::<i32>().unwrap() as i64;
    let m: i32 = it.next().unwrap().parse::<i32>().unwrap();

    // C: long long dp[302][302] = {};
    // Use i64; keep arithmetic in i64. This matches typical C long long behavior.
    // Note: C code can write dp[i+1][j] with i up to M, so indices may exceed 301
    // if inputs are large; that's undefined in C. We allocate a bit larger to avoid
    // panics for moderately larger inputs while preserving intended semantics.
    let size: usize = 305;
    let mut dp = vec![vec![0i64; size]; size];

    dp[1][1] = 1;

    for k in 1..=m {
        // for (i = k; i >= 1; i--)
        let mut i = k;
        while i >= 1 {
            // for (j = i; j >= 1; j--)
            let mut j = i;
            while j >= 1 {
                let iu = i as usize;
                let ju = j as usize;

                let tmp = dp[iu][ju] % MOD;
                dp[iu][ju] = 0;

                // dp[i+1][j] += tmp * (N - i);
                // dp[i][i] += tmp * j;
                // dp[i][j] += tmp * (i - j);
                // Keep i/j as i64 for arithmetic.
                let ii = i as i64;
                let jj = j as i64;

                let idx_i1 = (i + 1) as usize;
                if idx_i1 < size && ju < size {
                    dp[idx_i1][ju] = dp[idx_i1][ju].wrapping_add(tmp.wrapping_mul(n.wrapping_sub(ii)));
                }
                if iu < size && iu < size {
                    dp[iu][iu] = dp[iu][iu].wrapping_add(tmp.wrapping_mul(jj));
                }
                if iu < size && ju < size {
                    dp[iu][ju] = dp[iu][ju].wrapping_add(tmp.wrapping_mul(ii.wrapping_sub(jj)));
                }

                j -= 1;
            }
            i -= 1;
        }
    }

    let nn = n as usize;
    let ans = if nn < size { dp[nn][nn] % MOD } else { 0 };
    print!("{ans}\n");
    // Rust stdout is flushed on program exit; explicit flush not required to match output.
}
