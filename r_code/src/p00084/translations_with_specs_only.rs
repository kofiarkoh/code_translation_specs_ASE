use std::io::{self, Read, Write};

fn main() {
    // Emulate a C-style fixed buffer of 1025 bytes (1024 chars + NUL in C).
    // We'll store up to 1024 bytes read from stdin.
    let mut buf = [0u8; 1025];
    let mut len: usize = 0;

    // Read up to 1024 bytes or until a newline (inclusive), whichever comes first.
    // This matches the Specification's fgets-like behavior.
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut one = [0u8; 1];
    while len < 1024 {
        match handle.read(&mut one) {
            Ok(0) => break, // EOF
            Ok(1) => {
                buf[len] = one[0];
                len += 1;
                if one[0] == b'\n' {
                    break; // stop after reading a newline (fgets includes newline)
                }
            }
            Ok(_) => unreachable!(), // read returned >1 for a 1-byte buffer shouldn't happen
            Err(_) => break, // On I/O error, behave like EOF; spec says behavior undefined if read fails.
        }
    }

    // The Specification requires that the final character of the line be removed
    // (intended to remove a trailing newline). Do this regardless of what that
    // character actually is, but only if we read at least one byte.
    //
    // Note: The Specification states that if fgets fails (no input), behavior is
    // undefined. We choose to perform no output and exit in that case (i.e., len == 0).
    if len == 0 {
        return;
    }
    // Trim the final character (like dropping the trailing newline).
    let content_len = len - 1;
    let content = &buf[..content_len];

    // Tokenize on delimiters: space (0x20), comma ',' and period '.'
    // Collect slices of the original buffer (byte slices) for tokens that meet length criteria.
    let mut tokens: Vec<&[u8]> = Vec::new();
    let mut i = 0usize;
    while i < content_len {
        // Skip delimiters
        let b = content[i];
        if b == b' ' || b == b',' || b == b'.' {
            i += 1;
            continue;
        }
        // Start of token
        let start = i;
        while i < content_len {
            let c = content[i];
            if c == b' ' || c == b',' || c == b'.' {
                break;
            }
            i += 1;
        }
        let token = &content[start..i];
        let tlen = token.len(); // measured in bytes, per Specification
        if (3..=6).contains(&tlen) {
            tokens.push(token);
        }
    }

    // Print selected tokens separated by single spaces, no leading/trailing spaces,
    // followed by a newline. Write raw bytes to stdout to preserve original bytes.
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut first = true;
    for t in tokens {
        if !first {
            // separator
            let _ = out.write_all(b" ");
        }
        let _ = out.write_all(t);
        first = false;
    }
    let _ = out.write_all(b"\n");
}
