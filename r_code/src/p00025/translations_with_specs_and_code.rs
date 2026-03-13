// main.rs
// Translation of the given C program to Rust, following the provided specification.
// Note on deviation from the C code:
// - The original C used scanf in a loop and had unspecified behavior on malformed/partial input.
// - Per the SPEC, the program should repeatedly read pairs of 4 integers and output hit/blow counts.
// - This Rust program reads whitespace-separated tokens from stdin, parses integers, and processes
//   every complete group of 8 integers (A[0..3], B[0..3]). Incomplete trailing groups or a parse
//   failure stop further processing. This makes behavior deterministic and safe while preserving
//   the specified functional behavior and output format.
//
// I/O behavior:
// - For each complete pair of 4+4 integers, prints a line "<hit> <blow>\n".
// - Exits normally (return code 0) after processing input.

use std::io::Read;

fn main() {
    // Read all stdin into a String
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit quietly (no output), matching the program's non-verbose nature.
        return;
    }

    // Parse whitespace-separated tokens as i32. On first parse error, stop consuming further tokens.
    let mut nums: Vec<i32> = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<i32>() {
            Ok(n) => nums.push(n),
            Err(_) => break, // stop on malformed token to avoid producing undefined results
        }
    }

    // Process every complete group of 8 integers (4 for A, 4 for B)
    let mut idx = 0usize;
    while idx + 7 < nums.len() {
        let a_slice = &nums[idx..idx + 4];
        let b_slice = &nums[idx + 4..idx + 8];

        let mut hit: i32 = 0;
        let mut blow: i32 = 0;

        for a in 0..4 {
            for b in 0..4 {
                if a_slice[a] == b_slice[b] {
                    if a == b {
                        hit += 1;
                    } else {
                        blow += 1;
                    }
                }
            }
        }

        // Print "<hit> <blow>\n"
        println!("{} {}", hit, blow);

        idx += 8;
    }
}
