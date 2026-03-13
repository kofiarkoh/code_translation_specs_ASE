use std::io::{self, Read};

fn main() {
    // This translation follows the provided specification.
    // We choose i32 to represent C 'int' (typical on many platforms).
    // The original C program did not check scanf return values; to preserve
    // similar unchecked behavior we use `unwrap()` on token and parse operations,
    // which will panic on malformed/missing input (analogous to undefined behavior).
    //
    // Behavior notes:
    // - If n <= 0 the loop will not execute (matching the C loop semantics).
    // - Division uses Rust's integer division for i32, which truncates toward zero
    //   like C's integer division for signed integers.
    // - No extra validation or error handling is performed (per spec).

    // Read entire stdin as a string and split on whitespace tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    // Read n (first token). Unwrap is used to mirror the unchecked nature of scanf("%d",&n).
    let n: i32 = tokens.next().unwrap().parse().unwrap();

    let mut sum: i32 = 0;
    let mut i: i32 = 0;

    // Use a while loop with i and n as i32 to preserve C semantics when n is negative.
    while i < n {
        // Read next integer token for the sequence. Unwrap to mimic unchecked scanf.
        let val: i32 = tokens.next().unwrap().parse().unwrap();
        sum = sum + val;
        i += 1;
    }

    // Compute integer average using truncating division (matches C semantics).
    let avg = sum / n;

    // Print the result followed by a newline, exactly "<integer>\n".
    println!("{}", avg);
}
