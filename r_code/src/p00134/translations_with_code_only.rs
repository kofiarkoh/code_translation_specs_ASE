use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the first line of input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Failed to parse input");

    let mut sum = 0;

    // Loop to read n integers and accumulate their sum
    for _ in 0..n {
        input.clear(); // Clear the input buffer for the next read
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let in_value: i32 = input.trim().parse().expect("Failed to parse input");
        sum += in_value;
    }

    // Print the average, using integer division
    println!("{}", sum / n);
}
