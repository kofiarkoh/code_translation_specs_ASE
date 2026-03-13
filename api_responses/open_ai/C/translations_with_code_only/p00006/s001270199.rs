use std::io::{self, Read, Write};

fn main() {
    // Read all stdin and take the first whitespace-delimited token,
    // matching scanf("%s", ...) behavior.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let token = match input.split_whitespace().next() {
        Some(t) => t,
        None => return,
    };

    // The original C used char str[21]; so we emulate that capacity.
    // To avoid undefined behavior (buffer overflow) we clamp to 20 bytes
    // (leaving room for the implicit NUL in the original intent).
    // The C program then computed strlen and reversed that many bytes.
    let bytes = token.as_bytes();
    let n = bytes.len().min(20);

    // Emulate strsub[21] initialized to zeros so there is a NUL after the copied bytes.
    let mut strsub = [0u8; 21];

    // Copy reversed bytes into strsub[0..n], similar to the C loop.
    for i in 0..n {
        strsub[i] = bytes[n - 1 - i];
    }

    // Print exactly like printf("%s", strsub): write bytes up to the first NUL.
    // Since we initialized strsub with zeros and filled 0..n, the first NUL is at index n.
    let _ = io::stdout().write_all(&strsub[..n]);
}
