use std::io;

fn main() {
    // Declare variables for the inputs
    let mut input = String::new();
    
    // Read three integers from the user
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();

    // Parse the first two integers as base and height
    let a: i32 = inputs.next().unwrap().parse().expect("Invalid input");
    let b: i32 = inputs.next().unwrap().parse().expect("Invalid input");
    let _c: i32 = inputs.next().unwrap().parse().expect("Invalid input"); // Unused variable

    // Calculate the area of the triangle
    // Area = (base * height) / 2
    // Using integer division to ensure the result is an integer
    let area = (a * b) / 2;

    // Print the computed area as a rounded integer
    println!("{}", area);

    // Return 0 upon successful execution
    std::process::exit(0);
}
