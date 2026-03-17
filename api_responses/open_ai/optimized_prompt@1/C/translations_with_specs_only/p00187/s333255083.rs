use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut coordinates = Vec::new();

    // Read input until EOF
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        // Collect coordinates in sets of four
        if nums.len() == 4 {
            coordinates.push(nums);
        }
    }

    // Process sets of coordinates in groups of three
    for chunk in coordinates.chunks(3) {
        if chunk.len() < 3 {
            // Not enough data to form three sets of coordinates
            return;
        }

        let area = calculate_area(chunk);
        let classification = classify_area(area);
        println!("{}", classification);
    }
}

// Calculate the area of the triangle formed by the intersection of the line segments
fn calculate_area(sets: &[Vec<i32>]) -> f64 {
    // Extract coordinates from the sets
    let (x1, y1, x2, y2) = (sets[0][0], sets[0][1], sets[1][0], sets[1][1]);
    let (x3, y3, x4, y4) = (sets[2][0], sets[2][1], sets[2][2], sets[2][3]);

    // Calculate the area using the determinant method
    let area = 0.5 * ((x1 * y2 + x2 * y3 + x3 * y1) - (y1 * x2 + y2 * x3 + y3 * x1)).abs() as f64;

    area
}

// Classify the area into specific categories
fn classify_area(area: f64) -> &'static str {
    if area == 0.0 {
        "kyo"
    } else if area < 10.0 {
        "syo-kichi"
    } else if area < 20.0 {
        "kichi"
    } else if area < 30.0 {
        "chu-kichi"
    } else {
        "dai-kichi"
    }
}
