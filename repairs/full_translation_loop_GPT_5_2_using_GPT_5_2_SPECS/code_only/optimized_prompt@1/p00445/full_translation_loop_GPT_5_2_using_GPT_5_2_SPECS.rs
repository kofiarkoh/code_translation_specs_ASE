use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic scanf("%s") tokenization (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for token in input.split_whitespace() {
        let bytes = token.as_bytes();
        let mut joi: i32 = 0;
        let mut ioi: i32 = 0;

        // C loop: for (i = 0; i < strlen(str) - 2; i++)
        // In C, strlen is size_t; if len < 2, len - 2 underflows to a huge value,
        // and the loop would attempt out-of-bounds reads (undefined behavior).
        // Here we avoid UB and only scan valid 3-byte windows.
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

        // Match C output formatting: two lines per token.
        println!("{}", joi);
        println!("{}", ioi);
    }
}
