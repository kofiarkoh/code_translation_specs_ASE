// translated from the given C program
// - Uses f32 to mirror C's `float` semantics.
// - Reads all of stdin and splits on whitespace to emulate scanf("%f", ...).
// - If a token fails to parse as f32 we stop processing further tokens. The original C code
//   used `scanf("%f", &m) != EOF` which would behave differently on malformed input (it can
//   return 0 and cause scanf to not advance), but typical inputs are numeric so this is a
//   pragmatic and safe choice.
// - If there are no valid float tokens, this program prints nothing. The original C program
//   would invoke undefined behavior (printing uninitialized hm/lm), so reproducing that UB in
//   safe Rust is inappropriate.
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    // Read entire stdin
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut first = false;
    let mut hm: f32 = 0.0;
    let mut lm: f32 = 0.0;

    for token in input.split_whitespace() {
        // Parse token as f32 (C `float` ~= Rust f32)
        match token.parse::<f32>() {
            Ok(m) => {
                if !first {
                    first = true;
                    hm = m;
                    lm = m;
                } else {
                    if hm < m {
                        hm = m;
                    } else if lm > m {
                        lm = m;
                    }
                }
            }
            Err(_) => {
                // On parse error we stop processing further tokens.
                // (This differs from scanf's behavior on malformed input, which can cause
                // scanf to return 0 and not consume input. Reproducing that exactly would
                // require lower-level parsing; for typical numeric inputs this is fine.)
                break;
            }
        }
    }

    if first {
        // Print with one decimal place and a newline, like printf("%.1f\n", hm - lm);
        println!("{:.1}", hm - lm);
    }
}
