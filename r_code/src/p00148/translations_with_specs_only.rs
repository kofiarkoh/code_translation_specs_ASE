use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    for line in reader.lines() {
        // Unwrap the line to get the string value
        let line = line.unwrap();
        
        // Attempt to parse the line as an integer
        if let Ok(value) = line.trim().parse::<i32>() {
            // Process the value using modulo to ensure it falls within the range 1 to 39
            let processed_value = (value % 39 + 39) % 39; // Ensure non-negative result
            let final_value = if processed_value == 0 { 39 } else { processed_value };

            // Output the formatted string
            println!("3C{:02}", final_value);
        }
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input until EOF, as specified. It uses a buffered reader for efficiency.
2. **Integer Parsing**: The program assumes valid integer input as per the specification. It does not handle invalid formats, consistent with the requirements.
3. **Modulo Operation**: The modulo operation ensures the processed integer is within the range of 1 to 39. The expression `(value % 39 + 39) % 39` ensures that negative values are wrapped correctly, and if the result is 0, it is adjusted to 39.
4. **Output Formatting**: The output is formatted as specified, using `println!` with zero-padding for two digits.
5. **Graceful Termination**: The program naturally terminates upon reaching the end of input, as required.
