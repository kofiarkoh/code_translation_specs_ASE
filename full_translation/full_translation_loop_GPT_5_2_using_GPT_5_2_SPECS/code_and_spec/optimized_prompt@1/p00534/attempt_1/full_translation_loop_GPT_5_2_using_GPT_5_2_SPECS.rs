use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // 1-based arrays to match the C code indexing.
    let mut k = vec![0i32; n + 1];
    let mut t = vec![0i32; m + 1];

    for i in 1..=n {
        k[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 1..=m {
        t[i] = it.next().unwrap().parse().unwrap();
    }

    // DP table hi[i][j] = minimum cost selecting j items from first i candidates.
    // Sentinel is fixed at 1_000_000_000 as in the C code/spec.
    let inf: i32 = 1_000_000_000;
    let mut hi = vec![vec![inf; n + 1]; m + 1];

    // Invariant: selecting 0 items costs 0 for any prefix length i.
    for i in 0..=m {
        hi[i][0] = 0;
    }

    for i in 1..=m {
        for j in 1..=n {
            // Choose candidate i for item j:
            // cost = k[j] * t[i] + hi[i-1][j-1]
            // (Assumed not to overflow per spec.)
            let choose = k[j] * t[i] + hi[i - 1][j - 1];

            // Or skip candidate i:
            let skip = hi[i - 1][j];

            hi[i][j] = if choose > skip { skip } else { choose };
        }
    }

    print!("{}\n", hi[m][n]);
}
