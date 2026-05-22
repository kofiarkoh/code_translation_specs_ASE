use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // The specification defines matches in terms of "characters".
        // To avoid inventing Unicode-specific semantics, we interpret the token
        // as a byte string and match ASCII byte sequences "JOI" and "IOI".
        // This matches typical competitive-programming intent and preserves
        // exact 3-unit substring matching with overlaps.
        let b = token.as_bytes();

        let mut joi: usize = 0;
        let mut ioi: usize = 0;

        if b.len() >= 3 {
            for i in 0..=b.len() - 3 {
                if b[i] == b'J' && b[i + 1] == b'O' && b[i + 2] == b'I' {
                    joi += 1;
                }
                if b[i] == b'I' && b[i + 1] == b'O' && b[i + 2] == b'I' {
                    ioi += 1;
                }
            }
        }

        out.push_str(&format!("{joi}\n{ioi}\n"));
    }

    print!("{out}");
}
