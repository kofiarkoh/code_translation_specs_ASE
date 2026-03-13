// translator.rs
// Translated from the given C program to Rust following the provided SPEC.
// Behavior is driven by the SPEC; the C code was used only to resolve minor details.
// This implementation preserves the output formatting exactly (single spaces between tokens,
// no trailing space before newline, each produced line terminated by '\n').

use std::io::{self, Read};

fn main() {
    // Read entire stdin and split on whitespace to get integer tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, mirror the C program's lack of error handling by exiting silently.
        return;
    }
    let mut toks = input.split_whitespace();

    // Parse n (count). If not present or not an integer, exit (behavior undefined per spec).
    let n: i32 = match toks.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // In C, if n <= 0 the for-loop (i = 0; i < n; i++) does zero iterations.
    let pairs_to_process = if n > 0 { n as usize } else { 0usize };

    for _ in 0..pairs_to_process {
        // Parse from and to; if missing, stop early (mirrors undefined behavior).
        let from: i32 = match toks.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        let to: i32 = match toks.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };

        // Follow the SPEC logic precisely.

        if from < to {
            // Print from .. to inclusive, with single spaces between tokens, newline at end.
            // The C code prints j from from to to-1 with trailing spaces then prints to with newline.
            for j in from..to {
                print!("{} ", j);
            }
            println!("{}", to);
        } else if to == 5 {
            // Print ascending from..=9 (only if from <= 9), each followed by a space,
            // then print 5 and newline. If from > 9, only "5\n" is printed.
            if from <= 9 {
                for j in from..=9 {
                    print!("{} ", j);
                }
            }
            println!("5");
        } else if from > to {
            if (6..=9).contains(&from) {
                // Print ascending from..=9 (from is in 6..=9 so loop will run),
                // then descending 5 down to to+1 (if any) with spaces,
                // finally print to and newline.
                for j in from..=9 {
                    print!("{} ", j);
                }
                if to < 5 {
                    // print 5,4,...,to+1
                    for j in ((to + 1)..=5).rev() {
                        print!("{} ", j);
                    }
                }
                println!("{}", to);
            } else {
                // Print descending from down to to inclusive.
                // C prints from..(to+1) with trailing spaces then prints to with newline.
                if to + 1 <= from {
                    for j in ((to + 1)..=from).rev() {
                        print!("{} ", j);
                    }
                }
                println!("{}", to);
            }
        }
        // Important special case from == to and to != 5:
        // None of the above branches match and so nothing is printed for that pair;
        // the loop simply continues — matching the SPEC.
    }
}
