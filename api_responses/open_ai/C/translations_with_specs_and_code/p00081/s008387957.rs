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
        let line = line.expect("Failed to read line");
        let coords: Vec<&str> = line.split(',').collect();

        if coords.len() != 6 {
            // Undefined behavior as per spec; we assume input is valid.
            continue;
        }

        let p1 = Point {
            x: coords[0].parse().expect("Invalid input"),
            y: coords[1].parse().expect("Invalid input"),
        };
        let p2 = Point {
            x: coords[2].parse().expect("Invalid input"),
            y: coords[3].parse().expect("Invalid input"),
        };
        let q = Point {
            x: coords[4].parse().expect("Invalid input"),
            y: coords[5].parse().expect("Invalid input"),
        };

        let r = reflect_point(&p1, &p2, &q);
        println!("{} {}", r.x, r.y);
    }
}

fn reflect_point(p1: &Point, p2: &Point, q: &Point) -> Point {
    let mut r = Point { x: 0.0, y: 0.0 };

    if p1.x != p2.x && p1.y != p2.y {
        let m = (p2.y - p1.y) / (p2.x - p1.x);
        let o_x = (m / (m * m + 1.0)) * (m * p1.x + q.x / m + q.y - p1.y);
        let o_y = m * (o_x - p1.x) + p1.y;
        r.x = 2.0 * o_x - q.x;
        r.y = 2.0 * o_y - q.y;
    } else if p1.x != p2.x && p1.y == p2.y {
        r.x = q.x;
        r.y = 2.0 * p1.y - q.y;
    } else if p1.x == p2.x && p1.y != p2.y {
        r.x = 2.0 * p1.x - q.x;
        r.y = q.y;
    } else {
        r.x = 2.0 * p1.x - q.x;
        r.y = 2.0 * p1.y - q.y;
    }

    r
}
