// tictactoe_checker.rs
//
// Translates the given specification into Rust.
// Reads whitespace-separated tokens from stdin, each token representing a 3x3
// Tic-Tac-Toe board as a linear 9-character string (indices 0..8).
// For each token that has at least 9 characters, tests eight winning lines
// in a fixed priority order. If a tested line contains three identical
// non-'s' characters, that line determines the winner:
//   - prints "o" if the winning character is 'o'
//   - prints "x" for any other non-'s' character
// If no winning line is found, prints "d".
// Each printed result is exactly one character followed by a newline.
//
// Note on malformed tokens: The specification marks tokens shorter than 9 as
// "undefined behavior" in the original C-like program (which could read past
// its buffer). In safe Rust we cannot emulate arbitrary undefined memory reads.
// To remain conservative and avoid inventing behavior, this implementation
// skips tokens with length < 9 (i.e., produces no output for them). Well-formed
// tokens (len >= 9) are handled exactly per the specification.

use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit (no output). This mirrors "no processing" on failure.
        return;
    }

    // Fixed evaluation order of winning lines (as indices into the 9-char token)
    // 1. (0,1,2)
    // 2. (0,3,6)
    // 3. (0,4,8)
    // 4. (1,4,7)
    // 5. (2,5,8)
    // 6. (2,4,6)
    // 7. (3,4,5)
    // 8. (6,7,8)
    const LINES: [(usize, usize, usize); 8] = [
        (0, 1, 2),
        (0, 3, 6),
        (0, 4, 8),
        (1, 4, 7),
        (2, 5, 8),
        (2, 4, 6),
        (3, 4, 5),
        (6, 7, 8),
    ];

    // Process tokens split on whitespace until EOF
    for token in input.split_whitespace() {
        let bytes = token.as_bytes();

        // Specification requires tokens to have accessible indices 0..8.
        // If shorter, behavior in spec is undefined. We choose to skip such tokens.
        if bytes.len() < 9 {
            continue;
        }

        let mut decided = false;

        for &(a, b, c) in &LINES {
            let ca = bytes[a];
            // Check three equal and not equal to 's' (byte literal)
            if ca != b's' && ca == bytes[b] && ca == bytes[c] {
                // Representative index chosen as the first index of the trio (a).
                if ca == b'o' {
                    println!("o");
                } else {
                    // Any non-'s' and non-'o' winner prints as "x" per spec.
                    println!("x");
                }
                decided = true;
                break;
            }
        }

        if !decided {
            println!("d");
        }
    }
}
