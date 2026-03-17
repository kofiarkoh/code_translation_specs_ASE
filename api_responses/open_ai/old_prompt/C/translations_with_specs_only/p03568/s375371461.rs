use std::io;

fn main() {
    // Read the number of integers (N)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Initialize variables
    let mut count_even = 0;

    // Read N integers and count how many are even
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a: i32 = input.trim().parse().expect("Invalid input for A");

        // Check if the integer is even
        if a % 2 == 0 {
            count_even += 1;
        }
    }

    // Calculate 3^N and 2^(count of even integers)
    let exp = 3_i32.pow(n as u32); // 3^N
    let even_exp = 2_i32.pow(count_even as u32); // 2^(count of even integers)

    // Calculate the result
    let result = exp - even_exp;

    // Print the result
    println!("{}", result);
}
