use std::io::{self, Read, Write};

fn main() {
    // Read entire stdin into memory once. This simplifies implementing the
    // fgets-like chunked reads (up to 1023 bytes per read, newline inclusive).
    // The specification allows interpreting input as ASCII/UTF-8 compatible;
    // we operate on raw bytes to faithfully emulate C byte-oriented behavior.
    let mut all = Vec::new();
    if io::stdin().read_to_end(&mut all).is_err() {
        // If reading stdin fails, exit quietly (no guaranteed behavior in spec).
        return;
    }

    // Parse the initial integer n similar to scanf("%d ") behavior:
    // - skip leading whitespace
    // - parse optional sign and digits
    // - after the digits, skip all following whitespace (the trailing space in "%d ")
    //   so position points to the first non-whitespace after the integer (or EOF).
    let (n, mut pos) = match parse_int_and_consume_trailing_space(&all) {
        Some(tuple) => tuple,
        None => {
            // Spec: if initial integer cannot be read, behavior unspecified.
            // We choose to exit without producing output.
            return;
        }
    };

    // If n is negative, behavior is unspecified in the spec. We choose to do nothing.
    if n < 0 {
        return;
    }
    let mut remaining_reads = n as usize;

    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Process up to `remaining_reads` chunks, each emulating fgets with buffer size 1024:
    // read up to 1023 bytes or until a newline (inclusive), whichever comes first.
    while remaining_reads > 0 {
        // If we've reached EOF, then fgets would return NULL. Spec says behavior is
        // not strictly defined; we stop producing outputs here.
        if pos >= all.len() {
            break;
        }

        // Determine chunk end: read up to 1023 bytes or until newline inclusive.
        let max_chunk = 1023usize;
        let start = pos;
        let mut end = start;
        let mut bytes_read = 0usize;
        while pos < all.len() && bytes_read < max_chunk {
            let b = all[pos];
            pos += 1;
            end += 1;
            bytes_read += 1;
            if b == b'\n' {
                break; // include the newline then stop, like fgets
            }
        }

        // Extract the chunk as a mutable vector of bytes to perform substitutions.
        let mut chunk = all[start..end].to_vec();

        // Perform in-place replacement of every non-overlapping occurrence of
        // the ASCII byte sequence b"Hoshino" -> change last byte 'o' (0x6F) to 'a' (0x61)
        // resulting in b"Hoshina".
        let pattern = b"Hoshino";
        let pat_len = pattern.len(); // 7

        let mut i = 0usize;
        while i + pat_len <= chunk.len() {
            if &chunk[i..i + pat_len] == pattern {
                // Replace the 7th character (index 6) from 'o' to 'a'
                chunk[i + 6] = b'a';
                i += pat_len; // advance to avoid overlapping matches
            } else {
                i += 1;
            }
        }

        // Write the processed chunk exactly once to stdout.
        if out.write_all(&chunk).is_err() {
            // On write error, just stop (no special error handling per spec).
            break;
        }

        remaining_reads -= 1;
    }

    // Flush output to ensure all data is written before exit.
    let _ = out.flush();
}

/// Parse an initial signed integer from bytes similar to scanning with scanf("%d ").
/// Returns Some((value, next_pos)) where next_pos is the index into `buf` after
/// consuming the integer and any trailing whitespace (emulating the trailing space in "%d ").
/// Returns None if no valid integer is found.
fn parse_int_and_consume_trailing_space(buf: &[u8]) -> Option<(i32, usize)> {
    let len = buf.len();
    let mut i = 0usize;

    // Skip leading whitespace
    while i < len && is_ascii_whitespace(buf[i]) {
        i += 1;
    }

    if i >= len {
        return None;
    }

    // Optional sign
    let mut sign = 1i64;
    if buf[i] == b'+' {
        i += 1;
    } else if buf[i] == b'-' {
        sign = -1;
        i += 1;
    }

    // At least one digit must follow
    let start_digits = i;
    let mut acc: i64 = 0;
    while i < len && (buf[i] as char).is_ascii_digit() {
        // Accumulate in i64 to avoid overflow; final cast to i32 follows spec assumption.
        acc = acc * 10 + ((buf[i] - b'0') as i64);
        i += 1;
    }

    if i == start_digits {
        // No digits found
        return None;
    }

    acc *= sign;

    // Emulate trailing space in scanf format: skip all whitespace after the integer.
    while i < len && is_ascii_whitespace(buf[i]) {
        i += 1;
    }

    // Cast to i32; if out of range, we clamp to i32 range to produce a deterministic result.
    // Spec assumes a valid integer convertible by scanf, so this is only defensive.
    let value = if acc > (i32::MAX as i64) {
        i32::MAX
    } else if acc < (i32::MIN as i64) {
        i32::MIN
    } else {
        acc as i32
    };

    Some((value, i))
}

#[inline]
fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\t' | b'\r' | 0x0B | 0x0C)
}
