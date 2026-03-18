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

    // Read the number of circle pairs
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

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

        let result = classify_circles(&a, &b);
        println!("{}", result);
    }
}

fn classify_circles(a: &Circle, b: &Circle) -> i32 {
    let distance_sq = (a.x - b.x).powi(2) + (a.y - b.y).powi(2);
    let radius_sum_sq = (a.r + b.r).powi(2);
    let radius_diff_sq = (a.r - b.r).powi(2);

    if distance_sq > radius_sum_sq {
        0 // Circles do not intersect
    } else if distance_sq < radius_diff_sq {
        if a.r > b.r {
            2 // Circle a is completely within circle b
        } else {
            -2 // Circle b is completely within circle a
        }
    } else {
        if distance_sq < radius_sum_sq {
            1 // Circles intersect
        } else {
            1 // Circles are touching
        }
    }
}
