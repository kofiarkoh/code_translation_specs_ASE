// Translated from the given C program.
// Notes on translation decisions:
// - The C program reads whitespace-delimited strings with scanf("%s") into a 11-byte buffer
//   and then accesses the first 10 positions. That C behavior typically assumes input tokens
//   are at least 10 characters (digits). To remain predictable and safe in Rust, we read
//   tokens via split_whitespace() and, for tokens shorter than 10 bytes, emulate a '\0'
//   character for the missing positions (so n[j] == 0, and n[j] - '0' == -48), which is a
//   reasonable deterministic approximation of the C behavior when a '\0' terminator is present.
// - The triangular reduction uses signed remainder (%) just like C; Rust's % has the same
//   sign behavior as C for negative numbers, so we use i32 and the same computations.
// - The original C stored results in a fixed-size ans[10000]; here we use a Vec to avoid
//   UB for inputs >10000 while preserving the observable I/O behavior for typical inputs.
//
// The program preserves the I/O formatting: one integer per line on stdout.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and iterate tokens like scanf("%s") would do
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, behave like C would exit; here we just return.
        return;
    }

    let mut answers: Vec<i32> = Vec::new();

    for token in input.split_whitespace() {
        // Prepare seq[0..10] as in C: seq[j] = n[j] - '0' for j=0..9
        // If token has fewer than 10 bytes, treat missing bytes as '\0' (0),
        // so value becomes 0 - '0' == -48, which is a deterministic choice.
        let bytes = token.as_bytes();
        let mut seq = [0i32; 10];
        for j in 0..10 {
            let val = if j < bytes.len() {
                (bytes[j] as i32) - ('0' as i32)
            } else {
                // emulate reading a '\0' from the C buffer beyond the string terminator
                (0i32) - ('0' as i32)
            };
            seq[j] = val;
        }

        // Triangular reduction:
        // for (j=0;j<9;j++) for (k=0;k<9-j;k++) seq[k]=(seq[k]+seq[k+1])%10;
        for _step in 0..9 {
            for k in 0..(9 - _step) {
                seq[k] = (seq[k] + seq[k + 1]) % 10;
            }
        }

        answers.push(seq[0]);
    }

    // Print answers, one per line
    for a in answers {
        println!("{}", a);
    }
}
