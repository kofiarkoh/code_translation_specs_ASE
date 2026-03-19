use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let a = match it.next() {
            Some(tok) => tok.parse::<i128>().unwrap(),
            None => break, // end-of-input
        };
        let b = match it.next() {
            Some(tok) => tok.parse::<i128>().unwrap(),
            None => break, // incomplete trailing data: behavior not inferable; choose to stop
        };

        // x = |a|, y = |b| with non-negative result.
        // Using i128 to reduce overflow risk; spec does not define overflow behavior.
        let x = a.abs();
        let y = b.abs();

        let sum = x + y;

        let t = if sum == 1 {
            1
        } else {
            let x_odd = (x & 1) == 1;
            let y_odd = (y & 1) == 1;
            let x_even = !x_odd;
            let y_even = !y_odd;

            if x_odd && y_odd {
                sum - 1
            } else if x_even && y_even {
                sum
            } else if x > 0 && y > 0 {
                // Mixed parity, both non-zero
                sum
            } else {
                // Mixed parity and at least one is zero
                sum + 1
            }
        };

        out.push_str(&t.to_string());
        out.push('\n');
    }

    print!("{out}");
}
