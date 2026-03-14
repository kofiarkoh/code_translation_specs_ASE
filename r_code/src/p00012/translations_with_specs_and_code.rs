use std::f64;
use std::io::{self, BufRead};

fn area(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<f64> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if values.len() != 8 {
            continue; // Skip lines that do not have exactly 8 values
        }

        let (x1, y1, x2, y2, x3, y3, xp, yp) = (
            values[0], values[1], values[2], values[3],
            values[4], values[5], values[6], values[7]
        );

        let a = ((x1 - x2).hypot(y1 - y2));
        let b = ((x2 - x3).hypot(y2 - y3));
        let c = ((x1 - x3).hypot(y1 - y3));
        let ap = ((x1 - xp).hypot(y1 - yp));
        let bp = ((x2 - xp).hypot(y2 - yp));
        let cp = ((xp - x3).hypot(yp - y3));

        let s1 = area(a, ap, bp);
        let s2 = area(b, bp, cp);
        let s3 = area(c, cp, ap);
        let S = area(a, b, c);

        // Using a precision threshold for floating-point comparison
        if (s1 + s2 + s3 - S).abs() < 0.00000001 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
