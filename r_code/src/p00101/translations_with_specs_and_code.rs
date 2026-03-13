use std::io::{self, Read, Write};

/// Reads one byte from `reader`, first returning any bytes present in `pushback`.
/// Returns Ok(Some(byte)) on success, Ok(None) on EOF, Err on I/O error.
fn next_byte<R: Read>(reader: &mut R, pushback: &mut Vec<u8>) -> io::Result<Option<u8>> {
    if let Some(b) = pushback.pop() {
        return Ok(Some(b));
    }
    let mut buf = [0u8; 1];
    match reader.read(&mut buf) {
        Ok(0) => Ok(None),
        Ok(1) => Ok(Some(buf[0])),
        Ok(_) => unreachable!(),
        Err(e) => Err(e),
    }
}

/// Pushes a single byte back so it will be the next returned by `next_byte`.
fn pushback_byte(pushback: &mut Vec<u8>, b: u8) {
    pushback.push(b);
}

/// Reads an integer from `reader` mimicking the behavior of scanf("%d "):
/// - skips leading whitespace
/// - reads optional '+'/'-' and digits to form the integer
/// - after the integer, consumes whitespace until the first non-whitespace byte,
///   which is pushed back so it will be the next byte returned by subsequent reads.
///
/// Returns Ok(i32) on success, Err on parse or I/O error, or Err if EOF reached
/// unexpectedly while trying to read the integer.
fn read_int_and_consume_trailing_whitespace<R: Read>(
    reader: &mut R,
    pushback: &mut Vec<u8>,
) -> io::Result<i32> {
    // Skip leading whitespace
    loop {
        match next_byte(reader, pushback)? {
            Some(b) if (b as char).is_whitespace() => continue,
            Some(b) => {
                // non-whitespace: push it back and break to parse integer
                pushback_byte(pushback, b);
                break;
            }
            None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF before int")),
        }
    }

    // Read optional sign
    let mut token = Vec::new();
    if let Some(b) = next_byte(reader, pushback)? {
        if b == b'+' || b == b'-' {
            token.push(b);
        } else {
            pushback_byte(pushback, b);
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF in int"));
    }

    // Read digits
    let mut any_digit = false;
    loop {
        match next_byte(reader, pushback)? {
            Some(b) if (b as char).is_ascii_digit() => {
                token.push(b);
                any_digit = true;
            }
            Some(b) => {
                // Non-digit: push back and stop
                pushback_byte(pushback, b);
                break;
            }
            None => break,
        }
    }

    if !any_digit {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No digits for integer",
        ));
    }

    // Parse token as i32
    let s = String::from_utf8_lossy(&token);
    let n: i32 = match s.parse() {
        Ok(v) => v,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid integer format",
            ))
        }
    };

    // Mimic scanf("%d "): consume whitespace until a non-whitespace byte is seen,
    // then push that non-whitespace byte back.
    loop {
        match next_byte(reader, pushback)? {
            Some(b) if (b as char).is_whitespace() => continue,
            Some(b) => {
                // Found first non-whitespace after integer: push it back for next reads
                pushback_byte(pushback, b);
                break;
            }
            None => {
                // EOF: nothing more to consume; just return n
                break;
            }
        }
    }

    Ok(n)
}

/// Reads up to `max` bytes or until and including a newline from `reader`,
/// returning None on EOF (no bytes read), or Some(String) containing the bytes read.
/// Behavior mimics fgets with a fixed buffer size: a call reads at most `max` bytes.
/// If a newline is encountered and included before reaching `max`, reading stops after it.
/// The returned string is produced via loss-tolerant UTF-8 conversion (like C strings).
fn read_chunk_fgets_like<R: Read>(
    reader: &mut R,
    pushback: &mut Vec<u8>,
    max: usize,
) -> io::Result<Option<String>> {
    let mut buf: Vec<u8> = Vec::new();
    while buf.len() < max {
        match next_byte(reader, pushback)? {
            Some(b) => {
                buf.push(b);
                if b == b'\n' {
                    break;
                }
            }
            None => break, // EOF
        }
    }
    if buf.is_empty() {
        Ok(None)
    } else {
        // Convert to String, replacing invalid sequences (mirrors tolerant behavior)
        Ok(Some(String::from_utf8_lossy(&buf).into_owned()))
    }
}

fn main() -> io::Result<()> {
    // We'll read from stdin byte-by-byte and manage a small pushback buffer to emulate
    // scanf("%d ") behavior and fgets chunked reads.
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut pushback: Vec<u8> = Vec::new();

    // Read n with scanf("%d ") semantics
    let n = match read_int_and_consume_trailing_whitespace(&mut handle, &mut pushback) {
        Ok(v) => v,
        Err(_) => {
            // Specification: behavior undefined if scanf fails. We choose to exit quietly.
            return Ok(());
        }
    };

    // Per specification, internal line buffer size is 1024 bytes (1023 chars + NUL),
    // so we emulate fgets reading up to 1023 characters per call.
    let max_chunk = 1023usize;

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for _ in 0..n {
        // Read one chunk (like fgets)
        match read_chunk_fgets_like(&mut handle, &mut pushback, max_chunk)? {
            Some(mut line) => {
                // Perform case-sensitive substitution of "Hoshino" -> change last char to 'a'
                // Work on bytes to match C behavior precisely with ASCII pattern.
                let mut bytes = line.into_bytes();
                let pat = b"Hoshino";
                let pat_len = pat.len();
                let mut start = 0usize;
                while start <= bytes.len().saturating_sub(pat_len) {
                    // search for pattern at or after start
                    let mut found = None;
                    for i in start..=bytes.len().saturating_sub(pat_len) {
                        if &bytes[i..i + pat_len] == pat {
                            found = Some(i);
                            break;
                        }
                    }
                    match found {
                        Some(pos) => {
                            // Replace the 7th char (index 6) with 'a'
                            bytes[pos + 6] = b'a';
                            // As in the C code: call strstr(ptr, "Hoshino") with ptr being current match pointer.
                            // That effectively continues searching from the current position (which is pos).
                            // We set start = pos to emulate that behavior.
                            start = pos;
                            // But ensure forward progress: because we modified bytes[pos+6], the same match won't be found again
                            // so loop will advance. This mirrors the C program's behavior.
                            // To avoid an infinite loop in any unforeseen case, advance by 1 if no progress:
                            start = start.saturating_add(1);
                        }
                        None => break,
                    }
                }

                // Write out the processed chunk exactly (no extra newline added).
                out.write_all(&bytes)?;
            }
            None => {
                // EOF encountered before expected number of lines.
                // Specification: behavior is unspecified in this case. We'll stop processing.
                break;
            }
        }
    }

    Ok(())
}
