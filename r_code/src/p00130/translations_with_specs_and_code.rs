use std::io::{self, Read, Write};

fn parse_int(input: &[u8], pos: &mut usize) -> Option<i32> {
    // Skip leading whitespace
    while *pos < input.len() && input[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
    if *pos >= input.len() {
        return None;
    }
    // Optional sign
    let mut sign = 1i32;
    if input[*pos] == b'+' {
        *pos += 1;
    } else if input[*pos] == b'-' {
        sign = -1;
        *pos += 1;
    }
    // Read digits
    let start = *pos;
    let mut value: i32 = 0;
    while *pos < input.len() {
        let b = input[*pos];
        if b.is_ascii_digit() {
            // safe to parse into i32 given expected sensible inputs
            value = value.saturating_mul(10).saturating_add((b - b'0') as i32);
            *pos += 1;
        } else {
            break;
        }
    }
    if *pos == start {
        None
    } else {
        Some(value * sign)
    }
}

fn main() -> io::Result<()> {
    // Read all stdin bytes into memory.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let mut idx: usize = 0;

    // Parse n like scanf("%d",&n)
    let n = match parse_int(&input, &mut idx) {
        Some(v) => v,
        None => {
            // If we cannot read n, behave like the original program which would stop.
            return Ok(());
        }
    };

    // The original C program calls getchar() once after reading n; consume one byte if present.
    if idx < input.len() {
        idx += 1;
    }

    let mut out = Vec::new();

    // Process up to n sequences
    for _ in 0..n {
        // buffer of 27 slots initialized to zero
        let mut buf = [0u8; 27];
        // cursor starts at center 13
        let mut p: isize = 13;

        // Inner loop: read a visible character into buf[p], then attempt to read movement marker
        loop {
            // Read visible character (equivalent to scanf("%c",&l[p]))
            if idx >= input.len() {
                // EOF: stop reading this sequence (matches spec: may stop early)
                break;
            }
            // As per spec, input ensures cursor stays within [0,26]; assert for debug.
            // Convert p to usize for indexing; assume valid per spec.
            if !(0..=26).contains(&p) {
                // Cursor out-of-bounds is undefined per spec; stop processing this sequence to remain safe.
                break;
            }
            buf[p as usize] = input[idx];
            idx += 1;

            // Attempt to read next char into c (movement marker or sequence terminator)
            if idx >= input.len() {
                // EOF encountered while trying to read movement marker: stop sequence
                break;
            }
            let c = input[idx];
            idx += 1;

            if c == b'-' || c == b'<' {
                // The C code calls getchar() to consume an additional separator byte (if present).
                if idx < input.len() {
                    idx += 1;
                }
                // Move cursor: '-' means right, '<' means left.
                if c == b'-' {
                    p += 1;
                } else {
                    p -= 1;
                }
                // Continue to next visible character
                continue;
            } else {
                // Any other character terminates the sequence (the char has already been consumed).
                break;
            }
        }

        // Find first non-zero slot
        let mut j = 0usize;
        while j < buf.len() && buf[j] == 0 {
            j += 1;
        }

        if j >= buf.len() {
            // Per spec, this is undefined (sequence should have at least one visible char).
            // To avoid UB and maintain sensible output, print an empty line.
            out.extend_from_slice(b"\n");
        } else {
            // Print from first non-zero up to first zero (like puts in C which prints until NUL)
            let mut k = j;
            while k < buf.len() && buf[k] != 0 {
                k += 1;
            }
            out.extend_from_slice(&buf[j..k]);
            out.push(b'\n');
        }
    }

    // Write the collected output to stdout
    io::stdout().write_all(&out)?;
    Ok(())
}
