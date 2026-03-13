// reverse_token.rs
//
// Translation of the given C-style specification into Rust.
//
// Notes on fidelity to the specification:
// - We allocate two fixed-size buffers of 21 bytes each (sufficient for up to 20 chars + NUL).
// - We read exactly one whitespace-delimited token from stdin (like scanf("%s", ...)).
// - We assume the caller meets the precondition that the token length is 1..=20.
//   If no token is present or the token is longer than 20 bytes we treat that as a
//   precondition violation and abort (the original C program had undefined behavior
//   in those cases).
// - We copy the input into `str_buf` and NUL-terminate it (reflecting scanf's behavior).
// - We copy the reversed characters into `strsub[0..x]` but we intentionally do NOT write
//   a terminating NUL at strsub[x], preserving the specification that the output buffer
//   is not guaranteed to be NUL-terminated.
// - To emulate printf("%s", strsub) in a safe, deterministic way we print bytes starting
//   at strsub until a NUL is encountered inside the allocated buffer; if no NUL exists
//   in the 21-byte buffer we print the entire buffer. The original C program could read
//   beyond the buffer (undefined behavior); we choose the narrow, deterministic
//   interpretation consistent with the specification (the bytes beyond the reversed
//   characters are indeterminate/garbage).
//
// This implementation uses only safe Rust.

use std::io::{self, Read, Write};

fn main() {
    // Read all stdin and pick the first whitespace-delimited token (like scanf("%s", ...)).
    let mut stdin_contents = String::new();
    // We ignore the possibility of I/O error per the spec's spirit (the C program didn't check scanf's return).
    io::stdin().read_to_string(&mut stdin_contents).unwrap();

    // Pick the first token; the spec assumes one exists. If not, we treat that as a precondition violation.
    let token = stdin_contents
        .split_whitespace()
        .next()
        .expect("precondition violated: no token on stdin");

    let input_bytes = token.as_bytes();
    let x = input_bytes.len();

    // The spec's intended buffer size is 21 (20 chars + NUL). If the token is longer, behavior is undefined.
    // We choose to abort on that precondition violation rather than inventing extra behavior.
    if x == 0 || x > 20 {
        panic!("precondition violated: token length must be 1..=20");
    }

    // Simulate the C `char str[21]` buffer and scanf("%s", str) behavior (NUL-terminated).
    let mut str_buf = [0u8; 21];
    for (i, &b) in input_bytes.iter().enumerate() {
        str_buf[i] = b;
    }
    // NUL-terminate as scanf would do (only within bounds because x <= 20).
    str_buf[x] = 0;

    // Simulate the C `char strsub[21]` buffer. Fill with a non-zero pattern to model indeterminate (non-NUL) contents.
    // We choose 0xAA as deterministic "garbage".
    let mut strsub = [0xAAu8; 21];

    // Copy reversed characters: for i in [0..x-1], strsub[i] = str[x-1-i].
    for i in 0..x {
        // str_buf[x-1-i] is defined and non-NUL for i < x (because input bytes are non-NUL bytes of token).
        strsub[i] = str_buf[x - 1 - i];
    }

    // Intentionally do NOT write a terminating NUL at strsub[x] to match the specification.

    // Emulate printf("%s", strsub):
    // - Search for a NUL within the 21-byte buffer; if found, print up to (but not including) that NUL.
    // - If no NUL is found within the buffer, print the entire buffer (this is a conservative, deterministic
    //   approximation of the original undefined behavior which might read past the buffer).
    let nul_pos = strsub.iter().position(|&b| b == 0);
    let bytes_to_print: &[u8] = match nul_pos {
        Some(pos) => &strsub[..pos],
        None => &strsub[..],
    };

    // Write the selected bytes to stdout (no extra newline), matching printf("%s", ...)'s raw behavior.
    io::stdout().write_all(bytes_to_print).unwrap();

    // Exit with status 0 (normal termination). Returning from main does that.
}
