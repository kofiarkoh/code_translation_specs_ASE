use std::io::{self, Read};

fn main() {
    // Read entire stdin, then split by whitespace into tokens (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // Spec: if token length < 3 => both counts 0.
        // We count overlapping occurrences by checking every starting position.
        let bytes = token.as_bytes();
        let mut joi: i32 = 0;
        let mut ioi: i32 = 0;

        if bytes.len() >= 3 {
            for i in 0..=bytes.len() - 3 {
                if &bytes[i..i + 3] == b"JOI" {
                    joi += 1;
                }
                if &bytes[i..i + 3] == b"IOI" {
                    ioi += 1;
                }
            }
        }

        out.push_str(&format!("{joi}\n{ioi}\n"));
    }

    print!("{out}");
}
