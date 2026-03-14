use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                // Split the input by comma
                let parts: Vec<&str> = input.split(',').collect();
                if parts.len() == 3 {
                    // Attempt to parse the identifier, weight, and height
                    if let (Ok(num), Ok(weight), Ok(height)) = (
                        parts[0].parse::<i32>(),
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>(),
                    ) {
                        // Check for positive weight and height
                        if weight > 0.0 && height > 0.0 {
                            // Calculate BMI
                            let bmi = weight / (height * height);
                            // Output the identifier if BMI is 25 or higher
                            if bmi >= 25.0 {
                                println!("{}", num);
                            }
                        }
                    }
                }
                // If the input format is incorrect or values are invalid, we simply continue
            }
            Err(_) => {
                // Handle any potential errors in reading lines, but continue processing
                continue;
            }
        }
    }
}
