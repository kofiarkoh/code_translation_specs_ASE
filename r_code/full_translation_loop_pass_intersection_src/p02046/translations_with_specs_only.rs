use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let x = match it.next() {
            Some(s) => match s.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break, // behavior undefined; stop producing further output
            },
            None => break,
        };
        let y = match it.next() {
            Some(s) => match s.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break, // behavior undefined; stop producing further output
            },
            None => break, // incomplete pair; behavior undefined; stop
        };

        // Specification precondition: abs must be representable (no overflow).
        let a = x.abs();
        let b = y.abs();

        let sum = a + b;

        let t = if sum == 1 {
            1
        } else if (a & 1) == 1 && (b & 1) == 1 {
            sum - 1
        } else if (a & 1) == 0 && (b & 1) == 0 {
            sum
        } else if a != 0 && b != 0 {
            sum
        } else {
            // Exactly one of a,b is zero and sum != 1
            sum + 1
        };

        out.push_str(&t.to_string());
        out.push('\n');
    }

    print!("{out}");
}
