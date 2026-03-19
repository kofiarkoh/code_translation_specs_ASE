use std::io::{self, Read};

fn main() {
    // Specification is underspecified: it does not define the computation.
    // Narrowest reasonable interpretation that preserves stated postconditions:
    // - output is a single non-negative integer derived from n and m
    // - depends on their relationship
    // - m >= 1 (precondition), but no exceptional behavior is specified
    //
    // We therefore compute a non-negative "distance to threshold":
    // result = max(0, n - m)
    //
    // This uses only the relationship between n and m and guarantees non-negativity.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: i128 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // No inputs: no observable output specified.
    };
    let m: i128 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // Missing second input: no observable output specified.
    };

    // Enforce non-negative output without inventing error handling.
    let result: i128 = if n > m { n - m } else { 0 };

    print!("{result}");
}
