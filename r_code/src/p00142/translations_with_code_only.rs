// translate_c_to_rust.rs
// Translated from the provided C program.
// Preserves the C program's observable behavior for typical positive n inputs.
// Uses safe, idiomatic Rust. No `unsafe` needed.

use std::io::{self, Read};

fn main() {
    // Read all input from stdin, parse whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>());

    // Process each integer token in order. Stop on 0 or on parse failure (EOF).
    while let Some(Ok(n_i32)) = iter.next() {
        let n = n_i32;
        if n == 0 {
            break;
        }
        // Only positive n are meaningfully handled as in the original program.
        // (The original C code also effectively expects positive n.)
        if n <= 0 {
            // For non-positive n, mimic the C program's lack of output for such cases:
            // do nothing and continue to next token.
            continue;
        }
        let n_usize = n as usize;

        // Build the list `a` of unique residues exp[i] % n for i = 1 .. n-1,
        // keeping the first occurrence order (linear search like the C code).
        let mut a: Vec<i32> = Vec::new();

        for i in 1..n_usize {
            // Compute i*i using wrapping multiplication to emulate potential C int wrap behavior.
            let sq = (i as i32).wrapping_mul(i as i32);
            // C's `%` with positive divisor yields a non-negative remainder for positive operands.
            let m = sq % n; // n is positive here
            // Linear search for existence (preserve same ordering semantics as original C).
            if !a.iter().any(|&x| x == m) {
                a.push(m);
            }
        }

        let c = a.len();
        let h = ((n - 1) / 2) as usize;

        // Allocate count array of length n (to safely hold any index the original code could write),
        // initialized to zeros. The original C used a fixed size array and wrote to indices up to
        // potentially n/2; we allocate n to be safe and preserve observed behavior.
        let mut count = vec![0i32; n_usize];

        // For each ordered pair (i, j) with a[i] != a[j], compute the minimal circular distance
        // and increment count[d]. This mirrors the nested loops in the C code.
        for i in 0..c {
            for j in 0..c {
                if a[i] != a[j] {
                    // compute d = a[i] - a[j] (mod n) in range 0..n-1
                    let mut d = a[i] - a[j];
                    if d < 0 {
                        d += n;
                    }
                    // map to minimal distance around the circle as in the C code
                    if d as usize > h {
                        d = n - d;
                    }
                    // increment count[d]
                    let idx = d as usize;
                    // idx should be < n_usize due to construction; safe to index
                    count[idx] += 1;
                }
            }
        }

        // Print count[1..=h], each on its own line, preserving formatting exactly.
        for i in 1..=h {
            println!("{}", count[i]);
        }
    }
}
