use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens (like scanf with %lf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        // The C code loops while scanf(...) != EOF, i.e., it continues even on partial reads.
        // We emulate that: if we can't read 6 numbers, stop.
        let a: f64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let b: f64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let c: f64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let d: f64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let e: f64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let f: f64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };

        // Preserve floating-point semantics (including division by zero -> inf/nan).
        let bai = d / a;
        let aa = a * bai;
        let bb = b * bai;
        let cc = c * bai;
        let y = (f - cc) / (e - bb);
        let x = (c - b * y) / a;

        // Match printf("%.3f %.3f\n", x, y);
        println!("{:.3} {:.3}", x, y);

        // Keep variables to mirror C structure (aa is computed but unused in C as well).
        let _ = aa;
    }
}
