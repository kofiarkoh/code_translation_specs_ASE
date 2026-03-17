// main.rs

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into two long integers
    let mut iter = input.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i64 = iter.next().unwrap().parse().expect("Failed to parse b");

    // Loop from 1 to b + 1
    for i in 1..=b + 1 {
        // Check if (a * i) % b == 0
        if (a * i) % b == 0 {
            // Print the result
            println!("{}", a * i);
            break; // Exit the loop after the first match
        }
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: Used `io::stdin().read_line()` to read input, which is idiomatic in Rust for reading from standard input.
2. **Parsing**: Split the input string and parsed the values into `i64`, which is the Rust equivalent of `long int` in C.
3. **Looping**: Used a range with `1..=b + 1` to replicate the C for-loop behavior.
4. **Arithmetic and Modulus**: The arithmetic operations and modulus are directly translated, preserving the semantics of the original C code.
5. **Output**: Used `println!` for output, which matches the behavior of `printf` in C, ensuring the format is preserved.
6. **Error Handling**: Added basic error handling for input parsing to ensure robustness, which is a common practice in Rust.
