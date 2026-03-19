use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn sq(x: f64) -> f64 {
    x * x
}

fn main() {
    // Read entire stdin (matches C behavior: assumes valid input; no error handling required).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..n {
        let a = Circle {
            x: it.next().unwrap().parse().unwrap(),
            y: it.next().unwrap().parse().unwrap(),
            r: it.next().unwrap().parse().unwrap(),
        };
        let b = Circle {
            x: it.next().unwrap().parse().unwrap(),
            y: it.next().unwrap().parse().unwrap(),
            r: it.next().unwrap().parse().unwrap(),
        };

        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let d2 = sq(dx) + sq(dy);

        let sum_r = a.r + b.r;
        let sum_r2 = sq(sum_r);

        let diff_r = a.r - b.r;
        let diff_r2 = sq(diff_r);

        // Preserve the C code's strict comparisons and classification.
        if d2 > sum_r2 {
            out.push_str("0\n");
        } else if sum_r2 > d2 && d2 > diff_r2 {
            out.push_str("1\n");
        } else if d2 < diff_r2 {
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
