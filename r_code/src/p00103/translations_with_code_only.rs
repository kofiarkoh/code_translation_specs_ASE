// translated from the provided C program.
// Preserves the program's semantics for handling "HIT", "HOMERUN", and "OUT" events.
// Input is tokenized by whitespace (like scanf("%s")), and output prints the score
// for each inning on its own line.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and split by whitespace to emulate scanf("%s") tokenization.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut toks = input.split_whitespace();

    // Parse n (number of innings). The C code used `int n; scanf("%d", &n);`
    // We emulate that by parsing the first token as a 32-bit signed integer.
    let mut n: i32 = match toks.next() {
        Some(s) => s.parse().unwrap_or(0),
        None => return, // no input
    };

    while n != 0 {
        let mut score: i32 = 0;
        let mut base: [i32; 3] = [0, 0, 0];
        let mut out: i32 = 0;

        while out < 3 {
            // In C the code does scanf("%s", event) without checking the return value.
            // Here we take the next token; if missing, we exit to avoid panic.
            let event = match toks.next() {
                Some(s) => s,
                None => return, // input exhausted; stop program (similar to terminating)
            };

            if event == "HIT" {
                base[0] += 1;
                // The C code loops i = 0..2 and if base[i] > 1 pushes one runner forward.
                for i in 0..3 {
                    if base[i] > 1 {
                        if i < 2 {
                            base[i + 1] += 1;
                        } else {
                            score += 1;
                        }
                        base[i] -= 1;
                    }
                }
            } else if event == "HOMERUN" {
                // Score everyone on base, clear bases, then score batter
                for i in 0..3 {
                    if base[i] == 1 {
                        score += 1;
                        base[i] = 0;
                    }
                }
                score += 1;
            } else if event == "OUT" {
                out += 1;
            }
            // Other tokens are ignored in the original C (they would just not match
            // any strcmp branch), so here we do nothing for them.
        }

        println!("{}", score);
        n -= 1;
    }
}
