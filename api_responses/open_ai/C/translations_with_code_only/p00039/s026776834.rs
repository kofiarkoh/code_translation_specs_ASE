use std::io::{self, Read};

fn main() {
    // Read all stdin into a string, then iterate whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    // Emulate the C program's persistent int a[101] = {0};
    // The array is intentionally preserved across tokens and only overwritten
    // for positions corresponding to characters in the current token,
    // matching the original C semantics.
    let mut a: [i32; 101] = [0; 101];

    // For each whitespace-delimited token (like scanf("%s") in C)
    for token in input.split_whitespace() {
        // Map up to 100 characters, matching the original C buffer size of 101
        // (100 chars + terminating '\0'). If the token is longer, we cap it to 100.
        let bytes = token.as_bytes();
        let l = if bytes.len() > 100 { 100 } else { bytes.len() };

        // Fill a[i] for i in 0..l according to the same mapping as the C code.
        // Any byte not matching the listed Roman letters maps to 1000 (the C 'else' branch).
        for i in 0..l {
            a[i] = match bytes[i] {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                _ => 1000, // includes 'M' and any other byte
            };
        }

        // Compute the sum s with the same loop/skip semantics as the C code.
        // Note: accesses to a[l] (when comparing a[l-1] < a[l]) are valid because
        // `a` has length 101 and l <= 100.
        let mut s: i32 = 0;
        let mut i = 0usize;
        while i < l {
            if a[i] < a[i + 1] {
                s += a[i + 1] - a[i];
                i += 2;
            } else {
                s += a[i];
                i += 1;
            }
        }

        // Print result exactly as in C: "%d\n"
        println!("{}", s);
    }
}
