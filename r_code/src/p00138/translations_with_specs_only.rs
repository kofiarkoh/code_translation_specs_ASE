// translate_spec.rs
//
// Translation of the provided Specification into Rust.
// Notes:
// - The original specification described C-like behavior where some fields
//   (notably min1.p and min2) could remain uninitialized in certain edge cases.
//   Rust does not permit reading uninitialized memory safely. To remain practical
//   and avoid undefined behavior in Rust, this translation uses sentinel values
//   (p = 0, t = 100.0) where the specification described uninitialized fields.
//   This choice preserves the observable control flow and numeric comparisons
//   (including the sentinel min1.t = 100.0) and the update pattern that can
//   leave min2 effectively "not meaningfully set" in typical inputs.
// - Per-group sorting uses only the strict '>' comparison when deciding to swap,
//   preserving stability for equal times as required by the specification.
// - The selection scan over remaining entries (positions 2..7 inclusive)
//   updates min1 and (only when min1 changes) min2 in a single-pass fashion
//   using strict '<' comparisons, matching the described selection logic.

use std::io::{self, Read};
use std::process;

#[derive(Clone, Copy)]
struct Run {
    p: i32,
    t: f64,
}

fn main() {
    // Read all stdin to a string and split on whitespace.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Per specification: lack of input or parse errors lead to undefined
        // behavior in original program. Here we choose to exit with code 0
        // without producing output to avoid panicking.
        process::exit(0);
    }

    let toks: Vec<&str> = input.split_whitespace().collect();

    // Expect exactly 24 pairs => 48 tokens. If fewer, mimic "undefined" by exiting.
    if toks.len() < 48 {
        process::exit(0);
    }

    // Parse 24 pairs into a 3x8 array (vector of 3 groups each with 8 runs).
    let mut groups: Vec<Vec<Run>> = vec![vec![Run { p: 0, t: 0.0 }; 8]; 3];

    for i in 0..24 {
        let pi = 2 * i;
        let ti = pi + 1;
        let p_val = match toks[pi].parse::<i32>() {
            Ok(v) => v,
            Err(_) => process::exit(0),
        };
        let t_val = match toks[ti].parse::<f64>() {
            Ok(v) => v,
            Err(_) => process::exit(0),
        };
        let group_idx = i / 8;
        let slot_idx = i % 8;
        groups[group_idx][slot_idx] = Run { p: p_val, t: t_val };
    }

    // For each group of 8 runs: sort in non-decreasing order by t.
    // Use bubble-sort style swapping when previous.t > current.t to preserve stability.
    for g in 0..3 {
        // Perform enough passes to guarantee sorting (8 elements).
        for _pass in 0..8 {
            for j in 1..8 {
                if groups[g][j - 1].t > groups[g][j].t {
                    groups[g].swap(j - 1, j);
                }
            }
        }
    }

    // Print the two fastest runs of each group: g[i][0] and g[i][1]
    // Format: "<p> <t>\n" with t to two decimals.
    for g in 0..3 {
        for idx in 0..2 {
            let r = groups[g][idx];
            // Print using two decimal places for the time.
            println!("{} {:.2}", r.p, r.t);
        }
    }

    // Selection over remaining 18 runs: positions 2..7 in each group (inclusive).
    // Initialize min1.t = 100.0 sentinel. p is set to 0 here because Rust cannot
    // leave it truly uninitialized; this approximates the original program's
    // behavior where p might be indeterminate if not overwritten.
    let mut min1 = Run { p: 0, t: 100.0 };
    // min2 likewise starts as a sentinel; in the original code it may have been
    // uninitialized. The scan only assigns to min2 when min1 is replaced.
    let mut min2 = Run { p: 0, t: 100.0 };

    // Single-pass scan with strict '<' comparison. When a new min1 is found,
    // min2 is assigned the previous min1 value. There is no else branch to
    // update min2 otherwise (mirroring the specified behavior).
    for g in 0..3 {
        for j in 2..8 {
            let r = groups[g][j];
            if r.t < min1.t {
                // assign previous min1 into min2, then set min1 = r
                min2 = min1;
                min1 = r;
            }
        }
    }

    // Print min1 and min2 in the same format.
    println!("{} {:.2}", min1.p, min1.t);
    println!("{} {:.2}", min2.p, min2.t);

    // Normal termination with exit code 0 (Rust returns 0 by default).
}
