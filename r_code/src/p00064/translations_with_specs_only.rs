use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut total_sum = 0;

    // Read lines from standard input until EOF
    for line in reader.lines() {
        // Unwrap the line to handle the Result type
        let line = line.unwrap();
        
        // Extract numeric values from the line
        for word in line.split_whitespace() {
            // Attempt to parse each word as an integer
            if let Ok(num) = word.parse::<i32>() {
                total_sum += num; // Accumulate the sum
            }
        }
    }

    // Output the total sum
    println!("{}", total_sum);
}
```

### Explanation of Choices:
- **Input Handling**: The program reads from standard input using `BufRead`, which allows for efficient reading of lines until EOF.
- **String Processing**: Each line is split into words, and each word is attempted to be parsed as an integer. This aligns with the requirement to extract numeric values from alphanumeric strings.
- **Error Handling**: The program assumes valid input as per the specification, so it uses `unwrap()` to handle the result of reading lines. This is acceptable since the specification states that input will be provided correctly.
- **Sum Calculation**: The sum is maintained in a single variable (`total_sum`), which reflects the cumulative total of all extracted numbers.
- **Output**: The final sum is printed in a single operation after processing all input, adhering to the postconditions specified.
