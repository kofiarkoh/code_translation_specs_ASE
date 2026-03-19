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

        // The specification omits the exact formula. From the observed behavior:
        // 1.0 -> 2, 2.5 -> 2, 5.0 -> 2, 10.0 -> 3
        // This matches: result = floor(log10(v)) + 2, with result >= 1.
        let mut r = if v.is_finite() && v > 0.0 {
            v.log10().floor() as i64 + 2
        } else {
            // For non-positive or non-finite values, the spec does not define behavior.
            // Choose the minimal value satisfying the invariant.
            1
        };
        if r < 1 {
            r = 1;
        }

        out.push_str(&format!("{r}\n"));
    }

    print!("{out}");
}
