use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Initialize variables
    let mut sum = 0;
    let mut min_non_multiple_of_ten = 101; // Start with 101, as per specification

    // Read integers and compute sum and minimum non-multiple of ten
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        sum += value;

        // Check if the value is a non-multiple of ten
        if value % 10 != 0 {
            if value < min_non_multiple_of_ten {
                min_non_multiple_of_ten = value;
            }
        }
    }

    // Check if the sum is a multiple of ten
    if sum % 10 == 0 {
        // If the sum is a multiple of ten and there are no non-multiples of ten
        if min_non_multiple_of_ten == 101 {
            println!("0"); // Output 0 if all are multiples of ten
        } else {
            println!("{}", sum); // Output the sum as is
        }
    } else {
        println!("{}", sum); // Output the sum as is
    }
}
