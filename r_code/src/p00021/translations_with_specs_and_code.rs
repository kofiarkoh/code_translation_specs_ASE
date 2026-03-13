use std::io::{self, Read};

fn main() {
    // Read entire stdin like the original C program which used scanf.
    let mut input = String::new();
    // If reading fails, propagate the error by panicking (similar to unspecified behavior in C).
    io::stdin().read_to_string(&mut input).unwrap();

    let mut toks = input.split_whitespace();

    // Parse n as i32 to preserve semantics for negative values (negative -> zero iterations).
    // If the token is missing or not an integer, unwrap() will panic, which corresponds to
    // unspecified/error behavior as described in the spec.
    let n: i32 = toks.next().unwrap().parse().unwrap();

    // If n is negative, behave like the C for-loop (no iterations).
    let cases = if n > 0 { n as usize } else { 0usize };

    for _ in 0..cases {
        // Parse eight f64 values. Missing or malformed tokens will cause a panic (unspecified).
        let x1: f64 = toks.next().unwrap().parse().unwrap();
        let y1: f64 = toks.next().unwrap().parse().unwrap();
        let x2: f64 = toks.next().unwrap().parse().unwrap();
        let y2: f64 = toks.next().unwrap().parse().unwrap();
        let x3: f64 = toks.next().unwrap().parse().unwrap();
        let y3: f64 = toks.next().unwrap().parse().unwrap();
        let x4: f64 = toks.next().unwrap().parse().unwrap();
        let y4: f64 = toks.next().unwrap().parse().unwrap();

        // Compute slopes using f64 arithmetic. Division by zero yields ±inf or NaN per IEEE754,
        // matching the unspecified-but-intended behavior in the C version.
        let sl1 = (y2 - y1) / (x2 - x1);
        let sl2 = (y4 - y3) / (x4 - x3);

        // Exact floating-point equality as specified.
        if sl1 == sl2 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
