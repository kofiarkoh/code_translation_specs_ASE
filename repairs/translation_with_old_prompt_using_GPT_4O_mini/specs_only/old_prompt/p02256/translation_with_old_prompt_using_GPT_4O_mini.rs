use std::io;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    if io::stdin().read_line(&mut input).is_err() {
        // Print error message and exit
        eprintln!("Error reading input");
        std::process::exit(1); // Exit with code 1 on input error
    }

    // Split the input into two parts
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Validate input: must be exactly two parts
    if parts.len() != 2 {
        eprintln!("Invalid input");
        std::process::exit(1); // Exit with code 1 for invalid input
    }

    // Parse the two unsigned integers
    let u32_x: u32 = match parts[0].parse() {
        Ok(num) if num >= 1 && num <= 1_000_000_000 => num,
        _ => {
            eprintln!("Invalid input");
            std::process::exit(1); // Exit with code 1 for out of range or invalid input
        }
    };

    let u32_y: u32 = match parts[1].parse() {
        Ok(num) if num >= 1 && num <= 1_000_000_000 => num,
        _ => {
            eprintln!("Invalid input");
            std::process::exit(1); // Exit with code 1 for out of range or invalid input
        }
    };

    // Compute the GCD
    let result = gcd(u32_x, u32_y);
    
    // Output the result
    println!("{}", result);
}
