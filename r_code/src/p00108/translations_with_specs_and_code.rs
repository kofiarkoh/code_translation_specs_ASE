// translate_fixed_point_sequences.rs
//
// Translation of the described C program (see SPEC) to safe, idiomatic Rust.
// Notes on differences / fixes vs the provided C:
// - The C code had bugs (uninitialized `sn`, misplaced `fix=1`, and other
//   issues). The SPEC is authoritative; this Rust implementation follows the
//   SPEC semantics (repeat the "replace each element by its multiplicity"
//   operation until the sequence stabilizes) and corrects the bugs.
// - The SPEC assumes n <= 13 and up to 13 distinct values; this Rust program
//   will work for any n within practical memory limits, but the behavior for
//   malformed input or EOF mid-test is left simple (it will panic on parse
//   failure, as input is assumed well-formed by SPEC).
//
// Behavior:
// - Read test cases from stdin: each test begins with integer n, followed by n
//   integers. Stop when n == 0 (no output for terminating 0).
// - For each test case, repeatedly transform the sequence by replacing each
//   element with its multiplicity in the current sequence until it no longer
//   changes. Print the number of transformations applied and the stabilized
//   sequence (two lines).
//
// Implementation notes:
// - Uses a HashMap to compute multiplicities (safe and straightforward).
// - Preserves the exact stdout format: iterations on a line, then the n
//   integers separated by single spaces and a trailing newline.

use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    // Read entire stdin and parse integers (SPEC assumes well-formed integer input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("invalid integer input"));

    loop {
        // Read n for the next test case. If no more tokens, stop.
        let n = match tokens.next() {
            Some(v) => v,
            None => break,
        };

        if n == 0 {
            // Termination condition: do not produce output for the terminating 0.
            break;
        }

        if n < 0 {
            // SPEC assumes 0 <= n <= 13; negative n is malformed. We treat it as fatal.
            // (Spec leaves malformed input undefined; we fail-fast here.)
            panic!("negative n encountered");
        }

        // Read the sequence of length n. SPEC guarantees these exist.
        let mut seq: Vec<i32> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let v = tokens.next().expect("unexpected EOF while reading sequence");
            seq.push(v);
        }

        // Iteratively apply the multiplicity transformation until stable.
        let mut iterations: usize = 0;

        loop {
            // Compute counts (multiplicities) of each value in seq.
            let mut counts: HashMap<i32, i32> = HashMap::new();
            for &val in &seq {
                *counts.entry(val).or_insert(0) += 1;
            }

            // Build new sequence where each element is replaced by its multiplicity.
            let new_seq: Vec<i32> = seq.iter().map(|&val| counts[&val]).collect();

            // If applying the transformation produces no change, we're done.
            if new_seq == seq {
                break;
            } else {
                seq = new_seq;
                iterations += 1;
            }
        }

        // Output: number of transformations performed, then stabilized sequence.
        println!("{}", iterations);
        // Print sequence with single spaces between numbers and a trailing newline.
        let line = seq.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", line);
    }
}
