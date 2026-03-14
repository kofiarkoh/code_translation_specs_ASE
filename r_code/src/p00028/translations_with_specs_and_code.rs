use std::io::{self, BufRead};

fn main() {
    // Array to count occurrences of integers from 1 to 100
    let mut count_array = [0; 101];
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read integers from standard input until EOF
    for line in handle.lines() {
        if let Ok(line) = line {
            if let Ok(number) = line.trim().parse::<usize>() {
                // Increment count if the number is within the valid range
                if number >= 1 && number <= 100 {
                    count_array[number] += 1;
                }
            }
        }
    }

    // Find the maximum frequency and the corresponding integer(s)
    let mut max_number = 0;
    for number in 1..=100 {
        if count_array[number] > count_array[max_number] {
            max_number = number;
        }
    }

    // Output all integers that have the highest frequency
    for number in 1..=100 {
        if count_array[number] == count_array[max_number] && count_array[number] > 0 {
            println!("{}", number);
        }
    }
}
