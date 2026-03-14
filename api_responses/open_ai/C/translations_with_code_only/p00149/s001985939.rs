use std::io::{self, BufRead};

fn main() {
    let mut rn = [0; 4];
    let mut ln = [0; 4];
    let eps = 1e-8; // EPS is defined but not used in the logic

    // Read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers = line.split_whitespace();

        // Attempt to parse two doubles from the input
        if let (Some(right_str), Some(left_str)) = (numbers.next(), numbers.next()) {
            if let (Ok(right), Ok(left)) = (right_str.parse::<f64>(), left_str.parse::<f64>()) {
                // Update rn based on the value of right
                if right >= 1.1 {
                    rn[0] += 1;
                } else if right >= 0.6 {
                    rn[1] += 1;
                } else if right >= 0.2 {
                    rn[2] += 1;
                } else {
                    rn[3] += 1;
                }

                // Update ln based on the value of left
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
        } else {
            // If we don't have two numbers, we can break out of the loop
            break;
        }
    }

    // Print the results
    for i in 0..4 {
        println!("{} {}", rn[i], ln[i]);
    }
}
