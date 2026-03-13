// max_subarray.rs
// Translated from the provided C implementation, following the given specification.
// Preserves behavior including the INT_MIN sentinel of -1_000_000_000.
// Uses safe, idiomatic Rust (no unsafe).

use std::io::{self, Read};

const INT_MIN_SENTINEL: i32 = -1_000_000_000;

fn main() {
    // Read all input tokens as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();
    // Loop over test cases until a test case with n == 0 is encountered.
    loop {
        let n_token = match tokens.next() {
            Some(t) => t,
            None => break, // no more input
        };
        let n: i32 = n_token.parse().unwrap(); // spec assumes valid integers
        if n == 0 {
            break; // termination condition
        }

        let n_usize = n as usize;
        // Read n integers for this test case.
        let mut a: Vec<i32> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            // Spec assumes enough tokens; unwrap is consistent with that assumption.
            let v: i32 = tokens.next().unwrap().parse().unwrap();
            a.push(v);
        }

        // Auxiliary arrays left and right, initialized to 0.
        let mut left = vec![0i32; n_usize];
        let mut right = vec![0i32; n_usize];

        if n_usize > 0 {
            // left[i] = max(0, left[i-1] + a[i-1]) with left[0] = 0
            left[0] = 0;
            for i in 1..n_usize {
                let sum = left[i - 1] + a[i - 1];
                left[i] = if sum > 0 { sum } else { 0 };
            }

            // right[i] = max(0, right[i+1] + a[i+1]) with right[n-1] = 0
            right[n_usize - 1] = 0;
            if n_usize >= 2 {
                for i in (0..n_usize - 1).rev() {
                    let sum = right[i + 1] + a[i + 1];
                    right[i] = if sum > 0 { sum } else { 0 };
                }
            }
        }

        // Compute answer: max over i of left[i] + a[i] + right[i]
        let mut ans: i32 = INT_MIN_SENTINEL;
        for i in 0..n_usize {
            let total = left[i] + a[i] + right[i];
            if total > ans {
                ans = total;
            }
        }

        // Output one line per test case: "<integer>\n"
        println!("{}", ans);
    }
}
