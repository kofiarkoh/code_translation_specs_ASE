use std::io::{self, BufRead};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn sq(x: f64) -> f64 {
    x * x
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut values = line.split_whitespace().map(|s| s.parse::<f64>().unwrap());
        
        let a = Circle {
            x: values.next().unwrap(),
            y: values.next().unwrap(),
            r: values.next().unwrap(),
        };
        
        let b = Circle {
            x: values.next().unwrap(),
            y: values.next().unwrap(),
            r: values.next().unwrap(),
        };

        let distance_sq = sq(a.x - b.x) + sq(a.y - b.y);
        let radius_sum_sq = sq(a.r + b.r);
        let radius_diff_sq = sq(a.r - b.r);

        if distance_sq > radius_sum_sq {
            println!("0");
        } else if radius_sum_sq > distance_sq && distance_sq > radius_diff_sq {
            println!("1");
        } else if distance_sq < radius_diff_sq {
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
