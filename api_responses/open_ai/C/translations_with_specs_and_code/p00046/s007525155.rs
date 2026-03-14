use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut max_value: Option<f32> = None;
    let mut min_value: Option<f32> = None;

    for line in stdin.lock().lines() {
        // Read each line and parse it as a floating-point number
        if let Ok(input) = line {
            if let Ok(value) = input.trim().parse::<f32>() {
                // Initialize max and min values on the first valid input
                if max_value.is_none() && min_value.is_none() {
                    max_value = Some(value);
                    min_value = Some(value);
                } else {
                    // Update max and min values accordingly
                    if let Some(max) = max_value {
                        if value > max {
                            max_value = Some(value);
                        }
                    }
                    if let Some(min) = min_value {
                        if value < min {
                            min_value = Some(value);
                        }
                    }
                }
            }
        }
    }

    // Ensure at least one value was processed
    if let (Some(max), Some(min)) = (max_value, min_value) {
        // Calculate the difference and print it formatted to one decimal place
        println!("{:.1}", max - min);
    } else {
        // Behavior is not defined if no input is provided
        // No output is produced in this case as per the specification
    }
}
