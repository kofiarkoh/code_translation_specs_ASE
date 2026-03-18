use std::io::{self, Read};

const EPS: f64 = 1e-5;

fn main() {
    // Read entire stdin, then parse ints similarly to repeated scanf("%d", &q).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let q_i32: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => continue, // If token isn't an int, mimic scanf failing by skipping it.
        };
        if q_i32 == -1 {
            break;
        }

        let q = q_i32 as f64;

        // Match C initialization: px = (double)q / 2;
        let mut px = q / 2.0;
        let mut x: f64;

        // Newton iteration for cube root, preserving C's floating behavior as closely as possible.
        // Note: For q == 0, this will produce NaNs (division by zero), matching the C code's behavior.
        loop {
            x = px - (px * px * px - q) / (3.0 * px * px);
            px = x;

            // C condition: fabs(x*x*x - q) >= EPS * q
            // Note: if q is negative, EPS*q is negative, so the condition is always true unless NaN,
            // which can lead to an infinite loop in C; we preserve that logic.
            let lhs = (x * x * x - q).abs();
            let rhs = EPS * q;
            if !(lhs >= rhs) {
                break;
            }
        }

        out.push_str(&format!("{:.6}\n", x));
    }

    print!("{out}");
}
