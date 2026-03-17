fn main() {
    use std::io::{self, BufRead};

    // Define the maximum length for input strings
    const MAX_LENGTH: usize = 80;

    // Read the number of datasets from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of datasets
    let num_datasets: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..num_datasets {
        // Read the two strings representing non-negative integers
        let first = lines.next().unwrap().unwrap();
        let second = lines.next().unwrap().unwrap();

        // Check for length overflow
        if first.len() > MAX_LENGTH || second.len() > MAX_LENGTH {
            println!("overflow");
            continue; // Move to the next dataset
        }

        // Convert strings to u128 to handle larger numbers
        let first_num = match first.parse::<u128>() {
            Ok(num) => num,
            Err(_) => {
                println!("overflow");
                continue; // Move to the next dataset
            }
        };

        let second_num = match second.parse::<u128>() {
            Ok(num) => num,
            Err(_) => {
                println!("overflow");
                continue; // Move to the next dataset
            }
        };

        // Calculate the sum and check for overflow
        let sum = first_num.checked_add(second_num);
        match sum {
            Some(total) => {
                // Print the sum as a string without leading zeros
                println!("{}", total);
            }
            None => {
                println!("overflow");
            }
        }
    }
}
