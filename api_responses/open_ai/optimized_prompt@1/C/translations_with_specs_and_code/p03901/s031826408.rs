use std::io::{self, BufRead, Write};

static mut SD_RATE: f64 = 0.0; // Global variable for the probability rate

// Function to compute the expected value based on the position
fn get_val(pos: i32) -> f64 {
    // Even position
    if pos % 2 == 0 {
        return (pos as f64 / 2.0) / unsafe { SD_RATE };
    }

    // Odd position
    let ld_val = 1.0
        + get_val(pos - 1) * unsafe { SD_RATE }
        + get_val(pos + 1) * (1.0 - unsafe { SD_RATE });

    ld_val
}

// Main execution function
fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read position
    let li_pos: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    // Read probability rate
    let rate_line: String = lines.next().unwrap().unwrap();
    let rate: f64 = rate_line.parse().unwrap();
    unsafe {
        SD_RATE = rate / 100.0; // Convert percentage to a fraction
    }

    // Calculate expected value
    let ld_val = get_val(li_pos);

    // Output the result formatted to seven decimal places
    let output = format!("{:.7}", ld_val);
    println!("{}", output);
}
```

### Notes:
1. **Global Variable**: The `SD_RATE` is declared as a mutable static variable to mimic the behavior of the C code. This is generally discouraged in Rust due to safety concerns, but it was necessary to maintain the original structure.
2. **Input Handling**: The input is read using `io::stdin()` and processed line by line, which aligns with the C code's use of `fgets`.
3. **Output Formatting**: The output is formatted to seven decimal places using `format!`, ensuring compliance with the specification.
4. **Error Handling**: The Rust code assumes valid input as per the specification. If invalid input is provided, it will panic due to the `unwrap()` calls. This behavior is consistent with the C code's undefined behavior on invalid input.
5. **Recursion**: The recursive function `get_val` is directly translated from the C code, preserving the logic for calculating the expected value based on the position.
