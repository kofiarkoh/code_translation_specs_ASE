// translated from provided C code
// Preserves semantics of scanf("%d ", &n) and fgets(in, 1023, stdin) as implemented below.
// Operates on raw bytes to avoid UTF-8 issues and to match exact stdout output.

use std::io::{self, Read, Write};

fn is_ascii_whitespace(b: u8) -> bool {
    // mimic C isspace for ASCII whitespace characters
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
}

fn find_subslice_from(buf: &[u8], pat: &[u8], start: usize) -> Option<usize> {
    if pat.len() == 0 || buf.len() < pat.len() || start >= buf.len() {
        return None;
    }
    let max = buf.len().saturating_sub(pat.len());
    for i in start..=max {
        if &buf[i..i + pat.len()] == pat {
            return Some(i);
        }
    }
    None
}

fn main() -> io::Result<()> {
    // Read entire stdin into memory, then simulate the C-style buffered reads (fgets with limit)
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let mut pos: usize = 0;
    let len = input.len();

    // Emulate scanf("%d ", &n):
    // - skip leading whitespace
    while pos < len && is_ascii_whitespace(input[pos]) {
        pos += 1;
    }
    // parse optional sign
    let mut sign = 1i64;
    if pos < len {
        if input[pos] == b'+' {
            pos += 1;
        } else if input[pos] == b'-' {
            sign = -1;
            pos += 1;
        }
    }
    // parse digits
    let mut val: i64 = 0;
    let mut any_digit = false;
    while pos < len {
        let b = input[pos];
        if b'0' <= b && b <= b'9' {
            any_digit = true;
            val = val * 10 + (b - b'0') as i64;
            pos += 1;
        } else {
            break;
        }
    }
    // If no digits found, behavior is undefined in C; we proceed with 0 to stay safe.
    let mut n: i64 = if any_digit { sign * val } else { 0 };

    // trailing space in scanf's format string: skip whitespace up to the first non-whitespace (leave it unread)
    while pos < len && is_ascii_whitespace(input[pos]) {
        pos += 1;
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Emulate while (n--) in C. We use i64 for the loop counter to reduce the chance of
    // overflow panic in Rust; C's signed overflow would be UB, but typical inputs are non-negative.
    while n != 0 {
        // emulate fgets(in, 1023, stdin);
        // In C: fgets reads up to size-1 characters (here 1022), stops early if newline read (newline included).
        // It returns NULL on EOF with no characters read. The original C code does not check NULL.
        // We'll emulate by producing an empty buffer if at EOF.
        let max_read: usize = 1022;
        if pos >= len {
            // EOF: produce empty "line" (C would have returned NULL and left buffer unchanged;
            // original code doesn't check, so behavior is undefined if EOF; here we output nothing).
            // To avoid blocking, we just output nothing for this iteration.
            // (This is a pragmatic choice to keep behavior well-defined in Rust.)
            // Move on to next iteration as C would (but C would likely use stale buffer — undefined).
            // We'll write nothing.
            // Decrement loop counter like while(n--) would do.
            n -= 1;
            continue;
        } else {
            // Determine how many bytes to take: up to max_read, and stop if newline encountered (include newline).
            let remaining = len - pos;
            let take_limit = std::cmp::min(remaining, max_read);
            let mut take = 0usize;
            while take < take_limit {
                take += 1;
                if input[pos + take - 1] == b'\n' {
                    break;
                }
            }
            let mut chunk = input[pos..pos + take].to_vec();
            pos += take;

            // Now perform the substring-modify loop:
            // ptr = strstr(in, "Hoshino");
            // while (ptr != NULL) { ptr[6] = 'a'; ptr = strstr(ptr, "Hoshino"); }
            // We operate on bytes, pattern is ASCII.
            let pat = b"Hoshino";
            let mut search_from = 0usize;
            while let Some(i) = find_subslice_from(&chunk, pat, search_from) {
                // set ptr[6] = 'a'
                if i + 6 < chunk.len() {
                    chunk[i + 6] = b'a';
                }
                // Continue searching starting at the same pointer (as in the C code).
                search_from = i;
                // If search_from would be the same and the same substring matched again (it shouldn't,
                // because we changed last byte), this could loop; but this mirrors the C behavior.
            }

            // Print exactly as bytes (no extra newline or formatting).
            handle.write_all(&chunk)?;
        }

        // decrement like n--
        n -= 1;
    }

    Ok(())
}
