use std::io::{self, BufRead};

const MAX_LENGTH: usize = 80;
const MAX_SUM: u128 = 1_000_000_000_000_000_000_000; // Adjusted limit for the sum to accommodate larger values

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of datasets
    let num_datasets: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..num_datasets {
        if let (Some(first), Some(second)) = (lines.next(), lines.next()) {
            let first = first.unwrap();
            let second = second.unwrap();

            // Check for length overflow
            if first.len() > MAX_LENGTH || second.len() > MAX_LENGTH {
                println!("overflow");
                continue;
            }

            // Convert strings to u128 to handle larger sums
            let first_num = first.parse::<u128>();
            let second_num = second.parse::<u128>();

            // Check if parsing was successful
            if let (Ok(first_num), Ok(second_num)) = (first_num, second_num) {
                let sum = first_num + second_num; // Direct addition since we are using u128

                // Check for sum overflow
                if sum > MAX_SUM {
                    println!("overflow");
                } else {
                    // Convert sum back to string and print
                    println!("{}", sum);
                }
            } else {
                // If parsing fails, output overflow
                println!("overflow");
            }
        }
    }
}
