use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut sum: i64 = 0; // Using i64 to match long long int in C
    let mut num_sum: i32 = 0; // Using i32 to match int in C
    let mut times: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();
        
        if parts.len() != 2 {
            continue; // Skip lines that do not have exactly two parts
        }

        let prise: i32 = match parts[0].trim().parse() {
            Ok(p) => p,
            Err(_) => continue, // Skip invalid input
        };
        
        let num: i32 = match parts[1].trim().parse() {
            Ok(n) => n,
            Err(_) => continue, // Skip invalid input
        };

        sum += prise as i64 * num as i64; // Preserve integer multiplication semantics
        num_sum += num;
        times += 1;
    }

    // Handle division by zero case to avoid panic
    let average = if times > 0 {
        num_sum as f64 / times as f64
    } else {
        0.0 // If no valid input was processed, average is 0
    };

    println!("{}\n{:.0}", sum, average);
}
