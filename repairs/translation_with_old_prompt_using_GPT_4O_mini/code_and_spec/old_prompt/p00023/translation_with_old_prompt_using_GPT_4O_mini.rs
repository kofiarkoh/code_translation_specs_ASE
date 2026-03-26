use std::io::{self, Write};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for number of circles");

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<f64> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid circle properties"))
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

        let relationship_code = determine_relationship(&a, &b);
        println!("{}", relationship_code);
    }
}

fn determine_relationship(a: &Circle, b: &Circle) -> i32 {
    let distance_squared = (a.x - b.x).powi(2) + (a.y - b.y).powi(2);
    let radius_sum_squared = (a.r + b.r).powi(2);
    let radius_diff_squared = (a.r - b.r).powi(2);

    if distance_squared > radius_sum_squared {
        0 // Circles are separate
    } else if distance_squared < radius_diff_squared {
        if a.r > b.r {
            2 // Circle A contains Circle B
        } else {
            -2 // Circle B contains Circle A
        }
    } else {
        1 // Circles intersect
    }
}
