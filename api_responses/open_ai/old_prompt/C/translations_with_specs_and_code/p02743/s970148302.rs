use std::io::{self, Write};

fn main() {
    // Read three long double values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into three f64 values (Rust's closest equivalent to C's long double)
    let values: Vec<&str> = input.trim().split_whitespace().collect();
    if values.len() != 3 {
        eprintln!("Expected three input values.");
        return;
    }

    let a: f64 = values[0].parse().expect("Invalid input for a");
    let b: f64 = values[1].parse().expect("Invalid input for b");
    let c: f64 = values[2].parse().expect("Invalid input for c");

    // Calculate the difference
    let d = c - a - b;

    // Check the condition and print the result
    if d * d > 4.0 * a * b && d > 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
```

### Notes:
1. **Data Types**: In Rust, `f64` is used as the closest equivalent to C's `long double`. This is a safe choice as `f64` provides sufficient precision for most applications.
2. **Input Handling**: The program reads a single line of input and splits it into three parts. It checks for the correct number of inputs and handles parsing errors gracefully.
3. **Output**: The output format is preserved exactly as specified, printing "Yes" or "No" based on the condition.
4. **Error Handling**: The program does not handle invalid input gracefully beyond printing an error message and exiting, as per the specification's lack of defined behavior for invalid input.
5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
