use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin; on failure, terminate with non-zero exit status.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let mut it = input.split_whitespace();

    // If the integer input cannot be read successfully, terminate non-zero.
    let n: usize = match it.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // If the string input cannot be read successfully, terminate non-zero.
    let s = match it.next() {
        Some(v) => v,
        None => process::exit(1),
    };

    // Preconditions say n <= s.len(); we do not invent extra behavior if violated.
    // Evaluate only the first n characters.
    let mut r_count: usize = 0;
    for ch in s.chars().take(n) {
        if ch == 'R' {
            r_count += 1;
        }
    }

    // "Yes" if count of 'R' exceeds count of other characters among evaluated n.
    // Equivalent to r_count > n/2 (integer division).
    if r_count * 2 > n {
        print!("Yes");
    } else {
        print!("No");
    }
}
