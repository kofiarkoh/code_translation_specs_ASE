use std::io;

fn main() {
    // Read the number of integers (N)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Initialize the count of even integers
    let mut count_even = 0;

    // Read N integers and count how many are even
    for _ in 0..n {
        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input).expect("Failed to read line");
        let a: i32 = num_input.trim().parse().expect("Invalid input for A");

        // Check if the number is even
        if a % 2 == 0 {
            count_even += 1;
        }
    }

    // Calculate 3^N and 2^count_even
    let power_of_three = 3_i32.pow(n as u32);
    let power_of_two = 2_i32.pow(count_even as u32);

    // Calculate the result
    let result = power_of_three - power_of_two;

    // Print the result
    println!("{}", result);
}
