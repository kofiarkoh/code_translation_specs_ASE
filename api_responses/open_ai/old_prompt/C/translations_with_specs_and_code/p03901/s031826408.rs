use std::io::{self, BufRead};

static mut SD_RATE: f64 = 0.0; // Global variable for the probability rate

// Function to output a line
fn output_line(line: &str) {
    print!("{}", line);
}

// Recursive function to calculate expected value
fn get_val(pos: i32) -> f64 {
    // Even case
    if pos % 2 == 0 {
        return (pos as f64 / 2.0) / unsafe { SD_RATE };
    }

    // Odd case
    let mut value = 1.0;
    value += get_val(pos - 1) * unsafe { SD_RATE };
    value += get_val(pos + 1) * (1.0 - unsafe { SD_RATE });

    value
}

// Main execution function
fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Get position
    let li_pos: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Get probability rate
    let sd_rate: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    unsafe {
        SD_RATE = sd_rate / 100.0; // Convert percentage to fraction
    }

    // Calculate expected value
    let expected_value = get_val(li_pos);

    // Output the result formatted to seven decimal places
    let output = format!("{:.7}\n", expected_value);
    output_line(&output);
}
```

### Notes:
1. **Global Variable**: The `SD_RATE` is declared as a mutable static variable to mimic the C code's global variable behavior. This is generally discouraged in Rust, but it was necessary to maintain the original structure.
2. **Input Handling**: The input is read using `io::stdin()` and processed line by line, which is idiomatic in Rust.
3. **Error Handling**: The code assumes valid input as per the specification. In a production scenario, you would want to handle potential parsing errors.
4. **Output Formatting**: The output is formatted to seven decimal places using Rust's `format!` macro, ensuring it matches the specification.
5. **Recursion**: The recursive function `get_val` is directly translated from the C code, preserving its logic and structure.
