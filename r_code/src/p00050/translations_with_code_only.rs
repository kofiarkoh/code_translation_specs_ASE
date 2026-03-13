use std::io::{self, Read, Write};

fn main() {
    // Emulate C's fgets(text, 1002, stdin):
    // - reads at most 1001 bytes (n-1) and stops after including a newline if encountered.
    // We'll read bytes from stdin until newline or we've read 1001 bytes.
    const MAX_READ: usize = 1001;
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf: Vec<u8> = Vec::with_capacity(MAX_READ);

    let mut single = [0u8; 1];
    while buf.len() < MAX_READ {
        match handle.read(&mut single) {
            Ok(0) => break, // EOF
            Ok(_) => {
                buf.push(single[0]);
                if single[0] == b'\n' {
                    break; // fgets includes the newline and stops
                }
            }
            Err(_) => break,
        }
    }

    // In the C code: char text[1002], i, n; n = strlen(text);
    // Then n is a (signed) char holding strlen(text) truncated to 8 bits.
    // We emulate that by taking length % 256 and interpreting as signed i8.
    let len = buf.len();
    let n_u8 = (len % 256) as u8;
    let n_signed: i32 = (n_u8 as i8) as i32; // sign-extend like (char) in C

    // limit = n - 5 (in C this is done in int arithmetic after promotion)
    let limit = n_signed - 5;

    // Loop for(i = 0; i < n-5; i++) with i being a char in C promoted to int for comparison.
    // We iterate i as i32 starting from 0, matching the promoted comparison semantics.
    if limit > 0 {
        for i in 0..limit {
            // indexes used: i, i+1, ..., i+4. They must be valid indices into the original buffer.
            // Based on C semantics and the computation of limit from the (possibly truncated) n,
            // these accesses are the same as in the C program.
            let idx = i as usize;
            // Safe bounds check (should mirror C's behavior because limit is derived similarly).
            if idx + 4 >= buf.len() {
                // If for any reason this would go out of bounds, break to avoid panic.
                // (This should not happen for inputs that mirror the behavior of the original C program.)
                break;
            }

            // Lowercase apple <-> peach
            if buf[idx] == b'a'
                && buf[idx + 1] == b'p'
                && buf[idx + 2] == b'p'
                && buf[idx + 3] == b'l'
                && buf[idx + 4] == b'e'
            {
                buf[idx] = b'p';
                buf[idx + 1] = b'e';
                buf[idx + 2] = b'a';
                buf[idx + 3] = b'c';
                buf[idx + 4] = b'h';
            } else if buf[idx] == b'p'
                && buf[idx + 1] == b'e'
                && buf[idx + 2] == b'a'
                && buf[idx + 3] == b'c'
                && buf[idx + 4] == b'h'
            {
                buf[idx] = b'a';
                buf[idx + 1] = b'p';
                buf[idx + 2] = b'p';
                buf[idx + 3] = b'l';
                buf[idx + 4] = b'e';
            }

            // Uppercase A followed by "pple" <-> "Peach"/"Apple" swap.
            // Note: This is a separate if / else-if in the C code (not chained with the lowercase branch),
            // so we replicate that exact ordering and separation.
            if buf[idx] == b'A'
                && buf[idx + 1] == b'p'
                && buf[idx + 2] == b'p'
                && buf[idx + 3] == b'l'
                && buf[idx + 4] == b'e'
            {
                buf[idx] = b'P';
                buf[idx + 1] = b'e';
                buf[idx + 2] = b'a';
                buf[idx + 3] = b'c';
                buf[idx + 4] = b'h';
            } else if buf[idx] == b'P'
                && buf[idx + 1] == b'e'
                && buf[idx + 2] == b'a'
                && buf[idx + 3] == b'c'
                && buf[idx + 4] == b'h'
            {
                buf[idx] = b'A';
                buf[idx + 1] = b'p';
                buf[idx + 2] = b'p';
                buf[idx + 3] = b'l';
                buf[idx + 4] = b'e';
            }
        }
    }

    // Print the buffer exactly (no extra newline). This mirrors printf("%s", text);
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let _ = out.write_all(&buf);
}
