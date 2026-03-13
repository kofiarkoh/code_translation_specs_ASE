use std::io::{self, Read, Write};

fn main() {
    // Read entire stdin into a string and split on whitespace to get tokens.
    // Specification expects exactly 8 whitespace-delimited tokens, each up to 8 characters.
    // To produce deterministic behavior even if inputs are shorter than 8 tokens or rows,
    // we adopt the narrow, reasonable interpretation: each row is truncated to 8 bytes
    // if longer, or padded with ASCII space (0x20) if shorter. This yields a well-defined
    // 8x8 byte grid for rotation and printing.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, exit silently as original C program had no explicit error reporting.
        return;
    }

    let mut rows: [[u8; 8]; 8] = [[b' '; 8]; 8]; // default pad with spaces

    for (i, token) in input.split_whitespace().take(8).enumerate() {
        let bytes = token.as_bytes();
        for j in 0..8 {
            rows[i][j] = if j < bytes.len() { bytes[j] } else { b' ' };
        }
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Perform three rotations: 90, 180, 270 degrees clockwise.
    for &angle in &[90u32, 180u32, 270u32] {
        // Compute rotation: new[r][c] = old[7 - c][r]
        let mut rotated: [[u8; 8]; 8] = [[b' '; 8]; 8];
        for r in 0..8 {
            for c in 0..8 {
                rotated[r][c] = rows[7 - c][r];
            }
        }

        // Print angle line
        // Followed by 8 lines, each row of rotated grid.
        // No other text produced.
        let _ = writeln!(out, "{}", angle);
        for r in 0..8 {
            // Write the bytes of the row directly, then a newline.
            let _ = out.write_all(&rotated[r]);
            let _ = out.write_all(b"\n");
        }

        // Make rotated the current pattern for next rotation step.
        rows = rotated;
    }

    // Exit with code 0 implicitly by returning from main.
}
