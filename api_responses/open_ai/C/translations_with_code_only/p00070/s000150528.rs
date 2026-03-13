// Translated from the provided C program.
// Preserves semantics of the computation: counts permutations of the initial
// num array [1,2,3,4,5,6,7,8,9,0] accumulating weighted sums and filling dp.
//
// Notes on choices:
// - The original C program used plain `int` and a recursive permutation routine.
//   We use i32 for values and recursion up to depth 10 (safe).
// - The original `main` loop relied on scanf("%d %d") until EOF. Here we parse
//   whitespace-separated integers from stdin and process them in pairs. If the
//   input ends with an incomplete pair, that final lone value is ignored.
// - The C code only checked upper bounds (n <= 10 && s <= 330) before indexing
//   dp; indexing with negative values in C is undefined. To avoid undefined
//   behavior in Rust we require 0 <= n <= 10 and 0 <= s <= 330. This matches
//   the typical intended use and keeps behavior well-defined.
//
// The program prints one integer per line corresponding to dp[n][s] or 0 when
// out of allowed range, matching the original output format.

use std::io::{self, Read};

fn solve(k: usize, s: i32, num: &mut [i32; 10], dp: &mut [[i32; 331]; 11]) {
    for i in k..10 {
        num.swap(i, k);

        let ts = s + num[k] * (k as i32 + 1);
        // ts should be non-negative and <= 330 for valid counts; we assume
        // the problem domain keeps it within 0..=330 as in the original.
        if (0..=330).contains(&ts) {
            dp[k + 1][ts as usize] += 1;
        }
        if k < 9 {
            solve(k + 1, ts, num, dp);
        }

        num.swap(i, k);
    }
}

fn main() {
    // Initialize as in C: num = {1,2,3,4,5,6,7,8,9,0}
    let mut num: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let mut dp: [[i32; 331]; 11] = [[0; 331]; 11];

    solve(0, 0, &mut num, &mut dp);

    // Read all stdin, parse whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut nums = Vec::new();
    for tok in input.split_whitespace() {
        if let Ok(v) = tok.parse::<i32>() {
            nums.push(v);
        }
    }

    // Process pairs (n, s). If there's an incomplete final value, ignore it.
    let mut i = 0;
    while i + 1 < nums.len() {
        let n = nums[i];
        let s = nums[i + 1];

        // To avoid undefined behavior from negative indices (which the C code
        // could invoke), require non-negative indices as well as the original
        // upper bounds. If out of range, print 0.
        let out = if (0..=10).contains(&n) && (0..=330).contains(&s) {
            dp[n as usize][s as usize]
        } else {
            0
        };
        println!("{}", out);

        i += 2;
    }
}
