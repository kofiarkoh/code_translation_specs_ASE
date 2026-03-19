use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic `scanf("%s", S)`:
    // - skips leading whitespace
    // - reads a non-whitespace token
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C arrays: char S[11], ans[11]
    // We'll emulate fixed-size buffers with explicit NUL termination.
    let mut s = [0u8; 11];
    let mut ans = [0u8; 11];

    // Copy token bytes into s, truncating to 10 bytes, and NUL-terminate.
    // Note: In the original C, longer input would overflow (UB). We cannot
    // reproduce UB in safe Rust; truncation is the closest safe behavior.
    let bytes = token.as_bytes();
    let n = bytes.len().min(10);
    s[..n].copy_from_slice(&bytes[..n]);
    s[n] = 0;

    let mut j: usize = 0;

    for i in 0..11 {
        let c = s[i];
        if c == b'B' {
            if j > 0 {
                j -= 1;
            } else {
                j = 0;
            }
        } else {
            // In the C code, this can write past ans[10] if j grows too large (UB).
            // Here we clamp to avoid panic while keeping behavior as close as possible.
            if j < 11 {
                ans[j] = c;
                j += 1;
            }
            if c == 0 {
                break;
            }
        }
    }

    // Mimic `printf("%s", ans)` by printing bytes up to the first NUL.
    let end = ans.iter().position(|&b| b == 0).unwrap_or(ans.len());
    let out = String::from_utf8_lossy(&ans[..end]);
    print!("{out}");
}
