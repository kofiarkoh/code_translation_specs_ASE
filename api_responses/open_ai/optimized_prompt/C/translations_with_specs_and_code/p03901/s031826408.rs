use std::io::{self, BufRead};
use std::fmt;

static mut SD_RATE: f64 = 0.0; // Global variable for probability rate

// Function to calculate expected value
fn get_val(pos: i32) -> f64 {
    // Even case
    if pos % 2 == 0 {
        return (pos as f64 / 2.0) / unsafe { SD_RATE };
    }

    // Odd case
    let ld_val = 1.0
        + get_val(pos - 1) * unsafe { SD_RATE }
        + get_val(pos + 1) * (1.0 - unsafe { SD_RATE });

    ld_val
}

// Function to read input and compute expected value
fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut lines = handle.lines();
    
    // Read position
    let li_pos: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read probability rate
    let rate_line: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    unsafe {
        SD_RATE = rate_line / 100.0; // Convert percentage to a fraction
    }

    // Calculate expected value
    let ld_val = get_val(li_pos);

    // Output the result formatted to seven decimal places
    println!("{:.7}", ld_val);
}
```

### Notes:
1. **Global Variable**: The `SD_RATE` is declared as a mutable static variable to mimic the behavior of the C code. This is generally discouraged in Rust, but it was necessary to maintain the same structure as the original C code.
2. **Input Handling**: The input is read using `io::stdin()` and processed line by line, which is idiomatic in Rust.
3. **Error Handling**: The code assumes valid input as per the specification. If invalid input is provided, it will panic due to `unwrap()`. This aligns with the specification's note on undefined behavior for incorrect input.
4. **Output Formatting**: The output is formatted to seven decimal places using Rust's formatting capabilities, ensuring it matches the specification.
5. **Recursion**: The `get_val` function is implemented recursively, similar to the C version, preserving the logic for calculating expected values based on the position and probability rate.
