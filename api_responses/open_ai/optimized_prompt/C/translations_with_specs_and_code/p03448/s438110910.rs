use std::io;

fn main() {
    let mut input = String::new();
    
    // Read inputs A, B, C, and X
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();
    
    let a: usize = inputs.next().unwrap().parse().expect("Invalid input for A");
    let b: usize = inputs.next().unwrap().parse().expect("Invalid input for B");
    let c: usize = inputs.next().unwrap().parse().expect("Invalid input for C");
    let x: usize = inputs.next().unwrap().parse().expect("Invalid input for X");
    
    let mut count = 0;

    // Iterate through all combinations of coins
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    // Output the count of valid combinations
    println!("{}", count);
}
