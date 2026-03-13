use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse the first whitespace-separated token as a 32-bit signed integer.
    // This mirrors the C program's scanf("%d", &n) which expects a single int on stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    // If parsing fails we call unwrap(), which will panic. The original C program exhibited
    // undefined behavior if scanf failed; panicking here is a narrow, explicit failure mode
    // that avoids inventing extra handling.
    let n: i32 = input
        .split_whitespace()
        .next()
        .expect("no input")
        .parse()
        .unwrap();

    // Use i32 to reflect the Specification's "int (32-bit typical)".
    // Start value a = 100000.
    let mut a: i32 = 100_000;

    // The C loop semantics with a for-loop like `for (i = 0; i < n; ++i)` means that if n <= 0
    // the loop body executes zero times. Reproduce that behavior with a while loop using i32.
    let mut i: i32 = 0;
    while i < n {
        // Compute floor(1.05 * a). For positive integers this is exactly (a * 105) / 100
        // using integer arithmetic (truncating division). This preserves the intended discard
        // of fractional components before rounding up to the next multiple of 1000.
        let mut x = (a * 105) / 100;

        // Round up to the next multiple of 1000 unless already a multiple.
        if x % 1000 != 0 {
            x = ((x / 1000) + 1) * 1000;
        }

        a = x;
        i += 1;
    }

    // Print the final a followed by a newline, and exit implicitly with status 0.
    println!("{}", a);
}
