use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let x_str = match it.next() {
            Some(s) => s,
            None => break, // end-of-input
        };
        let y_str = match it.next() {
            Some(s) => s,
            None => break, // incomplete pair: behavior undefined by spec; stop producing output
        };

        // Parse as i32 to match C's typical `int` behavior.
        // Spec says abs must be representable; if not, behavior is undefined.
        let x: i32 = match x_str.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed input: undefined; stop
        };
        let y: i32 = match y_str.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed input: undefined; stop
        };

        // Use i64 for intermediate sums to avoid overflow in addition.
        // Abs overflow (e.g., i32::MIN) is undefined by spec; we stop in that case.
        let ax_i64 = match x.checked_abs() {
            Some(v) => v as i64,
            None => break,
        };
        let ay_i64 = match y.checked_abs() {
            Some(v) => v as i64,
            None => break,
        };

        let sum = ax_i64 + ay_i64;

        let t = if sum == 1 {
            1
        } else {
            let ax_odd = (ax_i64 % 2) != 0;
            let ay_odd = (ay_i64 % 2) != 0;
            let ax_even = !ax_odd;
            let ay_even = !ay_odd;

            if ax_odd && ay_odd {
                sum - 1
            } else if ax_even && ay_even {
                sum
            } else if ax_i64 != 0 && ay_i64 != 0 {
                sum
            } else {
                sum + 1
            }
        };

        out.push_str(&format!("{t}\n"));
    }

    print!("{out}");
}
