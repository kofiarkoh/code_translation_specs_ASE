use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf("%s", S) would read the next whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C code uses `char S[3];` and then reads with "%s" (no bounds checking).
    // We emulate the *intended* behavior for typical inputs by considering up to 3 bytes.
    // If fewer than 3 bytes are present, missing bytes are treated as not 'o'.
    let bytes = token.as_bytes();

    let mut m: i32 = 0;
    for i in 0..3 {
        if bytes.get(i) == Some(&b'o') {
            m += 1;
        }
    }

    let result = 700 + 100 * m;
    print!("{}\n", result);
}
