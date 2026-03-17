use std::io;

fn main() {
    // Declare variables for user input
    let mut input_a = String::new();
    let mut input_b = String::new();

    // Read first integer from standard input
    io::stdin().read_line(&mut input_a).expect("Failed to read line");
    // Read second integer from standard input
    io::stdin().read_line(&mut input_b).expect("Failed to read line");

    // Parse the inputs as integers
    let a: i32 = input_a.trim().parse().expect("Invalid input for a");
    let b: i32 = input_b.trim().parse().expect("Invalid input for b");

    // Calculate sum, difference, and product
    let sum = a + b; // sum of a and b
    let difference = a - b; // difference of a and b
    let product = a * b; // product of a and b

    // Determine the maximum value among sum, difference, and product
    let max = max_of_three(sum, difference, product);

    // Print the maximum value
    println!("{}", max);
}

// Function to compute the maximum of three integers
fn max_of_three(x: i32, y: i32, z: i32) -> i32 {
    // Use a simple comparison to find the maximum
    if x >= y && x >= z {
        x
    } else if y >= x && y >= z {
        y
    } else {
        z
    }
}
