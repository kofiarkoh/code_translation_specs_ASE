use std::io::{self, Read};

fn main() {
    // Fixed examined range: the specification states processing stops at the end marker
    // or after a fixed maximum number of examined positions. The exact bound is not given,
    // so we choose the narrowest reasonable "small fixed" bound without inventing extra
    // functionality: 256 examined positions.
    const MAX_EXAMINED: usize = 256;

    // Read all input; then take the first whitespace-delimited token as the character sequence.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let token = match input.split_whitespace().next() {
        Some(t) => t,
        None => return, // No token => no output specified; do nothing.
    };

    // Apply the backspace rule left-to-right, examining at most MAX_EXAMINED characters.
    // We operate on Unicode scalar values (Rust `char`) since the spec only says "character sequence".
    let mut out: Vec<char> = Vec::new();
    for (i, ch) in token.chars().enumerate() {
        if i >= MAX_EXAMINED {
            break;
        }
        if ch == 'B' {
            out.pop();
        } else {
            out.push(ch);
        }
    }

    // Output the processed result as a character sequence.
    // Note: The spec mentions undefined behavior if the terminating end marker is not encountered
    // within the examined range; in Rust we cannot emit unterminated strings, so we print the
    // computed prefix result for the examined range.
    let result: String = out.into_iter().collect();
    print!("{result}");
}
