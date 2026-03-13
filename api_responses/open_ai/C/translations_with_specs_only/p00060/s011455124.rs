// Translated from the provided specification.
// Reads triples of integers from stdin. For each triple (c1, c2, c3),
// counts how many integers i in 1..=10 satisfy:
//   i != c1 && i != c2 && i != c3 && (c1 + c2 + i) <= 20
// If the count is >= 4 prints "YES\n", otherwise "NO\n".
// Stops processing when fewer than three tokens remain or a token
// cannot be parsed as an integer. Exits with status 0.

use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, behave like EOF: stop processing and exit 0.
        return;
    }

    let mut iter = input.split_whitespace();

    loop {
        // Attempt to read three tokens; if any token is missing or fails to parse,
        // stop processing (do not emit an error).
        let t1 = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let c1 = match t1.parse::<i64>() {
            Ok(v) => v,
            Err(_) => break,
        };

        let t2 = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let c2 = match t2.parse::<i64>() {
            Ok(v) => v,
            Err(_) => break,
        };

        let t3 = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let c3 = match t3.parse::<i64>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Count candidates in 1..=10 meeting the conditions.
        let mut count: i32 = 0;
        for ii in 1..=10 {
            let i = ii as i64;
            if i != c1 && i != c2 && i != c3 && (c1 + c2 + i) <= 20 {
                count += 1;
            }
        }

        if count >= 4 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
