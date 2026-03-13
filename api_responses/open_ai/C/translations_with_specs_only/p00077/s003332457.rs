use std::io::{self, Read, Write};

const LENGTH: usize = 1001;
const MAX_INPUT: usize = 1000;

fn main() -> io::Result<()> {
    // Fixed-size buffer as specified.
    let mut buf = [0u8; LENGTH];

    // Read up to MAX_INPUT bytes from stdin into the fixed buffer.
    // If more bytes are available, they are ignored (matching the specification's
    // constraint that input should be <= 1000 bytes to avoid overflow).
    let mut stdin = io::stdin();
    let mut pos: usize = 0;
    while pos < MAX_INPUT {
        // read into the remaining portion up to MAX_INPUT
        let n = stdin.read(&mut buf[pos..MAX_INPUT])?;
        if n == 0 {
            break; // EOF
        }
        pos += n;
    }
    let input_len = pos;

    // Output assembled in a buffer, then written once to stdout.
    let mut out: Vec<u8> = Vec::new();

    // Process left-to-right. When an '@' is encountered, consume three bytes:
    // '@' + D + C. If D or C is missing, treat missing bytes as 0x00.
    // N = (D as signed) - '0'. If N > 0, write C repeated N times.
    let mut i: usize = 0;
    while i < input_len {
        if buf[i] == b'@' {
            // Fetch D and C with default 0 if out-of-range (missing bytes).
            let d = if i + 1 < input_len { buf[i + 1] } else { 0u8 };
            let c = if i + 2 < input_len { buf[i + 2] } else { 0u8 };

            // Compute signed difference to allow negative/large repeat counts
            // as specified: N = D - '0'.
            let n_signed = (d as i16) - (b'0' as i16);
            if n_signed > 0 {
                let n = n_signed as usize;
                out.extend(std::iter::repeat(c).take(n));
            }
            // Consume the three bytes of the marker (even if some were missing).
            i = i.saturating_add(3);
        } else {
            // Copy byte through unchanged.
            out.push(buf[i]);
            i += 1;
        }
    }

    // Always write a terminating newline before exiting.
    out.push(b'\n');

    // Write to stdout.
    let mut stdout = io::stdout();
    stdout.write_all(&out)?;
    stdout.flush()?;

    Ok(())
}
