use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap is safe here as per the spec, no error handling required
        let mut nums = line.split_whitespace();

        // Read two integers from the input
        let x: i32 = match nums.next().unwrap().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, we skip to the next line
        };
        let h: i32 = match nums.next().unwrap().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, we skip to the next line
        };

        // Termination condition: if the sum of x and h is zero, we stop processing
        if x + h == 0 {
            break;
        }

        // Calculate the result using the specified formula
        let result = ((x * x + 4 * h * h) as f64).sqrt() * (x as f64) + (x * x) as f64;

        // Output the result
        println!("{}", result);
    }
}
