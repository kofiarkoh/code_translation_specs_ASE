use std::io::{self, Read};

fn main() {
    // Read entire stdin, then split by whitespace into tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // Specification says behavior is undefined if token length > 10000.
        // We do not add explicit handling; we proceed normally.
        let bytes = token.as_bytes();
        let mut joi: u32 = 0;
        let mut ioi: u32 = 0;

        if bytes.len() >= 3 {
            for i in 0..=bytes.len() - 3 {
                let a = bytes[i];
                let b = bytes[i + 1];
                let c = bytes[i + 2];

                if a == b'J' && b == b'O' && c == b'I' {
                    joi += 1;
                }
                if a == b'I' && b == b'O' && c == b'I' {
                    ioi += 1;
                }
            }
        }

        out.push_str(&format!("{joi}\n{ioi}\n"));
    }

    print!("{out}");
}
