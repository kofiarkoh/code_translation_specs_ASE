use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut parts = input.split_whitespace();
    let n: i64 = parts.next().unwrap().parse().expect("Invalid input for N");
    let mut a: i32 = parts.next().unwrap().parse().expect("Invalid input for A");
    let mut b: i32 = parts.next().unwrap().parse().expect("Invalid input for B");

    let n = (n % 12).max(0); // Ensure N is non-negative and within the range [0, 11]

    for i in 0..n {
        a -= b; // A = A - B
        if i + 1 < n {
            b += a; // B = A + B
        }
    }

    // Output the final values of A and B
    println!("{} {}", a, b);
}
