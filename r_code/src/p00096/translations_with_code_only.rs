// translated from the provided C program.
// Reads integers from stdin (whitespace-separated), for each input `s` computes `ans`
// and prints it followed by a newline, matching the original program's I/O and logic.
//
// Notes on translation choices:
// - The original C used 32-bit `int`. To avoid accidental panics from Rust's
//   checked arithmetic in debug builds while preserving the intended arithmetic,
// - we perform the core arithmetic in i64 (which matches the mathematical intent
//   for typical inputs). This keeps behavior stable for reasonable inputs and
//   avoids undefined/overflow problems that the original C program would have
//   had for extreme values.
// - The loop conditions `a <= 1000 && s >= a` and `b <= 1000 && s >= a+b` are
//   preserved by breaking early when the `s` checks fail. This reproduces the
//   same iteration counts as the C nested for-loops.
//
// The program assumes input tokens are valid integers (as the original scanf("%d", &s))
// would. Non-integer tokens are ignored by parsing here.
use std::io::{self, Read};

fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, nothing to do.
        return;
    }

    // Parse whitespace-separated tokens as i64. Skip tokens that fail to parse.
    let mut out = String::new();
    for token in input.split_whitespace() {
        let s: i64 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mut ans: i64 = 0;
        // a from 0 ..= 1000, but only while s >= a
        for a_u in 0..=1000 {
            let a = a_u as i64;
            if s < a {
                break;
            }
            // b from 0 ..= 1000, but only while s >= a + b
            for b_u in 0..=1000 {
                let b = b_u as i64;
                if s < a + b {
                    break;
                }
                // term = 501 - abs(500 - (s - a - b))
                let term = 501 - (500 - (s - a - b)).abs();
                ans += term;
            }
        }

        // Match C's printf("%d\n", ans); -- print decimal then newline.
        out.push_str(&format!("{}\n", ans));
    }

    // Write all output at once.
    print!("{}", out);
}
