use std::io::{self, Write};

fn factorial(n: u64) -> u64 {
    // Calculate factorial iteratively to avoid stack overflow on large inputs
    let mut result = 1;
    for i in 1..=n {
        result = result.checked_mul(i).expect("Overflow occurred while calculating factorial");
    }
    result
}

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is shown before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a non-negative integer
    let n: u64 = input.trim().parse().expect("Input must be a non-negative integer");

    // Calculate and print the factorial
    let result = factorial(n);
    println!("{}", result);
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads a line from standard input and attempts to parse it as a `u64`. If parsing fails, it will panic, which aligns with the specification's undefined behavior for invalid inputs.
2. **Factorial Calculation**: The factorial is calculated iteratively to prevent stack overflow for large inputs. The use of `checked_mul` ensures that if an overflow occurs, the program will panic, which is consistent with the undefined behavior specified.
3. **Output**: The result is printed directly, adhering to the requirement of returning the factorial as an integer value.
4. **Non-negative Integer**: The use of `u64` ensures that only non-negative integers are handled, as specified.
