use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the three integers from the input
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");
    let c: i32 = iter.next().unwrap().parse().expect("Failed to parse c");

    // Calculate b / a, using integer division
    let division_result = b / a;

    // Check the condition and print the appropriate result
    if division_result <= c {
        println!("{}", division_result);
    } else {
        println!("{}", c);
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The C code uses `scanf` for input, which is replaced by reading a line from stdin in Rust. This approach captures the entire line and splits it into parts, which is idiomatic in Rust.
2. **Integer Division**: The division `b / a` is performed directly in Rust, which behaves the same as in C for integers. Rust will panic on division by zero, which is a safe behavior compared to C's undefined behavior.
3. **Output**: The `printf` function is replaced with `println!`, which formats the output similarly to C's `printf` with a newline.
4. **Error Handling**: The Rust code includes error handling for parsing integers, which is a safe practice. The C code does not handle input errors, so this is a slight deviation from the original behavior, but it ensures the program does not panic unexpectedly.
5. **Semantics Preservation**: The logic and flow of the program are preserved, ensuring that the output matches the expected behavior of the original C code.
