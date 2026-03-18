use std::io::{self, Read};

const SUP: i64 = 1i64 << 60;

fn recursion(l: usize, r: usize, dp: &mut Vec<Vec<Option<i64>>>, sum: &[i64]) -> i64 {
    if l == r {
        // Cost to "merge" a single element is 0.
        return 0;
    }
    if let Some(v) = dp[l][r] {
        return v;
    }

    let mut best = SUP;
    for k in (l + 1)..=r {
        let tmp = recursion(l, k - 1, dp, sum) + recursion(k, r, dp, sum);
        if tmp < best {
            best = tmp;
        }
    }

    // Add the cost of merging the two parts: sum of elements in [l..=r].
    best += sum[r] - sum[l - 1];
    dp[l][r] = Some(best);
    best
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // No input; unspecified behavior in spec, so just exit.
    };

    // 1-based arrays to match the C code indexing.
    let mut a = vec![0i64; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    let mut sum = vec![0i64; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i];
    }

    // dp[l][r] stores computed minimum cost for subarray [l..=r].
    // Use Option to avoid the C code's "0 means uncomputed" ambiguity (important with negatives).
    let mut dp: Vec<Vec<Option<i64>>> = vec![vec![None; n + 1]; n + 1];

    let ans = recursion(1, n, &mut dp, &sum);
    print!("{ans}\n");
}
