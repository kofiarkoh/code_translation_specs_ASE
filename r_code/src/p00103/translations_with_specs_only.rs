// baseball_sim.rs
//
// Implements the specification: a command-line program that reads an integer n
// followed by whitespace-separated event tokens ("HIT", "HOMERUN", "OUT") from
// stdin and for each of n segments simulates base-running until 3 outs occur,
// printing the number of runs scored for each segment.
//
// Design notes:
// - We read tokens directly from stdin byte-by-byte using a blocking Read.
//   This keeps the program's observable behavior close to the spec (e.g. when
//   run interactively it will block waiting for more tokens).
// - If a token is not exactly "HIT", "HOMERUN", or "OUT" it is ignored (no
//   state change), as required.
// - The loop over segments decrements the parsed integer `n` until it reaches
//   zero. This mirrors the described behavior where negative n would lead to
//   indefinite processing (i.e. the loop condition is `while n != 0`).
// - If EOF is encountered while expecting more input, the program exits
//   (the spec describes this as undefined; here we simply stop).
//
// No unsafe code is used.

use std::io::{self, Read};

/// Read the next whitespace-delimited token from `reader`.
/// Returns None on EOF or I/O error.
///
/// This function reads bytes directly from the reader, skipping leading ASCII
/// whitespace, then collecting bytes until the next ASCII whitespace (or EOF).
/// It blocks when the underlying reader blocks (matching the spec's blocking
/// behavior when input is interactive).
fn next_token<R: Read>(reader: &mut R) -> Option<String> {
    let mut buf: Vec<u8> = Vec::new();
    let mut byte = [0u8; 1];

    // Skip leading whitespace
    loop {
        match reader.read(&mut byte) {
            Ok(0) => return None, // EOF before any non-whitespace
            Ok(_) => {
                let b = byte[0];
                if !b.is_ascii_whitespace() {
                    buf.push(b);
                    break;
                } else {
                    // keep skipping
                }
            }
            Err(_) => return None,
        }
    }

    // Read token until next whitespace or EOF
    loop {
        match reader.read(&mut byte) {
            Ok(0) => break, // EOF ends token
            Ok(_) => {
                let b = byte[0];
                if b.is_ascii_whitespace() {
                    break;
                } else {
                    buf.push(b);
                }
            }
            Err(_) => break,
        }
    }

    match String::from_utf8(buf) {
        Ok(s) => Some(s),
        Err(_) => None, // invalid UTF-8 -> treat as no token
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Read first token: the integer n (number of segments).
    let first = match next_token(&mut handle) {
        Some(s) => s,
        None => return, // EOF or no token: nothing to do
    };

    let mut n: i64 = match first.parse() {
        Ok(v) => v,
        Err(_) => return, // malformed integer: exit (spec does not define error messages)
    };

    // Process segments until n == 0. This matches the described semantics where
    // the original code decremented n until it reached zero.
    while n != 0 {
        // Per-segment state
        let mut outs: i32 = 0;
        let mut bases = [false; 3]; // bases[0]=1st, bases[1]=2nd, bases[2]=3rd
        let mut score: i32 = 0;

        // Process events until 3 outs
        while outs < 3 {
            let token = match next_token(&mut handle) {
                Some(t) => t,
                None => return, // EOF encountered while expecting more tokens: exit
            };

            if token == "HIT" {
                // existing runners advance one base; runner on 3rd scores
                if bases[2] {
                    score += 1;
                }
                // shift: 3rd = 2nd, 2nd = 1st, 1st = batter
                bases[2] = bases[1];
                bases[1] = bases[0];
                bases[0] = true;
            } else if token == "HOMERUN" {
                // all runners score plus batter
                if bases[0] { score += 1; }
                if bases[1] { score += 1; }
                if bases[2] { score += 1; }
                score += 1; // batter
                // clear bases
                bases = [false; 3];
            } else if token == "OUT" {
                outs += 1;
                // no change to bases or score
            } else {
                // invalid token: do nothing and continue (per spec)
            }
        }

        // After third out, output the runs scored for this segment as a line.
        println!("{}", score);

        // decrement n and continue (if n negative, this never reaches zero).
        n -= 1;
    }
}
