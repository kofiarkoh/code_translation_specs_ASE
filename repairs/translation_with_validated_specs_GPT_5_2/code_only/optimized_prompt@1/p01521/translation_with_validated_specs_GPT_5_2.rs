use std::io::{self, Read};

fn main() {
    // C globals:
    // char B[200], c[256]; int i;
    // In C, globals are zero-initialized.
    let mut b = [0u8; 200];
    let mut c = [0u8; 256];
    let mut i: usize = 0;

    // Emulate gets(B):
    // - reads a line from stdin up to '\n' (not including it), then NUL-terminates.
    // - no bounds checking in C; here we truncate to 199 bytes to keep space for NUL.
    // - if input is empty/EOF, B[0] remains 0.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let line = input.splitn(2, '\n').next().unwrap_or("");
    let bytes = line.as_bytes();
    let n = bytes.len().min(199);
    b[..n].copy_from_slice(&bytes[..n]);
    b[n] = 0;

    // for(; B[i]; i++) if(!i || B[i]!=B[i-1]) c[B[i]]++;
    while i < b.len() && b[i] != 0 {
        if i == 0 || b[i] != b[i - 1] {
            let idx = b[i] as usize;
            // C's `char` may be signed; indexing `c[B[i]]` is UB for negative values.
            // Here we treat bytes as unsigned (common intent for such code).
            c[idx] = c[idx].wrapping_add(1); // emulate unsigned char overflow
        }
        i += 1;
    }

    // i = !puts(c['o']<c['x']?"x":"o");
    // puts prints the string plus '\n'. `!` in C yields 1 if puts returned 0 else 0.
    // We don't use `i` afterwards, but we preserve the output exactly.
    let out = if c[b'o' as usize] < c[b'x' as usize] { "x" } else { "o" };
    print!("{out}\n");
}
