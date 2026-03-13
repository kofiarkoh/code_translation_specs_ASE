// translated from the given C program
// Preserves original semantics wrt input truncation and trimming:
// - Reads up to 1024 bytes from stdin or until a newline is encountered (like fgets(buf, 1025,...)).
// - Unconditionally removes the last character read (the C code sets line[i-2] = 0).
// - Splits on separators ' ', ',' and '.' (like strtok with " ,.") collapsing consecutive separators.
// - Prints tokens whose byte-length is between 3 and 6 inclusive, separated by single spaces, then a newline.
// Tokens and output are handled as raw bytes to preserve exact byte-level behavior.

use std::io::{self, Read, Write};

fn main() {
    // Read up to 1024 bytes or until newline (inclusive), emulating fgets(line, 1025, stdin).
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buf: Vec<u8> = Vec::new();
    let mut byte = [0u8; 1];

    while buf.len() < 1024 {
        match handle.read(&mut byte) {
            Ok(0) => break, // EOF
            Ok(1) => {
                buf.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    // Emulate the C code's while(line[i++]); followed by line[i-2]=0;
    // That results in removing the last character that was read (if any).
    let content: &[u8] = if buf.is_empty() {
        &[]
    } else {
        &buf[..buf.len() - 1]
    };

    // Delimiters: space, comma, dot
    let is_delim = |b: u8| b == b' ' || b == b',' || b == b'.';

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut i = 0usize;
    let mut first_printed = false;

    while i < content.len() {
        // skip delimiters
        if is_delim(content[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < content.len() && !is_delim(content[i]) {
            i += 1;
        }
        let token = &content[start..i];
        let len = token.len();
        if (3..=6).contains(&len) {
            if first_printed {
                // print a single space between printed tokens
                let _ = out.write_all(b" ");
            } else {
                first_printed = true;
            }
            let _ = out.write_all(token);
        }
    }

    // puts("") prints a newline
    let _ = out.write_all(b"\n");
}
