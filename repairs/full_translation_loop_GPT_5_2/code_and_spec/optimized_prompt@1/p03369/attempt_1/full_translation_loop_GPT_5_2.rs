use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token (like scanf("%s", ...)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap_or("");

    // Per spec: behavior is undefined if input is not exactly three characters.
    // We assume valid input and index accordingly.
    let bytes = s.as_bytes();

    let mut m = 0;
    for i in 0..3 {
        if bytes[i] == b'o' {
            m += 1;
        }
    }

    let ans = 700 + 100 * m;
    println!("{ans}");
}
