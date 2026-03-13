// fixed_point_multiplicity.rs
//
// Translates the given specification: repeatedly replace each element of a sequence
// by the number of times that element occurs in the sequence, until the sequence
// no longer changes. For each test case prints the number of transformations
// performed and the stabilized sequence.
//
// Implementation notes (faithful to the specification):
// - Uses fixed-size arrays of length 13 for sequences and distinct-value tracking,
//   as described in the specification. If input violates the implied constraint
//   (n > 13 or >13 distinct values) behavior will panic (this corresponds to the
//   undefined behavior noted in the specification).
// - Assumes well-formed input: an integer n followed by exactly n integers.
// - Iteration counting: the number printed is the number of transformations
//   actually applied (0 if the input sequence is already a fixed point).
// - Uses safe, idiomatic Rust; no `unsafe` required.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and tokenize by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut toks = input.split_whitespace();

    // Fixed-size storage as per specification.
    const MAXN: usize = 13;
    let mut seq: [i32; MAXN] = [0; MAXN];
    let mut next: [i32; MAXN] = [0; MAXN];
    // Arrays for distinct-value tracking (value and its count)
    let mut vals: [i32; MAXN] = [0; MAXN];
    let mut cnts: [i32; MAXN] = [0; MAXN];

    // Process test cases until n == 0 or input exhausted.
    loop {
        // Read n
        let n_tok = match toks.next() {
            Some(t) => t,
            None => break, // EOF; specification does not define behavior, so stop.
        };
        let n_i32: i32 = n_tok.parse().expect("malformed integer for n");
        if n_i32 == 0 {
            // Termination condition: no output for terminating 0.
            break;
        }
        // Per specification, n is expected to be in practical range 0..=13.
        // We cast to usize and use it; out-of-range values may panic (undefined).
        let n = n_i32 as usize;

        // Read exactly n integers (spec assumes they are present).
        for i in 0..n {
            let tok = toks.next().expect("expected sequence element");
            seq[i] = tok.parse().expect("malformed integer in sequence");
        }

        // Compute fixed point by repeatedly applying the transformation.
        let mut iterations: i32 = 0;

        loop {
            // Build distinct-value list and counts for current seq[0..n]
            let mut distinct = 0usize;
            for i in 0..n {
                let x = seq[i];
                // find x in vals[0..distinct)
                let mut found = false;
                for k in 0..distinct {
                    if vals[k] == x {
                        cnts[k] += 1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    // Insert new distinct value
                    vals[distinct] = x;
                    cnts[distinct] = 1;
                    distinct += 1;
                }
            }

            // For each position j, next[j] = count of occurrences of seq[j]
            for j in 0..n {
                let x = seq[j];
                // find index in vals
                let mut idx = 0usize;
                while idx < distinct && vals[idx] != x {
                    idx += 1;
                }
                // According to construction above, x must be found among vals.
                // Set next[j] to cnts[idx].
                next[j] = cnts[idx];
            }

            // Reset cnts for next use (only first `distinct` entries were used).
            for k in 0..distinct {
                cnts[k] = 0;
            }

            // Check if next equals seq (on first n elements)
            let mut equal = true;
            for j in 0..n {
                if next[j] != seq[j] {
                    equal = false;
                    break;
                }
            }

            if equal {
                // Stabilized; seq already holds the fixed point.
                break;
            } else {
                // Move next into seq and continue.
                for j in 0..n {
                    seq[j] = next[j];
                }
                iterations += 1;
                // loop continues
            }
        }

        // Output: number of transformations, then stabilized sequence
        println!("{}", iterations);
        if n > 0 {
            // Print the first n integers separated by single spaces
            for i in 0..n {
                if i + 1 < n {
                    print!("{} ", seq[i]);
                } else {
                    println!("{}", seq[i]); // terminate line after last element
                }
            }
        } else {
            // For n == 0 this branch shouldn't happen because we break earlier,
            // but keep for completeness.
            println!();
        }
    }
}
