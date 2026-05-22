use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic repeated `scanf("%s", str)` by splitting on whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        let bytes = token.as_bytes();
        let mut joi: i32 = 0;
        let mut ioi: i32 = 0;

        // C loop: for (i = 0; i < strlen(str) - 2; i++)
        // If len < 2, `strlen - 2` underflows in unsigned arithmetic and then is compared to int,
        // which is undefined/implementation-defined. In practice for this problem, inputs are
        // expected to be long enough; we emulate the intended behavior by doing no iterations
        // when len < 3.
        if bytes.len() >= 3 {
            for i in 0..=(bytes.len() - 3) {
                if &bytes[i..i + 3] == b"JOI" {
                    joi += 1;
                }
                if &bytes[i..i + 3] == b"IOI" {
                    ioi += 1;
                }
            }
        }

        out.push_str(&format!("{}\n{}\n", joi, ioi));
    }

    print!("{out}");
}
