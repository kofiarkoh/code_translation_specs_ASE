// concordance.rs
//
// Translates the described C program into safe, idiomatic Rust following the SPEC.
// Behavior implemented (per SPEC):
//  - Read whitespace-separated pairs (word, integer) from stdin until EOF.
//  - Group integers (page numbers) by identical words.
//  - Produce a lexicographically ordered list of unique words.
//  - For each word, print the word on its own line, then a line with the associated
//    integers in ascending order, space-separated.
//
// Notes about the original C code:
//  - The C program contained multiple bugs (e.g. use of an uninitialized loop variable
//    that can invoke UB, and printing that skips the first collected integer per word).
//    The SPEC directs us to follow the intended behavior rather than reproduce UB-causing bugs.
//  - This Rust implementation follows the SPEC (correct grouping, sorting of words and integers).
//
// Implementation details:
//  - Reading stops when there is not a complete (word, integer) pair remaining or on EOF.
//    This mirrors the intended scanf behavior described in the SPEC.
//  - To respect the SPEC precondition, a maximum of 128 pairs are accepted; further input
//    pairs are ignored (the C program had fixed-size arrays sized to 128).
//  - Words are compared lexicographically using Rust's standard string ordering (equivalent to strcmp).
//
// No unsafe code is used.

use std::collections::HashMap;
use std::io::{self, Read};

const MAX_ENTRIES: usize = 128;

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit silently (no output), as the SPEC does not define special error output.
        return;
    }

    let mut tokens = input.split_whitespace();
    let mut groups: HashMap<String, Vec<i32>> = HashMap::new();
    let mut entries: usize = 0;

    loop {
        // Get next word token
        let word = match tokens.next() {
            Some(w) => w,
            None => break, // EOF / no more tokens
        };

        // Get next integer token; if missing or not parseable, stop processing further pairs.
        let num_token = match tokens.next() {
            Some(t) => t,
            None => break, // incomplete pair, stop as per SPEC
        };

        let num: i32 = match num_token.parse() {
            Ok(v) => v,
            Err(_) => break, // parse mismatch -> stop processing further pairs
        };

        if entries >= MAX_ENTRIES {
            // Respect the SPEC capacity precondition: ignore extra entries beyond 128.
            break;
        }

        // Insert into groups
        groups.entry(word.to_string()).or_default().push(num);
        entries += 1;
    }

    // Collect and sort words lexicographically
    let mut words: Vec<String> = groups.keys().cloned().collect();
    words.sort(); // lexicographic ordering, like strcmp

    // For each word, sort associated integers and print
    for w in words {
        if let Some(nums) = groups.get_mut(&w) {
            nums.sort_unstable(); // ascending order; stable not required
            println!("{}", w);
            if !nums.is_empty() {
                // Print first without leading space, then subsequent with a leading space
                print!("{}", nums[0]);
                for n in &nums[1..] {
                    print!(" {}", n);
                }
            }
            println!();
        }
    }
}
