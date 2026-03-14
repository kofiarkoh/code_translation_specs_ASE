use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                if let Some((id, weight, height)) = parse_input(&input) {
                    if let Some(bmi) = calculate_bmi(weight, height) {
                        if bmi >= 25.0 {
                            println!("{}", id);
                        }
                    }
                }
            }
            Err(_) => continue, // Ignore any read errors and continue processing
        }
    }
}

/// Parses the input line into an identifier, weight, and height.
/// Returns None if the input format is incorrect or values are invalid.
fn parse_input(input: &str) -> Option<(i32, f64, f64)> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 3 {
        return None; // Invalid format
    }

    let id = parts[0].parse::<i32>().ok()?;
    let weight = parts[1].parse::<f64>().ok()?;
    let height = parts[2].parse::<f64>().ok()?;

    // Check for positive weight and height
    if weight <= 0.0 || height <= 0.0 {
        return None; // Invalid values
    }

    Some((id, weight, height))
}

/// Calculates the Body Mass Index (BMI) given weight and height.
/// Returns None if height is zero to avoid division by zero.
fn calculate_bmi(weight: f64, height: f64) -> Option<f64> {
    if height == 0.0 {
        return None; // Avoid division by zero
    }
    Some(weight / (height * height))
}
