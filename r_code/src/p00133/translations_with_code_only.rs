// rotation_of_a_pattern.rs
//
// Translated from the given C program.
// Preserves the C semantics:
// - Uses 8x9 byte arrays for pat1/pat2 (8 chars + null terminator per row).
// - rot() writes only indices [0..7] of each row; the 9th byte is preserved/copyed
//   just like the original (globals are zero-initialized).
// - display() prints each row as a C string (bytes up to the first NUL).
// - After each rotation, the entire 8*9 bytes are memcpy'd from pat2 to pat1.
//
// I/O formatting matches the original exactly.

use std::io::{self, Read, Write};

fn rot(pat1: &[[u8; 9]; 8], pat2: &mut [[u8; 9]; 8]) {
    for i in 0..8 {
        for j in 0..8 {
            // pat2[i][j] = pat1[7-j][i];
            pat2[i][j] = pat1[7 - j][i];
        }
        // Intentionally do NOT touch pat2[i][8] to preserve the C behavior.
    }
}

fn display<W: Write>(pat2: &[[u8; 9]; 8], out: &mut W) -> io::Result<()> {
    for i in 0..8 {
        // Print bytes up to the first NUL (like printf("%s", &pat2[i][0])).
        let mut end = 9;
        for j in 0..9 {
            if pat2[i][j] == 0 {
                end = j;
                break;
            }
        }
        out.write_all(&pat2[i][0..end])?;
        out.write_all(b"\n")?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Read all input and split by whitespace to emulate scanf("%s", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tokens = input.split_whitespace();

    // pat1 and pat2 are zero-initialized (like C globals).
    let mut pat1: [[u8; 9]; 8] = [[0u8; 9]; 8];
    let mut pat2: [[u8; 9]; 8] = [[0u8; 9]; 8];

    // Read 8 tokens into pat1 rows, copy up to 8 bytes per row.
    for i in 0..8 {
        if let Some(tok) = tokens.next() {
            let bytes = tok.as_bytes();
            let len = bytes.len().min(8);
            for j in 0..len {
                pat1[i][j] = bytes[j];
            }
            // pat1[i][len] remains 0 (null terminator) because the array was zero-initialized.
        } else {
            // If fewer than 8 tokens are present, leave remaining rows as all zeros.
            // This mirrors the effect of scanf not filling those rows.
        }
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for iter in 1..=3 {
        rot(&pat1, &mut pat2);
        writeln!(out, "{}", iter * 90)?;
        display(&pat2, &mut out)?;
        // memcpy(pat1, pat2, 8*9);
        pat1 = pat2;
    }

    Ok(())
}
