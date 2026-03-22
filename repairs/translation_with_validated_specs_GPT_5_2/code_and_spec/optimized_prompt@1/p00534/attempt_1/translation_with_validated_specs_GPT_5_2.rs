use std::io::{self, Read};

fn main() {
    // Read all input (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // 1-based arrays to mirror the C code indexing.
    let mut k = vec![0i32; n + 1];
    for i in 1..=n {
        k[i] = it.next().unwrap().parse().unwrap();
    }

    let mut t = vec![0i32; m + 1];
    for i in 1..=m {
        t[i] = it.next().unwrap().parse().unwrap();
    }

    // DP table hi[i][j] for i in 0..=m, j in 0..=n.
    // Sentinel value matches C code.
    let inf: i32 = 1_000_000_000;
    let mut hi = vec![vec![inf; n + 1]; m + 1];

    // Base case: cost 0 to assign 0 items regardless of i.
    for i in 0..=m {
        hi[i][0] = 0;
    }

    // Transition:
    // hi[i][j] = min( k[j]*t[i] + hi[i-1][j-1], hi[i-1][j] )
    // Uses i32 arithmetic like C; overflow behavior is unspecified by the spec.
    for i in 1..=m {
        for j in 1..=n {
            let take = k[j].wrapping_mul(t[i]).wrapping_add(hi[i - 1][j - 1]);
            let skip = hi[i - 1][j];
            hi[i][j] = if take > skip { skip } else { take };
        }
    }

    println!("{}", hi[m][n]);
}
