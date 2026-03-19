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
    // Read entire stdin, then parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return, // No input: mimic doing nothing.
    };

    let mut out = String::new();

    // C loop: while (n-- != 0) { ... }
    // This runs exactly n times if n > 0; if n <= 0, it runs 0 times.
    for _ in 0..(n.max(0) as usize) {
        let mut a = Circle { x: 0.0, y: 0.0, r: 0.0 };
        let mut b = Circle { x: 0.0, y: 0.0, r: 0.0 };

        // The C code does not check scanf return values; we assume inputs exist.
        a.x = it.next().unwrap().parse().unwrap();
        a.y = it.next().unwrap().parse().unwrap();
        a.r = it.next().unwrap().parse().unwrap();
        b.x = it.next().unwrap().parse().unwrap();
        b.y = it.next().unwrap().parse().unwrap();
        b.r = it.next().unwrap().parse().unwrap();

        let dx = a.x - b.x;
        let dy = a.y - b.y;

        // Preserve evaluation structure closely; comparisons are on f64 like in C.
        if sq(dx) + sq(dy) > sq(a.r + b.r) {
            out.push_str("0\n");
        } else if sq(a.r + b.r) > sq(dx) + sq(dy) && sq(dx) + sq(dy) > sq(a.r - b.r) {
            out.push_str("1\n");
        } else if sq(dx) + sq(dy) < sq(a.r - b.r) {
            if a.r > b.r {
                out.push_str("2\n");
            } else {
                out.push_str("-2\n");
            }
        } else {
            out.push_str("1\n");
        }
    }

    print!("{out}");
}
