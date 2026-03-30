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

    // Read the number of test cases
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<f64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let a = Circle {
            x: values[0],
            y: values[1],
            r: values[2],
        };
        let b = Circle {
            x: values[3],
            y: values[4],
            r: values[5],
        };

        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dist_sq = dx * dx + dy * dy;
        let r_sum_sq = (a.r + b.r) * (a.r + b.r);
        let r_diff_sq = (a.r - b.r) * (a.r - b.r);

        if dist_sq > r_sum_sq {
            println!("0");
        } else if dist_sq < r_sum_sq && dist_sq > r_diff_sq {
            println!("1");
        } else if dist_sq < r_diff_sq {
            if a.r > b.r {
                println!("2");
            } else {
                println!("-2");
            }
        } else {
            println!("1");
        }
    }
}
