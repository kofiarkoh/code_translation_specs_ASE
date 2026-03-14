use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Handle potential I/O errors
        let line = match line {
            Ok(l) => l,
            Err(_) => continue, // Skip lines that can't be read
        };

        // Split the line by comma and parse the values
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            continue; // Skip lines that don't have exactly 3 parts
        }

        // Parse the integer and two doubles
        let num: i32 = match parts[0].trim().parse() {
            Ok(n) => n,
            Err(_) => continue, // Skip if parsing fails
        };

        let w: f64 = match parts[1].trim().parse() {
            Ok(weight) => weight,
            Err(_) => continue, // Skip if parsing fails
        };

        let t: f64 = match parts[2].trim().parse() {
            Ok(height) => height,
            Err(_) => continue, // Skip if parsing fails
        };

        // Calculate BMI
        let bmi = w / (t * t);

        // Print the number if BMI is 25 or greater
        if bmi >= 25.0 {
            println!("{}", num);
        }
    }
}
