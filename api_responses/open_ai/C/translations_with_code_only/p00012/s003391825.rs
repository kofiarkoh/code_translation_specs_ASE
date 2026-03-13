use std::io::{self, Read};

fn heron_area(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;
    // Note: preserve C semantics where negative inside sqrt yields NaN.
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    // Read entire stdin (scanf in C reads whitespace-separated tokens until EOF).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut it = input.split_whitespace();

    loop {
        // Attempt to read 8 f64 values per iteration. If any parse or token is missing, stop.
        let x1 = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let y1 = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let x2 = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let y2 = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let x3 = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let y3 = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let xp = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let yp = match it.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => break,
        };

        // Distances using hypot to match C's hypot behavior.
        let a = (x1 - x2).hypot(y1 - y2);
        let b = (x2 - x3).hypot(y2 - y3);
        let c = (x1 - x3).hypot(y1 - y3);

        let ap = (x1 - xp).hypot(y1 - yp);
        let bp = (x2 - xp).hypot(y2 - yp);
        let cp = (xp - x3).hypot(yp - y3);

        let mut s = heron_area(a, ap, bp);
        s += heron_area(b, bp, cp);
        s += heron_area(c, cp, ap);

        let S = heron_area(a, b, c);

        if (s - S).abs() < 0.00000001 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
