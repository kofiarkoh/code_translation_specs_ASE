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

    // Read the number of datasets
    let datasets: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..datasets {
        let a = read_circle(&mut lines);
        let b = read_circle(&mut lines);

        let (bigr, smallr) = if a.r >= b.r { (a.r, b.r) } else { (b.r, a.r) };

        let dist = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();

        let judge = if dist > bigr + smallr {
            0 // Circles are separate
        } else if dist > bigr - smallr {
            1 // Circles intersect
        } else {
            if bigr == a.r {
                2 // Circles are identical
            } else {
                -2 // One circle is contained within the other
            }
        };

        println!("{}", judge);
    }
}

fn read_circle<I>(lines: &mut I) -> Circle
where
    I: Iterator<Item = io::Result<String>>,
{
    let line = lines.next().unwrap().unwrap();
    let mut parts = line.split_whitespace();
    let x: f64 = parts.next().unwrap().parse().unwrap();
    let y: f64 = parts.next().unwrap().parse().unwrap();
    let r: f64 = parts.next().unwrap().parse().unwrap();
    Circle { x, y, r }
}
