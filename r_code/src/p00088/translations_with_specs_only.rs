// encoder.rs
//
// Translation of the provided specification into Rust.
// Behavior:
//  - Read bytes from stdin until the first newline ('\n').
//  - For each input byte (before the newline) map it to a variable-length
//    codeword composed of '0'/'1' characters according to the fixed table.
//  - Concatenate codewords into a bitstring buffer (stored as ASCII '0'/'1').
//  - The total number of stored bits (including padding) must be <= 4095.
//    If this would be exceeded, the program exits with a non-zero status.
//    (The original specification states this is undefined behavior; to keep
//    the Rust program safe we explicitly abort on overflow.)
//  - Pad with '0' bits up to a multiple of 5 (padding counts against the 4095 limit).
//  - Split into 5-bit groups (most-significant-bit first) and convert each
//    group to a value 0..31, then map that value to an output character
//    using the fixed 32-entry alphabet and write them to stdout, followed by
//    a newline. Exit with status 0 on success.
//
// Notes about design choices:
//  - The spec treats invalid input / missing newline / overflow as undefined.
//    In safe Rust we must choose defined behavior; we therefore treat those
//    cases as fatal and exit with status 1. This keeps all observable I/O for
//    valid inputs unchanged while preventing unsafety.
//  - All inputs are treated as raw bytes. Only the supported ASCII bytes are mapped.

use std::io::{self, Read, Write};
use std::process::exit;

const MAX_BITS: usize = 4095; // maximum stored '0'/'1' characters allowed (including padding)

fn codeword_for_byte(b: u8) -> Option<&'static [u8]> {
    // Return codeword as byte slice of ASCII '0'/'1'.
    // Mapping taken exactly from the specification.
    match b {
        b' '  => Some(b"101"),
        b'\'' => Some(b"000000"),
        b','  => Some(b"000011"),
        b'-'  => Some(b"10010001"),
        b'.'  => Some(b"010001"),
        b'?'  => Some(b"000001"),
        b'A'  => Some(b"100101"),
        b'B'  => Some(b"10011010"),
        b'C'  => Some(b"0101"),
        b'D'  => Some(b"0001"),
        b'E'  => Some(b"110"),
        b'F'  => Some(b"01001"),
        b'G'  => Some(b"10011011"),
        b'H'  => Some(b"010000"),
        b'I'  => Some(b"0111"),
        b'J'  => Some(b"10011000"),
        b'K'  => Some(b"0110"),
        b'L'  => Some(b"00100"),
        b'M'  => Some(b"10011001"),
        b'N'  => Some(b"10011110"),
        b'O'  => Some(b"00101"),
        b'P'  => Some(b"111"),
        b'Q'  => Some(b"10011111"),
        b'R'  => Some(b"1000"),
        b'S'  => Some(b"00110"),
        b'T'  => Some(b"00111"),
        b'U'  => Some(b"10011100"),
        b'V'  => Some(b"10011101"),
        b'W'  => Some(b"000010"),
        b'X'  => Some(b"10010010"),
        b'Y'  => Some(b"10010011"),
        b'Z'  => Some(b"10010000"),
        _ => None,
    }
}

fn value_to_output_char(v: u8) -> u8 {
    // Map 0..31 to output ASCII byte as per specification:
    // 0..25 -> 'A'..'Z'
    // 26 -> space
    // 27 -> '.'
    // 28 -> ','
    // 29 -> '-'
    // 30 -> '\''
    // 31 -> '?'
    match v {
        0..=25 => b'A' + v,
        26 => b' ',
        27 => b'.',
        28 => b',',
        29 => b'-',
        30 => b'\'',
        31 => b'?',
        _ => b'?', // unreachable for v in 0..31
    }
}

fn main() {
    // Read from stdin until first newline byte.
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    let mut bits: Vec<u8> = Vec::with_capacity(1024); // will grow but we enforce MAX_BITS

    loop {
        match stdin.read(&mut buffer) {
            Ok(0) => {
                // EOF before newline: specification says behavior undefined.
                // We choose to exit with non-zero status to avoid unpredictable behavior.
                exit(1);
            }
            Ok(1) => {
                let b = buffer[0];
                if b == b'\n' {
                    break; // stop processing at first newline
                }
                // Look up codeword
                match codeword_for_byte(b) {
                    Some(code) => {
                        // Append code bytes, enforcing MAX_BITS limit (including padding)
                        if bits.len() + code.len() > MAX_BITS {
                            // Would exceed buffer; treat as fatal (undefined in spec).
                            exit(1);
                        }
                        bits.extend_from_slice(code);
                    }
                    None => {
                        // Unsupported input character before newline: undefined in spec.
                        // Choose to exit non-zero.
                        exit(1);
                    }
                }
            }
            Ok(_) => unreachable!(), // read either 0 or 1 bytes
            Err(_) => {
                // I/O error reading stdin; treat as fatal.
                exit(1);
            }
        }
    }

    // Pad with '0' until multiple of 5, counting against MAX_BITS.
    let rem = bits.len() % 5;
    if rem != 0 {
        let pad = 5 - rem;
        if bits.len() + pad > MAX_BITS {
            // Would exceed buffer; treat as fatal.
            exit(1);
        }
        bits.extend(std::iter::repeat(b'0').take(pad));
    }

    // Convert each 5-bit block to output character.
    let mut out_bytes: Vec<u8> = Vec::with_capacity((bits.len() / 5) + 1);
    let mut i = 0usize;
    while i < bits.len() {
        let mut val: u8 = 0;
        for j in 0..5 {
            val <<= 1;
            // bits are ASCII '0'/'1'
            let bit = bits[i + j];
            if bit == b'1' {
                val |= 1;
            } else {
                // treat any non-'1' as '0' (but our construction only uses '0'/'1')
            }
        }
        out_bytes.push(value_to_output_char(val));
        i += 5;
    }

    // Write output characters, then a newline, to stdout.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if let Err(_) = handle.write_all(&out_bytes) {
        // I/O error; exit non-zero.
        exit(1);
    }
    if let Err(_) = handle.write_all(b"\n") {
        exit(1);
    }
    // flush (best-effort)
    let _ = handle.flush();

    // Successful termination; main returning () yields exit status 0.
}
