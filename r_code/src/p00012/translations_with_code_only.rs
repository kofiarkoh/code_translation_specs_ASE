use std::f64;
use std::io::{self, BufRead};

fn f(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut coords = line.split_whitespace().map(|s| s.parse::<f64>().unwrap());

        let x1 = coords.next().unwrap();
        let y1 = coords.next().unwrap();
        let x2 = coords.next().unwrap();
        let y2 = coords.next().unwrap();
        let x3 = coords.next().unwrap();
        let y3 = coords.next().unwrap();
        let xp = coords.next().unwrap();
        let yp = coords.next().unwrap();

        let a = (x1 - x2).hypot(y1 - y2);
        let b = (x2 - x3).hypot(y2 - y3);
        let c = (x1 - x3).hypot(y1 - y3);
        let ap = (x1 - xp).hypot(y1 - yp);
        let bp = (x2 - xp).hypot(y2 - yp);
        let cp = (xp - x3).hypot(yp - y3);

        let mut s = f(a, ap, bp);
        s += f(b, bp, cp);
        s += f(c, cp, ap);
        let S = f(a, b, c);

        if (s - S).abs() < 0.00000001 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
