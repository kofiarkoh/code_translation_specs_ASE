use std::io::{self, Read};

fn main() {
    // pat1 and pat2 mirror the C program's char pat1[8][9] / pat2[8][9].
    // Each row has 9 bytes so that we can store up to 8 characters plus a NUL (0).
    // Initialize with zeros to emulate C's global zero-initialization.
    let mut pat1: [[u8; 9]; 8] = [[0u8; 9]; 8];
    let mut pat2: [[u8; 9]; 8] = [[0u8; 9]; 8];

    // Read all stdin and split on whitespace to obtain tokens (rows).
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut tokens = input.split_whitespace();

    // For i in 0..8, read a token if available. If no token, leave row as all zeros
    // to mimic the C program's behavior when scanf hits EOF (global arrays remain zero).
    for i in 0..8 {
        if let Some(tok) = tokens.next() {
            // Copy up to 8 bytes of the token into pat1[i], then NUL-terminate.
            // This mirrors the C behavior which operates on bytes (char).
            let bytes = tok.as_bytes();
            let len = bytes.len().min(8);
            pat1[i][..len].copy_from_slice(&bytes[..len]);
            pat1[i][len] = 0;
            // remaining bytes are already zeroed from initialization
        } else {
            // leave pat1[i] as all zeros (empty string)
        }
    }

    // Rotation function: pat2[i][j] = pat1[7-j][i] for i,j in 0..8
    // After filling columns 0..7, ensure pat2[i][8] = 0 (NUL terminator).
    let mut rot = |src: &[[u8; 9]; 8], dst: &mut [[u8; 9]; 8]| {
        for i in 0..8 {
            for j in 0..8 {
                dst[i][j] = src[7 - j][i];
            }
            dst[i][8] = 0;
        }
    };

    // Display function: print each row of pat2 up to first NUL byte, followed by newline.
    let mut display = |buf: &[[u8; 9]; 8]| {
        for i in 0..8 {
            // find position of first 0 byte
            let len = match buf[i].iter().position(|&b| b == 0) {
                Some(p) => p,
                None => 9, // should not happen; treat whole array as content
            };
            // Interpret bytes up to len as UTF-8 lossily (matches expected ASCII use).
            let s = String::from_utf8_lossy(&buf[i][..len]);
            println!("{}", s);
        }
    };

    // Perform three rotations, printing angle and rotated pattern each time.
    for k in 1..=3 {
        rot(&pat1, &mut pat2);
        println!("{}", k * 90);
        display(&pat2);
        // memcpy(pat1, pat2, 8*9) in C: copy entire arrays including NULs.
        pat1 = pat2;
    }
}
