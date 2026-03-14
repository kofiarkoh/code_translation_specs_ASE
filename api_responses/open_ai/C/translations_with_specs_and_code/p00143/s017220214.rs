// Aizu Vol-2 0143   Altair and Vega
// 2017.8.8

use std::io::{self, BufRead};

fn at_same_side(x1: i32, y1: i32, x2: i32, y2: i32, xa: i32, ya: i32, xb: i32, yb: i32) -> bool {
    let sa = (x2 - x1) * (ya - y1) + (y2 - y1) * (x1 - xa);
    let sb = (x2 - x1) * (yb - y1) + (y2 - y1) * (x1 - xb);
    sa * sb >= 0 // Check if xa,ya and xb,yb are on the same side
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the coordinates for the triangle and the two points
        let coords: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, x2, y2, x3, y3, xk, yk, xs, ys) = (
            coords[0], coords[1], coords[2], coords[3],
            coords[4], coords[5], coords[6], coords[7],
            coords[8], coords[9]
        );

        let kin = at_same_side(x1, y1, x2, y2, x3, y3, xk, yk) &&
                  at_same_side(x2, y2, x3, y3, x1, y1, xk, yk) &&
                  at_same_side(x3, y3, x1, y1, x2, y2, xk, yk);
        
        let sin = at_same_side(x1, y1, x2, y2, x3, y3, xs, ys) &&
                  at_same_side(x2, y2, x3, y3, x1, y1, xs, ys) &&
                  at_same_side(x3, y3, x1, y1, x2, y2, xs, ys);
        
        // Output "OK" if kin and sin are different, otherwise "NG"
        if kin ^ sin {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}
