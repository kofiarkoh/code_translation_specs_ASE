use std::io::{self, Read};

fn main() {
    // C globals:
    // char B[200], c[256]; int i;
    // In C, globals are zero-initialized. We'll mirror that with local zeroed arrays.
    let mut b = [0u8; 200];
    let mut c = [0u8; 256];
    let mut i: usize = 0;

    // Emulate `gets(B);` as closely as practical:
    // - reads a line from stdin up to '\n' (not including it)
    // - stores bytes into B and NUL-terminates
    // - no bounds checking in C; here we truncate to fit (best-effort without unsafe)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.splitn(2, '\n').next().unwrap_or("");

    let bytes = line.as_bytes();
    let n = bytes.len().min(b.len().saturating_sub(1));
    b[..n].copy_from_slice(&bytes[..n]);
    b[n] = 0; // NUL terminator

    // for(;B[i];i++) if(!i||B[i]!=B[i-1])c[B[i]]++;
    while i < b.len() && b[i] != 0 {
        if i == 0 || b[i] != b[i - 1] {
            let idx = b[i] as usize;
            // C `char` arithmetic promotes to int then assigns back to char; overflow wraps
            // for unsigned char. We'll use wrapping_add on u8 to emulate wraparound.
            c[idx] = c[idx].wrapping_add(1);
        }
        i += 1;
    }

    // i=!puts(c['o']<c['x']?"x":"o");
    // puts prints the string plus '\n' and returns non-negative on success.
    // `!puts(...)` therefore yields 0 on success, 1 on failure. We don't use `i` after.
    let out = if c[b'o' as usize] < c[b'x' as usize] { "x" } else { "o" };
    print!("{out}\n");
}
