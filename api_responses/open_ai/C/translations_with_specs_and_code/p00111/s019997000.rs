// translated from the given C program according to the provided specification.
// Notes:
// - The original C used printf("%\n"), which is an invalid format string (undefined behavior).
//   Per the specification we treat the intent as attempting to emit a newline, but the
//   C behavior is undefined. To remain safe and well-defined in Rust we emit a newline
//   normally here. This comment documents that mismatch with the C source.
// - The program assumes input characters are in the range ' ' (32) ..= '_' (95).
//   Characters outside that range will cause a panic (the original C program would have
//   indexed out-of-bounds, which is undefined behavior per the spec).
// - Implementation is idiomatic, safe Rust, and preserves the decoding semantics.

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

const ENCODE: [&str; 64] = [
    "11010", "",     "",     "",     "",     "",     "",     "11110",
    "",       "",     "",     "",     "11100", "11101", "11011", "",
    "",       "",     "",     "",     "",     "",     "",     "",
    "",       "",     "",     "",     "",     "",     "",     "11111",
    "",       "00000", "00001", "00010", "00011", "00100", "00101", "00110",
    "00111",  "01000", "01001", "01010", "01011", "01100", "01101", "01110",
    "01111",  "10000", "10001", "10010", "10011", "10100", "10101", "10110",
    "10111",  "11000", "11001", "",     "",     "",     "",     "",
];

fn build_decode() -> HashMap<u32, char> {
    let mut m = HashMap::new();
    m.insert(212u32, ' ');
    m.insert(111111u32, '\'');
    m.insert(111122u32, ',');
    m.insert(21121112u32, '-');
    m.insert(121112u32, '.');
    m.insert(111112u32, '?');
    m.insert(211212u32, 'A');
    m.insert(21122121u32, 'B');
    m.insert(1212u32, 'C');
    m.insert(1112u32, 'D');
    m.insert(221u32, 'E');
    m.insert(12112u32, 'F');
    m.insert(21122122u32, 'G');
    m.insert(121111u32, 'H');
    m.insert(1222u32, 'I');
    m.insert(21122111u32, 'J');
    m.insert(1221u32, 'K');
    m.insert(11211u32, 'L');
    m.insert(21122112u32, 'M');
    m.insert(21122221u32, 'N');
    m.insert(11212u32, 'O');
    m.insert(222u32, 'P');
    m.insert(21122222u32, 'Q');
    m.insert(2111u32, 'R');
    m.insert(11221u32, 'S');
    m.insert(11222u32, 'T');
    m.insert(21122211u32, 'U');
    m.insert(21122212u32, 'V');
    m.insert(111121u32, 'W');
    m.insert(21121121u32, 'X');
    m.insert(21121122u32, 'Y');
    m.insert(21121111u32, 'Z');
    m
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let decode = build_decode();

    for line_res in stdin.lock().lines() {
        let line = line_res?; // line excludes the trailing newline
        // Build the concatenated bit string.
        let mut code = String::new();
        for &b in line.as_bytes() {
            // The C program indexes encode by (c - ' '), valid inputs are ' ' ..= ' ' + 63.
            // Per specification, characters outside that range are undefined; here we panic.
            if b < b' ' || b > (b' ' + 63) {
                panic!("input character out of supported range: {}", b);
            }
            let idx = (b - b' ') as usize;
            let s = ENCODE[idx];
            if !s.is_empty() {
                code.push_str(s);
            }
        }

        // Scan the bit string left-to-right, building the decimal key composed of digits 1 and 2.
        // bit '0' -> digit 1 ; bit '1' -> digit 2
        let mut stk: u32 = 0;
        // Collect decoded characters for this input line.
        let mut out = String::new();

        for &bb in code.as_bytes() {
            // Expect only '0' or '1' bytes in code; ignore any other bytes (there shouldn't be any).
            if bb != b'0' && bb != b'1' {
                continue;
            }
            let digit = (bb - b'0') as u32 + 1; // maps 0->1, 1->2
            // Multiply previous key by 10 and add the new digit.
            // Use checked operations to avoid silent overflow; the C program used a huge array and would have UB on overflow.
            stk = stk.checked_mul(10)
                .and_then(|v| v.checked_add(digit))
                .expect("numeric overflow while accumulating decode key");
            if let Some(&ch) = decode.get(&stk) {
                out.push(ch);
                stk = 0;
            }
        }

        // Write decoded characters and then a newline.
        // The original C program attempted to print a newline with an invalid format string (printf("%\n")),
        // which is undefined behavior; here we emit a newline reliably to reflect the intended behavior.
        stdout.write_all(out.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
    }

    Ok(())
}
