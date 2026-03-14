use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut inputs: Vec<i32> = line.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        // Check if we have at least 12 integers (10 + 2)
        if inputs.len() < 12 {
            break; // Terminate if input is incomplete
        }

        let r = &inputs[0..10]; // First 10 integers
        let v1 = inputs[10]; // 11th integer
        let v2 = inputs[11]; // 12th integer

        let rs: i32 = r.iter().sum(); // Sum of the first 10 integers
        let x = (v1 as f64) / (v1 + v2) as f64 * (rs as f64); // Calculate threshold

        let mut cumulative_sum = 0;
        let mut position = 0;

        for (i, &value) in r.iter().enumerate() {
            cumulative_sum += value;
            if cumulative_sum as f64 >= x {
                position = i + 1; // 1-based index
                break;
            }
        }

        println!("{}", position);
    }
}
