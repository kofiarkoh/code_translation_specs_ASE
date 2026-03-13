// translator.rs
//
// Translation of the provided Specification into Rust.
//
// Notes on translation choices (brief):
// - The C "int" type from the specification is mapped to Rust's i32.
//   This is a common practical mapping and matches the "platform's C int range"
//   assumption for typical platforms.
// - The program parses whitespace-separated decimal integer tokens with an
//   optional '+' or '-' sign, matching scanf("%d") token form. The
//   implementation assumes well-formed tokens as the Specification does.
//   If a malformed token is encountered (non-digit where a digit is expected),
//   the program stops processing and exits with status 0. The Specification
//   marks such behavior as unspecified; choosing to terminate is the narrowest
//   reasonable behavior.
//
// - Arithmetic (both parsing accumulation and the 1 + sum_{1..a} calculation)
//   uses wrapping arithmetic (i32::wrapping_*). The Specification states that
//   overflow in C is undefined; many real-world C implementations exhibit
//   two's-complement wrapping behavior. Using wrapping arithmetic here is a
//   conservative choice to emulate typical low-level behavior while keeping
//   the program defined in Rust. This choice is documented here.
//
// - The sum is computed by iterating i = 1..=a (linear in a for a >= 1),
//   matching the specified O(a) implementation detail rather than using a
//   closed-form formula.
//
// - No heap allocation is performed by this code: input is processed from
//   stdin in fixed-size stack buffer chunks and integers are parsed streaming
//   without constructing owned Strings or Vecs.
//
// Behavior summary:
// - For each integer token a read from stdin (decimal, optional sign), write one
//   line to stdout containing:
//     - if a >= 1: 1 + sum_{i=1..a} i (computed by iterating and summing)
//     - if a <= 0: 1
// - Processing continues until EOF or until a malformed token is encountered.
// - On EOF or termination, the program exits normally with status 0.

use std::io::{self, Read, Write};

fn main() {
    // Map "C int" to i32
    // Use a fixed-size buffer to avoid dynamic allocation.
    const BUF_SIZE: usize = 8192;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdin.lock();
    let mut out = stdout.lock();

    let mut buf = [0u8; BUF_SIZE];
    // Current window positions within buf
    let mut start: usize = 0;
    let mut end: usize = 0;
    // Whether we've reached EOF on input
    let mut eof = false;

    // Helper to get next byte, refilling buffer as necessary.
    // Returns Ok(Some(byte)) if a byte is available,
    // Ok(None) if EOF,
    // Err(io::Error) on read error.
    let mut next_byte = || -> io::Result<Option<u8>> {
        if start >= end {
            // refill
            end = handle.read(&mut buf)?;
            start = 0;
            if end == 0 {
                eof = true;
                return Ok(None);
            }
        }
        let b = buf[start];
        start += 1;
        Ok(Some(b))
    };

    // Main loop: repeatedly parse tokens and produce outputs.
    loop {
        // Skip leading whitespace
        let mut b_opt = match next_byte() {
            Ok(opt) => opt,
            Err(_) => {
                // On read error, per specification stop processing and exit 0.
                break;
            }
        };
        // Skip whitespace bytes; if EOF reached, exit loop normally.
        while let Some(b) = b_opt {
            if !(b as char).is_ascii_whitespace() {
                break;
            }
            b_opt = match next_byte() {
                Ok(opt) => opt,
                Err(_) => None,
            };
        }
        // If EOF and no non-whitespace byte found, we're done.
        let b = match b_opt {
            Some(x) => x,
            None => break,
        };

        // Now b is the first byte of a token (non-whitespace).
        // Parse optional sign.
        let mut negative = false;
        let mut cur_b = b;
        if cur_b == b'+' || cur_b == b'-' {
            if cur_b == b'-' {
                negative = true;
            }
            // peek next byte to see if there's at least one digit.
            let next = match next_byte() {
                Ok(opt) => opt,
                Err(_) => {
                    // read error -> stop processing
                    break;
                }
            };
            let nb = match next {
                Some(v) => v,
                None => {
                    // EOF immediately after sign: this is a malformed token.
                    // The Specification leaves malformed tokens unspecified.
                    // Choose to terminate processing.
                    break;
                }
            };
            cur_b = nb;
        }

        // Now cur_b should be a digit [0-9]; otherwise token is malformed.
        if !((cur_b as char).is_ascii_digit()) {
            // Malformed token encountered. Spec: behavior unspecified.
            // We terminate processing here (narrowest reasonable choice).
            break;
        }

        // Parse digits streaming without heap allocation, using wrapping arithmetic.
        let mut val: i32 = 0;
        loop {
            // cur_b is a digit byte here
            let digit = (cur_b - b'0') as i32;
            val = val.wrapping_mul(10).wrapping_add(digit);

            // Peek next byte without consuming extra reads beyond buffer refill:
            // Use next_byte to consume; if it's whitespace, token ends.
            // If it's digit, continue loop. If it's non-digit non-whitespace,
            // treat it as malformed per earlier decision and terminate processing.
            let next = match next_byte() {
                Ok(opt) => opt,
                Err(_) => {
                    // read error -> stop processing
                    eof = true;
                    None
                }
            };
            match next {
                Some(nb) => {
                    if (nb as char).is_ascii_digit() {
                        cur_b = nb;
                        continue;
                    } else if (nb as char).is_ascii_whitespace() {
                        // token finished
                        break;
                    } else {
                        // Non-digit non-whitespace inside token -> malformed.
                        // Spec marks such cases unspecified; terminate.
                        eof = true;
                        // We choose to stop processing entirely.
                        // (We do not attempt to un-read this byte.)
                        break;
                    }
                }
                None => {
                    // EOF after digits: token finished; proceed.
                    break;
                }
            }
        }

        // Apply sign
        if negative {
            val = val.wrapping_neg();
        }

        // Compute output: if a <= 0 => 1; else 1 + sum_{i=1..a} i.
        // Sum is computed iteratively (linear in a) using wrapping arithmetic.
        let result: i32 = if val <= 0 {
            1
        } else {
            let mut sum: i32 = 0;
            // Using i32 inclusive range. This will be linear in a as required.
            // Note: iterating up to i32::MAX would be extremely slow but matches spec.
            let mut i = 1i32;
            // Use while loop to avoid creating large Range objects and to be explicit.
            while i <= val {
                sum = sum.wrapping_add(i);
                i = i.wrapping_add(1);
            }
            sum.wrapping_add(1)
        };

        // Write result followed by newline. Ignore write errors per spec.
        let _ = out.write_fmt(format_args!("{}\n", result));

        // Continue to next token. If EOF indicator was set due to malformed token
        // handling above, break now.
        if eof {
            break;
        }
    }

    // Normal termination with exit code 0 (implicit).
}
