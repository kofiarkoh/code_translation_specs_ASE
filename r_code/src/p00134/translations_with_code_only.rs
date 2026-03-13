// Translated from the provided C program.
// Preserves C int semantics as i32 and uses wrapping add to emulate typical
// two's-complement wraparound on overflow. The loop logic matches the C
// `for(i = 0; i < n; i++)` behavior (i is a signed int), so if `n` is
// non-positive the loop does not run. Division is the native i32 division,
// which in Rust will panic on divide-by-zero similarly to the undefined
// behavior in C (this preserves observable crash behavior).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Parse n as i32 to match C's `int`.
    let n: i32 = it
        .next()
        .expect("expected n")
        .parse()
        .expect("invalid integer for n");

    // sum and i as i32 to mirror C `int` behavior.
    let mut sum: i32 = 0;
    let mut i: i32 = 0;

    while i < n {
        let val: i32 = it
            .next()
            .expect("expected input value")
            .parse()
            .expect("invalid integer in input");
        // Use wrapping_add to emulate typical two's-complement wraparound on overflow.
        sum = sum.wrapping_add(val);
        i += 1;
    }

    // Integer division with i32; will panic on division by zero like C's undefined behavior.
    let result = sum / n;
    // Print with a single trailing newline to match printf("%d\n", ...);
    println!("{}", result);
}
