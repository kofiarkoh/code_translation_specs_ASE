use std::io::{self, Read, Write};

fn main() {
    // Read all stdin bytes. In typical judge I/O, the input is not actually NUL-terminated;
    // it is line-based. We therefore:
    // - stop at the first NUL if present (to respect the spec),
    // - otherwise stop at the first '\n' or '\r' (to match common stdin string behavior),
    // - and do NOT print an explicit NUL terminator (stdout is not C-string based).
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let mut end = input.len();
    for (i, &b) in input.iter().enumerate() {
        if b == 0 || b == b'\n' || b == b'\r' {
            end = i;
            break;
        }
    }

    let mut output = Vec::with_capacity(end);
    for &b in &input[..end] {
        let transformed = if (b'A'..=b'Z').contains(&b) {
            b.wrapping_sub(3)
        } else {
            b.wrapping_add(23)
        };
        output.push(transformed);
    }

    io::stdout().write_all(&output).unwrap();
}
