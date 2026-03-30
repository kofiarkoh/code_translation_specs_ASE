use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the number of integers N
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a non-negative integer");

    let mut count_even = 0;

    // Read N integers and count the even numbers
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a: i32 = input.trim().parse().expect("Input must be a valid integer");
        
        if a % 2 == 0 {
            count_even += 1;
        }
    }

    // Calculate 3^N and 2^count_even
    let exp = 3_i32.pow(n as u32);
    let odd = 2_i32.pow(count_even as u32);

    // Print the result
    println!("{}", exp - odd);
}
