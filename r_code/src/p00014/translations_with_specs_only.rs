use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                // Attempt to parse the input as a positive integer
                if let Ok(n) = input.trim().parse::<u32>() {
                    // Calculate the weighted sum of squares of multiples of n
                    let sum_of_squares: u32 = (n..=600)
                        .filter(|&x| x % n == 0) // Filter multiples of n
                        .map(|x| x * x) // Square the multiples
                        .sum(); // Sum the squares

                    // Output the result
                    println!("{}", n * sum_of_squares);
                }
                // If parsing fails, we ignore the input as per the specification
            }
            Err(_) => {
                // Handle any potential errors in reading lines, but continue processing
                continue;
            }
        }
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input until EOF, processing each line individually. Invalid inputs are ignored, as specified.
2. **Integer Parsing**: We use `u32` to ensure that only positive integers are processed, adhering to the precondition.
3. **Sum of Squares Calculation**: The range is filtered to include only multiples of the input integer, and their squares are summed. This preserves the semantics of the specification.
4. **Output**: The result is printed immediately after computation for each valid input.
5. **Error Handling**: Invalid inputs are simply ignored, which aligns with the specification's requirements.
