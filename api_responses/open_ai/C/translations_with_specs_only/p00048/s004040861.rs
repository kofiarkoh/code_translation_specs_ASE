use std::io::{self, BufRead};

fn main() {
    // Define weight class boundaries and their corresponding names
    let weight_classes = [
        (0.0, 50.0, "Lightweight"),
        (50.0, 100.0, "Middleweight"),
        (100.0, 150.0, "Heavyweight"),
        (150.0, f64::INFINITY, "Super Heavyweight"),
    ];

    // Read from standard input until EOF
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Unwrap the line to get the string value
        let line = line.unwrap();
        
        // Attempt to parse the line as a double-precision floating-point value
        if let Ok(weight) = line.parse::<f64>() {
            // Determine the weight class based on the parsed weight
            let class_name = weight_classes.iter()
                .find(|&&(min, max, _)| weight >= min && weight < max)
                .map(|&(_, _, name)| name)
                .unwrap_or("Unknown"); // Fallback in case of no match (should not happen per spec)

            // Output the corresponding weight class name
            println!("{}", class_name);
        }
        // If parsing fails, the behavior is undefined as per the specification
    }
}
