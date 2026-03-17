use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of pairs
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Expected a positive integer");

    // Ensure n is positive as per the specification
    if n == 0 {
        return; // No pairs to process
    }

    for _ in 0..n {
        // Read each pair of integers
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let x: i32 = nums.next().unwrap().parse().expect("Expected an integer");
        let y: i32 = nums.next().unwrap().parse().expect("Expected an integer");

        // Calculate the output based on the specification
        let output = (x - 1) % y + 1;

        // Print the output
        println!("{}", output);
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` for reading input, which is idiomatic and safe. It reads all input lines and processes them accordingly.
2. **Error Handling**: The code uses `expect` to handle potential parsing errors, ensuring that the program will panic with a clear message if the input is not as expected.
3. **Looping and Output**: The loop iterates `n` times, processing each pair of integers and calculating the output as specified.
4. **Output Calculation**: The output calculation `(x - 1) % y + 1` is directly translated from the C code, preserving the intended behavior.
5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees. 

This translation maintains the semantics of the original C code while ensuring it adheres to the provided specification.
