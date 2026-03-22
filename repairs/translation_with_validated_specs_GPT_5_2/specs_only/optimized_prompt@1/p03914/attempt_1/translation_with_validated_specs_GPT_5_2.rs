use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX: usize = 301;

fn main() {
    // No explicit error handling per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap(); // size parameter
    let k: i64 = it.next().unwrap().parse().unwrap(); // step count

    // Fixed-size memory per spec; behavior is undefined if inputs exceed bounds.
    let mut dp = vec![vec![0i64; MAX + 2]; MAX + 2];
    let mut next = vec![vec![0i64; MAX + 2]; MAX + 2];

    // Initialization: state (1,1) = 1.
    dp[1][1] = 1;

    // Perform exactly k steps.
    for _step in 0..k {
        // Clear next table.
        for a in 0..=MAX + 1 {
            for b in 0..=MAX + 1 {
                next[a][b] = 0;
            }
        }

        // Iterate over all possible states within fixed bounds.
        for a in 1..=MAX {
            for b in 1..=a {
                let cur = dp[a][b];
                if cur == 0 {
                    continue;
                }

                // Multiplicities determined by (size - a), b, and (a - b).
                // Spec assumes no 64-bit signed overflow; we keep i64 arithmetic.
                let a_i = a as i64;
                let b_i = b as i64;

                let m1 = n - a_i;      // to (a+1, b)
                let m2 = b_i;          // to (a, a)
                let m3 = a_i - b_i;    // to (a, b)

                // Successor 1: increase first component by 1, keep second unchanged.
                // No bounds checks beyond fixed table; out-of-range is undefined by spec.
                if a + 1 <= MAX {
                    let add = (cur * (m1 % MOD)) % MOD;
                    next[a + 1][b] = (next[a + 1][b] + add) % MOD;
                } else {
                    // If it would go out of the fixed table, we drop it to keep memory safe.
                    // Spec says behavior would be undefined; this is the narrowest safe choice.
                }

                // Successor 2: set second component equal to the first component (a, a).
                {
                    let add = (cur * (m2 % MOD)) % MOD;
                    next[a][a] = (next[a][a] + add) % MOD;
                }

                // Successor 3: keep both components unchanged (a, b).
                {
                    let add = (cur * (m3 % MOD)) % MOD;
                    next[a][b] = (next[a][b] + add) % MOD;
                }
            }
        }

        std::mem::swap(&mut dp, &mut next);
    }

    // Output: state (size, size) after all steps, modulo MOD.
    // If unreachable, it remains 0.
    let ni = n as usize;
    let ans = if ni <= MAX { dp[ni][ni].rem_euclid(MOD) } else { 0 };
    println!("{}", ans);
}
