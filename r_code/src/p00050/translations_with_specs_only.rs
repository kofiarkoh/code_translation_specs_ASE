// translator.rs
//
// Translation of the provided specification into Rust.
// Behavior summary:
// - Read a single line from stdin, up to 1001 bytes of content (newline allowed).
// - Scan left-to-right and swap exact 5-byte occurrences:
//     "apple" <-> "peach"  and  "Apple" <-> "Peach"
// - Replacements are applied in-place immediately as the scan advances.
// - The entire (possibly modified) line is written to stdout (the newline, if read, is preserved).
//
// Notes on fidelity to the specification:
// - The original C description mentioned a fixed-size buffer with room for 1001 bytes
//   of content plus a NUL terminator. We emulate that by limiting input to 1001 bytes
//   and using a buffer of size 1002 (final slot can hold a NUL if needed).
// - The spec also remarked that the original stored lengths/indices in `char`, which
//   can cause implementation-defined behavior for large inputs. To remain practical
//   and safe in Rust, this implementation uses usize for indexing and processes the
//   line correctly for lengths up to the stated capacity (1001). The spec's undefined
//   behaviors for out-of-range lengths are not reproduced here.
// - All character matching is done on raw bytes (u8), consistent with the spec's
//   ASCII/single-byte assumption.

use std::io::{self, Read, Write};

fn main() {
    // Buffer size: 1001 bytes of content + 1 byte for terminating NUL (emulated).
    const CONTENT_CAPACITY: usize = 1001;
    const BUF_LEN: usize = CONTENT_CAPACITY + 1; // +1 for NUL as in C

    let mut buf = [0u8; BUF_LEN];
    let mut pos: usize = 0;

    // Read bytes from stdin until newline, EOF, or we reached CONTENT_CAPACITY.
    // This emulates fgets-like behavior: at most CONTENT_CAPACITY bytes of content
    // are read (the terminating NUL is kept in buf[pos]).
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut single_byte = [0u8; 1];
    loop {
        if pos >= CONTENT_CAPACITY {
            // reached maximum content capacity; stop reading further bytes
            break;
        }
        match handle.read(&mut single_byte) {
            Ok(0) => {
                // EOF
                break;
            }
            Ok(1) => {
                let b = single_byte[0];
                buf[pos] = b;
                pos += 1;
                if b == b'\n' {
                    // stop after reading newline, like fgets
                    break;
                }
                // otherwise continue reading
            }
            Ok(_) => unreachable!(),
            Err(_) => {
                // On read error, narrow reasonable behavior: stop reading.
                break;
            }
        }
    }

    // Place terminating NUL as in C semantics (not strictly required for Rust output,
    // but the spec describes printing up to the first NUL terminator).
    if pos < BUF_LEN {
        buf[pos] = 0;
    } else {
        // shouldn't happen because pos <= CONTENT_CAPACITY and BUF_LEN = CONTENT_CAPACITY+1
        buf[BUF_LEN - 1] = 0;
    }

    // Work on the content up to the first NUL (pos). If pos < 5, no substitutions occur.
    if pos >= 5 {
        // Byte patterns to match/replace
        let apple = b"apple";
        let peach = b"peach";
        let Apple = b"Apple";
        let Peach = b"Peach";

        // Scan left-to-right, applying replacements immediately.
        // We advance by 1 each iteration; replacements can affect later matches.
        let mut i: usize = 0;
        // last starting index where a 5-byte substring can begin is pos - 5
        let last_start = pos - 5;
        while i <= last_start {
            let slice = &buf[i..i + 5];
            if slice == apple {
                buf[i..i + 5].copy_from_slice(peach);
            } else if slice == peach {
                buf[i..i + 5].copy_from_slice(apple);
            } else if slice == Apple {
                buf[i..i + 5].copy_from_slice(Peach);
            } else if slice == Peach {
                buf[i..i + 5].copy_from_slice(Apple);
            }
            i += 1;
        }
    }

    // Write the modified content up to the first NUL terminator to stdout.
    // The spec expects writing the buffer up to the first NUL. We use `pos`
    // which is the index where we placed the NUL; this prints the read content,
    // with modifications, preserving any newline that was read.
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let _ = out.write_all(&buf[..pos]);
    let _ = out.flush();

    // Exit code 0 on normal completion (implicit).
}
