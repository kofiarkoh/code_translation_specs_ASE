fn main() {
    use std::io::{self, Write};

    // Read the number of integers, N
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is shown before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Ensure N is a positive integer
    if n == 0 {
        // If N is zero, we may exhibit undefined behavior as per the specification
        return;
    }

    // Create a vector to hold the integers
    let mut a = Vec::with_capacity(n);

    // Read N integers into the vector
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let value: i32 = input.trim().parse().expect("Invalid input for integer");
        a.push(value);
    }

    // Calculate the sum of each integer minus one
    let mut ans = 0;
    for &value in &a {
        ans += value - 1; // Each integer is reduced by one
    }

    // Print the result
    println!("{}", ans);
}
