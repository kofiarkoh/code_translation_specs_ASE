use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf("%s", S) which reads a whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C code uses `char S[3];` and then loops i=0..3 reading S[i].
    // This is potentially out-of-bounds if the input token is shorter than 3,
    // and also `scanf("%s", S)` can overflow the 3-byte buffer if longer.
    // To preserve the intended behavior for normal inputs (length >= 3),
    // we only count within the first 3 bytes available.
    let bytes = token.as_bytes();
    let mut m = 0i32;
    for i in 0..3 {
        if i < bytes.len() && bytes[i] == b'o' {
            m += 1;
        }
    }

    let result = 700 + 100 * m;
    print!("{}\n", result);
}
