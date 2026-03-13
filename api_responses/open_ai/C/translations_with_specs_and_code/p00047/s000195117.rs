// ball_swaps.rs
//
// Translated from the provided C program according to the SPEC.
// Behavior:
// - Starts with the ball under 'A'.
// - Reads stdin as a stream of commands, each exactly three bytes: <Label1>,<Label2>
//   where each Label is one of 'A', 'B', 'C' and the middle byte is a literal comma.
// - For each well-formed command, swap the ball between the two labels.
// - After processing input (EOF or upon encountering the first malformed command), print
//   the label currently holding the ball followed by a newline.
//
// Notes on differences / choices:
// - The SPEC states malformed input has unspecified behavior and the C program
//   performs no validation. In the C code, a malformed label could leave indices
//   uninitialized (undefined behavior). In safe Rust we avoid undefined behavior:
//   we process only well-formed commands (exactly 3 bytes with a comma in the middle
//   and labels in {'A','B','C'}). On the first malformed/incomplete command we stop
//   processing further input. This is a conservative, safe interpretation consistent
//   with the SPEC's statement that malformed input is not guaranteed to produce
//   meaningful results.
// - I/O formatting is preserved: output is exactly one of "A\n", "B\n", or "C\n".
//
// Implementation decisions:
// - Use a single `pos` index (0=A,1=B,2=C) to represent ball position (O(1) space).
// - Read all stdin into a byte buffer and iterate in chunks of 3 bytes.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a buffer.
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // If reading stdin fails, per SPEC stdin should be available; just exit silently.
        // Exiting with status 0 as SPEC requires normal termination on EOF; if read fails,
        // there's nothing sensible to do in this small utility.
        return;
    }

    // pos: 0 => A, 1 => B, 2 => C. Initially under A.
    let mut pos: usize = 0;

    // Process input in sequential 3-byte commands: <Label1> ',' <Label2>
    let mut i = 0usize;
    while i + 2 < input.len() {
        let a = input[i];
        let sep = input[i + 1];
        let b = input[i + 2];

        // Require literal comma in the middle; if not present, stop processing.
        if sep != b',' {
            break;
        }

        // Map label bytes to indices. If a label is not one of A/B/C, stop processing.
        let x = match a {
            b'A' => 0usize,
            b'B' => 1usize,
            b'C' => 2usize,
            _ => break,
        };
        let y = match b {
            b'A' => 0usize,
            b'B' => 1usize,
            b'C' => 2usize,
            _ => break,
        };

        // Perform the swap effect on the single position variable.
        if pos == x {
            pos = y;
        } else if pos == y {
            pos = x;
        }

        i += 3;
    }

    // Print the final location with a newline.
    let out = match pos {
        0 => 'A',
        1 => 'B',
        _ => 'C',
    };
    println!("{}", out);
}
