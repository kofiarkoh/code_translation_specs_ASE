use std::io;

fn main() {
    // Read input values A, B, C, and X
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let A: usize = input.trim().parse().expect("Invalid input for A");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let B: usize = input.trim().parse().expect("Invalid input for B");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let C: usize = input.trim().parse().expect("Invalid input for C");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let X: i32 = input.trim().parse().expect("Invalid input for X");

    // Initialize the count of valid combinations
    let mut count = 0;

    // Iterate through all combinations of i, j, and k
    for i in 0..=A {
        for j in 0..=B {
            for k in 0..=C {
                // Check if the weighted sum equals X
                if 500 * i as i32 + 100 * j as i32 + 50 * k as i32 == X {
                    count += 1; // Increment count for valid combination
                }
            }
        }
    }

    // Output the count of valid combinations
    println!("{}", count);
}
