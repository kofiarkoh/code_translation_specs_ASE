use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(value) => {
                // Attempt to parse the input as a floating-point number
                if let Ok(v) = value.trim().parse::<f32>() {
                    // Compute the integer output based on the specified formula
                    let n = ((4.9 * v * v / 9.8 / 9.8) + 5.0) / 5.0;
                    let result = n.floor() as i32; // Ensure we round down
                    println!("{}", result);
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
}
