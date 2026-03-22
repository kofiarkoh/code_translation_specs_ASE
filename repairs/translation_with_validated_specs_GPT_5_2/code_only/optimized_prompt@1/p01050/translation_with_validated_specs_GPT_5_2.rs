use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic scanf("%s", s) by taking the first whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = match input.split_whitespace().next() {
        Some(tok) => tok.as_bytes(),
        None => {
            // If no token is present, C's scanf would fail and `s` would be uninitialized (UB).
            // We choose to do nothing meaningful; print 0 to keep program total.
            // (This case is outside well-defined C behavior anyway.)
            println!("0");
            return;
        }
    };

    // C uses `char s[100]` and indexes b[s[l]] where b has size 256.
    // To preserve the intended behavior, treat bytes as unsigned 0..255.
    let mut b = [0i32; 256];
    let mut l: i32 = 0;
    for &ch in s {
        b[ch as usize] += 1;
        l += 1;
    }

    let mut a: i32 = 0;
    let mut c: i32 = 0;

    while l != 0 {
        for i in 0..256usize {
            if b[i] != 0 {
                a += 1;
                l -= 1;
                c += 1;
                b[i] -= 1;
            } else {
                if c > 3 {
                    a -= c - 3;
                }
                c = 0;
            }
        }
    }

    // Note: The C code does not apply the c>3 adjustment after the loop ends unless a zero-count
    // bucket is encountered; we preserve that exact behavior.
    println!("{}", a);
}
