use std::io::{self, Read};

fn main() {
    // C uses scanf("%s") which tokenizes on whitespace (not line-based).
    // We mirror that by reading all input and splitting on whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        let bytes = token.as_bytes();
        let mut i: usize = 0;

        while i < bytes.len() {
            if bytes[i] == b'@' {
                // Spec precondition: well-formed pattern '@' + digit + char.
                // Undefined behavior for malformed input; we do not add extra handling.
                i += 1; // move to digit
                let count = (bytes[i] - b'0') as usize;
                let ch = bytes[i + 1] as char;
                for _ in 0..count {
                    out.push(ch);
                }
                i += 2; // skip digit and repeated character
            } else {
                out.push(bytes[i] as char);
                i += 1;
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
