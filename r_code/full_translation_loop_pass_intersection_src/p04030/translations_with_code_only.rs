use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic `scanf("%s", S)`:
    // - skips leading whitespace
    // - reads a non-whitespace token
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C arrays: char S[11], ans[11]
    // We'll model them as fixed-size byte arrays, NUL-terminated like C strings.
    let mut s = [0u8; 11];
    let mut ans = [0u8; 11];

    // `scanf("%s", S)` in C is unsafe if token is longer than 10 bytes (overflow UB).
    // Here we emulate the in-bounds behavior by truncating to 10 bytes and adding NUL.
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
            // In the original C, `ans[j++] = S[i];` can overflow `ans` if j reaches 11 (UB).
            // With the truncation above and the loop bound, j stays within 0..=10 here.
            ans[j] = s[i];
            j += 1;
            if s[i] == 0 {
                break;
            }
        }
    }

    // Mimic `printf("%s", ans)` by printing up to the first NUL byte.
    let end = ans.iter().position(|&b| b == 0).unwrap_or(ans.len());
    let out = String::from_utf8_lossy(&ans[..end]);
    print!("{out}");
}
