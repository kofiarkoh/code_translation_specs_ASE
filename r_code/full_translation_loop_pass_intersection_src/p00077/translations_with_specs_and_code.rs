use std::io::{self, Read};

fn main() {
    // Read entire stdin, then split into whitespace-delimited tokens (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // SPEC preconditions guarantee:
        // - token length <= 99
        // - every '@' is followed by a digit and then a character
        // We implement accordingly; if violated, behavior is unspecified by the spec.
        let bytes = token.as_bytes();
        let mut i: usize = 0;

        while i < bytes.len() {
            if bytes[i] == b'@' {
                // C code does: for(++i, j=0; j<str[i]-'0'; ++j) print str[i+1]; i += 1;
                i += 1; // now at digit
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
