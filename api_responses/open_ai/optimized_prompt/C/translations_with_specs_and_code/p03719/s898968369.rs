use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the three integers from the input
    let mut nums = input.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
    
    // Unpack the integers into a, b, and c
    let a = nums[0];
    let b = nums[1];
    let c = nums[2];

    // Check if c is within the inclusive range [a, b]
    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` to read input, which is idiomatic for handling standard input in Rust. It reads a line, splits it into whitespace-separated values, and parses them into integers.
  
2. **Variable Types**: The integers are parsed as `i32`, which is a common choice for integer types in Rust. This aligns with the C code's use of `int`.

3. **Control Flow**: The conditional check (`if a <= c && c <= b`) directly mirrors the logic in the C code, ensuring that the semantics are preserved.

4. **Output**: The output is handled using `println!`, which is the standard way to print to the console in Rust, ensuring that the output format matches the specification.

5. **Error Handling**: The code assumes valid input as specified, using `unwrap()` to handle potential parsing errors. This is acceptable given the specification's precondition that input will always be valid integers.

6. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees while maintaining the intended functionality.
