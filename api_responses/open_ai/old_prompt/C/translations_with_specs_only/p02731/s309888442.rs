use std::io;

fn main() {
    // Read an integer input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as an integer
    let l: i32 = input.trim().parse().expect("Input must be a valid integer");

    // Calculate one-third of the input
    let one_third = l as f64 / 3.0;

    // Calculate the cube of one-third
    let cube = one_third.powi(3);

    // Print the result formatted to twelve decimal places
    println!("{:.12}", cube);
}
