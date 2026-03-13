use std::io::{self, Read};
use std::str;
use std::time::Duration;
use std::thread::sleep;

/// Translated from the provided C program according to the SPEC.
/// Key semantic choices made to follow the SPEC:
/// - We read the entire stdin into a buffer and then scan it token-by-token,
///   emulating scanf("%d") behavior: skip whitespace, try to parse an optional
///   sign followed by at least one digit. If no digit is present after an
///   optional sign (i.e. the next token is a non-integer token), the original
///   C program's scanf would return 0 and repeatedly fail on that same token,
///   causing the program to "spin" (not progress). To preserve this behavior
///   we enter an infinite sleep loop (so the process does not terminate nor
///   produce further output). This matches the SPEC description of that case.
/// - We do not validate that parsed integers lie in 0..=100. As in the SPEC/C
///   program, out-of-range values will lead to out-of-bounds indexing. In Rust,
///   that will typically cause a panic rather than undefined memory behavior.
///   (Avoiding `unsafe` was preferred; intentionally reproducing undefined
///   memory corruption would require `unsafe` and is not done here.)
fn main() {
    // Fixed-size counts for values 0..=100 inclusive.
    let mut counts: [u64; 101] = [0u64; 101];

    // Read all stdin into a buffer.
    let mut input = Vec::new();
    // Read to end; this blocks until EOF, which matches the SPEC assumption
    // that stdin eventually hits EOF for normal completion.
    io::stdin().read_to_end(&mut input).expect("failed to read stdin");

    let len = input.len();
    let mut pos: usize = 0;

    // Scanner that emulates scanf("%d") semantics:
    // - skip whitespace
    // - if EOF -> stop
    // - if next sequence is optional sign followed by >=1 digit -> parse and consume
    // - otherwise -> scanf would return 0 and not consume anything; emulate by
    //   spinning indefinitely (sleep loop) so the program does not progress.
    while {
        // skip ASCII whitespace
        while pos < len {
            let b = input[pos];
            if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' || b == 0x0B || b == 0x0C {
                pos += 1;
            } else {
                break;
            }
        }

        if pos >= len {
            // EOF reached
            false
        } else {
            // attempt to parse integer at current pos
            let start = pos;
            // optional sign
            if input[pos] == b'+' || input[pos] == b'-' {
                pos += 1;
            }

            // must have at least one digit after optional sign
            if pos >= len || !(input[pos] as char).is_ascii_digit() {
                // Emulate scanf("%d") returning 0 and not consuming input:
                // infinite "spin" described in SPEC. To avoid burning CPU,
                // sleep repeatedly (still never progresses nor produces output).
                loop {
                    sleep(Duration::from_secs(1));
                }
            }

            // consume digits
            while pos < len && (input[pos] as char).is_ascii_digit() {
                pos += 1;
            }

            // parse the substring start..pos as an integer (i32 as in C int)
            // We create a &str from the slice; inputs are assumed valid per SPEC.
            let slice = &input[start..pos];
            let s = match str::from_utf8(slice) {
                Ok(s) => s,
                // If invalid UTF-8 appears here, treat it as non-integer token:
                // emulate scanf behavior -> spin indefinitely.
                Err(_) => loop {
                    sleep(Duration::from_secs(1));
                },
            };

            // Parse into i32. Per SPEC, tokens are assumed valid integers.
            // If parsing fails (e.g. out-of-range for i32), we propagate the error
            // by panicking; this approximates the unspecified behavior for
            // malformed/out-of-range tokens.
            let number: i32 = match s.parse() {
                Ok(n) => n,
                Err(_) => {
                    // emulate repeated scanf failure by spinning indefinitely
                    loop {
                        sleep(Duration::from_secs(1));
                    }
                }
            };

            // Increment count for this number. This intentionally does NOT
            // validate that 0 <= number <= 100 to mirror the C program and
            // SPEC (out-of-range inputs result in undefined / erroneous behavior).
            // In Rust, an out-of-range index will panic (bounds check).
            let idx = number as isize as usize;
            counts[idx] = counts[idx].wrapping_add(1);

            // continue reading
            true
        }
    } {}

    // Determine mx_number according to the original tie-breaking:
    // Start with mx_number = 0, scan number = 100 down to 1; if counts[number] > counts[mx_number]
    // then mx_number = number. This yields the largest index among tied maxima.
    let mut mx_number: usize = 0;
    for number in (1..=100).rev() {
        if counts[number] > counts[mx_number] {
            mx_number = number;
        }
    }

    // Print every integer in 1..=mx_number whose count equals counts[mx_number],
    // one per line, in ascending order.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for number in 1..=mx_number {
        if counts[number] == counts[mx_number] {
            // write "number\n"
            use std::io::Write;
            writeln!(handle, "{}", number).expect("write failed");
        }
    }

    // exit status 0 by returning from main
}
