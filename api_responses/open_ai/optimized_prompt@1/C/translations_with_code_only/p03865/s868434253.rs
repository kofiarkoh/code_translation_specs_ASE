use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 114514]; // Buffer to hold the input
    let n = io::stdin().read(&mut buf).unwrap_or(0); // Read from stdin

    let n = n.saturating_sub(1); // Decrement n, ensuring it doesn't go below 0

    // Check if the first and last character (if n > 0) are the same
    let result = if n > 0 && buf[0] == buf[n as usize] {
        "First"
    } else {
        "Second"
    };

    // Print the result
    println!("{}", result);
}
```

### Explanation of Translation Choices:
1. **Buffer Declaration**: The buffer is declared as a fixed-size array of `u8`, similar to the C code.
2. **Reading Input**: The `read` function is used to read from standard input, and `unwrap_or(0)` is used to handle any potential errors by defaulting `n` to 0.
3. **Decrementing `n`**: The `saturating_sub(1)` method is used to decrement `n` safely, ensuring it does not go below 0, which mimics the behavior of the original C code.
4. **Conditional Check**: The condition checks if `n > 0` to avoid accessing out of bounds when comparing `buf[0]` and `buf[n]`.
5. **Output**: The result is printed using `println!`, which is idiomatic in Rust for formatted output. 

This translation preserves the semantics of the original C code while adhering to Rust's safety and idiomatic practices.
