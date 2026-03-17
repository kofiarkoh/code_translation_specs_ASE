use std::io::{self, Write};

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input values as long doubles (f64 in Rust)
    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let a: f64 = values[0].parse().expect("Invalid input for a");
    let b: f64 = values[1].parse().expect("Invalid input for b");
    let c: f64 = values[2].parse().expect("Invalid input for c");

    // Calculate d
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
1. **Input Handling**: The Rust code reads a single line of input and splits it into three parts, which are then parsed as `f64`. This is the closest equivalent to `long double` in Rust, as Rust does not have a direct `long double` type.
2. **Mathematical Operations**: The mathematical operations and conditions are directly translated from C to Rust, preserving the semantics as specified.
3. **Output**: The output format is preserved using `println!`, which ensures that the output matches the expected "Yes" or "No" response.
4. **Error Handling**: The code includes basic error handling for input parsing, which is a common practice in Rust to ensure that the program does not panic unexpectedly.
5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to the requirement for safe, idiomatic Rust.
