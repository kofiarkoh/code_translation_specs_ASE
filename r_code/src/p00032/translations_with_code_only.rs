use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tyou = 0;
    let mut hisi = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 3 {
            continue; // Skip lines that do not have exactly three parts
        }

        // Parse the integers, using `parse` which will handle errors
        let a: i32 = match parts[0].trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };
        let b: i32 = match parts[1].trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };
        let c: i32 = match parts[2].trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };

        // Check the conditions as per the original C code
        if c * c == a * a + b * b {
            tyou += 1;
        }
        if a == b {
            hisi += 1;
        }
    }

    // Print the results as per the original C code
    println!("{}\n{}", tyou, hisi);
}
