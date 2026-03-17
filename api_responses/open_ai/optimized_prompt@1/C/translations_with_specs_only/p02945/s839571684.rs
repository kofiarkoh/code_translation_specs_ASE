use std::io;

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut numbers = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Ensure exactly two integers are provided
    if numbers.len() != 2 {
        eprintln!("Error: Exactly two integers are required.");
        return;
    }

    let a = numbers[0];
    let b = numbers[1];

    // Calculate sum, difference, and product
    let sum = a + b;
    let difference = a - b;
    let product = a * b;

    // Determine the maximum value among sum, difference, and product
    let max_value = max_of_three(sum, difference, product);

    // Output the maximum value
    println!("{}", max_value);
}

// Function to find the maximum of three integers
fn max_of_three(x: i32, y: i32, z: i32) -> i32 {
    // Compare the three values and return the maximum
    if x >= y && x >= z {
        x
    } else if y >= x && y >= z {
        y
    } else {
        z
    }
}
