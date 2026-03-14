use std::f64;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let input: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..input {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<f64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5]);

        let midx1 = (x1 + x2) / 2.0;
        let midy1 = (y1 + y2) / 2.0;
        let midx2 = (x1 + x3) / 2.0;
        let midy2 = (y1 + y3) / 2.0;

        let slant1 = -(x2 - x1) / (y2 - y1);
        let slant2 = -(x3 - x1) / (y3 - y1);
        let slice1 = midy1 - slant1 * midx1;
        let slice2 = midy2 - slant2 * midx2;

        let mut circlex = (slice2 - slice1) / (slant1 - slant2);
        let mut circley = slant1 * circlex + slice1;

        if (x2 - x1).abs() < f64::EPSILON { // Check for x2 == x1
            circley = midy1;
            circlex = (circley - slice2) / slant2;
        } else if (y2 - y1).abs() < f64::EPSILON { // Check for y2 == y1
            circlex = midx1;
            circley = slant2 * circlex + slice2;
        }
        if (x3 - x1).abs() < f64::EPSILON { // Check for x3 == x1
            circley = midy2;
            circlex = (circley - slice1) / slant1;
        } else if (y3 - y1).abs() < f64::EPSILON { // Check for y3 == y1
            circlex = midx2;
            circley = slant1 * circlex + slice1;
        }
        if (x2 - x1).abs() < f64::EPSILON && (y3 - y1).abs() < f64::EPSILON { // Check for x2 == x1 && y3 == y1
            circley = midy1;
            circlex = midx2;
        } else if (x3 - x1).abs() < f64::EPSILON && (y2 - y1).abs() < f64::EPSILON { // Check for x3 == x1 && y2 == y1
            circley = midy2;
            circlex = midx1;
        }

        let radius = ((circlex - x1).powi(2) + (circley - y1).powi(2)).sqrt();
        println!("{:.3} {:.3} {:.3}", circlex, circley, radius);
    }
}
