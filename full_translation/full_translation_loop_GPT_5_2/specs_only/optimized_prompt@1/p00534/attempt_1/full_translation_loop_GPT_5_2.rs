use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say n,m >= 1; otherwise behavior undefined.
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let mut b: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        b.push(it.next().unwrap().parse().unwrap());
    }

    // Cost matrix initialized to a high value.
    // Use i128 to reduce overflow risk when multiplying values and indices.
    // The specification does not define overflow behavior; this is the narrowest
    // reasonable choice that preserves intended arithmetic for typical inputs.
    let inf: i128 = i128::MAX / 4;
    let mut dp = vec![vec![inf; m + 1]; n + 1];

    // Invariant: first column always zero (base case).
    for i in 0..=n {
        dp[i][0] = 0;
    }

    // State transitions: update cost matrix based on previous states.
    //
    // Interpretation (narrow, consistent with description):
    // - We must end at the last elements of both sequences (i = n, j = m).
    // - We can advance through A and B; matching A[i] with B[j] incurs a cost
    //   derived from the product of elements and their respective indices.
    // - Skipping elements in A is allowed (advance i without consuming j) at no cost,
    //   consistent with dp[*][0] = 0 and typical "choose a subsequence" DP.
    //
    // Transition:
    // dp[i][j] = min(
    //   dp[i-1][j],                                  // skip a[i-1]
    //   dp[i-1][j-1] + a[i-1]*b[j-1]*i*j             // match a[i-1] with b[j-1]
    // )
    //
    // Indices are 1-based in the cost term (i and j), as "respective indices".
    for i in 1..=n {
        for j in 1..=m {
            let skip = dp[i - 1][j];

            let ai = a[i - 1] as i128;
            let bj = b[j - 1] as i128;
            let ii = i as i128;
            let jj = j as i128;

            let add = ai * bj * ii * jj;
            let take = dp[i - 1][j - 1].saturating_add(add);

            dp[i][j] = if skip < take { skip } else { take };
        }
    }

    // Output: minimum cost associated with the last elements of both sequences.
    // (i.e., dp[n][m]).
    println!("{}", dp[n][m]);
}
