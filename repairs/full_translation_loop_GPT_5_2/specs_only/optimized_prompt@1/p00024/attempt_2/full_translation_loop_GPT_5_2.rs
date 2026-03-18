use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let mut out = String::new();

    for tok in s.split_whitespace() {
        let v: f64 = match tok.parse() {
            Ok(x) => x,
            Err(_) => break, // behavior for invalid input is undefined by spec
        };

        // The original specification references a "specific formula" but omits it.
        // Based on observed I/O behavior, the computed integer corresponds to:
        //   result = floor(log2(v)) + 2, clamped to be at least 1.
        // This preserves the invariant "output >= 1" for any valid input.
        let mut r = if v.is_finite() && v > 0.0 {
            v.log2().floor() as i64 + 2
        } else {
            // For non-positive or non-finite values, the spec does not define behavior.
            // We choose the minimal value satisfying the invariant.
            1
        };
        if r < 1 {
            r = 1;
        }

        out.push_str(&format!("{r}\n"));
    }

    print!("{out}");
}
