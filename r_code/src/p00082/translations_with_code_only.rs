// Translated from the provided C program.
// Notes:
// - The original C code used uninitialized local variables (curS, minP) which is undefined behavior.
//   The apparent intent is to start with shift 0 as the baseline; therefore cur_s is initialized to 0
//   before the first accumulation and min_p is initialized to 0. This preserves the likely intended
//   semantics while avoiding UB in Rust.
// - I/O behavior (whitespace, newlines) is preserved: for each group of 8 input integers, the program
//   prints 8 numbers separated by single spaces and a terminating newline.

use std::io::{self, Read};

fn main() {
    // c as in the C program: pattern of length 16 (repeats every 8)
    let c: [i32; 16] = [
        1, 2, 1, 2, 1, 4, 1, 4,
        1, 2, 1, 2, 1, 4, 1, 4,
    ];

    // Read all whitespace-separated tokens from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>());

    // Process in groups of 8 integers
    loop {
        // Try to read 8 integers; if any parse fails or EOF, stop
        let mut p = [0i32; 8];
        let mut ok = true;
        for i in 0..8 {
            match tokens.next() {
                Some(Ok(v)) => p[i] = v,
                _ => { ok = false; break; }
            }
        }
        if !ok {
            break;
        }

        // Compute initial curS for shift 0
        let mut cur_s: i32 = 0; // initialized to 0 to avoid UB in C version
        for j in 0..8 {
            let diff = p[j] - c[j];
            if diff > 0 {
                cur_s += diff;
            }
        }

        let mut min_s = cur_s;
        let mut min_p: usize = 0; // initialize to 0 (C code left this uninitialized)

        // The specific iteration order from the C code:
        let order = [1usize, 7, 3, 5, 8, 6, 2, 4];
        for &i in &order {
            cur_s = 0;
            for j in 0..8 {
                let idx = i + j; // safe: i in 1..8 and j in 0..7 so idx in 1..15
                let diff = p[j] - c[idx];
                if diff > 0 {
                    cur_s += diff;
                }
            }
            if cur_s < min_s {
                min_s = cur_s;
                min_p = i;
            }
        }

        // Print c[min_p .. min_p+8] with single spaces and a trailing newline
        for offset in 0..8 {
            if offset > 0 {
                print!(" ");
            }
            print!("{}", c[min_p + offset]);
        }
        println!();
    }
}
