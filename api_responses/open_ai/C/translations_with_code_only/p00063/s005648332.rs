// translated_program.rs
// Translation of the provided C program to Rust.
// Notes:
// - The original C used fixed-size char arrays of length N=100 and scanf("%s", ...).
//   To emulate storing into a char[100] (and avoid UB from overflowing the buffer),
// - we truncate tokens to at most 99 bytes (leaving space for the implicit NUL in C).
// - Tokenization uses ASCII whitespace (space, tab, newline, carriage return, vertical tab, form feed),
//   which matches typical C isspace behavior for these tests.
// - Comparison and reversal operate on raw bytes to mirror C's char-array semantics.
// - cnt is an i32 and incremented with wrapping_add to emulate C's signed-int wraparound semantics.

use std::io::{self, Read};

const N: usize = 100;
const MAX_STORE: usize = N - 1; // emulate room for a terminating NUL in the C buffer

fn is_ascii_whitespace(b: &u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c')
}

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let mut cnt: i32 = 0;

    for token in input.split(is_ascii_whitespace) {
        if token.is_empty() {
            continue;
        }

        // Emulate copying into char str1[N] and str2[N] by truncating to MAX_STORE bytes.
        let mut buf: Vec<u8> = token.iter().take(MAX_STORE).cloned().collect();
        let buf_copy = buf.clone();

        // hanten: reverse in place by swapping first half with second half
        let n = buf.len();
        for j in 0..(n / 2) {
            buf.swap(j, n - j - 1);
        }

        // if reversed equals original (as stored in fixed buffer), increment count
        if buf == buf_copy {
            cnt = cnt.wrapping_add(1);
        }
    }

    // Preserve output formatting: number followed by a newline
    println!("{}", cnt);
    Ok(())
}
