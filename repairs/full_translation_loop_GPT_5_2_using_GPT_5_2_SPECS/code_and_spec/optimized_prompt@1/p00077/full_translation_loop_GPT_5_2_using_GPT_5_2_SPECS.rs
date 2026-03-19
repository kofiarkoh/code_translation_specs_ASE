use std::io::{self, Read};

const MAX: usize = 100;

fn main() {
    // Read entire stdin, then split into whitespace-delimited tokens (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // Spec precondition: token fits within MAX (including terminator in C).
        // In C, str has size MAX and scanf("%s") would overflow if longer; behavior undefined.
        // We do not attempt to emulate UB; we just proceed.
        if token.len() >= MAX {
            // Undefined behavior in the original; we keep behavior unspecified.
            // Proceeding may differ from C in overflow cases.
        }

        let bytes = token.as_bytes();
        let mut i: usize = 0;

        while i < bytes.len() {
            if bytes[i] == b'@' {
                // Spec precondition: well-formed marker: at least two more chars exist.
                // C code:
                //   for(++i, j=0; j<str[i]-'0'; ++j) putchar(str[i+1]);
                //   i += 1;
                i += 1; // move to digit
                if i >= bytes.len() {
                    // Would be UB in C; stop to avoid panic.
                    break;
                }
                let count = bytes[i].wrapping_sub(b'0') as usize;
                let ch = if i + 1 < bytes.len() { bytes[i + 1] } else { b'\0' };
                for _ in 0..count {
                    out.push(ch as char);
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
