use std::io::{self, Read};

/// Affine substitution and search program translated from the given specification.
///
/// Behavior summary (per specification):
/// - Read an initial integer n (first token) from stdin.
/// - Then read exactly n newline-terminated lines.
/// - For each line, for a in 0..=99, for b in 0..=99 (a outer, b inner),
///   check every contiguous 4-character window: apply the affine mapping
///   to each character in the window and see if it becomes "this" or "that".
/// - If a match is found, print the entire line transformed by the same affine
///   mapping (spaces preserved) followed by a newline, then exit (return 0).
/// - If no match is ever found, exit without printing anything.
///
/// Notes on implementation decisions (to stay faithful to the specification):
/// - The affine mapping is defined for lowercase letters as:
///     mapped = ((a * (c - 'a') + b) mod 26) + 'a'
///   For non-space characters outside 'a'..'z', we still compute a result by
///   treating the byte as unsigned and using a wrapped subtraction from 'a',
///   then reducing mod 26. This mirrors the idea that inputs are expected to be
///   lowercase or spaces but tries not to panic on other bytes.
/// - Spaces are preserved when printing (and are mapped to space by affine_byte).
/// - Lines are extracted from the input after the first integer token. We skip
///   a single newline immediately following the integer token if present to
///   align with the specification that the integer precedes newline-terminated lines.
///
/// This program avoids unsafe code and uses only the Rust standard library.

fn affine_byte(c: u8, a: i32, b: i32) -> u8 {
    // Preserve spaces explicitly as specified.
    if c == b' ' {
        return b' ';
    }

    // For lowercase ASCII letters, do the standard affine mapping.
    if (b'a'..=b'z').contains(&c) {
        let x = (c - b'a') as i32;
        let mapped = (a * x + b).rem_euclid(26);
        (b'a' + mapped as u8)
    } else {
        // For other bytes, emulate a reasonable mapping by wrapping subtraction
        // then reducing mod 26. The spec does not define behavior for these,
        // so this choice keeps the program defined and consistent.
        let x = c.wrapping_sub(b'a') as i32;
        let mapped = (a * x + b).rem_euclid(26);
        (b'a' + mapped as u8)
    }
}

fn main() {
    // Read all stdin into a string. The specification assumes text input.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Per spec, error behavior is unspecified; simply exit quietly.
        return;
    }

    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0_usize;

    // Skip leading whitespace to find the first token (the integer n).
    while pos < len && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    if pos >= len {
        // No token present; exit quietly.
        return;
    }

    // Find end of integer token.
    let start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    let int_token = &input[start..pos];
    let n: usize = match int_token.parse() {
        Ok(v) => v,
        Err(_) => {
            // Malformed integer: specification leaves this unspecified.
            // Exit without output.
            return;
        }
    };

    // After the integer token, skip a single newline if present to align with
    // "Then n newline-terminated text lines."
    if pos < len {
        if bytes[pos] == b'\r' {
            // Handle optional CRLF: skip '\r' and possibly '\n'
            pos += 1;
            if pos < len && bytes[pos] == b'\n' {
                pos += 1;
            }
        } else if bytes[pos] == b'\n' {
            pos += 1;
        }
        // If the next character is some other whitespace (e.g., space), we do not
        // skip it here because the specification says lines are newline-terminated
        // and typically the integer will be followed by a newline.
    }

    // Extract the remainder string slice from which to obtain exactly n lines.
    let remainder = &input[pos..];

    // Use split_terminator to respect newline-terminated lines (it does not
    // produce a final empty element if the string ends with a newline).
    let mut lines_iter = remainder.split_terminator('\n');

    // Targets to check after mapping
    let target_this = b"this";
    let target_that = b"that";

    // For each of the n lines (in order)
    for _line_idx in 0..n {
        let line = match lines_iter.next() {
            Some(s) => s,
            None => {
                // Fewer than n lines available: unspecified in spec. Exit quietly.
                return;
            }
        };

        let line_bytes = line.as_bytes();
        let L = line_bytes.len();

        // Lines shorter than 4 cannot match; continue to next line.
        if L < 4 {
            continue;
        }

        // Iterate a then b in lexicographic order a=0..=99, b=0..=99
        let mut found = false;
        let mut found_a = 0_i32;
        let mut found_b = 0_i32;

        'outer_ab: for a in 0_i32..=99 {
            for b in 0_i32..=99 {
                // For each 4-character window left-to-right
                for start_idx in 0..=(L - 4) {
                    // Map the 4 bytes and compare to targets
                    let c0 = affine_byte(line_bytes[start_idx], a, b);
                    let c1 = affine_byte(line_bytes[start_idx + 1], a, b);
                    let c2 = affine_byte(line_bytes[start_idx + 2], a, b);
                    let c3 = affine_byte(line_bytes[start_idx + 3], a, b);

                    if [c0, c1, c2, c3] == target_this[..] || [c0, c1, c2, c3] == target_that[..] {
                        found = true;
                        found_a = a;
                        found_b = b;
                        break 'outer_ab;
                    }
                }
            }
        }

        if found {
            // Print the entire line transformed by (found_a, found_b).
            let mut out = Vec::with_capacity(L + 1);
            for &bch in line_bytes {
                if bch == b' ' {
                    out.push(b' ');
                } else {
                    out.push(affine_byte(bch, found_a, found_b));
                }
            }
            out.push(b'\n');
            // Write to stdout. Ignore potential write errors; spec does not define them.
            let _ = io::stdout().write_all(&out);
            return; // Terminate immediately after printing.
        }
    }

    // If no match was found after all lines and parameters, exit without printing.
}

// Need to import Write for stdout write_all
use std::io::Write;
