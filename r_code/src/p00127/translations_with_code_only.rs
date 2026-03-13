use std::io::{self, Read, Write};

fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut bytes_iter = stdin.lock().bytes();

    let mut stdout = io::stdout();

    // Emulate: char str[256] = {0};
    // And fgets(str, 255, stdin) behavior: read up to 254 bytes or until newline (inclusive).
    // Repeat until EOF. Each chunk corresponds to one fgets call.
    loop {
        let mut buf = [0u8; 256]; // zero-initialized like in the C code
        let mut count: usize = 0;

        // Read up to 254 bytes or until newline is read.
        while count < 254 {
            match bytes_iter.next() {
                Some(Ok(b)) => {
                    buf[count] = b;
                    count += 1;
                    if b == b'\n' {
                        break;
                    }
                }
                Some(Err(e)) => {
                    // I/O error while reading
                    return Err(e);
                }
                None => {
                    // EOF
                    break;
                }
            }
        }

        // If EOF and nothing was read, break like fgets returning NULL.
        if count == 0 && bytes_iter.next().is_none() {
            // Note: In the above we already consumed the iterator to None,
            // but calling next() again returns None. To avoid changing behavior,
            // we check count==0 and the iterator being at EOF. However, since we
            // already hit None in the inner loop we can treat count==0 as EOF.
            break;
        } else if count == 0 {
            // It is possible that we encountered EOF earlier and the iterator is now exhausted.
            // To match fgets behavior: if EOF and some bytes were read, we should process them.
            // count == 0 here means nothing was read and we've seen EOF; break.
            break;
        }

        // Ensure NUL termination as fgets does: there's already a 0 at buf[count] because of initialization.

        // Compute strlen equivalent: index of first NUL (which we know is count)
        let strlen = {
            // Find first zero byte (should be at buf[count] due to initialization and fgets behavior),
            // but we'll scan to be faithful.
            let mut len = 0usize;
            while len < 256 && buf[len] != 0 {
                len += 1;
            }
            len
        };

        // If strlen % 2 == 0, puts("NA") i.e., print "NA\n"
        if strlen % 2 == 0 {
            stdout.write_all(b"NA\n")?;
            continue;
        }

        // for (i = 0; str[i] != 10; i += 2) { ... }
        // Emulate by iterating i by 2 while buf[i] != '\n' (10).
        // To avoid undefined behavior from out-of-bounds access in the original C if no newline exists,
        // we bound i < 256 (the buffer size). This is a safe choice that preserves intended behavior
        // for well-formed inputs and avoids UB.
        let mut i: usize = 0;
        while i < 256 && buf[i] != 10 {
            // Access buf[i+1] is safe because i ranges up to 254 (even numbers) in normal cases.
            // But ensure we don't go out of bounds:
            let b1 = buf[i];
            let b2 = if i + 1 < 256 { buf[i + 1] } else { 0u8 };

            // dec = (str[i]  - '0') * 10 + str[i + 1] - '0';
            // Implement with signed arithmetic to match C's behavior for negatives.
            let dec: i32 = (b1 as i32 - ('0' as i32)) * 10 + (b2 as i32) - ('0' as i32);

            let out_byte: u8 = match dec {
                62 => b'.',
                63 => b'?',
                64 => b'!',
                65 => b' ',
                _ => {
                    // default: putchar((dec / 10 - 1) * 5 + dec % 10 + 96);
                    // Use i32 division/modulo semantics (trunc toward zero, remainder sign like numerator),
                    // then cast to u8 which wraps modulo 256, matching C's unsigned char conversion.
                    let val = (dec / 10 - 1) * 5 + dec % 10 + 96;
                    val as u8
                }
            };

            stdout.write_all(&[out_byte])?;

            i = i.wrapping_add(2); // match increment by 2
        }

        // putchar(10);
        stdout.write_all(&[10u8])?;
    }

    Ok(())
}

fn main() {
    // If an I/O error occurs, exit with status 1 silently (no extra stderr output),
    // to avoid adding output not present in the original C program.
    if let Err(_) = run() {
        std::process::exit(1);
    }
}
