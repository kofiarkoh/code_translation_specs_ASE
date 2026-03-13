// translate_expand.rs
//
// Translation of the given C program according to the SPEC.
// Notes about mismatch with C:
// - The original C code stores input bytes into a fixed buffer and then
//   uses a NUL (0) sentinel to detect end-of-input (it sets some trailing
//   bytes to 0). That behavior would cause embedded NUL bytes in the input
//   to terminate processing early. The SPEC, however, defines that the
//   program should process the entire stdin stream until EOF.
// - Per instruction (3) we follow the SPEC (process until EOF) rather than
//   the C program's NUL-terminated behavior. To remain faithful to the
//   SPEC's fixed-buffer constraint, we read at most LENGTH-1 (1000) bytes.
//
// Behavior implemented:
// - Read up to 1000 bytes from stdin into a fixed-size buffer.
// - Process bytes left-to-right. When encountering '@' (0x40), compute
//   count = (next_byte) - b'0' (signed). Repeat the following byte that
//   many times (if count > 0); if the next bytes are missing they are
//   treated as 0x00 per SPEC (which yields a negative count and thus no repeats).
// - All other bytes including newline are copied through unchanged.
// - Always write a terminating newline to stdout before exiting.
//
// Implementation notes:
// - Fully safe Rust, no unsafe used.
// - Uses std::io::Read/Write for raw byte I/O.

use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    const LENGTH: usize = 1001;
    // Reserve LENGTH bytes but read at most LENGTH-1 (1000) per SPEC constraint.
    let mut buf = [0u8; LENGTH];
    let max_read = LENGTH - 1;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut total: usize = 0;

    // Read until EOF or buffer full.
    while total < max_read {
        let n = handle.read(&mut buf[total..max_read])?;
        if n == 0 {
            break; // EOF
        }
        total += n;
    }

    let mut stdout = io::stdout();
    let mut out_handle = stdout.lock();

    let mut i: usize = 0;
    while i < total {
        if buf[i] == b'@' {
            // Per SPEC: missing bytes are treated as 0x00.
            let b1 = if i + 1 < total { buf[i + 1] } else { 0 };
            let b2 = if i + 2 < total { buf[i + 2] } else { 0 };
            let count = (b1 as i32) - (b'0' as i32);
            if count > 0 {
                // Repeat b2 count times.
                // Write in a loop to preserve exact byte outputs.
                for _ in 0..count {
                    out_handle.write_all(&[b2])?;
                }
            }
            // Advance past the three-byte marker (even if past end; loop will terminate).
            i = i.saturating_add(3);
        } else if buf[i] == b'\n' {
            // Newline: copy through as newline.
            out_handle.write_all(b"\n")?;
            i += 1;
        } else {
            // Any other byte: copy as-is.
            out_handle.write_all(&[buf[i]])?;
            i += 1;
        }
    }

    // Always append a terminating newline before exiting.
    out_handle.write_all(b"\n")?;
    out_handle.flush()?;
    Ok(())
}
