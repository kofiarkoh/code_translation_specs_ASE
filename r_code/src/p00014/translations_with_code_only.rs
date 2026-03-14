use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        let n: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };

        let mut sum: i64 = 0; // Use i64 to match long long int in C
        let upper_limit = 600 - n;

        for i in (n..=upper_limit).step_by(n as usize) {
            sum += (i as i64) * (i as i64); // Ensure multiplication is done in i64
        }

        sum *= n as i64; // Multiply sum by n, ensuring n is treated as i64
        println!("{}", sum); // Print the result
    }
}
