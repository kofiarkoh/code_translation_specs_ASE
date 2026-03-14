use std::io::{self, BufRead};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut coords = line.split(',');

        let p1 = Point {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
        };
        let p2 = Point {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
        };
        let q = Point {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
        };

        let (r_x, r_y) = if p1.x != p2.x && p1.y != p2.y {
            let m = (p2.y - p1.y) / (p2.x - p1.x);
            let o_x = (m / (m * m + 1.0)) * (m * p1.x + q.x / m + q.y - p1.y);
            let o_y = m * (o_x - p1.x) + p1.y;
            let r_x = 2.0 * o_x - q.x;
            let r_y = 2.0 * o_y - q.y;
            (r_x, r_y)
        } else if p1.x != p2.x && p1.y == p2.y {
            (q.x, 2.0 * p1.y - q.y)
        } else if p1.x == p2.x && p1.y != p2.y {
            (2.0 * p1.x - q.x, q.y)
        } else {
            (2.0 * p1.x - q.x, 2.0 * p1.y - q.y)
        };

        // Print the result with the same formatting as in C
        println!("{:.6} {:.6}", r_x, r_y);
    }
}
