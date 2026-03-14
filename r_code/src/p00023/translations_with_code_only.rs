use std::f64;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let datasets: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..datasets {
        let a: Circle = {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            Circle {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                r: parts.next().unwrap().parse().unwrap(),
            }
        };

        let b: Circle = {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            Circle {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                r: parts.next().unwrap().parse().unwrap(),
            }
        };

        let (bigr, smallr) = if a.r >= b.r {
            (a.r, b.r)
        } else {
            (b.r, a.r)
        };

        let dist = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();

        let judge = if dist > bigr + smallr {
            0
        } else if dist > bigr - smallr {
            1
        } else {
            if bigr == a.r {
                2
            } else {
                -2
            }
        };

        println!("{}", judge);
    }
}
