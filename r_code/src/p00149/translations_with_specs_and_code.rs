use std::io::{self, BufRead};

fn main() {
    // Initialize counts for right and left values
    let mut rn = [0; 4];
    let mut ln = [0; 4];

    // Define the epsilon value for floating-point comparison
    const EPS: f64 = 1e-8;

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Handle potential errors in reading lines
        let line = line.expect("Failed to read line");
        let mut values = line.split_whitespace();

        // Parse the right and left values
        if let (Some(right_str), Some(left_str)) = (values.next(), values.next()) {
            if let (Ok(right), Ok(left)) = (right_str.parse::<f64>(), left_str.parse::<f64>()) {
                // Categorize the right value
                if right >= 1.1 {
                    rn[0] += 1;
                } else if right >= 0.6 {
                    rn[1] += 1;
                } else if right >= 0.2 {
                    rn[2] += 1;
                } else {
                    rn[3] += 1;
                }

                // Categorize the left value
                if left >= 1.1 {
                    ln[0] += 1;
                } else if left >= 0.6 {
                    ln[1] += 1;
                } else if left >= 0.2 {
                    ln[2] += 1;
                } else {
                    ln[3] += 1;
                }
            }
        }
    }

    // Output the counts for right and left values
    for i in 0..4 {
        println!("{} {}", rn[i], ln[i]);
    }
}
