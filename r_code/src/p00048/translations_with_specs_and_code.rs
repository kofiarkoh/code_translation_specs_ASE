use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                if let Ok(weight) = input.parse::<f64>() {
                    let class_name = match weight {
                        w if w <= 48.0 => "light fly",
                        w if w <= 51.0 => "fly",
                        w if w <= 54.0 => "bantam",
                        w if w <= 57.0 => "feather",
                        w if w <= 60.0 => "light",
                        w if w <= 64.0 => "light welter",
                        w if w <= 69.0 => "welter",
                        w if w <= 75.0 => "light middle",
                        w if w <= 81.0 => "middle",
                        w if w <= 91.0 => "light heavy",
                        _ => "heavy",
                    };
                    println!("{}", class_name);
                }
                // If the input cannot be parsed as a f64, the behavior is not defined as per the spec.
            }
            Err(_) => {
                // Handle potential read errors, but per the spec, we do not define behavior for invalid input.
            }
        }
    }
}
