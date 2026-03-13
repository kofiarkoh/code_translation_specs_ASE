use std::io::{self, Read, Write};

fn main() {
    // Emulate a fixed-size fgets-like buffer of 256 bytes (we read up to 255 bytes per "line").
    // This follows the Specification's constraint about platform buffer size.
    const BUF_LIMIT: usize = 255;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stdout = io::stdout();
    let mut out_lock = stdout.lock();

    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(BUF_LIMIT);
        // Read bytes one at a time up to BUF_LIMIT or until a newline or EOF.
        for _ in 0..BUF_LIMIT {
            let mut byte = [0u8; 1];
            match handle.read(&mut byte) {
                Ok(0) => break, // EOF
                Ok(1) => {
                    buf.push(byte[0]);
                    if byte[0] == b'\n' {
                        break;
                    }
                }
                Ok(_) => unreachable!(),
                Err(_) => {
                    // On read error, behave conservatively: stop reading further.
                    break;
                }
            }
        }

        if buf.is_empty() {
            // EOF reached with no data; normal termination.
            // Per specification, exit code 0.
            break;
        }

        // Determine if a newline byte was present in this chunk.
        let has_newline = buf.last().map(|&b| b == b'\n').unwrap_or(false);

        // Per specification, parity check is performed on the string as read INCLUDING the newline.
        // That means the read length must be odd to pass (i.e., number of chars before newline even).
        // We require the newline to be present in the buffer for decoding to be attempted.
        let parity_ok = has_newline && (buf.len() % 2 == 1);

        if !parity_ok {
            // Output "NA\n" for parity failure.
            // Ignore write errors per specification (no exception mechanism).
            let _ = out_lock.write_all(b"NA\n");
            continue;
        }

        // Process pairs of characters up to but excluding the terminating newline.
        let data_len = buf.len() - 1; // exclude newline
        let mut decoded: Vec<u8> = Vec::with_capacity(data_len / 2 + 1);

        let mut i = 0usize;
        while i + 1 < data_len {
            let c1 = buf[i];
            let c2 = buf[i + 1];

            // Compute dec as numeric computation on character codes similar to C: (c1 - '0')*10 + (c2 - '0').
            // Use signed arithmetic to mirror behavior for non-digit characters as described.
            let dec = ((c1 as i32) - 48) * 10 + ((c2 as i32) - 48);

            // Special-case mappings
            let out_byte = match dec {
                62 => b'.',
                63 => b'?',
                64 => b'!',
                65 => b' ',
                _ => {
                    // Default mapping: 'a' + (dec - 11)
                    // For dec outside 11..61 this can yield non-alphabetic bytes; emulate by wrapping modulo 256.
                    let ch_code = ('a' as i32) + (dec - 11);
                    let wrapped = ch_code.rem_euclid(256) as u8;
                    wrapped
                }
            };

            decoded.push(out_byte);
            i += 2;
        }

        // Write decoded bytes followed by newline.
        let _ = out_lock.write_all(&decoded);
        let _ = out_lock.write_all(b"\n");
    }
}
