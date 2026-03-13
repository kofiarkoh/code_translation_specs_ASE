use std::io::{self, Read};

fn main() {
    // The original C program:
    // - reads 5 ints into an array a[5] using scanf("%d", &a[i]);
    // - performs a simple selection-like sort to arrange values in descending order
    //   (loops i=0..4, j=i..4, if a[i] < a[j] swap)
    // - prints each value with "%d " (a trailing space after each number, no newline)
    //
    // This Rust translation preserves those behaviors:
    // - use i32 to match typical C `int` on common platforms
    // - read whitespace-separated tokens from stdin and parse as i32
    // - if there are fewer than 5 tokens or a token fails to parse, we fill that slot with 0
    //   (C would have left such memory uninitialized which is undefined; to avoid UB in Rust
    //   we choose a deterministic filler of 0)
    //
    // The sorting loops and swaps mimic the C logic exactly.

    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    // Parse tokens; on parse error use 0
    let mut toks = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap_or(0));

    let mut a: [i32; 5] = [0; 5];
    for i in 0..5 {
        if let Some(v) = toks.next() {
            a[i] = v;
        } else {
            // fewer than 5 tokens: leave as 0 (see comment above)
            a[i] = 0;
        }
    }

    // Same nested loops and swap logic as the C code
    for i in 0..5 {
        for j in i..5 {
            if a[i] < a[j] {
                let t = a[i];
                a[i] = a[j];
                a[j] = t;
            }
        }
    }

    // Print with a trailing space after each number, no newline (matches printf("%d ", a[i]))
    for i in 0..5 {
        print!("{} ", a[i]);
    }
}
