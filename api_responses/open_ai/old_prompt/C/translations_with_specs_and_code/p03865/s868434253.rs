use std::io::{self, Read};

fn main() {
    // Create a buffer to hold up to 114514 characters
    let mut buf = [0u8; 114514];
    
    // Read from standard input
    let n = io::stdin().read(&mut buf).expect("Failed to read from stdin");
    
    // Check if there is at least one character
    if n == 0 {
        // If no input is provided, we do not output anything as per the spec
        return;
    }

    // Determine the first and last characters
    let first_char = buf[0] as char;
    let last_char = buf[n - 1] as char;

    // Compare the first and last characters and output the result
    if first_char == last_char {
        println!("First");
    } else {
        println!("Second");
    }
}
```

### Explanation of Translation Choices:
1. **Buffer Initialization**: The buffer is initialized to hold up to 114514 characters, matching the C code's intent.
2. **Reading Input**: The `read` method is used to read from standard input, and the number of bytes read is stored in `n`.
3. **Input Validation**: The code checks if `n` is zero to handle the case of no input gracefully. The specification does not define behavior for empty input, so we simply return without outputting anything.
4. **Character Comparison**: The first and last characters are compared, and the appropriate output is printed based on their equality.
5. **Output Formatting**: The output is formatted as specified, with no additional formatting or whitespace. 

This Rust code preserves the semantics of the original C code while adhering to the provided specification.
