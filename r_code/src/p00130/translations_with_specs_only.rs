// main.rs
//
// Translation of the provided specification into Rust.
// Behavior notes (kept faithful to the spec):
// - Reads an integer `n` from stdin (skipping initial whitespace).
// - Then processes up to `n` encoded sequences from the remaining input bytes.
// - For each sequence:
//     * Uses a fixed buffer of 27 bytes (indices 0..=26), cursor starts at index 13.
//     * Repeatedly reads one visible byte and writes it at the cursor.
//     * After each write, reads one byte as a movement marker:
//         - '-' => move cursor +1
//         - '<' => move cursor -1
//         - any other byte => consumed and terminates the current sequence
//     * If a movement marker was '-' or '<', the implementation will also attempt
//       to consume one subsequent byte (a separator) if available, as described
//       in the specification.
//     * Sequence ends when the movement-marker byte is neither '-' nor '<',
//       or when EOF is encountered while reading required bytes.
// - For each completed sequence, prints the contiguous run from the first
//   occupied slot to the first empty slot after it, then a newline.
// - If EOF or parsing failure occurs early, processing stops early silently.
//
// Notes about preserved corner cases:
// - The code indexes the fixed-size buffer directly with the cursor (usize).
//   If input causes the cursor to move out of bounds (not in [0,26]), this
//   will panic (matching the specification's notion of undefined behavior).
//
// Implementation choices:
// - Read the entire stdin into a Vec<u8> to allow simple byte-wise parsing and
//   consumption semantics as outlined in the specification.
// - Parse the initial integer by skipping ASCII whitespace and consuming digits.
// - All byte reads are non-whitespace-preserving except for the integer parse
//   which skips whitespace per typical scanf-like behavior.

use std::io::{self, Read, Write};

fn parse_nonnegative_int(bytes: &[u8], mut pos: usize) -> Option<(usize, usize)> {
    // Skip ASCII whitespace
    while pos < bytes.len() {
        match bytes[pos] {
            b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c => pos += 1,
            _ => break,
        }
    }
    if pos >= bytes.len() {
        return None;
    }
    // Optional '+' allowed; '-' would make it negative which the spec does not expect.
    if bytes[pos] == b'+' {
        pos += 1;
    }
    // Must have at least one digit
    if pos >= bytes.len() || !(b'0'..=b'9').contains(&bytes[pos]) {
        return None;
    }
    let mut value: usize = 0;
    while pos < bytes.len() {
        let b = bytes[pos];
        if !(b'0'..=b'9').contains(&b) {
            break;
        }
        // Decimal parsing; on overflow we wrap as in typical C unsigned behavior is not specified
        // in the spec for n, but we'll saturate by allowing usize wrapping (default in Rust will panic
        // on overflow in debug). To stay simple and safe, use checked_mul/add and return None on overflow.
        // This is a conservative choice (do not invent behavior).
        match value.checked_mul(10).and_then(|v| v.checked_add((b - b'0') as usize)) {
            Some(v) => value = v,
            None => return None,
        }
        pos += 1;
    }
    Some((value, pos))
}

fn main() {
    // Read all stdin
    let mut stdin_bytes = Vec::new();
    if io::stdin().read_to_end(&mut stdin_bytes).is_err() {
        // If reading stdin fails, exit quietly.
        return;
    }

    let mut pos: usize = 0;
    // Parse n
    let (n, new_pos) = match parse_nonnegative_int(&stdin_bytes, pos) {
        Some(p) => p,
        None => {
            // As spec: if scanf fails when reading n, processing may stop early.
            return;
        }
    };
    pos = new_pos;

    let mut stdout = io::stdout();
    let mut out_buf = Vec::new();

    for _seq_idx in 0..n {
        // Initialize buffer of 27 slots; use Option<u8> to track empty/occupied.
        let mut buf: [Option<u8>; 27] = [None; 27];
        // Cursor starts at center index 13
        let mut cursor: usize = 13;

        // Read first visible character for the sequence
        // If EOF, stop processing further sequences.
        if pos >= stdin_bytes.len() {
            break;
        }
        // Visible character is consumed as-is (no skipping)
        let ch = stdin_bytes[pos];
        pos += 1;
        // Write into buffer; this may panic if cursor out of bounds, matching undefined behavior.
        buf[cursor] = Some(ch);

        // After each write, we attempt to read a movement marker byte.
        loop {
            if pos >= stdin_bytes.len() {
                // EOF: sequence ends here.
                break;
            }
            let marker = stdin_bytes[pos];
            pos += 1; // consume marker byte

            if marker == b'-' {
                // move right
                cursor = cursor.wrapping_add(1);
                // Consume one subsequent byte (separator) if available.
                if pos < stdin_bytes.len() {
                    pos += 1;
                }
                // Attempt to read next visible char
                if pos >= stdin_bytes.len() {
                    // EOF -> sequence ends prematurely
                    break;
                }
                let next_ch = stdin_bytes[pos];
                pos += 1;
                buf[cursor] = Some(next_ch);
                continue;
            } else if marker == b'<' {
                // move left
                cursor = cursor.wrapping_sub(1);
                // Consume one subsequent byte (separator) if available.
                if pos < stdin_bytes.len() {
                    pos += 1;
                }
                // Attempt to read next visible char
                if pos >= stdin_bytes.len() {
                    // EOF -> sequence ends prematurely
                    break;
                }
                let next_ch = stdin_bytes[pos];
                pos += 1;
                buf[cursor] = Some(next_ch);
                continue;
            } else {
                // Any other byte terminates this sequence (the byte is consumed already).
                break;
            }
        }

        // Now produce output: find first occupied slot, then print from there until first empty slot.
        let mut start = None;
        for i in 0..27 {
            if buf[i].is_some() {
                start = Some(i);
                break;
            }
        }
        if let Some(s) = start {
            let mut i = s;
            while i < 27 {
                match buf[i] {
                    Some(b) => {
                        out_buf.push(b);
                        i += 1;
                    }
                    None => break,
                }
            }
        }
        // Append newline
        out_buf.push(b'\n');

        // Flush output buffer if it grows large, to avoid holding too much memory (practical).
        if out_buf.len() > 4096 {
            let _ = stdout.write_all(&out_buf);
            out_buf.clear();
        }
        // Buffer and cursor are reinitialized at next iteration as required by spec.
    }

    // Write any remaining output
    let _ = stdout.write_all(&out_buf);
}
