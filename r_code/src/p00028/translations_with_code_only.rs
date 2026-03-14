use std::io::{self, BufRead};

fn main() {
    let mut count_array = [0; 101]; // Array to hold counts, initialized to 0
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    // Read integers from stdin until EOF
    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        if let Ok(number) = line.trim().parse::<usize>() {
            if number <= 100 { // Ensure we don't go out of bounds
                count_array[number] += 1;
            }
        }
    }

    let mut mx_number = 0;

    // Find the maximum number with the highest count
    for number in (1..=100).rev() {
        if count_array[number] > count_array[mx_number] {
            mx_number = number;
        }
    }

    // Print all numbers that have the same count as mx_number
    for number in 1..=mx_number {
        if count_array[number] == count_array[mx_number] {
            println!("{}", number);
        }
    }
}
