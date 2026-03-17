fn main() {
    // Read two unsigned integers from user input
    let (num1, num2) = match read_input() {
        Some((n1, n2)) => (n1, n2),
        None => {
            println!("Failure: Invalid input.");
            return;
        }
    };

    // Check if inputs are within the specified range
    if !is_valid_input(num1) || !is_valid_input(num2) {
        println!("Failure: Inputs out of range.");
        return;
    }

    // Calculate and output the GCD
    let gcd = calculate_gcd(num1, num2);
    println!("{}", gcd); // Output the GCD directly
}

// Function to read input from the user
fn read_input() -> Option<(u32, u32)> {
    use std::io::{self, Write};

    let mut input = String::new();
    print!("Enter two positive integers: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed

    io::stdin().read_line(&mut input).ok()?;
    let nums: Vec<&str> = input.trim().split_whitespace().collect();

    if nums.len() == 2 {
        if let (Ok(n1), Ok(n2)) = (nums[0].parse::<u32>(), nums[1].parse::<u32>()) {
            return Some((n1, n2));
        }
    }
    None // Return None if input is invalid
}

// Function to check if the input is within the valid range
fn is_valid_input(num: u32) -> bool {
    num >= 1 && num <= 1_000_000_000
}

// Function to calculate the GCD using the Euclidean algorithm
fn calculate_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b; // Remainder
        a = temp;  // Update a
    }
    a // Return the GCD
}
