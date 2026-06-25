use std::io::{self, Read};

fn main() {
    // Read entire stdin, tokenized like scanf("%d %d") and scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();

    // C: char s[51][60] = {""};
    // We'll model as fixed-size grid with zero-initialization.
    // Important: the C code reads s[i+1][j] and s[i][j+1] without bounds checks.
    // In C, this is undefined behavior when i==H-1 or j==W-1, but because the array
    // is larger than the used HxW region and is zero-initialized, the *intended*
    // behavior is effectively reading '\0' from the padding.
    let mut s = vec![[0u8; 60]; 51];

    for i in 0..h {
        let tok = it.next().unwrap_or("");
        let bytes = tok.as_bytes();
        // scanf("%s") writes a NUL terminator; we already have zeros, so just copy.
        let len = bytes.len().min(59); // leave room for terminator (not strictly needed here)
        s[i][..len].copy_from_slice(&bytes[..len]);
    }

    for i in 0..h {
        for j in 0..w {
            let u = if i == 0 { 0u8 } else { s[i - 1][j] };
            let l = if j == 0 { 0u8 } else { s[i][j - 1] };

            // Matches the C code's out-of-range reads into the zero-initialized padding.
            let d = s[i + 1][j];
            let r = s[i][j + 1];

            if s[i][j] == b'#' {
                if u == b'#' || d == b'#' || r == b'#' || l == b'#' {
                    continue;
                } else {
                    print!("No");
                    return;
                }
            }
        }
    }

    print!("Yes");
}
