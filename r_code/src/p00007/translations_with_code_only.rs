use std::io;

fn main() {
    let mut a = 100_000; // Initialize a to 100000
    let mut n = String::new();

    // Read input from stdin
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        a = (a as f64 * 1.05) as i32; // Increase a by 5%
        if a % 1000 != 0 {
            a = (a / 1000 + 1) * 1000; // Round up to the next thousand
        }
    }

    // Print the result
    println!("{}", a);
}
