use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens like scanf("%d%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let x_tok = match it.next() {
            Some(v) => v,
            None => break,
        };
        let y_tok = match it.next() {
            Some(v) => v,
            None => break, // scanf would fail to read a full pair; stop.
        };

        // C uses int; parse into i32.
        let mut x: i32 = x_tok.parse().unwrap();
        let mut y: i32 = y_tok.parse().unwrap();

        // Emulate C's abs(int) behavior as closely as possible.
        // In C, abs(INT_MIN) is undefined; here we use wrapping_abs to avoid panic
        // and to mimic typical two's-complement wraparound behavior.
        x = x.wrapping_abs();
        y = y.wrapping_abs();

        let t: i32;
        if x.wrapping_add(y) == 1 {
            t = 1;
        } else if (x % 2 != 0) && (y % 2 != 0) {
            t = x.wrapping_add(y).wrapping_sub(1);
        } else if (x % 2 == 0) && (y % 2 == 0) {
            t = x.wrapping_add(y);
        } else if x != 0 && y != 0 {
            t = x.wrapping_add(y);
        } else {
            t = x.wrapping_add(y).wrapping_add(1);
        }

        out.push_str(&format!("{t}\n"));
    }

    print!("{out}");
}
