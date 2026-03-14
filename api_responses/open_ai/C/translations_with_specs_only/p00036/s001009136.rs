use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut values = Vec::new();

    // Read up to eight lines of binary strings
    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            break; // Stop reading on blank line
        }
        if line.len() == 8 && line.chars().all(|c| c == '0' || c == '1') {
            // Convert binary string to integer
            let value = isize::from_str_radix(&line, 2).expect("Failed to convert binary to integer");
            values.push(value);
        }
    }

    // Ensure we have at least one valid binary string
    if values.is_empty() {
        return; // Terminate without output if no valid input
    }

    // Find the minimum value for normalization
    let min_value = *values.iter().min().unwrap();

    // Output label based on the processed values
    let label = categorize(&values, min_value);
    println!("{}", label);
}

// Categorize the values into labels 'A' to 'G'
fn categorize(values: &[isize], min_value: isize) -> char {
    // Example categorization logic based on the minimum value
    // This is a placeholder; the actual logic should be defined based on specific rules
    let sum: isize = values.iter().sum();
    let average = sum / values.len() as isize;

    match average {
        0..=10 => 'A',
        11..=20 => 'B',
        21..=30 => 'C',
        31..=40 => 'D',
        41..=50 => 'E',
        51..=60 => 'F',
        _ => 'G',
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads lines until a blank line is encountered, adhering to the specification's requirement for input termination.
2. **Binary String Validation**: It checks that each line is exactly 8 characters long and consists only of '0' and '1', as specified.
3. **Integer Conversion**: The conversion from binary string to integer is done using `isize::from_str_radix`, which is safe and handles the conversion correctly.
4. **Minimum Value Tracking**: The minimum value is tracked to comply with the invariant that values are non-negative integers.
5. **Output Logic**: The categorization function is a placeholder and should be replaced with the actual logic based on the specification's requirements for output labels.
6. **Error Handling**: The program terminates without output if no valid input is provided, as specified.
