use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Emulate the C program's fgets behaviour with a fixed per-call buffer size.
    // The original code called fgets(str, 255, stdin) which reads up to 254 chars
    // (n-1). We emulate that by allowing up to MAX_READ-1 bytes per "line read".
    const FGETS_N: usize = 255;
    const MAX_BYTES: usize = FGETS_N - 1;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut one_byte = [0u8; 1];

    loop {
        // Read up to MAX_BYTES bytes or until '\n' or EOF, whichever comes first.
        let mut buf: Vec<u8> = Vec::new();

        while buf.len() < MAX_BYTES {
            match handle.read(&mut one_byte) {
                Ok(0) => break, // EOF
                Ok(1) => {
                    buf.push(one_byte[0]);
                    if one_byte[0] == b'\n' {
                        break;
                    }
                }
                Ok(_) => continue, // shouldn't happen for 1-byte read
                Err(e) => return Err(e),
            }
        }

        // If nothing was read and EOF reached, exit the loop.
        if buf.is_empty() {
            break;
        }

        // Parity check: emulate C's strlen(str) % 2 == 0 -> "NA"
        // (strlen in C counts bytes up to the terminating null; here buf holds
        // exactly what fgets would have stored, so use buf.len()).
        if buf.len() % 2 == 0 {
            out.write_all(b"NA\n")?;
            continue;
        }

        // Process pairs until newline (10) or until we can't form a full pair.
        let mut i = 0usize;
        while i + 1 < buf.len() {
            if buf[i] == b'\n' {
                break;
            }

            // Compute dec similar to C: (str[i] - '0') * 10 + str[i+1] - '0'
            let tens = buf[i].wrapping_sub(b'0') as i32;
            let ones = buf[i + 1].wrapping_sub(b'0') as i32;
            let dec = tens * 10 + ones;

            let out_byte: u8 = match dec {
                62 => b'.',
                63 => b'?',
                64 => b'!',
                65 => b' ',
                _ => {
                    // default: (dec / 10 - 1) * 5 + dec % 10 + 96
                    let val = (dec / 10 - 1) * 5 + (dec % 10) + 96;
                    // Cast to u8 as C putchar would write the low byte; behave similarly.
                    (val as u8)
                }
            };

            out.write_all(&[out_byte])?;
            i += 2;
        }

        // Write terminating newline for this decoded (or processed) "line".
        out.write_all(b"\n")?;
    }

    Ok(())
}
