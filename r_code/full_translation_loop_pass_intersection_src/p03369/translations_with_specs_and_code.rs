use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap();

    // SPEC: count 'o' among the first three characters.
    // Preconditions say the token has at least three characters; otherwise behavior is undefined.
    let mut m = 0;
    for ch in s.chars().take(3) {
        if ch == 'o' {
            m += 1;
        }
    }

    let score = 700 + 100 * m;
    println!("{score}");
}
