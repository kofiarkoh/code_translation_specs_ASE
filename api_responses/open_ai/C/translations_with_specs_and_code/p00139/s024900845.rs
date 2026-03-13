// translator note:
// This Rust program implements the specification (see SPEC) for classifying
// input tokens into pattern A, pattern B, or NA. It follows the SPEC as the
// source of truth. The original C implementation contained a few quirks/bugs
// in its scanning logic that do not match the SPEC (for example, the way it
// searched for '#' for pattern A and its loop conditions for pattern B).
// This Rust version implements the intended patterns exactly as specified.
//
// I/O behaviour:
// - Read an integer n (first whitespace-delimited token).
// - Then read up to n whitespace-delimited tokens and classify each one,
//   printing exactly one line "A", "B", or "NA" per token (with newline).
// - If the first token cannot be parsed as an integer, the program exits
//   without printing anything (mirrors the SPEC precondition that n is
//   successfully read).
// - If fewer than n tokens are present, remaining iterations do not produce
//   output (this follows a reasonable mapping of the unspecified scanf EOF
//   behavior in the C program).
//
// This implementation uses only safe Rust.

use std::io::{self, Read};

fn is_pattern_a(s: &str) -> bool {
    // Pattern A: ^>'(=*)#\1~$
    // - starts with ">'"
    // - then k>=0 '=' characters
    // - then '#'
    // - then exactly k '=' characters
    // - then '~' and end of string
    let b = s.as_bytes();
    if b.len() < 4 {
        // minimal case when k = 0: ">'#~" length 4
        return false;
    }
    if b[0] != b'>' || b[1] != b'\'' {
        return false;
    }

    // count leading '=' after position 2 until we hit '#'
    let mut j = 2usize;
    while j < b.len() && b[j] == b'=' {
        j += 1;
    }
    // now b[j] must be '#' and j must be in bounds
    if j >= b.len() || b[j] != b'#' {
        return false;
    }
    let k = j - 2; // number of '=' before '#'

    // after '#' there must be exactly k '=' characters
    let start_after_hash = j + 1;
    // index of '~' should be start_after_hash + k
    // ensure indices are within bounds
    if start_after_hash + k >= b.len() {
        return false; // either missing '='s or missing '~'
    }
    // verify the k '=', if k==0 the loop is skipped
    for idx in start_after_hash..start_after_hash + k {
        if b[idx] != b'=' {
            return false;
        }
    }
    let tilde_idx = start_after_hash + k;
    if b[tilde_idx] != b'~' {
        return false;
    }
    // tilde must be the last character
    tilde_idx + 1 == b.len()
}

fn is_pattern_b(s: &str) -> bool {
    // Pattern B: ^>\^(?:Q=)+~~$
    // - starts with ">^"
    // - then one or more "Q=" repetitions
    // - then "~~" and end of string
    let b = s.as_bytes();
    if b.len() < 6 {
        // minimal: ">^Q=~~" length 6
        return false;
    }
    if b[0] != b'>' || b[1] != b'^' {
        return false;
    }
    // must end with "~~"
    if b[b.len() - 1] != b'~' || b[b.len() - 2] != b'~' {
        return false;
    }
    // middle part between position 2 (inclusive) and len-2 (exclusive)
    let mid = &b[2..b.len() - 2];
    if mid.len() < 2 {
        return false; // need at least one "Q="
    }
    if mid.len() % 2 != 0 {
        return false; // must be whole number of "Q=" pairs
    }
    // check each pair is "Q="
    let mut i = 0usize;
    while i < mid.len() {
        if mid[i] != b'Q' || mid[i + 1] != b'=' {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    // Read entire stdin and split on whitespace to emulate scanf("%d") then scanf("%s")...
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we cannot read stdin, exit quietly (no outputs), matching the
        // unspecified/error behavior in the original C.
        return;
    }
    let mut tokens = input.split_whitespace();

    // Read n
    let n_token = match tokens.next() {
        Some(t) => t,
        None => return, // nothing to do if no tokens
    };
    let n: i32 = match n_token.parse() {
        Ok(x) => x,
        Err(_) => return, // if parsing fails, do nothing (SPEC assumes n read successfully)
    };
    if n <= 0 {
        // Per SPEC: no iterations and no outputs if n <= 0
        return;
    }

    for _ in 0..n {
        let token = match tokens.next() {
            Some(t) => t,
            None => break, // fewer tokens than n: stop producing outputs (maps reasonably to unspecified scanf EOF behavior)
        };
        if is_pattern_a(token) {
            println!("A");
        } else if is_pattern_b(token) {
            println!("B");
        } else {
            println!("NA");
        }
    }
}
