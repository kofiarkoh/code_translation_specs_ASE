use std::io::{self, Read};

fn main() {
    // Read entire stdin, like repeated scanf calls would.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In C, failing scanf would leave n uninitialized (UB). Here we default to 0 if missing.
    let n: i32 = match it.next() {
        Some(tok) => tok.parse::<i32>().unwrap_or(0),
        None => 0,
    };

    for _ in 0..n {
        // Read 6 doubles; if missing, default to 0.0 (best-effort; C would be UB).
        let mut next_f64 = || -> f64 {
            it.next()
                .and_then(|t| t.parse::<f64>().ok())
                .unwrap_or(0.0)
        };

        let x1 = next_f64();
        let y1 = next_f64();
        let x2 = next_f64();
        let y2 = next_f64();
        let x3 = next_f64();
        let y3 = next_f64();

        let a1 = 2.0 * (x2 - x1);
        let b1 = 2.0 * (y2 - y1);
        let c1 = x1 * x1 - x2 * x2 + y1 * y1 - y2 * y2;

        let a2 = 2.0 * (x3 - x1);
        let b2 = 2.0 * (y3 - y1);
        let c2 = x1 * x1 - x3 * x3 + y1 * y1 - y3 * y3;

        let denom = a1 * b2 - a2 * b1;
        let px = (b1 * c2 - b2 * c1) / denom;
        let py = (c1 * a2 - c2 * a1) / denom;

        let mut x = 0.0f64;
        let s = (x1 - px) * (x1 - px) + (y1 - py) * (y1 - py);
        let mut r = s / 2.0;

        // Preserve the exact C loop condition (bitwise float equality).
        // Note: if r becomes NaN, (r != x) is true forever in both C and Rust.
        while r != x {
            x = r;
            r = (r + s / r) / 2.0;
        }

        if r < 0.0 {
            r *= -1.0;
        }

        // Match C's printf("%.3lf %.3lf %.3lf\n", ...)
        println!("{:.3} {:.3} {:.3}", px, py, r);
    }
}
