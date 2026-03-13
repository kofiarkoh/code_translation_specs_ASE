// Translated from the given C implementation, following the provided SPEC.
// Computes number of strictly increasing sequences of length n (1..9)
// with elements in [0,100] summing to s. Reads pairs (n, s) from stdin,
// stops when n == 0. For n > 0 and s > 864 prints "0".
//
// Implementation notes:
// - Uses a 3D table p[len][last_value][sum] where:
//     len in 0..=9 (we use indices 0..9, but only 1..9 populated/used)
//     last_value in 0..=100
//     sum in 0..=901 (to match the C array of size 902)
// - Counts are stored in i64 to avoid any potential 32-bit overflow; the
//   original C code used 'int'. Using i64 is safe and preserves semantics
//   for all valid inputs per spec.
// - For inputs outside the spec (e.g., n < 0 or n > 9, s < 0) the program
//   prints 0 for robustness (the SPEC states behavior for such inputs is undefined).
use std::io::{self, Read};

fn main() {
    const MAX_N: usize = 9;
    const MAX_VAL: usize = 100;
    const SUM_SIZE: usize = 902; // matches the C code's 902 dimension

    // Allocate p as [10][101][902] of i64
    let mut p = vec![vec![vec![0i64; SUM_SIZE]; MAX_VAL + 1]; MAX_N + 1];

    // Base: sequences of length 1
    for i in 0..=MAX_VAL {
        // sum == i
        p[1][i][i] = 1;
    }

    // Build up for n = 1..8 producing n+1 up to 9
    // Mirrors the C loops: min starts 0, max 100 and are updated each iteration
    let mut min = 0usize;
    let mut mut_max = 100usize;
    let mut n = 1usize;
    while n < 9 {
        // for each possible previous last value i, and sum j in [min..mut_max]
        for i in 0..=MAX_VAL {
            for j in min..=mut_max {
                let cnt = p[n][i][j];
                if cnt == 0 {
                    continue;
                }
                // choose next value as i + k where k >= 1 and i + k <= 100
                // so k from 1..=(100 - i)
                for k in 1..=(MAX_VAL - i) {
                    let next_val = i + k;
                    let next_sum = j + next_val;
                    // next_sum will be <= mut_max + 100 <= 900 in the final iteration,
                    // and SUM_SIZE (902) is large enough to hold it.
                    if next_sum < SUM_SIZE {
                        p[n + 1][next_val][next_sum] += cnt;
                    }
                }
            }
        }

        // increment for next iteration
        n += 1;
        min += 1;
        mut_max += 100;
    }

    // Read all input tokens (whitespace separated integers)
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    loop {
        let n_token = tokens.next();
        if n_token.is_none() {
            break; // EOF
        }
        let s_token = tokens.next();
        if s_token.is_none() {
            break; // malformed/incomplete pair -> stop as SPEC allows EOF termination
        }

        // Parse integers, if parse fails stop processing (spec: behavior on malformed input not specified)
        let n_parsed: i32 = match n_token.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let s_parsed: i32 = match s_token.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n_parsed == 0 {
            break; // termination condition
        }

        // For robustness, handle out-of-spec cases by printing 0:
        // - s > 864 -> print 0 (spec explicitly requires this)
        // - s < 0 or s index out of our arrays -> print 0
        // - n not in 1..9 -> print 0 (SPEC marks behavior undefined; we choose safe handling)
        if s_parsed > 864 || s_parsed < 0 || n_parsed < 1 || n_parsed as usize > MAX_N {
            println!("0");
            continue;
        }

        let n_usize = n_parsed as usize;
        let s_usize = s_parsed as usize;
        if s_usize >= SUM_SIZE {
            // Shouldn't happen given earlier check, but guard anyway
            println!("0");
            continue;
        }

        let mut ans: i64 = 0;
        for i in 0..=MAX_VAL {
            ans += p[n_usize][i][s_usize];
        }

        println!("{}", ans);
    }
}
