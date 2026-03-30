use std::io::{self, BufRead};

static mut SD_RATE: f64 = 0.0; // Global variable for probability rate

// Function to output a line
fn output_line(line: &str) {
    print!("{}", line);
}

// Recursive function to calculate expected value
fn get_val(pos: i32) -> f64 {
    // Even position
    if pos % 2 == 0 {
        return (pos as f64 / 2.0) / unsafe { SD_RATE };
    }

    // Odd position
    let mut val = 1.0;
    val += get_val(pos - 1) * unsafe { SD_RATE };
    val += get_val(pos + 1) * (1.0 - unsafe { SD_RATE });

    val
}

// Main execution function
fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read position
    let li_pos: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read probability rate
    let rate_line: String = lines.next().unwrap().unwrap();
    let rate: f64 = rate_line.trim().parse().unwrap();
    unsafe {
        SD_RATE = rate / 100.0; // Convert percentage to decimal
    }

    // Calculate expected value
    let ld_val = get_val(li_pos);

    // Output result formatted to seven decimal places
    let output = format!("{:.7}\n", ld_val);
    output_line(&output);
}
``` 

### Notes:
1. **Global Variable**: The `SD_RATE` is declared as a mutable static variable to mimic the C code's global variable behavior. This is generally discouraged in Rust, but it was necessary to maintain the original structure.
2. **Input Handling**: The input is read using `io::stdin()` and processed line by line, similar to the C code's `fgets`.
3. **Error Handling**: The code assumes valid input as per the specification. In a production scenario, additional error handling would be advisable.
4. **Output Formatting**: The output is formatted to seven decimal places using Rust's `format!` macro, ensuring compliance with the specification.
5. **Recursion**: The recursive function `get_val` directly translates the logic from the C code, preserving the expected behavior for even and odd positions.
