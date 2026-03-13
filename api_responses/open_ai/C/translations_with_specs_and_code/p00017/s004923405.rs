use std::io::{self, Read, Write};

/// Apply a Caesar-style shift to `src` bytes according to the specification.
///
/// Behavior (per specification/C code):
/// - Preserves space (0x20) and period (0x2E) bytes unchanged.
/// - For other bytes: add `shift` to the byte value; if resulting value > ASCII 'z' (122),
///   subtract 26 (wrap-around).
/// - Processes the entire provided byte slice (which mirrors the C code iterating until '\0';
///   here the input is a length-delimited buffer).
///
/// Returns a newly allocated Vec<u8> containing the transformed bytes (no trailing NUL).
fn caesar(src: &[u8], shift: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len());
    for &b in src {
        if b == b' ' {
            out.push(b' ');
        } else if b == b'.' {
            out.push(b'.');
        } else {
            // Use u16 to avoid overflow when adding shift
            let sum = (b as u16) + (shift as u16);
            if sum <= 122 {
                out.push(sum as u8);
            } else {
                out.push((sum - 26) as u8);
            }
        }
    }
    out
}

/// Return true if `haystack` contains any of the byte patterns in `patterns`.
fn contains_any(haystack: &[u8], patterns: &[&[u8]]) -> bool {
    for &pat in patterns {
        if pat.len() == 0 {
            continue;
        }
        if haystack.windows(pat.len()).any(|w| w == pat) {
            return true;
        }
    }
    false
}

fn run() -> io::Result<()> {
    // Read one line from stdin into a byte buffer, preserving the newline if present.
    // We use read_until to replicate C's fgets behavior more closely (fgets reads up to
    // n-1 bytes and includes the newline if it fits).
    let mut stdin = io::stdin();
    let mut input = Vec::new();
    // read_until returns 0 on EOF; that's acceptable (input will be empty)
    let _ = stdin.read_until(b'\n', &mut input)?;

    // Truncate to at most 80 bytes to match fgets(str, sizeof(str), stdin) with sizeof == 81.
    if input.len() > 80 {
        input.truncate(80);
    }

    let stdout = io::stdout();
    let mut out_lock = stdout.lock();

    // Patterns to search for, checked in the order similar to the C code.
    let patterns: [&[u8]; 3] = [b"the", b"this", b"that"];

    // Try shifts 0..25 inclusive (0..26)
    for shift in 0u8..26u8 {
        let shifted = caesar(&input, shift);
        if contains_any(&shifted, &patterns) {
            // Print the shifted bytes followed by a newline, matching printf("%s\n", ...)
            out_lock.write_all(&shifted)?;
            out_lock.write_all(b"\n")?;
        }
    }

    Ok(())
}

fn main() {
    // Per specification, the program exits with status 0 on completion.
    // Any I/O errors are ignored here to preserve the intended exit behavior.
    let _ = run();
}
