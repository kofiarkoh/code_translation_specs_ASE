// factorial_cli.rs
//
// Translation of the given Specification into Rust.
// - Reads one integer from stdin using behavior equivalent to C's scanf("%d",&i):
//   it skips initial ASCII whitespace, accepts an optional '+' or '-' sign, then
//   consumes as many ASCII digits as possible (at least one digit is required).
//   Any following non-digit characters are left unread (i.e., ignored for parsing).
// - If the parsed integer n > 0, computes n! as the product of integers from n down to 1.
//   If n <= 0, produces 1.
// - Multiplication uses wrapping arithmetic (two's complement wrap) to emulate
//   typical low-level wraparound behavior rather than detecting/avoiding overflow.
// - Writes the resulting value as a decimal integer followed by '\n' to stdout.
//
// Notes on choices:
// - The Specification treats the input as a C signed int. We model this as Rust i32,
//   which is the closest portable signed-32-bit integer type and the common width
//   for C's int on many platforms. The Specification did not fix width, so this is
//   the narrowest reasonable choice.
// - The Spec assumes scanf succeeds; this code will panic if input does not contain
//   a parseable integer per the scanf-like rules above. That follows the Spec's
//   assumption (undefined behavior in C when scanf fails).
// - Wrapping multiplication is used to reflect the Spec's note about integer
//   overflow/wraparound rather than attempting to detect overflow.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a buffer
    let mut buf = String::new();
    // If reading fails, propagate the error by panicking. The Specification assumes
    // a readable stdin providing at least one token; behavior is not defined otherwise.
    io::stdin()
        .read_to_string(&mut buf)
        .expect("failed to read stdin");

    let bytes = buf.as_bytes();
    let len = bytes.len();
    let mut idx: usize = 0;

    // Skip initial ASCII whitespace (same classification as C's isspace for ASCII)
    while idx < len && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }

    // Now parse optional sign
    let mut start = idx;
    if idx < len && (bytes[idx] == b'+' || bytes[idx] == b'-') {
        idx += 1;
    }

    // Consume as many ASCII digits as possible
    let digits_start = idx;
    while idx < len && (bytes[idx] >= b'0' && bytes[idx] <= b'9') {
        idx += 1;
    }

    // There must be at least one digit (scanf("%d") requires a digit). The Spec
    // assumes scanf succeeds; here we follow that assumption and panic if not.
    if digits_start == idx {
        panic!("input does not contain an integer in scanf(\"%d\") sense");
    }

    // Extract the substring corresponding to sign + digits
    let substr = &buf[start..idx];

    // Parse into i32. The Specification assumes the input is representable in the
    // platform's signed int; we use i32 and thus expect this to succeed.
    let n: i32 = substr
        .parse()
        .expect("parsed integer not representable in i32");

    // Compute factorial per Specification:
    // - If n <= 0 => 1
    // - If n > 0 => product of integers from n down to 1
    // Use wrapping multiplication to emulate low-level wraparound behavior.
    let result: i32 = if n <= 0 {
        1
    } else {
        // n > 0, iterate from 1..=n and multiply
        let mut prod: i32 = 1;
        // Convert n to i32 (already i32) and to i64 for loop bound safety if desired,
        // but per Spec we keep it simple and loop using i32 arithmetic.
        let mut k: i32 = 1;
        while k <= n {
            prod = prod.wrapping_mul(k);
            k += 1;
        }
        prod
    };

    // Print the result followed by a newline
    // Using println! ensures a trailing '\n'.
    println!("{}", result);
}
