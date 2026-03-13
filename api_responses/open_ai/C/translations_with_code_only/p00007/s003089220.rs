use std::io::{self, Read};

fn main() {
    // Mirror the C program which uses a 32-bit `int`.
    // Start a as 100000 like in the C code.
    let mut a: i32 = 100_000;

    // Read one integer n from stdin. The original C uses scanf("%d",&n);
    // If parsing fails (no input or invalid), we'll default n to 0.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let n: i32 = input
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    // Loop n times, preserving the C semantics:
    // a = a + a * 0.05; computed in double, then assigned to int (truncating).
    // We emulate that by converting to f64, doing the math, and casting back (truncates toward zero).
    for _ in 0..n {
        let af = a as f64;
        let updated = af + af * 0.05;
        // Casting f64 -> i32 to model the C assignment from double to int (truncation).
        a = updated as i32;

        // If not a multiple of 1000, round up to the next thousand:
        if a % 1000 != 0 {
            a = (a / 1000 + 1) * 1000;
        }
    }

    // Print result followed by a newline, exactly like printf("%d\n",a);
    println!("{}", a);
}
