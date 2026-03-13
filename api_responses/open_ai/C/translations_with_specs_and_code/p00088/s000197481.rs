use std::io::{self, Read, Write};

/// Encode a single input character (as ASCII) to its codeword of '0'/'1' bits.
/// Returns None if the character is not in the supported alphabet.
fn codeword_for(ch: u8) -> Option<&'static str> {
    match ch {
        b' '  => Some("101"),
        b'\'' => Some("000000"),
        b','  => Some("000011"),
        b'-'  => Some("10010001"),
        b'.'  => Some("010001"),
        b'?'  => Some("000001"),
        b'A'  => Some("100101"),
        b'B'  => Some("10011010"),
        b'C'  => Some("0101"),
        b'D'  => Some("0001"),
        b'E'  => Some("110"),
        b'F'  => Some("01001"),
        b'G'  => Some("10011011"),
        b'H'  => Some("010000"),
        b'I'  => Some("0111"),
        b'J'  => Some("10011000"),
        b'K'  => Some("0110"),
        b'L'  => Some("00100"),
        b'M'  => Some("10011001"),
        b'N'  => Some("10011110"),
        b'O'  => Some("00101"),
        b'P'  => Some("111"),
        b'Q'  => Some("10011111"),
        b'R'  => Some("1000"),
        b'S'  => Some("00110"),
        b'T'  => Some("00111"),
        b'U'  => Some("10011100"),
        b'V'  => Some("10011101"),
        b'W'  => Some("000010"),
        b'X'  => Some("10010010"),
        b'Y'  => Some("10010011"),
        b'Z'  => Some("10010000"),
        // Note: newline is a terminator and is not encoded.
        _ => None,
    }
}

fn main() {
    // Read bytes from stdin until and including the first newline, or until EOF.
    // We treat input as bytes; per the specification, the first newline terminates processing.
    let mut stdin = io::stdin().lock();
    let mut input_buf = Vec::new();

    // Read one byte at a time until newline or EOF.
    loop {
        let mut byte = [0u8; 1];
        match stdin.read(&mut byte) {
            Ok(0) => {
                // EOF reached. Specification assumes a newline is present; behavior is undefined
                // if it's not. We proceed with what we have (graceful handling).
                break;
            }
            Ok(1) => {
                input_buf.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            Ok(_) => {} // shouldn't happen
            Err(_) => {
                // On read error, exit (no output). Keep behavior simple.
                return;
            }
        }
    }

    // Process bytes up to but not including the first newline.
    let mut bits = String::new();
    for &b in &input_buf {
        if b == b'\n' {
            break;
        }
        // Map character to codeword; per spec, every non-newline input character must be supported.
        // If not supported, behavior is undefined; here we simply skip encoding unknown characters.
        // (This is a minimal deviation to keep the program from panicking.)
        if let Some(code) = codeword_for(b) {
            bits.push_str(code);
        } else {
            // Character not supported by spec; undefined behavior per spec.
            // To avoid panic in Rust, we'll treat it as no-op (skip). In normal usage this should not occur.
        }
    }

    // Pad with '0' until total bit length is a multiple of 5.
    while bits.len() % 5 != 0 {
        bits.push('0');
    }

    // Output alphabet: indices 0..25 -> 'A'..'Z', 26 -> ' ', 27 -> '.', 28 -> ',', 29 -> '-', 30 -> '\'', 31 -> '?'
    // Represented as a byte slice for direct indexing.
    const OUT_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ .,-'?";
    let out_bytes = OUT_ALPHABET.as_bytes();

    let mut stdout = io::stdout().lock();
    let mut out_buf = Vec::new();

    // Process bits in 5-bit groups, MSB-first.
    let total_groups = bits.len() / 5;
    for g in 0..total_groups {
        let start = g * 5;
        let slice = &bits.as_bytes()[start..start + 5];
        let mut val = 0u8;
        for (i, &bit_char) in slice.iter().enumerate() {
            if bit_char == b'1' {
                // Most significant bit is at i == 0, corresponding to shift 4.
                val |= 1 << (4 - i);
            }
        }
        // Map val (0..31) to output character.
        // Safety: val is in 0..=31 because we consumed exactly 5 bits.
        out_buf.push(out_bytes[val as usize]);
    }

    // Write output characters followed by a newline.
    // Ensure exact formatting per specification.
    let _ = stdout.write_all(&out_buf);
    let _ = stdout.write_all(b"\n");
    let _ = stdout.flush();
}
