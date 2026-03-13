use std::io::{self, Read, Write};

/// Translates the given C program semantics into Rust.
///
/// Notes on semantics preserved:
/// - Input is read similarly to fgets(buf, 81, stdin): at most 80 bytes are kept,
///   and a trailing newline is included only if it occurred within those 80 bytes.
/// - All operations are performed on raw bytes to preserve C's byte-wise behavior.
/// - The Caesar shift logic is implemented exactly:
///     if byte == ' ' -> unchanged
///     else if byte == '.' -> unchanged
///     else if (byte + shift) <= 122 -> byte + shift
///     else -> byte + shift - 26
///   Arithmetic is done in a signed-enough integer (i32) and then cast back to u8,
///   which mirrors the C promotion to int and storage back into a char-sized type.
/// - Search for "the", "this", "that" uses byte-wise substring search (like strstr).
/// - When printing a matching transformed string, we print the bytes up to the logical
///   end (no explicit NUL is present in our slices) and then write a newline,
///   mirroring printf("%s\n", ...).
fn caesar(src: &[u8], shift: i32) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len());
    for &b in src {
        // In the original C, the loop stops at the first NUL byte.
        if b == 0 {
            break;
        }
        if b == b' ' {
            out.push(b' ');
        } else if b == b'.' {
            out.push(b'.');
        } else {
            let sum = (b as i32) + shift;
            let val = if sum <= 122 { sum } else { sum - 26 };
            out.push(val as u8);
        }
    }
    // In C, caesar_str is NUL-terminated. Our returned Vec represents the bytes
    // up to (but not including) that NUL. Callers treat the Vec length as the C string length.
    out
}

fn contains_subslice(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if hay.len() < needle.len() {
        return false;
    }
    hay.windows(needle.len()).any(|w| w == needle)
}

fn main() {
    // Read up to and including the first newline from stdin, similar to fgets.
    let mut stdin = io::stdin();
    let mut buf: Vec<u8> = Vec::new();
    // read_until reads until newline (inclusive) or EOF.
    let _ = stdin.read_until(b'\n', &mut buf);

    // Emulate fgets(str, sizeof(str), stdin) where sizeof(str) == 81, so at most 80 bytes read.
    if buf.len() > 80 {
        buf.truncate(80);
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for i in 0..26 {
        let res = caesar(&buf, i);
        if contains_subslice(&res, b"the") {
            let _ = out.write_all(&res);
            let _ = out.write_all(b"\n");
        } else if contains_subslice(&res, b"this") {
            let _ = out.write_all(&res);
            let _ = out.write_all(b"\n");
        } else if contains_subslice(&res, b"that") {
            let _ = out.write_all(&res);
            let _ = out.write_all(b"\n");
        }
    }
}
