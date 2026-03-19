use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    // Specification: behavior is not inferable if fewer than three integers are successfully read.
    // Narrowest reasonable interpretation: if we can't read/parse three integers, produce no output.
    let a: i128 = match it.next().and_then(|s| s.parse::<i128>().ok()) {
        Some(v) => v,
        None => return,
    };
    let b: i128 = match it.next().and_then(|s| s.parse::<i128>().ok()) {
        Some(v) => v,
        None => return,
    };
    let c: i128 = match it.next().and_then(|s| s.parse::<i128>().ok()) {
        Some(v) => v,
        None => return,
    };

    // Specification does not provide the "specified combinations in Section 6".
    // Therefore, there are no defined input triples that produce output.
    // For all input combinations, produce no output.
    let _ = (a, b, c);
}
