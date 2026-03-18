use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap_or("");

    // Spec: max length 10 (undefined behavior if longer). We do not add extra handling.
    // Process bytes to match C's char-by-char behavior for ASCII inputs.
    let bytes = s.as_bytes();

    let mut ans: Vec<u8> = Vec::with_capacity(11);
    let mut j: usize = 0;

    // C loops up to 11 and relies on '\0' terminator; in Rust we iterate actual bytes.
    // Semantics: 'B' acts like backspace (removes previous output char if any).
    for &b in bytes.iter() {
        if b == b'B' {
            if j > 0 {
                j -= 1;
                ans.truncate(j);
            } else {
                j = 0;
            }
        } else {
            if j == ans.len() {
                ans.push(b);
            } else {
                ans[j] = b;
            }
            j += 1;
        }
    }

    // C prints ans as a C-string; here we print exactly the constructed bytes as UTF-8.
    // (Input is expected to be in the usual ASCII range for this problem.)
    let out = String::from_utf8_lossy(&ans);
    print!("{}", out);
}
