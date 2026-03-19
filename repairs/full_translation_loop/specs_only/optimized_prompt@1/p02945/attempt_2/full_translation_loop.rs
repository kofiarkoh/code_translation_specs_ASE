use std::io;

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into two parts and parse them as integers
    let mut numbers = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());

    // Collect the two integers
    let a = match numbers.next() {
        Some(num) => num,
        None => return, // Handle case where no integers are provided
    };
    let b = match numbers.next() {
        Some(num) => num,
        None => return, // Handle case where only one integer is provided
    };

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
    // Compare x, y, and z to find the maximum
    if x >= y && x >= z {
        x // x is the maximum
    } else if y >= z {
        y // y is the maximum
    } else {
        z // z is the maximum
    }
}
