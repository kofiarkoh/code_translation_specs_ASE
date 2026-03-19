use std::io::{self, Read};

const MAX_TOKEN_LEN_INCL_TERMINATOR: usize = 100;
const MARKER: char = '@';

fn main() {
    // Read entire stdin, then split into whitespace-delimited tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // Specification: if token exceeds max supported length, behavior is undefined.
        // In Rust we must define behavior; we choose the narrowest reasonable action:
        // stop processing further to avoid inventing recovery semantics.
        if tok.chars().count() + 1 > MAX_TOKEN_LEN_INCL_TERMINATOR {
            break;
        }

        let chars: Vec<char> = tok.chars().collect();
        let mut i = 0usize;

        while i < chars.len() {
            let c = chars[i];
            if c != MARKER {
                out.push(c);
                i += 1;
            } else {
                // Specification precondition: marker is well-formed and followed by digit + char.
                // If not, behavior is undefined; we stop to avoid inventing output.
                if i + 2 >= chars.len() {
                    break;
                }

                let digit_ch = chars[i + 1];
                let repeat_ch = chars[i + 2];

                // Specification: digit is assumed 0-9; otherwise behavior not inferable.
                // We interpret as a digit-derived count only when it is a decimal digit.
                let count = digit_ch.to_digit(10).unwrap_or(0) as usize;

                for _ in 0..count {
                    out.push(repeat_ch);
                }

                i += 3;
            }
        }

        out.push('\n');
    }

    print!("{out}");
}
