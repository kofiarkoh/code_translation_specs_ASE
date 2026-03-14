use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                // Attempt to parse the input as an integer
                if let Ok(n) = input.trim().parse::<i32>() {
                    // Calculate the sum of integers from 1 to n
                    // Using the formula n * (n + 1) / 2
                    let sum = n * (n + 1) / 2;
                    // Add 1 to the result
                    let result = sum + 1;
                    // Output the result
                    println!("{}", result);
                } else {
                    // If parsing fails, terminate without output as per specification
                    return;
                }
            }
            Err(_) => {
                // If reading the line fails, terminate without output as per specification
                return;
            }
        }
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input until EOF, which is consistent with the specification.
2. **Integer Parsing**: The program attempts to parse each line as an `i32`. If parsing fails, it terminates without output, adhering to the specification's error handling.
3. **Sum Calculation**: The sum of integers from 1 to `n` is computed using the formula `n * (n + 1) / 2`, which is efficient and avoids looping.
4. **Output**: Each result is printed immediately after computation, maintaining the linear time complexity requirement.
5. **Graceful Termination**: The program exits without output on invalid input, as specified.
