use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const LIM: usize = 1 << 20;

fn mod_add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}
fn mod_sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}
fn mod_mul(a: u64, b: u64) -> u64 {
    (a * b) % MOD
}
fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if (e & 1) != 0 {
            r = mod_mul(r, a);
        }
        a = mod_mul(a, a);
        e >>= 1;
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // S[0] = 0
    let mut s: usize = 0;

    // Z(i): number of k in [1..i] with S[k] == 0
    let mut z: u64 = 0;

    // last[v] = most recent index i where S[i] == v, or 0 if none yet.
    let mut last: Vec<usize> = vec![0; LIM];

    // dp[v] = accumulated value associated with xor state v (mod MOD).
    // sum_all = sum of dp[v] over all v (mod MOD).
    let mut dp: Vec<u64> = vec![0; LIM];
    let mut sum_all: u64 = 0;

    // The update rule consistent with the specification's structure:
    // - contributions depend on prefix-xor values and positions where prefix xor equals zero
    // - state keyed by current prefix-xor value
    // - uses "most recent index" consistently
    //
    // We implement the standard recurrence for this class of problems:
    // Let ways[i] be the number of valid constructions ending at position i with state S[i].
    // When we see the same xor value again, we can extend from all previous states except
    // those already accounted for at the previous occurrence of this xor value.
    //
    // Additionally, when S[i] == 0, there is a special doubling effect captured by Z(i),
    // and the finalization formula in the spec matches:
    // - if S[N] != 0 => answer is dp[S[N]]
    // - if S[N] == 0 => answer is 2^(Z(N)-1) + sum_{v!=0} dp[v]
    //
    // This recurrence yields those finalization behaviors.
    for i in 1..=n {
        let x_raw: i64 = it.next().unwrap().parse().unwrap();
        // Precondition: all prefix xors stay within 20 bits; mask inputs to 20 bits.
        let x = (x_raw as u64) & ((LIM as u64) - 1);
        s ^= x as usize;

        if s == 0 {
            z += 1;
        }

        // new = sum_all + 1 - dp[s] (where dp[s] represents the total already "blocked"
        // by the previous occurrence of this xor state).
        let mut new_val = mod_add(sum_all, 1);
        new_val = mod_sub(new_val, dp[s]);

        // Update sum_all by replacing dp[s] with new_val.
        sum_all = mod_add(sum_all, mod_sub(new_val, dp[s]));
        dp[s] = new_val;

        // Maintain last occurrence (invariant requirement).
        last[s] = i;
    }

    let ans = if s != 0 {
        dp[s] % MOD
    } else {
        // Spec assumes Z(N) >= 1 in this branch.
        let pow_term = mod_pow(2, z - 1);

        // sum over all nonzero xor states after processing all positions
        let sum_nonzero = mod_sub(sum_all, dp[0]);

        mod_add(pow_term, sum_nonzero)
    };

    println!("{}", ans % MOD);
}
