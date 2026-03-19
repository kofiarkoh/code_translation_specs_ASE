use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    loop {
        let a: f64 = match it.next() {
            Some(s) => match s.parse() {
                Ok(v) => v,
                Err(_) => break, // unspecified behavior; choose to stop processing
            },
            None => break,
        };

        let b: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let c: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let d: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let e: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let f: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };

        // Compute per specification:
        // y = (f - (c*d)/a) / (e - (b*d)/a)
        // x = (c - b*y) / a
        // Preconditions (a != 0, denominator != 0) are assumed for defined behavior.
        let y = (f - (c * d) / a) / (e - (b * d) / a);
        let x = (c - b * y) / a;

        println!("{:.3} {:.3}", x, y);
    }
}
