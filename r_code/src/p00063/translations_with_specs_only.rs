// palindrome_counter.rs
//
// Reads whitespace-delimited tokens (byte sequences) from stdin and counts how many
// tokens are exact palindromes (byte-wise equality to their reverse). Prints the
// final count as a decimal integer followed by a newline.
//
// This implementation follows the provided specification:
// - Uses a fixed-size internal buffer. If a token would exceed the buffer capacity,
//   the program will panic (crash). This choice models the specification's statement
//   that behavior is undefined for oversized tokens (the original C-program would
//   likely overflow memory in that case). We deliberately do not attempt to recover
//   or silently truncate tokens.
// - Tokenization uses the standard C notion of whitespace: space, form-feed, newline,
//   carriage return, horizontal tab, vertical tab.
// - Comparison is byte-wise (no Unicode/UTF-8 validation or normalization).
// - On I/O read errors we stop scanning and print the current count.

use std::io::{self, Read};

const BUFFER_SIZE: usize = 4096; // fixed buffer capacity; includes room for a terminating NUL in the C-spec sense

fn is_c_whitespace(byte: u8) -> bool {
    match byte {
        b' ' | b'\t' | b'\n' | b'\r' | 0x0b /* vertical tab */ | 0x0c /* form feed */ => true,
        _ => false,
    }
}

fn main() {
    // count of palindromic tokens seen so far
    let mut count: u64 = 0;

    // fixed-size buffer used to accumulate a token's bytes
    let mut buf = [0u8; BUFFER_SIZE];

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut byte_iter = handle.bytes();

    // Helper closure to get the next byte.
    // Returns Some(byte) on success, None on EOF or read error (per spec, treat read errors as EOF).
    let mut next_byte = || -> Option<u8> {
        match byte_iter.next() {
            Some(Ok(b)) => Some(b),
            Some(Err(_)) => None, // treat read errors as EOF per specification
            None => None,
        }
    };

    // Main loop: skip whitespace, then read a token, check palindrome.
    loop {
        // Skip leading whitespace
        let b = match next_byte() {
            Some(b) => {
                if is_c_whitespace(b) {
                    // keep skipping
                    let mut bb = b;
                    loop {
                        match next_byte() {
                            Some(nb) => {
                                bb = nb;
                                if !is_c_whitespace(bb) {
                                    break Some(bb);
                                }
                            }
                            None => break None, // EOF or read error while skipping whitespace
                        }
                    }
                } else {
                    // first non-whitespace byte found
                    Some(b)
                }
            }
            None => None, // EOF immediately
        };

        let first_byte = match b {
            Some(x) => x,
            None => break, // EOF; done reading tokens
        };

        // Start collecting token bytes into buf.
        // According to the specification, tokens must fit into the internal buffer including a terminating NUL;
        // if they don't, behavior is undefined. Here we model that by panicking when the capacity would be exceeded.
        let mut len: usize = 0;

        // store first byte
        if BUFFER_SIZE == 0 {
            // defensive: BUFFER_SIZE should be > 0, but if it were 0, can't store anything.
            // Panic to reflect undefined behavior for zero-sized buffer.
            panic!("internal buffer has zero capacity");
        }
        buf[len] = first_byte;
        len += 1;

        // Read remaining bytes of the token
        loop {
            match next_byte() {
                Some(nb) => {
                    if is_c_whitespace(nb) {
                        // token terminated; whitespace should be considered as separator (and not part of next token)
                        break;
                    } else {
                        // check capacity:
                        // The specification counts the terminating NUL in buffer capacity. We emulate that by
                        // allowing token byte count up to BUFFER_SIZE - 1. If len == BUFFER_SIZE - 1, adding another
                        // byte would exceed capacity; panic to model undefined behavior.
                        if len >= BUFFER_SIZE - 1 {
                            // Token too long for fixed buffer: undefined per spec; cause a crash/panic here.
                            panic!("token exceeds internal buffer capacity (undefined behavior per specification)");
                        }
                        buf[len] = nb;
                        len += 1;
                    }
                }
                None => {
                    // EOF or read error: token ends here
                    break;
                }
            }
        }

        // At this point, token bytes are in buf[0..len].
        // (Per C-style treatment we could set buf[len] = 0 as NUL terminator, but not required for palindrome check.)
        // Check palindrome: exact byte-wise equality to its reverse.
        let mut is_palindrome = true;
        let mut i = 0usize;
        while i < len / 2 {
            if buf[i] != buf[len - 1 - i] {
                is_palindrome = false;
                break;
            }
            i += 1;
        }
        if is_palindrome {
            count = count.wrapping_add(1); // wrapping add matches monotonic non-negative integer; overflow extremely unlikely
        }
        // Continue to next token
    }

    // Print the final count followed by a newline. Exit with status 0 on normal completion.
    // Use a simple write to stdout to avoid any extra formatting or locale dependence.
    {
        let mut out = io::stdout();
        // We choose to format using ASCII decimal and a newline.
        // Using unwrap here is acceptable: on stdout write errors there's nothing further to do per spec.
        write!(out, "{}\n", count).unwrap();
    }
}

// Minimal imports used by write! macro
use std::fmt::Write as _; // to satisfy write! macro resolution when using out.write instead of fmt (not strictly necessary)
use std::io::Write;
