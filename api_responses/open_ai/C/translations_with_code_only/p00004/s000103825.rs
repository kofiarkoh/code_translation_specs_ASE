// translated from the provided C program
// Preserves the original program semantics as closely as practical in Rust.
// Notes:
// - Uses i32 for the six integer inputs (a..f).
// - Integer arithmetic (multiplication/subtraction) uses wrapping arithmetic
//   (i32::wrapping_*) to emulate the typical two's-complement wrap behavior
//   that many C implementations exhibit for overflowing signed ints.
// - Floating computations use f32 to match C's float type.
// - The near-zero correction `if (-0.0005 < x && x <= 0) x = 0;` is preserved.
// - Input is read as whitespace-separated tokens; groups of six integers are
//   processed until tokens are exhausted.

use std::io::{self, Read};

fn main() {
    // Read entire stdin to a string (behaves like scanning whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();
    let mut output = String::new();

    loop {
        // Attempt to read six integers; if any is missing, stop (like EOF).
        let a = match tokens.next() {
            Some(t) => match t.parse::<i32>() { Ok(v) => v, Err(_) => break },
            None => break,
        };
        let b = match tokens.next() {
            Some(t) => match t.parse::<i32>() { Ok(v) => v, Err(_) => break },
            None => break,
        };
        let c = match tokens.next() {
            Some(t) => match t.parse::<i32>() { Ok(v) => v, Err(_) => break },
            None => break,
        };
        let d = match tokens.next() {
            Some(t) => match t.parse::<i32>() { Ok(v) => v, Err(_) => break },
            None => break,
        };
        let e = match tokens.next() {
            Some(t) => match t.parse::<i32>() { Ok(v) => v, Err(_) => break },
            None => break,
        };
        let f = match tokens.next() {
            Some(t) => match t.parse::<i32>() { Ok(v) => v, Err(_) => break },
            None => break,
        };

        // Compute integer expressions using wrapping arithmetic to emulate common C behavior.
        let num_y = c.wrapping_mul(d).wrapping_sub(a.wrapping_mul(f));
        let den_y = b.wrapping_mul(d).wrapping_sub(a.wrapping_mul(e));

        let num_x = c.wrapping_mul(e).wrapping_sub(b.wrapping_mul(f));
        let den_x = a.wrapping_mul(e).wrapping_sub(b.wrapping_mul(d));

        // Cast to float (f32) and divide (matching C's (float) cast and float division).
        let mut y = (num_y as f32) / (den_y as f32);
        let mut x = (num_x as f32) / (den_x as f32);

        // Preserve the near-zero correction from the C code.
        if -0.0005f32 < x && x <= 0.0f32 {
            x = 0.0;
        }
        if -0.0005f32 < y && y <= 0.0f32 {
            y = 0.0;
        }

        // Print with three decimal places, space-separated, newline at end of line.
        // This corresponds to the C printf("%.3lf %.3f\n", x, y);
        output.push_str(&format!("{:.3} {:.3}\n", x, y));
    }

    // Emit all output at once.
    print!("{}", output);
}
