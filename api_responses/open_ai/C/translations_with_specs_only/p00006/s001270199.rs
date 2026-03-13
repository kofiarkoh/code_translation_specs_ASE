// reverse_token.rs
//
// Translates the provided specification into Rust.
// Behavior:
// - Reads a single whitespace-delimited token (sequence of non-whitespace bytes) from stdin.
// - Uses two fixed-size buffers of 21 bytes each (inbuf and outbuf).
// - Assumes token length L in [1, 20]. If preconditions are violated (EOF, or token length > 20),
//   the program exhibits "undefined" behavior per the specification; here we choose to abort
//   (panic) in those cases rather than silently alter behavior.
// - Writes the token's characters in reverse order to stdout as a contiguous byte sequence,
//   with no trailing newline. The program exits with status 0 on normal termination.
//
// Notes on fidelity to the specification:
// - We treat input as raw bytes and detect whitespace using ASCII whitespace semantics,
//   matching the common behavior of C's scanf("%s") for typical byte-oriented inputs.
// - We place a terminating NUL (0) in the input buffer after reading the token (scanf does so),
//   but we do not place a terminating NUL in the output buffer (the specification notes that the
//   original program did not necessarily NUL-terminate the output buffer).
// - For inputs that violate preconditions (EOF or token length > 20) the spec states behavior is
//   undefined; we represent that by panicking (aborting) so callers are clearly alerted.

use std::io::{self, Read, Write};

fn main() {
    // Two fixed-size buffers of capacity 21 bytes each.
    let mut inbuf: [u8; 21] = [0u8; 21];
    let mut outbuf: [u8; 21] = [0u8; 21];

    let mut stdin = io::stdin().lock();
    let mut one = [0u8; 1];

    // Skip leading ASCII whitespace (mimics scanf("%s") behavior).
    loop {
        match stdin.read(&mut one) {
            Ok(0) => {
                // EOF while expecting a token -> undefined per spec.
                panic!("EOF reached while expecting a token (undefined behavior per spec)");
            }
            Ok(1) => {
                let b = one[0];
                if (b as char).is_whitespace() {
                    continue;
                } else {
                    // First non-whitespace byte is part of the token.
                    inbuf[0] = b;
                    break;
                }
            }
            Err(e) => {
                // Read error -> undefined per spec.
                panic!("Error reading stdin: {} (undefined behavior per spec)", e);
            }
        }
    }

    // Read subsequent non-whitespace bytes into inbuf up to at most 20 bytes.
    let mut len: usize = 1;
    loop {
        match stdin.read(&mut one) {
            Ok(0) => {
                // EOF terminates token reading (acceptable if at least one byte was read).
                break;
            }
            Ok(1) => {
                let b = one[0];
                if (b as char).is_whitespace() {
                    // Token ended.
                    break;
                } else {
                    if len < 20 {
                        inbuf[len] = b;
                        len += 1;
                    } else {
                        // Token length would exceed 20 -> undefined per spec.
                        // We abort (panic) here to avoid silent truncation or memory corruption.
                        panic!("Input token length exceeds 20 bytes (undefined behavior per spec)");
                    }
                }
            }
            Err(e) => {
                panic!("Error reading stdin: {} (undefined behavior per spec)", e);
            }
        }
    }

    // Place terminating NUL in input buffer to mirror scanf("%s") behavior.
    // This is consistent with the specification's precondition that scanf null-terminates the token.
    if len <= 20 {
        inbuf[len] = 0;
    } else {
        // Shouldn't happen due to checks above; treat as undefined.
        panic!("Internal length exceeded buffer capacity (undefined behavior per spec)");
    }

    // Compute length as would strlen (we already have `len`, but compute from inbuf to stay faithful).
    // This also follows the specification's mention of computing length; using the computed `len`
    // is equivalent and avoids extra scanning, but we still show the equivalent operation.
    let mut strlen_len: usize = 0;
    while strlen_len < 21 && inbuf[strlen_len] != 0 {
        strlen_len += 1;
    }
    // According to the preconditions, strlen_len is in [1,20]. If not, behavior is undefined;
    // we check to ensure safe operation.
    if strlen_len == 0 || strlen_len > 20 {
        panic!("Token length out of expected range after reading (undefined behavior per spec)");
    }

    // Reverse characters from inbuf into outbuf. Note: per specification the original program
    // did not necessarily put a terminating NUL in outbuf; we deliberately do not set outbuf[strlen_len]=0.
    for i in 0..strlen_len {
        outbuf[i] = inbuf[strlen_len - 1 - i];
    }

    // Write exactly L bytes to stdout (the reversed token), no newline appended.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle
        .write_all(&outbuf[..strlen_len])
        .expect("Failed to write to stdout");

    // Normal termination: return exit code 0 by falling off main.
}
