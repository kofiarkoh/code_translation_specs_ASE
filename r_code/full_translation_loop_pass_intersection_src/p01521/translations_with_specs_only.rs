use std::io::{self, Read};

fn main() {
    // Fixed-size internal buffer; exceeding it is undefined per specification.
    const BUF_CAP: usize = 1 << 20; // Chosen fixed bound; spec does not define the exact size.

    // Read all available stdin bytes (no explicit error handling; failures are undefined).
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // Take the first line (up to but not including '\n' or '\r').
    let mut line_end = input.len();
    for (i, &b) in input.iter().enumerate() {
        if b == b'\n' || b == b'\r' {
            line_end = i;
            break;
        }
    }
    let line = &input[..line_end];

    // Enforce fixed buffer capacity; exceeding it is undefined, so we do not define behavior.
    if line.len() > BUF_CAP {
        // Intentionally do nothing / diverge from defined behavior.
        // (Undefined behavior in spec; in Rust we must still be well-formed.)
        return;
    }

    // Count runs for 'x' and 'o' by counting run starts.
    let mut runs_x: u32 = 0;
    let mut runs_o: u32 = 0;

    let mut prev: Option<u8> = None;
    for &b in line {
        if prev != Some(b) {
            if b == b'x' {
                runs_x = runs_x.wrapping_add(1);
            } else if b == b'o' {
                runs_o = runs_o.wrapping_add(1);
            }
        }
        prev = Some(b);
    }

    // Output rule:
    // - 'x' if runs(o) < runs(x)
    // - 'o' otherwise (including ties)
    let out = if runs_o < runs_x { b"x\n" } else { b"o\n" };

    // No explicit error handling; failures are undefined.
    print!("{}", std::str::from_utf8(out).unwrap());
}
