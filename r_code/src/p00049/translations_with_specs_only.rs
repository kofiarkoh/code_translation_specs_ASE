// blood_counts.rs
//
// Translation of the given Specification into Rust.
// Preserves semantics described in the Specification as closely as possible.
//
// Notes on semantic choices and corner-cases (as required by the spec):
// - MAX = 50. We allocate an array of size MAX+1 (indices 0..=50).
//   If an input ID > MAX is encountered, indexing will panic at runtime.
//   The spec marks IDs > MAX as undefined; a panic approximates a crash.
// - For ID parsing we consume at most two consecutive ASCII digits (0-9).
//   If no digit is present, ID is treated as 0 (per spec allowing ID 0).
// - After the digit(s) we consume exactly one separator character if present
//   (the spec expects a separator such as a comma).
// - Blood-group token parsing follows the spec:
//   accepted tokens: "A", "B", "AB", "O". Unknown tokens default to group A.
// - If a later line assigns a different blood group to an already-seen ID,
//   the program enters a non-terminating failure state (infinite loop).
//   We implement that as an infinite sleep loop so the process never exits.
// - Repeated identical assignments for an ID are ignored.
// - Empty input prints four zeros (one per line).
//
// This program reads lines from stdin and writes four lines on stdout:
// counts for A, B, AB, O (in that order).

use std::io::{self, BufRead};
use std::time::Duration;
use std::thread;

const MAX: usize = 50;

fn main() {
    // mapping[id] == 0 => unseen
    // 1 => A, 2 => B, 3 => AB, 4 => O
    let mut mapping = [0u8; MAX + 1];
    let mut counts = [0usize; 4]; // indices 0:A,1:B,2:AB,3:O

    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(s) => s,
            Err(_) => continue, // If reading a line fails, skip it (not specified).
        };
        // Work with bytes to emulate ASCII/C-like parsing.
        let bytes = line.as_bytes();
        let mut i = 0usize;
        let len = bytes.len();

        // Skip leading whitespace
        while i < len && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r') {
            i += 1;
        }

        // Parse at most two consecutive digits for the ID
        let mut id: usize = 0;
        let mut digits_read = 0;
        while i < len && digits_read < 2 {
            let b = bytes[i];
            if b >= b'0' && b <= b'9' {
                id = id * 10 + ((b - b'0') as usize);
                digits_read += 1;
                i += 1;
            } else {
                break;
            }
        }
        // If no digits were present, id stays 0 (spec allows ID 0)

        // Consume exactly one separator character if present (spec expects a separator)
        if i < len {
            i += 1;
        }

        // Skip whitespace before token
        while i < len && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r') {
            i += 1;
        }

        // Determine group token. Default to A (1) for unknown tokens.
        let group = if i < len {
            match bytes[i] {
                b'A' => {
                    if i + 1 < len && bytes[i + 1] == b'B' {
                        3u8 // AB
                    } else {
                        1u8 // A
                    }
                }
                b'B' => 2u8, // B
                b'O' => 4u8, // O
                _ => 1u8,    // unknown -> default to A
            }
        } else {
            // No token present -> default to A
            1u8
        };

        // Now apply semantics: first assignment wins; identical repeats ignored;
        // conflicting assignment -> infinite loop (non-terminating failure state).
        // mapping index: id (may be 0..=MAX). If id > MAX, this will panic (as spec allows UB).
        if id > MAX {
            // Intentionally index out of bounds to match "undefined behavior" when id > MAX.
            // In Rust this will panic on indexing below; we do the indexing anyway.
            // Accessing mapping[id] will panic and terminate with an error, approximating UB.
        }

        let existing = mapping[id];
        if existing == 0 {
            mapping[id] = group;
            match group {
                1 => counts[0] += 1, // A
                2 => counts[1] += 1, // B
                3 => counts[2] += 1, // AB
                4 => counts[3] += 1, // O
                _ => counts[0] += 1, // shouldn't happen; treat as A
            }
        } else if existing == group {
            // repeated identical assignment: ignore
        } else {
            // conflicting assignment: enter non-terminating failure state (infinite loop)
            // Implemented as an infinite sleep loop so process never exits.
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    // On EOF and no conflicts, print counts: A, B, AB, O each on its own line.
    println!("{}", counts[0]);
    println!("{}", counts[1]);
    println!("{}", counts[2]);
    println!("{}", counts[3]);
}
