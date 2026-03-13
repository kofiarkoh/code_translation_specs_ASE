use std::io::{self, Read};

fn main() {
    // The ball's position; start under 'A'.
    let mut pos: u8 = b'A';

    // Read stdin as a stream of bytes and consume exactly 3 bytes per command:
    // <Label1>,<Label2>
    //
    // Per specification, commands must be exactly: Label,comma,Label with Label in {A,B,C},
    // and there is no guaranteed tolerance for other characters or separators.
    //
    // We implement a streaming parser that reads three bytes at a time. If fewer than
    // three bytes remain (EOF) or the 3-byte pattern does not match the expected
    // form, we stop processing further commands. This choice preserves a narrow,
    // deterministic interpretation of the specification.
    let stdin = io::stdin();
    let mut handle = stdin.lock().bytes();

    loop {
        // Attempt to read three bytes; if EOF or read error occurs, stop processing.
        let b1 = match handle.next() {
            Some(Ok(b)) => b,
            _ => break,
        };
        let b2 = match handle.next() {
            Some(Ok(b)) => b,
            _ => break,
        };
        let b3 = match handle.next() {
            Some(Ok(b)) => b,
            _ => break,
        };

        // Expect the middle byte to be a comma. If not, stop (behavior unspecified per spec).
        if b2 != b',' {
            break;
        }

        // Expect labels to be one of 'A','B','C'. If not, stop (behavior unspecified per spec).
        let valid = |x: u8| matches!(x, b'A' | b'B' | b'C');
        if !valid(b1) || !valid(b3) {
            break;
        }

        // Apply swap semantics:
        // If ball is at L1, it moves to L2; if at L2, it moves to L1; otherwise unchanged.
        if pos == b1 {
            pos = b3;
        } else if pos == b3 {
            pos = b1;
        }
        // continue to next command
    }

    // Print final position followed by a newline. This prints exactly one of "A\n","B\n","C\n".
    let mut out = io::stdout();
    let _ = out.write_all(&[pos, b'\n']);
}
