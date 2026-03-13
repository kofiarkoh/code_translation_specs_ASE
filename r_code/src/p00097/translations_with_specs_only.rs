use std::io::{self, Read};

fn main() {
    // Constants from the specification
    const MAX_N: usize = 9;
    const MAX_VALUE: usize = 100;
    const MAX_SUM: usize = 864; // sum of the largest 9 numbers 92..100

    // dp[k][s] = number of ways to choose k distinct integers from [0..100]
    // (order enforced as strictly increasing) that sum to s.
    // We use u64 which is sufficient for the combinatorial counts in these bounds.
    let mut dp = [[0u64; MAX_SUM + 1]; MAX_N + 1];
    dp[0][0] = 1;

    // Classic subset-sum / combination DP: for each value v in 0..=100,
    // consider taking it (increase k by 1 and sum by v). We iterate k and s
    // downward to avoid reusing the same value multiple times.
    for v in 0..=MAX_VALUE {
        // iterate k from MAX_N down to 1
        for k in (1..=MAX_N).rev() {
            // iterate sums from MAX_SUM down to v
            // v as usize is already
            let v_usize = v;
            for s in (v_usize..=MAX_SUM).rev() {
                dp[k][s] = dp[k][s].wrapping_add(dp[k - 1][s - v_usize]);
            }
        }
    }

    // Read all input, parse integers
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, just exit (spec allows EOF / malformed input to end execution).
        return;
    }

    let mut iter = input.split_whitespace();
    loop {
        // Try to read next two integers (n, s). If not available, exit.
        let n_token = match iter.next() {
            Some(t) => t,
            None => break,
        };
        let s_token = match iter.next() {
            Some(t) => t,
            None => break,
        };

        // Parse as signed integers to follow spec which allows negative inputs (undefined),
        // but we handle parsing gracefully.
        let n_parsed = n_token.parse::<i32>();
        let s_parsed = s_token.parse::<i32>();
        if n_parsed.is_err() || s_parsed.is_err() {
            // Malformed input -> end processing (spec leaves this unspecified; we stop).
            break;
        }
        let n = n_parsed.unwrap();
        let s = s_parsed.unwrap();

        if n == 0 {
            // Termination condition: do not produce output for this query.
            break;
        }

        // If s > MAX_SUM and n > 0, spec requires printing "0".
        if s as i64 > MAX_SUM as i64 {
            println!("0");
            continue;
        }

        // For defined behavior, 1 <= n <= 9 and 0 <= s <= 864.
        // The specification states behavior is undefined for other n or negative s.
        // To keep the program robust, we treat out-of-range n or negative s as printing 0.
        // This choice avoids panics while staying close to the specification's intent.
        if n < 1 || n as usize > MAX_N || s < 0 {
            println!("0");
            continue;
        }

        // Safe to index now.
        let n_usize = n as usize;
        let s_usize = s as usize;
        let ans = dp[n_usize][s_usize];
        println!("{}", ans);
    }
}
