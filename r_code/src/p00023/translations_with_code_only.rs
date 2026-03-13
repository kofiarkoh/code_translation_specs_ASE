use std::io::{self, Read};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn main() {
    // Read entire stdin and split on whitespace to emulate scanf behavior.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("failed to read stdin");
    let mut toks = s.split_whitespace();

    // Parse datasets as i32 to match typical C `int`.
    let datasets: i32 = match toks.next() {
        Some(t) => t.parse().expect("failed to parse datasets"),
        None => return,
    };

    // We need to emulate the C idiom `while (datasets--) { ... }`.
    // That checks the old value for being non-zero and then decrements.
    // To avoid Rust debug-overflow panics and to approximate common two's-complement
    // behavior on platforms where C signed overflow wraps, we use wrapping_sub here.
    let mut datasets = datasets;
    while {
        let old = datasets;
        // emulate post-decrement behavior: use wrapping_sub to avoid panic on overflow
        // and to mimic typical two's-complement wrap-around that may occur in C.
        datasets = datasets.wrapping_sub(1);
        old != 0
    } {
        // Read two circles (each: x y r)
        let ax: f64 = toks.next().expect("missing a.x").parse().expect("bad a.x");
        let ay: f64 = toks.next().expect("missing a.y").parse().expect("bad a.y");
        let ar: f64 = toks.next().expect("missing a.r").parse().expect("bad a.r");

        let bx: f64 = toks.next().expect("missing b.x").parse().expect("bad b.x");
        let by: f64 = toks.next().expect("missing b.y").parse().expect("bad b.y");
        let br: f64 = toks.next().expect("missing b.r").parse().expect("bad b.r");

        let a = Circle { x: ax, y: ay, r: ar };
        let b = Circle { x: bx, y: by, r: br };

        let bigr: f64;
        let smallr: f64;
        if a.r >= b.r {
            bigr = a.r;
            smallr = b.r;
        } else {
            bigr = b.r;
            smallr = a.r;
        }

        // Compute Euclidean distance between centers exactly as in C:
        // dist = sqrt( (dx)^2 + (dy)^2 )
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let mut dist = dx.powi(2) + dy.powi(2);
        dist = dist.sqrt();

        let judge: i32;
        if dist > bigr + smallr {
            judge = 0;
        } else if dist > bigr - smallr {
            judge = 1;
        } else {
            if bigr == a.r {
                judge = 2;
            } else {
                judge = -2;
            }
        }

        // Preserve exact output formatting: one integer per line.
        println!("{}", judge);
    }
}
