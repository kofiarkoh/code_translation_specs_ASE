// Aizu Vol-2 0143   Altair and Vega
// 2017.8.8

use std::io::{self, BufRead};

fn at_same_side(x1: i32, y1: i32, x2: i32, y2: i32, xa: i32, ya: i32, xb: i32, yb: i32) -> bool {
    let sa = (x2 - x1) * (ya - y1) + (y2 - y1) * (x1 - xa);
    let sb = (x2 - x1) * (yb - y1) + (y2 - y1) * (x1 - xb);
    // Return true if sa and sb have the same sign or if either is zero
    sa * sb >= 0
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, x2, y2, x3, y3, xk, yk, xs, ys) = (
            nums[0], nums[1], nums[2], nums[3],
            nums[4], nums[5], nums[6], nums[7],
            nums[8], nums[9]
        );

        let kin = at_same_side(x1, y1, x2, y2, x3, y3, xk, yk) &&
                  at_same_side(x2, y2, x3, y3, x1, y1, xk, yk) &&
                  at_same_side(x3, y3, x1, y1, x2, y2, xk, yk);
        
        let sin = at_same_side(x1, y1, x2, y2, x3, y3, xs, ys) &&
                  at_same_side(x2, y2, x3, y3, x1, y1, xs, ys) &&
                  at_same_side(x3, y3, x1, y1, x2, y2, xs, ys);
        
        // Output "OK" if kin and sin differ, otherwise "NG"
        if kin ^ sin {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}
