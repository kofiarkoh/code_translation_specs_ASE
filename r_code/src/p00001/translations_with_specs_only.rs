// top_three.rs
//
// Reads exactly ten integers from standard input (whitespace-separated).
// Maintains three recorded values initialized to zero placeholders.
// For each input value v, if v is strictly greater than one or more of the recorded
// values it is inserted into the top-three in the correct position (displacing lower values).
// After ten inputs are processed, prints the three recorded values in non-increasing order,
// one per line, then exits successfully.
//
// The specification does not guarantee behavior for malformed or insufficient input.
// This implementation follows that: it unwraps when reading/parsing tokens, so malformed
// or insufficient input will cause the program to panic (undefined by the spec).

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read failed");

    let mut iter = s.split_whitespace();

    // Initialize three placeholders to zero as specified.
    let mut first: i64 = 0;
    let mut second: i64 = 0;
    let mut third: i64 = 0;

    // Read exactly ten integers. If fewer tokens or non-integer tokens are present,
    // behavior is not defined by the specification; we use unwrap() to reflect that.
    for _ in 0..10 {
        let token = iter.next().unwrap(); // may panic if fewer than 10 tokens
        let v: i64 = token.parse().unwrap(); // may panic if token not an integer

        if v > first {
            // shift down
            third = second;
            second = first;
            first = v;
        } else if v > second {
            third = second;
            second = v;
        } else if v > third {
            third = v;
        }
    }

    // Print the three recorded values in non-increasing order, one per line.
    println!("{}", first);
    println!("{}", second);
    println!("{}", third);
}
