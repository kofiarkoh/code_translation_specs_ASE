use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn sq(x: f64) -> f64 {
    x * x
}

fn main() {
    // Read entire stdin, then parse whitespace-separated tokens like scanf.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    // Emulate: while (n-- != 0) { ... }
    // In C, this loops exactly n times if n > 0; if n <= 0, it loops forever.
    // We preserve that behavior: for n <= 0, keep attempting to read cases; if input ends, stop.
    if n > 0 {
        for _ in 0..n {
            let ax: f64 = it.next().unwrap().parse().unwrap();
            let ay: f64 = it.next().unwrap().parse().unwrap();
            let ar: f64 = it.next().unwrap().parse().unwrap();
            let bx: f64 = it.next().unwrap().parse().unwrap();
            let by: f64 = it.next().unwrap().parse().unwrap();
            let br: f64 = it.next().unwrap().parse().unwrap();

            let a = Circle { x: ax, y: ay, r: ar };
            let b = Circle { x: bx, y: by, r: br };

            let d2 = sq(a.x - b.x) + sq(a.y - b.y);
            let rp2 = sq(a.r + b.r);
            let rm2 = sq(a.r - b.r);

            if d2 > rp2 {
                out.push_str("0\n");
            } else if rp2 > d2 && d2 > rm2 {
                out.push_str("1\n");
            } else if d2 < rm2 {
                if a.r > b.r {
                    out.push_str("2\n");
                } else {
                    out.push_str("-2\n");
                }
            } else {
                out.push_str("1\n");
            }
        }
    } else {
        loop {
            // If input ends, we cannot continue; C would block on scanf, but for offline input
            // we terminate to avoid an infinite loop.
            let ax = match it.next() {
                Some(t) => t.parse::<f64>().unwrap(),
                None => break,
            };
            let ay: f64 = match it.next() { Some(t) => t.parse().unwrap(), None => break };
            let ar: f64 = match it.next() { Some(t) => t.parse().unwrap(), None => break };
            let bx: f64 = match it.next() { Some(t) => t.parse().unwrap(), None => break };
            let by: f64 = match it.next() { Some(t) => t.parse().unwrap(), None => break };
            let br: f64 = match it.next() { Some(t) => t.parse().unwrap(), None => break };

            let a = Circle { x: ax, y: ay, r: ar };
            let b = Circle { x: bx, y: by, r: br };

            let d2 = sq(a.x - b.x) + sq(a.y - b.y);
            let rp2 = sq(a.r + b.r);
            let rm2 = sq(a.r - b.r);

            if d2 > rp2 {
                out.push_str("0\n");
            } else if rp2 > d2 && d2 > rm2 {
                out.push_str("1\n");
            } else if d2 < rm2 {
                if a.r > b.r {
                    out.push_str("2\n");
                } else {
                    out.push_str("-2\n");
                }
            } else {
                out.push_str("1\n");
            }
        }
    }

    print!("{out}");
}
