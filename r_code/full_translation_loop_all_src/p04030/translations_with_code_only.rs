use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C arrays: char S[11], ans[11]; uninitialized in C.
    // We'll model them as fixed-size byte arrays. `ans` is intentionally left without
    // guaranteed NUL-termination to mirror the C bug/UB as closely as we can in safe Rust.
    let mut s = [0u8; 11];
    let mut ans = [0u8; 11];

    // Copy up to 10 bytes and NUL-terminate (typical behavior of scanf into a 11-byte buffer
    // for <=10-char tokens). If the token is longer, C would overflow (UB); we truncate.
    let bytes = token.as_bytes();
    let n = bytes.len().min(10);
    s[..n].copy_from_slice(&bytes[..n]);
    s[n] = 0;

    let mut j: usize = 0;

    for i in 0..11 {
        if s[i] == b'B' {
            if j > 0 {
                j -= 1;
            } else {
                j = 0;
            }
        } else {
            // In C, this can write out of bounds if j reaches 11 (UB).
            // Here we saturate to avoid panic while keeping behavior close for valid inputs.
            if j < 11 {
                ans[j] = s[i];
            }
            j = j.saturating_add(1);

            if s[i] == 0 {
                break;
            }
        }
    }

    // C prints ans as a C-string: printf("%s", ans);
    // That reads until the first NUL byte; if none, it would read past the array (UB).
    // In Rust, we print up to the first NUL within the 11-byte buffer.
    let end = ans.iter().position(|&b| b == 0).unwrap_or(ans.len());
    let out = String::from_utf8_lossy(&ans[..end]);
    print!("{}", out);
}
