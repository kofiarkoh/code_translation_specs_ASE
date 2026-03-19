use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token as the input string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let token = input.split_whitespace().next().unwrap_or("");

    // Fixed-size internal storage: up to 10 characters plus a terminating marker (conceptually).
    // The specification states behavior is undefined if input exceeds capacity; we do not add
    // explicit error handling. We still process whatever is present in `token`.
    let mut out: Vec<char> = Vec::with_capacity(10);

    // Process left-to-right with the specified "B as backspace" semantics.
    // Processing is bounded by the fixed maximum input size in the spec; we iterate over the token.
    for ch in token.chars() {
        if ch == 'B' {
            if !out.is_empty() {
                out.pop();
            }
        } else {
            out.push(ch);
        }
    }

    // Output the transformed sequence.
    // The spec's "properly terminated within fixed output storage" corresponds to producing a
    // valid Rust String; if the spec's precondition is violated, behavior is undefined there,
    // and we do not attempt to emulate UB beyond simply printing what we computed.
    let result: String = out.into_iter().collect();
    print!("{result}");
}
