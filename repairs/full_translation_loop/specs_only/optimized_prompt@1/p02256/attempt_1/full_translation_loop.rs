use std::io;

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    // Read two unsigned integers from user input
    let mut input = String::new();
    println!("Enter two positive integers (1 to 1,000,000,000):");
    
    // Read the first number
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    
    if nums.len() != 2 {
        println!("Failure: Please provide exactly two numbers.");
        return;
    }

    // Parse the first number
    let num1: u64 = match nums[0].parse() {
        Ok(n) if n >= 1 && n <= 1_000_000_000 => n,
        _ => {
            println!("Failure: First number is out of range.");
            return;
        }
    };

    // Parse the second number
    let num2: u64 = match nums[1].parse() {
        Ok(n) if n >= 1 && n <= 1_000_000_000 => n,
        _ => {
            println!("Failure: Second number is out of range.");
            return;
        }
    };

    // Calculate and output the GCD
    let result = gcd(num1, num2);
    println!("GCD: {}", result);
}
