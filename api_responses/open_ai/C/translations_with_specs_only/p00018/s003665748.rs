// main.rs
//
// Translation of the provided specification into Rust.
// Notes on semantic choices:
// - The C 'int' type is represented here as i32, which is the typical size on many platforms.
//   The specification requires values that "fit in the C int type"; parsing will fail (and the
//   program will abort) if a token does not fit in i32. This matches the specification's
//   statement that input errors are not explicitly handled and may cause abnormal termination.
// - The program assumes at least five integer tokens are present on stdin. No validation is
//   performed beyond parsing; if fewer than five tokens exist or parsing fails, the program will
//   panic (runtime error), consistent with the spec's unspecified behavior in that case.
// - Output: five integers printed in non-increasing (descending) order, separated by single
//   spaces, with a single trailing space after the fifth integer and no trailing newline.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    // If reading from stdin fails, let the program panic (spec allows runtime errors to abort).
    io::stdin().read_to_string(&mut input).unwrap();

    // Split on whitespace and parse the first five tokens as i32.
    // Per spec: exactly five integers are read (tokens beyond five are ignored).
    let mut tokens = input.split_whitespace();

    // Collect exactly five integers in the order read.
    // Use unwrap() so that failures (missing tokens or parse errors) cause a panic,
    // reflecting the unspecified behavior described in the specification.
    let mut vals: [i32; 5] = [
        tokens.next().unwrap().parse::<i32>().unwrap(),
        tokens.next().unwrap().parse::<i32>().unwrap(),
        tokens.next().unwrap().parse::<i32>().unwrap(),
        tokens.next().unwrap().parse::<i32>().unwrap(),
        tokens.next().unwrap().parse::<i32>().unwrap(),
    ];

    // Sort in non-increasing (descending) order.
    vals.sort_unstable_by(|a, b| b.cmp(a));

    // Print the five integers separated by single spaces, with a trailing space after the fifth.
    // No trailing newline is printed (as required).
    for v in &vals {
        print!("{} ", v);
    }

    // Normal termination returns exit status 0.
}
