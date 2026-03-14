use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        // Prompt for input
        print!("Enter an integer (-1 to terminate): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        // Read input
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input.");
            continue; // Continue to prompt for input on error
        }

        // Parse input as an integer
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid integer.");
                continue; // Continue to prompt for input on invalid input
            }
        };

        // Check for termination signal
        if number == -1 {
            break; // Exit the loop on termination signal
        }

        // Calculate the cube root
        let cube_root = cube_root(number);
        // Output the result
        println!("Cube root of {}: {:.6}", number, cube_root);
    }
}

// Function to compute the cube root of an integer
fn cube_root(n: i32) -> f64 {
    // Using f64::from to convert the integer to a floating-point number
    // and then calculating the cube root using the pow function
    (f64::from(n)).powf(1.0 / 3.0)
}
```

### Explanation of Choices:
- **Input Handling**: The program continuously prompts for input until `-1` is received, as specified. Invalid inputs are handled by prompting again without crashing.
- **Cube Root Calculation**: The cube root is calculated using the `powf` method, which is idiomatic in Rust for floating-point exponentiation.
- **Output Formatting**: The output is formatted to six decimal places to ensure precision, as implied by the specification.
- **Error Handling**: The program does not handle invalid types explicitly beyond prompting for valid integers, in line with the specification's requirements.
