use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 114514]; // Buffer to hold the input
    let n = io::stdin().read(&mut buf).unwrap_or(0); // Read from stdin

    let n = n.saturating_sub(1); // Decrement n, ensuring it doesn't go below 0

    // Check if the first and last character (if n > 0) are the same
    let first_last_equal = n > 0 && buf[0] == buf[n as usize - 1];

    // Print "First" if they are equal, otherwise "Second"
    if first_last_equal {
        println!("First");
    } else {
        println!("Second");
    }
}
```

### Explanation of Translation Choices:
1. **Buffer Declaration**: The buffer is declared as a fixed-size array of `u8`, which is the equivalent of `char` in C.
2. **Reading Input**: The `read` function is used to read from standard input, and `unwrap_or(0)` is used to handle any potential errors by defaulting `n` to 0.
3. **Decrementing `n`**: The `saturating_sub(1)` method is used to ensure that `n` does not go below 0, which mimics the behavior of the original C code.
4. **Comparison Logic**: The comparison of the first and last characters is done with a check to ensure `n` is greater than 0 to avoid out-of-bounds access.
5. **Output**: The output is handled with `println!`, which automatically adds a newline, preserving the output format.
