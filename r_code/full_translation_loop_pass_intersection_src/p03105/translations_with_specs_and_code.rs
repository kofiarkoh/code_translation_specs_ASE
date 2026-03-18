use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();
    let a: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let b: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // SPEC: If the first input value is zero, do not perform division and produce no output.
    // Note: This differs from the provided C code, which would divide by `a` and thus
    // would be undefined behavior when `a == 0`.
    if a == 0 {
        return;
    }

    // C semantics: integer division truncates toward zero; Rust's i32 division matches this.
    let div = b / a;
    let out = if div <= c { div } else { c };
    print!("{out}\n");
}
