// tic_tac_toe_winner.rs
//
// Translates the provided C program according to the SPEC.
// Reads whitespace-separated tokens from stdin, each token encoding a 3x3 board
// as nine characters (indices 0..8). For each token that has at least 9 bytes,
// checks the fixed set of eight winning lines in the prioritized order given
// by the SPEC. Prints exactly one line per processed token:
//   "o\n" if the representative character is 'o'
//   "x\n" if the representative character is any non-'o' (and non-'s') winner
//   "d\n" if no winning line is found
//
// Note on short tokens: the SPEC declares behavior undefined for tokens
// shorter than 9 characters. To avoid unsafe memory access in Rust, this
// implementation skips tokens with length < 9 (no output). This is a safe
// choice consistent with "undefined" behavior in the SPEC (the original C
// could crash or read out-of-bounds). For tokens with len >= 9, semantics
// follow the SPEC exactly.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and split by whitespace into tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut out = String::new();

    for token in input.split_whitespace() {
        let bytes = token.as_bytes();
        // SPEC: tokens are required to have at least 9 chars (indices 0..8).
        // Behavior for shorter tokens is undefined; here we choose to skip them.
        if bytes.len() < 9 {
            continue;
        }

        // Determine the winning representative index according to SPEC's order:
        // 1. (0,1,2)
        // 2. (0,3,6)
        // 3. (0,4,8)
        // 4. (1,4,7)
        // 5. (2,5,8)
        // 6. (2,4,6)
        // 7. (3,4,5)
        // 8. (6,7,8)
        //
        // A line is a winner if all three positions are equal and not 's'.
        let winner_idx: Option<usize> = if bytes[0] != b's' && bytes[0] == bytes[1] && bytes[1] == bytes[2] {
            Some(0)
        } else if bytes[0] != b's' && bytes[0] == bytes[3] && bytes[3] == bytes[6] {
            Some(0)
        } else if bytes[0] != b's' && bytes[0] == bytes[4] && bytes[4] == bytes[8] {
            Some(0)
        } else {
            if bytes[1] != b's' && bytes[1] == bytes[4] && bytes[4] == bytes[7] {
                Some(1)
            } else {
                if bytes[2] != b's' && bytes[2] == bytes[5] && bytes[5] == bytes[8] {
                    Some(2)
                } else if bytes[2] != b's' && bytes[2] == bytes[4] && bytes[4] == bytes[6] {
                    Some(2)
                } else {
                    if bytes[3] != b's' && bytes[3] == bytes[4] && bytes[4] == bytes[5] {
                        Some(3)
                    } else {
                        if bytes[6] != b's' && bytes[6] == bytes[7] && bytes[7] == bytes[8] {
                            Some(6)
                        } else {
                            None
                        }
                    }
                }
            }
        };

        match winner_idx {
            None => out.push_str("d\n"),
            Some(idx) => {
                if bytes[idx] == b'o' {
                    out.push_str("o\n");
                } else {
                    out.push_str("x\n");
                }
            }
        }
    }

    // Write all results to stdout.
    print!("{}", out);
}
