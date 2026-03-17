use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a long long integer (i64 in Rust)
    let k: i64 = input.trim().parse().expect("Invalid input");

    // Match on the value of k to determine the output sequence
    match k {
        k if k >= 50 => {
            // For k >= 50, output "50" followed by 50 integers
            println!("50");
            for i in 0..50 {
                print!("{} ", i);
            }
            println!(); // Newline after the sequence
        },
        k if k >= 2 => {
            // For 2 <= k < 50, output k followed by k integers
            println!("{}", k);
            for i in 0..k {
                print!("{} ", i);
            }
            println!(); // Newline after the sequence
        },
        0 => {
            // For k == 0, output "4" followed by four integers
            println!("4");
            for i in 0..4 {
                print!("{} ", i);
            }
            println!(); // Newline after the sequence
        },
        _ => {
            // For k < 0, output "3" followed by three integers
            println!("3");
            for i in 0..3 {
                print!("{} ", i);
            }
            println!(); // Newline after the sequence
        },
    }
}
