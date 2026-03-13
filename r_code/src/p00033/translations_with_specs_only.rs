// specification_translation.rs
//
// Translates the provided specification into Rust.
// Behavior notes (per specification):
// - Reads an integer `n` then `10*n` integers (each test case has exactly 10 ints).
// - For each test case initialize track1 = first element, track2 = 0.
// - For each of the remaining 9 elements in order:
//     - if x > track1 then track1 = x
//     - else if x > track2 then track2 = x
//     - else reject the test case immediately
// - Print "YES" or "NO" (each followed by newline) for each test case processed.
// - If input is missing (fewer tokens than required), the program stops processing further test cases.
//   This is a narrow, safe interpretation of the unspecified scanf-failure behavior in the original C program.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string and parse whitespace-separated tokens as i32 (C int = 32-bit).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (matches "no explicit checks" spirit; narrow safe handling).
        return;
    }

    let mut tokens = input.split_whitespace().map(|w| w.parse::<i32>());

    // Read n (number of test cases). If missing or not an integer, stop.
    let n_i32 = match tokens.next() {
        Some(Ok(v)) => v,
        _ => return,
    };

    // If n <= 0 do nothing (no per-case output).
    if n_i32 <= 0 {
        return;
    }
    let n = n_i32 as usize;

    // Process up to n test cases, stopping early if input runs out or parsing fails.
    for _case_idx in 0..n {
        // Read exactly 10 integers for this test case.
        let mut seq = [0i32; 10];
        let mut ok = true;
        for i in 0..10 {
            match tokens.next() {
                Some(Ok(v)) => seq[i] = v,
                // If a token is missing or fails to parse, stop processing further test cases.
                _ => return,
            }
        }

        // Initialize tracks: track1 = first element, track2 = 0
        let mut track1 = seq[0];
        let mut track2 = 0i32;

        // Process remaining 9 elements in order using the greedy rule.
        for &x in &seq[1..] {
            if x > track1 {
                track1 = x;
            } else if x > track2 {
                track2 = x;
            } else {
                ok = false;
                break;
            }
        }

        if ok {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
