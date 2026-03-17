use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: usize = input.trim().parse().expect("Input must be a positive integer");
    
    // Ensure N is within the specified limit
    if n == 0 || n > 101 {
        panic!("N must be a positive integer and less than or equal to 101");
    }

    let mut numbers = Vec::with_capacity(n);
    
    // Read N integers
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.split_whitespace().take(n) {
        let number: i32 = num.parse().expect("Input must be an integer");
        numbers.push(number);
    }

    let mut count = 0;
    let mut i = 1;

    // Count pairs of consecutive equal integers
    while i < n {
        if numbers[i] == numbers[i - 1] {
            count += 1;
            i += 1; // Skip the next integer as it's part of the pair
        }
        i += 1; // Move to the next integer
    }

    // Output the count
    println!("{}", count);
    io::stdout().flush().expect("Failed to flush stdout");
}
