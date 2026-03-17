use std::io;

fn main() {
    // Create a vector to hold the three long integers
    let mut numbers = Vec::new();

    // Read three long integers from standard input
    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse the input as a long integer (i64 in Rust)
        // Assuming valid input as per the specification
        let number: i64 = input.trim().parse().expect("Invalid input");
        numbers.push(number);
    }

    // Check if any number is even
    if numbers.iter().any(|&x| x % 2 == 0) {
        // If any number is even, output "0"
        println!("0");
    } else {
        // If all numbers are odd, find the maximum
        let max_index = numbers.iter().enumerate().max_by_key(|&(_, &v)| v).map(|(i, _)| i).unwrap();
        
        // Set the maximum number to 1
        numbers[max_index] = 1;

        // Calculate the product of the three numbers
        let product: i64 = numbers.iter().product();
        
        // Output the product
        println!("{}", product);
    }

    // Return 0 upon successful execution
    std::process::exit(0);
}
