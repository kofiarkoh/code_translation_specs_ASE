// translate_merge.rs
//
// Translation of the provided C program into safe, idiomatic Rust,
// following the SPEC as the source of truth.
//
// Notes on choices made to preserve semantics from the SPEC:
// - Selection rule implemented exactly: choose j maximizing key1 = c2[j]*c1[j+1],
//   tie-breaking by minimal key2 = c1[j]*c2[j+1].
// - To avoid undefined behaviour present in the C code (uninitialized index when
//   no candidate sets amaxi), the first adjacent index (j = 0) is used to
//   initialize the "best" candidate for each merge step. This matches the
//   SPEC's intended deterministic selection rule.
// - Integer arithmetic uses wrapping operations (i32::wrapping_mul / wrapping_add)
//   to mirror typical C behavior on two's-complement platforms and to avoid
//   debug-mode panics on overflow in Rust. The SPEC documents that overflow
//   is undefined for the mathematical result; using wrapping preserves a
//   well-defined runtime behavior consistent with common C implementations.
// - Input parsing is whitespace-separated like scanf; if parsing fails or if
//   there are not enough integers, the program exits with a non-zero code.
//   The SPEC treats malformed input as undefined; here we fail fast instead.
//
// I/O: prints a single integer followed by a newline (the accumulated sum s).

use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, exit non-zero (input error).
        std::process::exit(1);
    }

    // Parse tokens into i32 values.
    let mut tokens = input.split_whitespace().map(|w| w.parse::<i32>());

    // Parse n
    let n = match tokens.next() {
        Some(Ok(v)) => v,
        _ => {
            // malformed input
            std::process::exit(1);
        }
    };

    // According to SPEC, valid n is 0..=100. Out-of-range is undefined in C;
    // here we detect and exit non-zero to avoid unexpected memory behavior.
    if n < 0 || n > 100 {
        std::process::exit(1);
    }
    let n_usize = n as usize;

    // Read n pairs
    let mut pairs: Vec<(i32, i32)> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        let a = match tokens.next() {
            Some(Ok(v)) => v,
            _ => std::process::exit(1),
        };
        let b = match tokens.next() {
            Some(Ok(v)) => v,
            _ => std::process::exit(1),
        };
        pairs.push((a, b));
    }

    // Accumulator s. Use wrapping arithmetic to reflect typical C int wrapping.
    let mut s: i32 = 0;

    // Repeat merges until one element remains.
    while pairs.len() > 1 {
        let m = pairs.len();
        // Initialize best candidate with j = 0 to avoid uninitialized-index issues.
        // This follows the SPEC selection rule deterministically.
        let mut best: usize = 0;
        let mut best_key1 = pairs[0].1.wrapping_mul(pairs[1].0);
        let mut best_key2 = pairs[0].0.wrapping_mul(pairs[1].1);

        // Scan remaining adjacent pairs to find the optimal j.
        for j in 1..(m - 1) {
            let key1 = pairs[j].1.wrapping_mul(pairs[j + 1].0);
            let key2 = pairs[j].0.wrapping_mul(pairs[j + 1].1);
            if key1 > best_key1 || (key1 == best_key1 && key2 < best_key2) {
                best = j;
                best_key1 = key1;
                best_key2 = key2;
            }
        }

        // Add term c1[best]*c2[best]*c1[best+1]*c2[best+1] with wrapping multiplication.
        let (c1b, c2b) = pairs[best];
        let (c1n, c2n) = pairs[best + 1];
        let term = c1b
            .wrapping_mul(c2b)
            .wrapping_mul(c1n)
            .wrapping_mul(c2n);
        s = s.wrapping_add(term);

        // Replace selected pair by (c1[best], c2[best+1]) and remove element at best+1.
        pairs[best] = (c1b, c2n);
        pairs.remove(best + 1);
    }

    // Print result: single integer followed by newline.
    println!("{}", s);
}
