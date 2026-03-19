use std::io;

fn main() {
    // Read inputs
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<&str> = input.trim().split_whitespace().collect();

    // Parse inputs
    let long_input: u64 = inputs[0].parse().expect("First input must be a non-negative long integer");
    let mut int1: i32 = inputs[1].parse().expect("Second input must be an integer");
    let mut int2: i32 = inputs[2].parse().expect("Third input must be an integer");

    // Determine the number of iterations based on the first input modulo 12
    let iterations = (long_input % 12) as usize;

    // Perform the iterative arithmetic operations
    for _ in 0..iterations {
        int1 = int1.wrapping_add(int2); // Use wrapping_add to emulate potential overflow behavior
        int2 = int2.wrapping_sub(int1); // Use wrapping_sub to emulate potential underflow behavior
    }

    // Output the final values of the two integers
    println!("{} {}", int1, int2);
}
