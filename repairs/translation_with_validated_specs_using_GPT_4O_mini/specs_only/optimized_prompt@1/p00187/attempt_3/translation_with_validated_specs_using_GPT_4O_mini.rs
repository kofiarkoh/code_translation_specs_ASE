use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut coordinates = Vec::new();

    // Read input until three complete sets of coordinates are provided
    for _ in 0..3 {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() != 6 {
            println!("kyo");
            return;
        }
        coordinates.push(nums);
    }

    // Extract coordinates for the two line segments
    let (ax, ay, bx, by) = (
        coordinates[0][0], coordinates[0][1],
        coordinates[1][0], coordinates[1][1],
    );
    let (cx, cy, dx, dy) = (
        coordinates[2][0], coordinates[2][1],
        coordinates[2][2], coordinates[2][3],
    );

    // Check for collinearity using the area of the triangle formula
    let area = (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)).abs();

    // Check if the area is zero (collinear or overlapping)
    if area == 0 {
        println!("kyo");
        return;
    }

    // Classify the area based on its size
    let classification = classify_area(area / 2);
    println!("{}", classification);
}

// Classifies the area into specific categories
fn classify_area(area: i32) -> &'static str {
    match area {
        1..=10 => "syo-kichi",
        11..=100 => "kichi",
        101..=1000 => "chu-kichi",
        _ => "dai-kichi",
    }
}
