use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic `scanf("%s", str)` tokenization (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // C uses a fixed buffer `char str[MAX]` and `scanf("%s", str)` which can overflow.
        // We cannot (safely) emulate overflow; we just process the token as provided.
        let bytes = token.as_bytes();
        let length = bytes.len();

        let mut i: usize = 0;
        while i < length {
            if bytes[i] == b'@' {
                // C code:
                // for(++i, j=0; j<str[i]-'0'; ++j) printf("%c", str[i+1]);
                // i += 1;
                // continue;
                i += 1;

                // The original C has undefined behavior if '@' is last char, or if i+1 is OOB,
                // or if `str[i]-'0'` is negative (signed char) etc.
                // Here we emulate the "intended" behavior for well-formed inputs:
                // repeat next character (i+1) (digit - '0') times.
                if i < length {
                    let count = (bytes[i] as i32) - (b'0' as i32);
                    if count > 0 && i + 1 < length {
                        let ch = bytes[i + 1] as char;
                        for _ in 0..(count as usize) {
                            out.push(ch);
                        }
                    }
                }

                i += 1; // matches `i += 1;` in C after the loop
                continue;
            } else {
                out.push(bytes[i] as char);
            }
            i += 1;
        }
        out.push('\n');
    }

    print!("{out}");
}
